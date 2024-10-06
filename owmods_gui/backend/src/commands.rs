use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

use anyhow::{anyhow, Context};
use log::error;
use owmods_core::{
    alerts::{fetch_alert, Alert},
    analytics::{send_analytics_event, AnalyticsEventName},
    config::Config,
    constants::OWML_UNIQUE_NAME,
    db::{LocalDatabase, RemoteDatabase},
    download::{
        download_and_install_owml, install_mod_from_db, install_mod_from_url, install_mod_from_zip,
        install_mods_parallel,
    },
    file::get_app_path,
    game::launch_game,
    mods::{
        local::{LocalMod, UnsafeLocalMod},
        remote::RemoteMod,
    },
    open::{open_github, open_readme, open_shortcut},
    owml::OWMLConfig,
    progress::bars::{ProgressBar, ProgressBars},
    protocol::{ProtocolPayload, ProtocolVerb},
    remove::{remove_failed_mod, remove_mod},
    socket::{LogServer, SocketMessageType},
    updates::{check_mod_needs_update, fix_version_post_update},
    validate::fix_deps,
};
use serde::Serialize;
use tauri::{async_runtime, AppHandle, DragDropEvent, Manager, WindowEvent};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tokio::{select, sync::mpsc, try_join};
use typeshare::typeshare;

use crate::RemoteDatabaseOption;
use crate::{
    error::{Error, Result},
    events::{CustomEventEmitter, CustomEventEmitterAll, Event},
    protocol::PROTOCOL_LISTENER_AMOUNT,
};
use crate::{events::CustomEventListener, game::LogData};
use crate::{
    game::{make_log_window, show_warnings, GameMessage},
    gui_config::GuiConfig,
    LogPort, State,
};
//use crate::events::CustomEventTriggerGlobal;

pub async fn mark_mod_busy(
    unique_name: &str,
    busy: bool,
    send_event: bool,
    state: &tauri::State<'_, State>,
    handle: &tauri::AppHandle,
) {
    let mut mods_in_progress = state.mods_in_progress.write().await;
    if busy {
        mods_in_progress.push(unique_name.to_string());
    } else {
        mods_in_progress.retain(|m| m != unique_name);
    }
    if send_event {
        handle.typed_emit_all(&Event::ModBusy(())).ok();
    }
}

#[tauri::command]
pub async fn initial_setup(handle: tauri::AppHandle, state: tauri::State<'_, State>) -> Result {
    let mut config = state.config.write().await;
    *config = Config::get(None).map_err(|e| {
        anyhow!(
            "Error loading Settings: {}. Please delete or fix {}",
            e,
            config.path.to_str().unwrap()
        )
    })?;
    let mut gui_config = state.gui_config.write().await;
    *gui_config = GuiConfig::get().map_err(|e| {
        anyhow!(
            "Error loading GUI Settings: {}. Please delete or fix {}",
            e,
            GuiConfig::path()
                .map(|p| p.to_str().unwrap().to_string())
                .unwrap_or("Error Loading Path".to_string())
        )
    })?;
    let event = Event::GuiConfigReload(gui_config.watch_fs);
    handle.typed_emit_all(&event).ok();
    // handle.typed_trigger_global(&event).ok();
    handle.typed_emit_all(&Event::ConfigReload(())).ok();

    Ok(())
}

pub fn sync_remote_and_local(handle: &AppHandle) -> Result {
    let handle2 = handle.clone();
    // Defer checking if a mod needs to update to prevent deadlock
    async_runtime::spawn(async move {
        let state = handle2.state::<State>();
        let mut local_db = state.local_db.write().await;
        let remote_db = state.remote_db.read().await.clone();
        if let Some(remote_db) = remote_db.get() {
            local_db.validate_updates(remote_db);
            handle2.typed_emit_all(&Event::LocalRefresh(())).ok();
        }
    });
    Ok(())
}

#[tauri::command]
pub async fn refresh_local_db(handle: tauri::AppHandle, state: tauri::State<'_, State>) -> Result {
    let conf = state.config.read().await;
    {
        let mut db = state.local_db.write().await;
        let local_db = LocalDatabase::fetch(&conf.owml_path)?;
        *db = local_db;
    }
    handle.typed_emit_all(&Event::LocalRefresh(())).ok();
    sync_remote_and_local(&handle)?;
    Ok(())
}

#[tauri::command]
pub async fn get_local_mods(
    filter: &str,
    tags: Vec<String>,
    state: tauri::State<'_, State>,
) -> Result<Vec<String>> {
    let db = state.local_db.read().await.clone();
    let mut mods: Vec<&UnsafeLocalMod> = db.all().collect();
    if filter.is_empty() {
        mods.sort_by(|a, b| {
            let name_ord = a.get_name().cmp(b.get_name());
            let errors_ord = a.get_errs().len().cmp(&b.get_errs().len()).reverse();
            let enabled_ord = a.get_enabled().cmp(&b.get_enabled()).reverse();
            errors_ord.then(enabled_ord.then(name_ord))
        });
    } else if let Some(remote_db) = state.remote_db.read().await.get() {
        mods = db.search_with_remote(filter, remote_db);
    }
    if !tags.is_empty() {
        let db_opt = state.remote_db.read().await;
        let remote_db = db_opt.get().unwrap();
        let remote_mods_matching: Vec<&str> = remote_db
            .matches_tags(tags)
            .map(|m| m.unique_name.as_str())
            .collect();
        mods.retain(|m| remote_mods_matching.contains(&m.get_unique_name().as_str()))
    }

    let first_disabled_index = mods
        .iter()
        .position(|m| matches!(m, UnsafeLocalMod::Valid(_)) && !m.get_enabled());

    let mut unique_names: Vec<String> = mods
        .into_iter()
        .map(|m| m.get_unique_name().clone())
        .collect();

    // Only way to get a separator in the list is to insert a fake mod
    if filter.is_empty() && first_disabled_index.map(|i| i > 0).unwrap_or(false) {
        if let Some(index) = first_disabled_index {
            unique_names.insert(index, "~~SEPARATOR~~".to_string());
        }
    }

    Ok(unique_names)
}

#[tauri::command]
pub async fn get_local_mod(
    unique_name: &str,
    state: tauri::State<'_, State>,
) -> Result<Option<UnsafeLocalMod>> {
    if unique_name == OWML_UNIQUE_NAME {
        let config = state.config.read().await;
        let owml = LocalDatabase::get_owml(&config.owml_path)
            .with_context(|| format!("Couldn't Find OWML at path {}", &config.owml_path))?;
        Ok(Some(UnsafeLocalMod::Valid(Box::new(owml))))
    } else {
        Ok(state
            .local_db
            .read()
            .await
            .get_mod_unsafe(unique_name)
            .cloned())
    }
}

#[tauri::command]
pub async fn refresh_remote_db(handle: tauri::AppHandle, state: tauri::State<'_, State>) -> Result {
    let conf = state.config.read().await;
    let new_db = RemoteDatabase::fetch(&conf.database_url).await;

    let first_load = {
        let mut remote_db = state.remote_db.write().await;
        let was_unloaded = remote_db.is_pending();
        *remote_db = match new_db {
            Ok(db) => RemoteDatabaseOption::Connected(Box::new(db)),
            Err(err) => {
                error!("Error Loading Remote Database: {}", err);
                RemoteDatabaseOption::Error(err.into())
            }
        };
        was_unloaded
    };

    if first_load {
        handle.typed_emit_all(&Event::RemoteInitialized(())).ok();
    }

    handle.typed_emit_all(&Event::RemoteRefresh(())).ok();
    sync_remote_and_local(&handle)?;
    Ok(())
}

#[tauri::command]
pub async fn get_remote_mods(
    filter: &str,
    tags: Vec<String>,
    state: tauri::State<'_, State>,
) -> Result<Option<Vec<String>>> {
    let db_ref = state.remote_db.read().await.clone();
    match db_ref {
        RemoteDatabaseOption::PreInit | RemoteDatabaseOption::Loading => Ok(None),
        RemoteDatabaseOption::Error(e) => Err(e),
        RemoteDatabaseOption::Connected(remote_db) => {
            let gui_config = state.gui_config.read().await.clone();
            let mut mods: Vec<&RemoteMod> = remote_db
                .mods
                .values()
                .filter(|m| m.unique_name != OWML_UNIQUE_NAME)
                .collect();
            if filter.is_empty() {
                mods.sort_by(|a, b| b.download_count.cmp(&a.download_count));
            } else {
                mods = remote_db.search(filter);
            }
            if !tags.is_empty() {
                mods = RemoteDatabase::filter_by_tags(mods.into_iter(), tags).collect();
            }
            if gui_config.hide_installed_in_remote {
                let local_db = state.local_db.read().await.clone();
                mods.retain(|m| local_db.get_mod(&m.unique_name).is_none());
            }
            if gui_config.hide_dlc {
                mods.retain(|m| !m.requires_dlc());
            }
            Ok(Some(
                mods.into_iter().map(|m| m.unique_name.clone()).collect(),
            ))
        }
    }
}

#[typeshare]
#[derive(Serialize, Clone)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum RemoteModOption {
    Loading,
    Connected(Box<Option<RemoteMod>>),
    Err(Error),
}

#[tauri::command]
pub async fn get_remote_mod(
    unique_name: &str,
    state: tauri::State<'_, State>,
) -> Result<RemoteModOption> {
    match state.remote_db.read().await.clone() {
        RemoteDatabaseOption::Connected(db) => {
            let mod_opt = if unique_name == OWML_UNIQUE_NAME {
                db.get_owml().cloned()
            } else {
                db.get_mod(unique_name).cloned()
            };
            Ok(RemoteModOption::Connected(Box::new(mod_opt)))
        }
        RemoteDatabaseOption::Loading | RemoteDatabaseOption::PreInit => {
            Ok(RemoteModOption::Loading)
        }
        RemoteDatabaseOption::Error(e) => Ok(RemoteModOption::Err(e)),
    }
}

#[tauri::command]
pub async fn open_mod_folder(unique_name: &str, state: tauri::State<'_, State>) -> Result {
    let db = state.local_db.read().await;
    let conf = state.config.read().await;
    open_shortcut(unique_name, &conf, &db)?;
    Ok(())
}

#[tauri::command]
pub async fn toggle_mod(
    unique_name: &str,
    enabled: bool,
    recursive: bool,
    state: tauri::State<'_, State>,
) -> Result<Vec<String>> {
    let db = state.local_db.read().await;
    let show_warnings_for = owmods_core::toggle::toggle_mod(unique_name, &db, enabled, recursive)?;
    Ok(show_warnings_for)
}

#[tauri::command]
pub async fn toggle_all(enabled: bool, state: tauri::State<'_, State>) -> Result<Vec<String>> {
    let local_db = state.local_db.read().await;
    let mut show_warnings_for: Vec<String> = vec![];
    for local_mod in local_db.valid() {
        show_warnings_for.extend(owmods_core::toggle::toggle_mod(
            &local_mod.manifest.unique_name,
            &local_db,
            enabled,
            false,
        )?);
    }
    Ok(show_warnings_for)
}

#[tauri::command]
pub async fn install_mod(
    unique_name: &str,
    prerelease: Option<bool>,
    window: tauri::Window,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result {
    mark_mod_busy(unique_name, true, true, &state, &handle).await;
    let local_db = state.local_db.read().await.clone();
    let remote_db = state.remote_db.read().await.clone();
    let remote_db = remote_db.try_get()?;
    let conf = state.config.read().await.clone();
    let mut should_install = true;
    if let Some(current_mod) = local_db.get_mod(unique_name) {
        should_install = window
            .dialog()
            .message(format!(
                "{} is already installed, reinstall it?",
                current_mod.manifest.name
            ))
            .kind(MessageDialogKind::Info)
            .buttons(MessageDialogButtons::OkCancelCustom("Yes".to_string(), "No".to_string()))
            .title("Reinstall?")
            .blocking_show();
    }
    let res = if should_install {
        install_mod_from_db(
            &unique_name.to_string(),
            &conf,
            remote_db,
            &local_db,
            true,
            prerelease.unwrap_or(false),
        )
        .await
        .map(|_| ())
    } else {
        Ok(())
    };
    mark_mod_busy(unique_name, false, true, &state, &handle).await;
    res?;
    Ok(())
}

#[tauri::command]
pub async fn install_url(
    url: &str,
    state: tauri::State<'_, State>,
    _handle: tauri::AppHandle,
) -> Result {
    let conf = state.config.read().await.clone();
    let db = state.local_db.read().await.clone();
    install_mod_from_url(url, None, &conf, &db).await?;

    Ok(())
}

#[tauri::command]
pub async fn install_zip(
    path: &str,
    state: tauri::State<'_, State>,
    _handle: tauri::AppHandle,
) -> Result {
    let conf = state.config.read().await.clone();
    let db = state.local_db.read().await.clone();
    println!("Installing {}", path);
    install_mod_from_zip(&PathBuf::from(path), &conf, &db)?;

    Ok(())
}

#[tauri::command]
pub async fn uninstall_mod(
    unique_name: &str,
    state: tauri::State<'_, State>,
    _handle: tauri::AppHandle,
) -> Result<Vec<String>> {
    let db = state.local_db.read().await;
    let local_mod = db
        .get_mod(unique_name)
        .with_context(|| format!("Mod {} not found", unique_name))?;
    let warnings = remove_mod(local_mod, &db, false)?;

    Ok(warnings)
}

#[tauri::command]
pub async fn uninstall_broken_mod(mod_path: &str, state: tauri::State<'_, State>) -> Result {
    let db = state.local_db.read().await;
    let local_mod = db
        .get_mod_unsafe(mod_path)
        .with_context(|| format!("Mod {} not found", mod_path))?;
    match local_mod {
        UnsafeLocalMod::Invalid(m) => {
            remove_failed_mod(m)?;
        }
        _ => {
            return Err(Error(anyhow!("This mod is valid, refusing to remove")));
        }
    }
    Ok(())
}

#[tauri::command]
pub async fn open_mod_readme(unique_name: &str, state: tauri::State<'_, State>) -> Result {
    let db = state.remote_db.read().await;
    let db = db.try_get()?;
    open_readme(unique_name, db)?;
    Ok(())
}

#[tauri::command]
pub async fn open_owml(state: tauri::State<'_, State>) -> Result {
    let config = state.config.read().await;
    let local_db = state.local_db.read().await;
    open_shortcut("owml", &config, &local_db)?;
    Ok(())
}

#[tauri::command]
pub async fn save_config(
    config: Config,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result {
    let mut config = config.clone();
    config.path = Config::default_path()?;
    config.save()?;
    {
        let mut conf_lock = state.config.write().await;
        *conf_lock = config;
    }
    handle.typed_emit_all(&Event::ConfigReload(())).ok();

    Ok(())
}

#[tauri::command]
pub async fn get_config(state: tauri::State<'_, State>) -> Result<Config> {
    Ok(state.config.read().await.clone())
}

#[tauri::command]
pub async fn save_gui_config(
    gui_config: GuiConfig,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result {
    let watch_fs = gui_config.watch_fs;
    gui_config.save()?;
    {
        let mut conf_lock = state.gui_config.write().await;
        *conf_lock = gui_config;
    }
    let event = Event::GuiConfigReload(watch_fs);
    handle.typed_emit_all(&event).ok();
    // handle.typed_trigger_global(&event).ok();
    Ok(())
}

#[tauri::command]
pub async fn get_gui_config(state: tauri::State<'_, State>) -> Result<GuiConfig> {
    Ok(state.gui_config.read().await.clone())
}

#[tauri::command]
pub async fn save_owml_config(
    owml_config: OWMLConfig,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result {
    let config = state.config.read().await;
    owml_config.save(&config)?;
    handle.typed_emit_all(&Event::OwmlConfigReload(())).ok();

    Ok(())
}

#[tauri::command]
pub async fn get_owml_config(state: tauri::State<'_, State>) -> Result<OWMLConfig> {
    let config = state.config.read().await;
    let owml_config = OWMLConfig::get(&config)?;
    Ok(owml_config)
}

#[tauri::command]
pub async fn install_owml(
    prerelease: bool,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result {
    let config = state.config.read().await;
    let db = state.remote_db.read().await;
    let db = db.try_get()?;
    let owml = db.get_owml().context("Error Installing OWML")?;
    download_and_install_owml(&config, owml, prerelease).await?;
    handle.typed_emit_all(&Event::OwmlConfigReload(())).ok();
    Ok(())
}

#[tauri::command]
pub async fn set_owml(
    path: &str,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result<bool> {
    let mut new_config = state.config.read().await.clone();
    new_config.owml_path = path.to_string();
    if new_config.check_owml() {
        let mut config = state.config.write().await;
        *config = new_config;
        config.save()?;
        handle.typed_emit_all(&Event::ConfigReload(())).ok();
        handle.typed_emit_all(&Event::OwmlConfigReload(())).ok();
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn get_updatable_mods(
    filter: &str,
    state: tauri::State<'_, State>,
) -> Result<Vec<String>> {
    let mut updates: Vec<String> = vec![];
    let local_db = state.local_db.read().await;
    if let Some(remote_db) = state.remote_db.read().await.get() {
        let config = state.config.read().await;

        let mut mods: Vec<&LocalMod> = local_db.valid().collect();

        if filter.is_empty() {
            mods.sort_by(|a, b| {
                let name_ord = a.manifest.name.cmp(&b.manifest.name);
                let enabled_ord = a.enabled.cmp(&b.enabled);
                enabled_ord.then(name_ord)
            });
        } else {
            mods = local_db
                .search(filter)
                .iter()
                .filter_map(|m| match m {
                    UnsafeLocalMod::Invalid(_) => None,
                    UnsafeLocalMod::Valid(m) => Some(m.as_ref()),
                })
                .collect();
        }

        for local_mod in mods {
            let (needs_update, _) = check_mod_needs_update(local_mod, remote_db);
            if needs_update {
                updates.push(local_mod.manifest.unique_name.clone());
            }
        }
        if let Some(owml) = LocalDatabase::get_owml(&config.owml_path) {
            let (needs_update, _) = check_mod_needs_update(&owml, remote_db);
            if needs_update {
                updates.push(OWML_UNIQUE_NAME.to_string());
            }
        }
        Ok(updates)
    } else {
        Ok(vec![])
    }
}

#[tauri::command]
pub async fn update_mod(
    unique_name: &str,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result {
    mark_mod_busy(unique_name, true, true, &state, &handle).await;
    let config = state.config.read().await.clone();
    let local_db = state.local_db.read().await.clone();
    let remote_db = state.remote_db.read().await.clone();
    let remote_db = remote_db.try_get()?;

    let remote_mod = if unique_name == OWML_UNIQUE_NAME {
        remote_db.get_owml().context("Can't find OWML")?
    } else {
        remote_db
            .get_mod(unique_name)
            .with_context(|| format!("Can't find mod {} in remote", unique_name))?
    };

    let res = if unique_name == OWML_UNIQUE_NAME {
        download_and_install_owml(&config, remote_mod, false).await
    } else {
        install_mod_from_db(
            &unique_name.to_string(),
            &config,
            remote_db,
            &local_db,
            false,
            false,
        )
        .await
        .and_then(|m| fix_version_post_update(&m, remote_mod))
    };
    mark_mod_busy(unique_name, false, true, &state, &handle).await;
    res?;
    Ok(())
}

#[tauri::command]
pub async fn update_all_mods(
    unique_names: Vec<String>,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result {
    let config = state.config.read().await.clone();
    let local_db = state.local_db.read().await.clone();
    let remote_db = state.remote_db.read().await.clone();
    let remote_db = remote_db.try_get()?;
    let mut busy_mods = state.mods_in_progress.write().await;
    let owml_in_list = unique_names.contains(&OWML_UNIQUE_NAME.to_string());
    let unique_names: Vec<String> = unique_names
        .iter()
        .filter(|m| !busy_mods.contains(m) && m != &&OWML_UNIQUE_NAME.to_string())
        .cloned()
        .collect();
    busy_mods.extend(unique_names.clone());
    if owml_in_list {
        busy_mods.push(OWML_UNIQUE_NAME.to_string());
    }
    drop(busy_mods);
    handle.typed_emit_all(&Event::ModBusy(())).ok();
    let updated_mods =
        install_mods_parallel(unique_names.clone(), &config, remote_db, &local_db).await?;
    if owml_in_list {
        download_and_install_owml(
            &config,
            remote_db
                .get_owml()
                .context("Couldn't find OWML in database")?,
            false,
        )
        .await?;
    }
    let mut busy_mods = state.mods_in_progress.write().await;
    busy_mods.retain(|m| !unique_names.contains(m) && (!owml_in_list || m != OWML_UNIQUE_NAME));
    handle.typed_emit_all(&Event::ModBusy(())).ok();
    for updated_mod in updated_mods {
        fix_version_post_update(
            &updated_mod,
            remote_db
                .get_mod(&updated_mod.manifest.unique_name)
                .unwrap(),
        )?; // Unwrap is safe because any mod in this list must have a remote counterpart
        send_analytics_event(
            AnalyticsEventName::ModUpdate,
            &updated_mod.manifest.unique_name,
            &config,
        )
        .await;
    }
    Ok(())
}

#[tauri::command]
pub async fn start_logs(state: tauri::State<'_, State>, handle: tauri::AppHandle) -> Result {
    let game_logs = state.game_log.read().await;
    let gui_config = state.gui_config.read().await;
    let config = state.config.read().await.clone();
    if gui_config.no_log_server {
        drop(gui_config);
        launch_game(&config, true, None).await?;
        return Ok(());
    } else if gui_config.log_multi_window || game_logs.keys().count() == 0 {
        drop(game_logs);
        drop(gui_config);
        make_log_window(&handle).await?;
    } else {
        drop(gui_config);
        let config = state.config.read().await.clone();
        let port = *game_logs.keys().next().unwrap_or(&0);
        drop(game_logs);
        launch_game(&config, false, Some(&port)).await?;
    }
    Ok(())
}

#[tauri::command]
pub async fn active_log(port: LogPort, state: tauri::State<'_, State>) -> Result<bool> {
    Ok(state.game_log.read().await.get(&port).is_some())
}

#[tauri::command]
pub async fn run_game(
    state: tauri::State<'_, State>,
    window: tauri::Window,
    handle: tauri::AppHandle,
) -> Result {
    let config = state.config.read().await.clone();
    {
        let local_db = state.local_db.read().await;
        let new_config = show_warnings(&window, &local_db, &config)?;

        new_config.save()?;
        {
            let mut config = state.config.write().await;
            *config = new_config;
        }
        handle.typed_emit_all(&Event::ConfigReload(())).ok();
    }

    let log_server = LogServer::new(0).await?;
    let port = log_server.port;

    {
        let mut game_log = state.game_log.write().await;
        game_log.insert(port, LogData::new(port, &handle)?);
    }

    let close_handle = handle.clone();

    window.on_window_event(move |e| {
        if let WindowEvent::CloseRequested { .. } = e {
            let handle = close_handle.clone();
            async_runtime::spawn(async move {
                let state = handle.state::<State>();
                let mut logs = state.game_log.write().await;
                logs.remove(&port);
            });
        }
    });

    window.typed_emit(&Event::GameStart(port)).ok();

    let (tx, mut rx) = mpsc::channel(32);

    let log_handler = async {
        loop {
            select! {
                msg = rx.recv() => {
                    if let Some(msg) = msg {
                        let mut game_log = state.game_log.write().await;
                        if let Some(log_data) = game_log.get_mut(&port) {
                            log_data.take_message(msg);
                        }
                    } else {
                        break;
                    }
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)) => {
                    let mut game_log = state.game_log.write().await;
                    if let Some(log_data) = game_log.get_mut(&port) {
                        log_data.process_emit_queue();
                    }
                }
            }
        }
        Ok(())
    };

    try_join!(
        log_server.listen(tx, false),
        launch_game(&config, false, Some(&port)),
        log_handler
    )
    .map_err(|e| anyhow!("Can't Start Game: {:?}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn clear_logs(port: LogPort, state: tauri::State<'_, State>) -> Result {
    let mut data = state.game_log.write().await;
    if let Some(log_data) = data.get_mut(&port) {
        log_data.clear();
    }
    Ok(())
}

#[tauri::command]
pub async fn get_log_lines(
    port: LogPort,
    filter_type: Option<SocketMessageType>,
    search: &str,
    state: tauri::State<'_, State>,
) -> Result<(Vec<usize>, u32)> {
    let mut logs = state.game_log.write().await;
    if let Some(&mut ref mut log_data) = logs.get_mut(&port) {
        let sum = log_data.get_count();
        let lines = log_data.get_lines(filter_type, search);
        Ok((lines, sum))
    } else {
        Err(Error(anyhow!("Log Server Not Running")))
    }
}

#[tauri::command]
pub async fn get_game_message(
    port: LogPort,
    line: usize,
    state: tauri::State<'_, State>,
) -> Result<GameMessage> {
    let logs = state.game_log.read().await;
    if let Some(log_data) = logs.get(&port) {
        if let Some(msg) = log_data.get_message(line) {
            Ok(msg.clone())
        } else {
            Err(Error(anyhow!("Log Line {line} Not Found")))
        }
    } else {
        Err(Error(anyhow!("Log Server Not Running")))
    }
}

#[tauri::command]
pub async fn force_log_update(port: LogPort, state: tauri::State<'_, State>) -> Result {
    let mut logs = state.game_log.write().await;
    if let Some(log_data) = logs.get_mut(&port) {
        log_data.process_emit_queue();
    }
    Ok(())
}

#[tauri::command]
pub async fn export_mods(path: String, state: tauri::State<'_, State>) -> Result {
    let path = PathBuf::from(path);
    let local_db = state.local_db.read().await;
    let output = owmods_core::io::export_mods(&local_db)?;
    let file = File::create(&path).map_err(|e| anyhow!("Error Saving File: {:?}", e))?;
    let mut writer = BufWriter::new(file);
    write!(&mut writer, "{}", output).map_err(|e| anyhow!("Error Saving File: {:?}", e))?;
    opener::open(path).ok();
    Ok(())
}

#[tauri::command]
pub async fn import_mods(
    path: String,
    disable_missing: bool,
    state: tauri::State<'_, State>,
    _handle: tauri::AppHandle,
) -> Result {
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    let remote_db = remote_db.try_get()?;
    let config = state.config.read().await;
    let path = PathBuf::from(path);
    owmods_core::io::import_mods(&config, &local_db, remote_db, &path, disable_missing).await?;

    Ok(())
}

#[tauri::command]
pub async fn fix_mod_deps(
    unique_name: &str,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result {
    let config = state.config.read().await.clone();
    let local_db = state.local_db.read().await.clone();
    let remote_db = state.remote_db.read().await.clone();
    let remote_db = remote_db.try_get()?;
    let local_mod = local_db
        .get_mod(unique_name)
        .with_context(|| format!("Can't find mod {}", unique_name))?;

    mark_mod_busy(unique_name, true, true, &state, &handle).await;
    let res = fix_deps(local_mod, &config, &local_db, remote_db).await;
    mark_mod_busy(unique_name, false, true, &state, &handle).await;
    res?;
    Ok(())
}

#[tauri::command]
pub async fn db_has_issues(state: tauri::State<'_, State>, window: tauri::Window) -> Result<bool> {
    let local_db = state.local_db.read().await.clone();
    let config = state.config.read().await.clone();
    let mut has_errors =
        local_db.active().any(|m| !m.errors.is_empty()) || local_db.invalid().count() > 0;

    let owml = LocalDatabase::get_owml(&config.owml_path);
    if let Some(owml) = owml {
        let remote_db = state.remote_db.read().await.clone();
        if let Some(remote_db) = remote_db.get() {
            let (needs_update, remote_owml) = check_mod_needs_update(&owml, remote_db);
            if needs_update {
                let answer = window
                    .dialog()
                    .message(format!(
                        "OWML is out of date, update it? (You have {} installed)",
                        owml.manifest.version
                    ))
                    .kind(MessageDialogKind::Info)
                    .buttons(MessageDialogButtons::OkCancelCustom("Yes".to_string(), "No".to_string()))
                    .title("Update OWML?")
                    .blocking_show();
                if answer {
                    let handle = window.app_handle();
                    mark_mod_busy(OWML_UNIQUE_NAME, true, true, &state, handle).await;
                    download_and_install_owml(&config, remote_owml.unwrap(), false).await?;
                    mark_mod_busy(OWML_UNIQUE_NAME, false, true, &state, handle).await;
                    let event = Event::RequestReload("LOCAL".to_string());
                    handle.typed_emit_all(&event).unwrap();
                } else {
                    has_errors = true;
                }
            }
        }
    }
    Ok(has_errors)
}

#[tauri::command]
pub async fn get_alert(state: tauri::State<'_, State>) -> Result<Alert> {
    let config = state.config.read().await;
    let mut alert = fetch_alert(&config.alert_url).await?;
    if config
        .last_viewed_db_alert
        .as_ref()
        .map(|lv| lv == &alert.compute_hash())
        .unwrap_or(false)
    {
        alert.enabled = false;
    }
    Ok(alert)
}

#[tauri::command]
pub async fn dismiss_alert(
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
    alert: Alert,
) -> Result {
    let mut config = state.config.write().await;
    config.last_viewed_db_alert = Some(alert.compute_hash());
    config.save()?;
    handle.typed_emit_all(&Event::ConfigReload(())).ok();
    Ok(())
}

#[tauri::command]
pub async fn pop_protocol_url(
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
    id: &str,
) -> Result {
    let id = id.to_string();

    let mut protocol_listeners = state.protocol_listeners.write().await;
    if protocol_listeners.contains(&id) {
        return Ok(());
    }
    protocol_listeners.push(id.clone());

    if protocol_listeners.len() >= PROTOCOL_LISTENER_AMOUNT {
        let mut protocol_url = state.protocol_url.write().await;
        if let Some(url) = protocol_url.as_ref().cloned() {
            let handle_2 = handle.clone();
            handle.typed_listen(move |e| {
                if let Event::RemoteInitialized(_) = e {
                    handle_2
                        .typed_emit_all(&Event::ProtocolInvoke(url.clone()))
                        .ok();
                }
            });
        }
        *protocol_url = None;
    }

    Ok(())
}

#[tauri::command]
pub async fn check_owml(state: tauri::State<'_, State>) -> Result<bool> {
    let config = state.config.read().await;
    Ok(config.check_owml())
}

#[tauri::command]
pub async fn get_defaults(
    state: tauri::State<'_, State>,
) -> Result<(Config, GuiConfig, OWMLConfig)> {
    let old_config = state.config.read().await;
    let config = Config::default(None)?;
    let gui_config = GuiConfig::default();
    let owml_config = OWMLConfig::default(&old_config)?;
    Ok((config, gui_config, owml_config))
}

#[tauri::command]
pub async fn get_downloads(state: tauri::State<'_, State>) -> Result<ProgressBars> {
    let bars = state.progress_bars.read().await;
    Ok(bars.clone())
}

#[tauri::command]
pub async fn clear_downloads(state: tauri::State<'_, State>, handle: tauri::AppHandle) -> Result {
    let mut bars = state.progress_bars.write().await;
    bars.bars.clear();
    handle.typed_emit_all(&Event::ProgressUpdate(())).ok();
    Ok(())
}

#[tauri::command]
pub async fn get_busy_mods(state: tauri::State<'_, State>) -> Result<Vec<String>> {
    let in_progress = state.mods_in_progress.read().await.clone();
    Ok(in_progress)
}

#[tauri::command]
pub async fn get_mod_busy(unique_name: &str, state: tauri::State<'_, State>) -> Result<bool> {
    let mods_in_progress = state.mods_in_progress.read().await;
    let exists = mods_in_progress.contains(&unique_name.to_string());
    Ok(exists)
}

#[tauri::command]
pub async fn get_bar_by_unique_name(
    unique_name: &str,
    state: tauri::State<'_, State>,
) -> Result<Option<ProgressBar>> {
    let bars = state.progress_bars.read().await;
    Ok(bars.by_unique_name(unique_name).cloned())
}

#[tauri::command]
pub async fn has_disabled_deps(unique_name: &str, state: tauri::State<'_, State>) -> Result<bool> {
    let db = state.local_db.read().await;
    let local_mod = db
        .get_mod(unique_name)
        .context("Mod Not Found: {unique_name}")?;
    let mut flag = false;
    if let Some(deps) = &local_mod.manifest.dependencies {
        for dep in deps.iter() {
            if let Some(dep) = db.get_mod(dep) {
                if !dep.enabled {
                    flag = true;
                }
            }
        }
    }
    Ok(flag)
}

#[tauri::command]
pub async fn register_drop_handler(window: tauri::Window) -> Result {
    let handle = window.app_handle().clone();
    window.on_window_event(move |e| {
        if let WindowEvent::DragDrop(e) = e {
            match e {
                DragDropEvent::Drop { paths, position: _ } => {
                    if let Some(f) = paths.first() {
                        if f.extension().map(|e| e == "zip").unwrap_or(false) {
                            handle.typed_emit_all(&Event::DragLeave(())).ok();
                            handle
                                .typed_emit_all(&Event::ProtocolInvoke(ProtocolPayload {
                                    verb: ProtocolVerb::InstallZip,
                                    payload: f.to_str().unwrap().to_string(),
                                }))
                                .ok();
                        }
                    }
                }
                DragDropEvent::Enter { paths, position: _ } => {
                    if let Some(f) = paths.first() {
                        if f.extension().map(|e| e == "zip").unwrap_or(false) {
                            handle.typed_emit_all(&Event::DragEnter(())).ok();
                        }
                    }
                }
                DragDropEvent::Leave => {
                    handle.typed_emit_all(&Event::DragLeave(())).ok();
                }
                _ => {}
            }
        }
    });
    Ok(())
}

#[tauri::command]
pub async fn get_db_tags(state: tauri::State<'_, State>) -> Result<Vec<String>> {
    let db = state.remote_db.read().await;
    let db = db.get();
    Ok(db.map(|db| db.get_tags()).unwrap_or_default())
}

#[tauri::command]
pub async fn log_error(err: &str) -> Result {
    error!("Error Received From Frontend: {}", err);
    Ok(())
}

#[tauri::command]
pub async fn open_mod_github(unique_name: &str, state: tauri::State<'_, State>) -> Result {
    let db = state.remote_db.read().await;
    let db = db.try_get()?;
    open_github(unique_name, db)?;
    Ok(())
}

#[tauri::command]
pub async fn show_log_folder() -> Result {
    let path = get_app_path()?;
    opener::open(path.join("logs")).ok();
    Ok(())
}
