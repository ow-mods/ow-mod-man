use std::result::Result as StdResult;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use owmods_core::{
    alerts::{fetch_alert, Alert},
    config::Config,
    constants::OWML_UNIQUE_NAME,
    db::{LocalDatabase, RemoteDatabase},
    download::{
        download_and_install_owml, install_mod_from_db, install_mod_from_url, install_mod_from_zip,
        install_mods_parallel,
    },
    file::{create_all_parents, get_app_path},
    game::launch_game,
    mods::{local::UnsafeLocalMod, remote::RemoteMod},
    open::{open_readme, open_shortcut},
    owml::OWMLConfig,
    remove::{remove_failed_mod, remove_mod},
    socket::{LogServer, SocketMessage, SocketMessageType},
    updates::check_mod_needs_update,
    validate::fix_deps,
};
use serde::Serialize;
use tauri::{api::dialog, AppHandle, Manager};
use time::{macros::format_description, OffsetDateTime};
use tokio::try_join;

use crate::{
    game::{get_logs_indices, make_log_window, show_warnings, write_log, GameMessage},
    gui_config::GuiConfig,
    LogPort, State,
};

type Result<T = ()> = StdResult<T, Error>;

pub struct Error(anyhow::Error);

impl From<anyhow::Error> for Error {
    fn from(item: anyhow::Error) -> Self {
        Self(item)
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

fn toggle_fs_watch(handle: &AppHandle, enabled: bool) {
    handle.emit_all("TOGGLE_FS_WATCH", enabled).ok();
}

#[tauri::command]
pub async fn initial_setup(handle: tauri::AppHandle, state: tauri::State<'_, State>) -> Result {
    let mut config = state.config.write().await;
    *config = Config::get(None)?;
    let mut gui_config = state.gui_config.write().await;
    *gui_config = GuiConfig::get()?;
    handle.emit_all("GUI_CONFIG_RELOAD", "").ok();
    handle.emit_all("CONFIG_RELOAD", "").ok();
    Ok(())
}

#[tauri::command]
pub async fn refresh_local_db(handle: tauri::AppHandle, state: tauri::State<'_, State>) -> Result {
    toggle_fs_watch(&handle, false);
    let conf = state.config.read().await;
    {
        let mut db = state.local_db.write().await;
        let local_db = LocalDatabase::fetch(&conf.owml_path);
        *db = local_db.unwrap_or_else(|_| LocalDatabase::default());
    }
    handle.emit_all("LOCAL-REFRESH", "").ok();
    toggle_fs_watch(&handle, true);
    Ok(())
}

#[tauri::command]
pub async fn get_local_mods(filter: &str, state: tauri::State<'_, State>) -> Result<Vec<String>> {
    let db = state.local_db.read().await;
    let mut mods: Vec<&UnsafeLocalMod> = db.all().collect();
    if filter.is_empty() {
        mods.sort_by(|a, b| {
            let name_ord = a.get_name().cmp(b.get_name());
            let errors_ord = a.get_errs().len().cmp(&b.get_errs().len()).reverse();
            errors_ord.then(name_ord)
        });
    } else {
        mods = db.search(filter);
    }
    Ok(mods
        .into_iter()
        .map(|m| m.get_unique_name().clone())
        .collect())
}

#[tauri::command]
pub async fn get_local_mod(
    unique_name: &str,
    state: tauri::State<'_, State>,
) -> Result<Option<UnsafeLocalMod>> {
    if unique_name == OWML_UNIQUE_NAME {
        let config = state.config.read().await;
        let owml = LocalDatabase::get_owml(&config.owml_path)
            .ok_or_else(|| anyhow!("Couldn't Find OWML at path {}", &config.owml_path))?;
        Ok(Some(UnsafeLocalMod::Valid(owml)))
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
    toggle_fs_watch(&handle, false);
    let conf = state.config.read().await;
    {
        let mut db = state.remote_db.write().await;
        let remote_db = RemoteDatabase::fetch(&conf.database_url).await;
        *db = remote_db.unwrap_or_else(|_| RemoteDatabase::default());
    }
    handle.emit_all("REMOTE-REFRESH", "").ok();
    toggle_fs_watch(&handle, true);
    Ok(())
}

#[tauri::command]
pub async fn get_remote_mods(filter: &str, state: tauri::State<'_, State>) -> Result<Vec<String>> {
    let db = state.remote_db.read().await;
    let mut mods: Vec<&RemoteMod> = db
        .mods
        .values()
        .filter(|m| m.unique_name != OWML_UNIQUE_NAME)
        .collect();
    if filter.is_empty() {
        mods.sort_by(|a, b| b.download_count.cmp(&a.download_count));
    } else {
        mods = db.search(filter);
    }
    Ok(mods.into_iter().map(|m| m.unique_name.clone()).collect())
}

#[tauri::command]
pub async fn get_remote_mod(
    unique_name: &str,
    state: tauri::State<'_, State>,
) -> Result<Option<RemoteMod>> {
    let db = state.remote_db.read().await;
    if unique_name == OWML_UNIQUE_NAME {
        Ok(db.get_owml().cloned())
    } else {
        Ok(db.get_mod(unique_name).cloned())
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
    state: tauri::State<'_, State>,
) -> Result {
    let db = state.local_db.read().await;
    owmods_core::toggle::toggle_mod(unique_name, &db, enabled, false)?;
    Ok(())
}

#[tauri::command]
pub async fn toggle_all(enabled: bool, state: tauri::State<'_, State>) -> Result {
    let local_db = state.local_db.read().await;
    for local_mod in local_db.valid() {
        owmods_core::toggle::toggle_mod(
            &local_mod.manifest.unique_name,
            &local_db,
            enabled,
            false,
        )?;
    }
    Ok(())
}

#[tauri::command]
pub async fn install_mod(
    unique_name: &str,
    prerelease: Option<bool>,
    window: tauri::Window,
    state: tauri::State<'_, State>,
) -> Result {
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    let conf = state.config.read().await;
    if let Some(current_mod) = local_db.get_mod(unique_name) {
        let res = dialog::blocking::confirm(
            Some(&window),
            "Reinstall?",
            format!(
                "{} is already installed, reinstall it?",
                current_mod.manifest.name
            ),
        );
        if !res {
            return Ok(());
        }
    }
    install_mod_from_db(
        &unique_name.to_string(),
        &conf,
        &remote_db,
        &local_db,
        true,
        prerelease.unwrap_or(false),
    )
    .await?;
    Ok(())
}

#[tauri::command]
pub async fn install_url(url: &str, state: tauri::State<'_, State>) -> Result {
    let conf = state.config.read().await;
    let db = state.local_db.read().await;
    install_mod_from_url(url, &conf, &db).await?;
    Ok(())
}

#[tauri::command]
pub async fn install_zip(path: &str, state: tauri::State<'_, State>) -> Result {
    let conf = state.config.read().await;
    let db = state.local_db.read().await;
    println!("Installing {}", path);
    install_mod_from_zip(&PathBuf::from(path), &conf, &db)?;
    Ok(())
}

#[tauri::command]
pub async fn uninstall_mod(unique_name: &str, state: tauri::State<'_, State>) -> Result {
    let db = state.local_db.read().await;
    let local_mod = db
        .get_mod(unique_name)
        .ok_or_else(|| anyhow!("Mod {} not found", unique_name))?;
    remove_mod(local_mod, &db, false)?;
    Ok(())
}

#[tauri::command]
pub async fn uninstall_broken_mod(mod_path: &str, state: tauri::State<'_, State>) -> Result {
    let db = state.local_db.read().await;
    let local_mod = db
        .get_mod_unsafe(mod_path)
        .ok_or_else(|| anyhow!("Mod {} not found", mod_path))?;
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
    open_readme(unique_name, &db)?;
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
    handle.emit_all("CONFIG_RELOAD", "").ok();
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
    gui_config.save()?;
    {
        let mut conf_lock = state.gui_config.write().await;
        *conf_lock = gui_config;
    }
    handle.emit_all("GUI_CONFIG_RELOAD", "").ok();
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
    handle.emit_all("OWML_CONFIG_RELOAD", "").ok();
    Ok(())
}

#[tauri::command]
pub async fn get_owml_config(state: tauri::State<'_, State>) -> Result<OWMLConfig> {
    let config = state.config.read().await;
    let owml_config = OWMLConfig::get(&config)?;
    Ok(owml_config)
}

#[tauri::command]
pub async fn install_owml(state: tauri::State<'_, State>, handle: tauri::AppHandle) -> Result {
    let config = state.config.read().await;
    let db = state.remote_db.read().await;
    let owml = db
        .get_owml()
        .ok_or_else(|| anyhow!("Couldn't Find OWML In The Database"))?;
    download_and_install_owml(&config, owml).await?;
    handle.emit_all("OWML_CONFIG_RELOAD", "").ok();
    Ok(())
}

#[tauri::command]
pub async fn set_owml(
    path: &str,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result<bool> {
    let path = Path::new(path);
    if path.is_dir() && path.join("OWML.Manifest.json").is_file() {
        let mut config = state.config.write().await;
        config.owml_path = path.to_str().unwrap().to_string();
        config.save()?;
        handle.emit_all("OWML_CONFIG_RELOAD", "").ok();
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn get_updatable_mods(state: tauri::State<'_, State>) -> Result<Vec<String>> {
    let mut updates: Vec<String> = vec![];
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    let config = state.config.read().await;
    for local_mod in local_db.valid() {
        let (needs_update, _) = check_mod_needs_update(local_mod, &remote_db);
        if needs_update {
            updates.push(local_mod.manifest.unique_name.clone());
        }
    }
    if let Some(owml) = LocalDatabase::get_owml(&config.owml_path) {
        let (needs_update, _) = check_mod_needs_update(&owml, &remote_db);
        if needs_update {
            updates.push(OWML_UNIQUE_NAME.to_string());
        }
    }
    Ok(updates)
}

#[tauri::command]
pub async fn update_mod(
    unique_name: &str,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result {
    let config = state.config.read().await;
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    toggle_fs_watch(&handle, false);
    if unique_name == OWML_UNIQUE_NAME {
        download_and_install_owml(
            &config,
            remote_db
                .get_owml()
                .ok_or_else(|| anyhow!("OWML Not Found!"))?,
        )
        .await?;
    } else {
        install_mod_from_db(
            &unique_name.to_string(),
            &config,
            &remote_db,
            &local_db,
            false,
            false,
        )
        .await?;
    }
    toggle_fs_watch(&handle, true);
    Ok(())
}

#[tauri::command]
pub async fn update_all_mods(
    unique_names: Vec<String>,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result {
    toggle_fs_watch(&handle, false);
    let config = state.config.read().await;
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    install_mods_parallel(unique_names, &config, &remote_db, &local_db).await?;
    toggle_fs_watch(&handle, true);
    Ok(())
}

#[tauri::command]
pub async fn start_logs(state: tauri::State<'_, State>, handle: tauri::AppHandle) -> Result {
    let game_logs = state.game_log.read().await;
    let gui_config = state.gui_config.read().await;
    if gui_config.log_multi_window || game_logs.keys().count() == 0 {
        drop(game_logs);
        drop(gui_config);
        make_log_window(&handle).await?;
    } else {
        drop(gui_config);
        let config = state.config.read().await.clone();
        let port = *game_logs.keys().next().unwrap_or(&0);
        drop(game_logs);
        launch_game(&config, &port).await?;
    }
    Ok(())
}

#[tauri::command]
pub async fn active_log(port: LogPort, state: tauri::State<'_, State>) -> Result<bool> {
    Ok(state.game_log.read().await.get(&port).is_some())
}

#[tauri::command]
pub async fn run_game(state: tauri::State<'_, State>, window: tauri::Window) -> Result {
    let config = state.config.read().await.clone();
    {
        let local_db = state.local_db.read().await;
        let new_config = show_warnings(&window, &local_db, &config)?;
        new_config.save()?;
        {
            let mut config = state.config.write().await;
            *config = new_config;
        }
    }
    let log_server = LogServer::new(0).await?;
    let port = log_server.port;
    let now = OffsetDateTime::now_utc();
    let logs_path = get_app_path()?
        .join("game_logs")
        .join(
            now.format(format_description!("[year]-[month]-[day]"))
                .unwrap(),
        )
        .join(format!(
            "{}_{port}.log",
            now.format(format_description!("[hour]-[minute]-[second]"))
                .unwrap()
        ));
    create_all_parents(&logs_path)?;
    let file = File::options()
        .read(true)
        .append(true)
        .create(true)
        .open(&logs_path)
        .map_err(|e| anyhow!("Couldn't create log file: {:?}", e))?;
    {
        let mut game_log = state.game_log.write().await;
        let writer = BufWriter::new(file);
        game_log.insert(port, (vec![], writer));
    }
    let handle_log = |msg: &SocketMessage, _: &u16| {
        let msg = msg.clone();
        let logs_map = state.game_log.clone();
        let window_handle = window.app_handle();
        tokio::spawn(async move {
            let mut game_log = logs_map.write().await;
            if let Some((lines, writer)) = game_log.get_mut(&port) {
                write_log(writer, &msg).unwrap();
                let msg = GameMessage::new(port, msg);
                if matches!(msg.message.message_type, SocketMessageType::Fatal) {
                    window_handle
                        .emit_all("LOG-FATAL", &msg)
                        .expect("Couldn't Send Event");
                }
                lines.push(msg);
                window_handle
                    .emit_all("LOG-UPDATE", port)
                    .expect("Can't Send Event");
            }
        });
    };
    window.emit("GAME-START", &port).expect("Can't Send Event");
    try_join!(
        log_server.listen(&handle_log, false),
        launch_game(&config, &port)
    )
    .map_err(|e| anyhow!("Can't Start Game: {:?}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn clear_logs(
    port: LogPort,
    handle: tauri::AppHandle,
    state: tauri::State<'_, State>,
) -> Result {
    let mut data = state.game_log.write().await;
    if let Some((lines, _)) = data.get_mut(&port) {
        lines.clear();
        handle
            .emit_all("LOG-UPDATE", "")
            .map_err(|e| anyhow!("Can't Send Event: {:?}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn stop_logging(port: LogPort, state: tauri::State<'_, State>) -> Result {
    let mut logs = state.game_log.write().await;
    if let Some((_, ref mut writer)) = logs.get_mut(&port) {
        writer
            .flush()
            .map_err(|e| anyhow!("Error flushing buffer: {:?}", e))?;
    }
    logs.remove(&port);
    Ok(())
}

#[tauri::command]
pub async fn get_log_lines(
    port: LogPort,
    filter_type: Option<SocketMessageType>,
    search: &str,
    state: tauri::State<'_, State>,
) -> Result<Vec<(usize, usize)>> {
    let logs = state.game_log.read().await;
    if let Some((lines, _)) = logs.get(&port) {
        let lines = get_logs_indices(lines, filter_type, search)?;
        Ok(lines)
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
    if let Some((lines, _)) = logs.get(&port) {
        let msg = lines
            .get(line)
            .ok_or_else(|| anyhow!("Invalid Log Line {line}"))?;
        Ok(msg.clone())
    } else {
        Err(Error(anyhow!("Log Server Not Running")))
    }
}

#[tauri::command]
pub async fn export_mods(path: String, state: tauri::State<'_, State>) -> Result {
    let path = PathBuf::from(path);
    let local_db = state.local_db.read().await;
    let output = owmods_core::io::export_mods(&local_db)?;
    let file = File::create(path).map_err(|e| anyhow!("Error Saving File: {:?}", e))?;
    let mut writer = BufWriter::new(file);
    write!(&mut writer, "{}", output).map_err(|e| anyhow!("Error Saving File: {:?}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn import_mods(path: String, state: tauri::State<'_, State>) -> Result {
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    let config = state.config.read().await;
    let path = PathBuf::from(path);
    owmods_core::io::import_mods(&config, &local_db, &remote_db, &path, false).await?;
    Ok(())
}

#[tauri::command]
pub async fn fix_mod_deps(unique_name: &str, state: tauri::State<'_, State>) -> Result {
    let config = state.config.read().await;
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    let local_mod = local_db
        .get_mod(unique_name)
        .ok_or_else(|| anyhow!("Can't find mod {}", unique_name))?;
    fix_deps(local_mod, &config, &local_db, &remote_db).await?;
    Ok(())
}

#[tauri::command]
pub async fn db_has_issues(state: tauri::State<'_, State>) -> Result<bool> {
    let local_db = state.local_db.read().await;
    let has_errors = local_db.active().any(|m| !m.errors.is_empty());
    Ok(has_errors)
}

#[tauri::command]
pub async fn get_alert(state: tauri::State<'_, State>) -> Result<Alert> {
    let config = state.config.read().await;
    let alert = fetch_alert(&config.alert_url).await?;
    Ok(alert)
}

#[tauri::command]
pub async fn get_watcher_paths(state: tauri::State<'_, State>) -> Result<Vec<String>> {
    let config = state.config.read().await;
    Ok(vec![
        config.owml_path.clone(),
        GuiConfig::path().unwrap().to_str().unwrap().to_string(),
        Config::default_path()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    ])
}

#[tauri::command]
pub async fn pop_protocol_url(state: tauri::State<'_, State>, handle: tauri::AppHandle) -> Result {
    let mut protocol_url = state.protocol_url.write().await;
    if let Some(url) = protocol_url.as_ref() {
        handle.emit_all("PROTOCOL_INVOKE", url).ok();
    }
    *protocol_url = None;
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
