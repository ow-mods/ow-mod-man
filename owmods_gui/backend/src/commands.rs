use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use chrono::Local;
use owmods_core::config::Config;
use owmods_core::constants::OWML_UNIQUE_NAME;
use owmods_core::db::{LocalDatabase, RemoteDatabase};
use owmods_core::download::{
    download_and_install_owml, install_mod_from_db, install_mod_from_url, install_mod_from_zip,
    install_mods_parallel,
};
use owmods_core::file::{create_all_parents, get_app_path};
use owmods_core::game::launch_game;
use owmods_core::mods::{LocalMod, OWMLConfig, RemoteMod};
use owmods_core::open::{open_readme, open_shortcut};
use owmods_core::remove::remove_mod;
use owmods_core::socket::{LogServer, SocketMessage, SocketMessageType};
use owmods_core::updates::check_mod_needs_update;
use rust_fuzzy_search::fuzzy_compare;
use tauri::api::dialog;
use tauri::Manager;
use tokio::try_join;

use crate::{LogPort, State};

use crate::game::{get_logs_indices, make_log_window, show_warnings, write_log, GameMessage};
use crate::gui_config::GuiConfig;

fn e_to_str(e: anyhow::Error) -> String {
    e.to_string()
}

const SEARCH_THRESHOLD: f32 = 0.04;

fn search<'a, T>(
    source_list: Vec<&'a T>,
    filter: &str,
    get_values: impl Fn(&T) -> Vec<String>,
) -> Vec<&'a T> {
    let mut scores: Vec<(&T, f32)> = source_list
        .into_iter()
        .filter_map(|m| {
            let mut final_score: Option<f32> = None;
            for search in get_values(m).iter() {
                let score = fuzzy_compare(search, filter);
                if (score >= SEARCH_THRESHOLD || search.contains(filter))
                    && score > final_score.unwrap_or(0.0)
                {
                    final_score = Some(score);
                }
            }
            final_score.map(|score| (m, score))
        })
        .collect();
    scores.sort_by(|(_, a), (_, b)| b.total_cmp(a));
    scores.iter().map(|(m, _)| *m).collect()
}

#[tauri::command]
pub async fn initial_setup(state: tauri::State<'_, State>) -> Result<(), String> {
    let mut config = state.config.write().await;
    *config = Config::get(None).map_err(e_to_str)?;
    let mut gui_config = state.gui_config.write().await;
    *gui_config = GuiConfig::get().map_err(e_to_str)?;
    Ok(())
}

#[tauri::command]
pub async fn refresh_local_db(
    handle: tauri::AppHandle,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let conf = state.config.read().await;
    {
        let mut db = state.local_db.write().await;
        handle.emit_all("LOCAL-REFRESH", "").ok();
        let local_db = LocalDatabase::fetch(&conf.owml_path);
        *db = local_db.unwrap_or_else(|_| LocalDatabase::default());
    }
    Ok(())
}

#[tauri::command]
pub async fn get_local_mods(
    filter: &str,
    state: tauri::State<'_, State>,
) -> Result<Vec<String>, ()> {
    let db = state.local_db.read().await;
    let mut mods: Vec<&LocalMod> = db.mods.values().collect();
    if filter.is_empty() {
        mods.sort_by(|a, b| a.manifest.name.cmp(&b.manifest.name));
    } else {
        mods = search(mods, &filter.to_ascii_lowercase(), |m| {
            vec![
                m.manifest.name.to_ascii_lowercase(),
                m.manifest.author.to_ascii_lowercase(),
                m.manifest.unique_name.to_ascii_lowercase(),
            ]
        });
    }
    Ok(mods
        .into_iter()
        .map(|m| m.manifest.unique_name.clone())
        .collect())
}

#[tauri::command]
pub async fn get_local_mod(
    unique_name: &str,
    state: tauri::State<'_, State>,
) -> Result<Option<LocalMod>, ()> {
    if unique_name == OWML_UNIQUE_NAME {
        let config = state.config.read().await;
        Ok(LocalDatabase::get_owml(&config.owml_path))
    } else {
        Ok(state.local_db.read().await.get_mod(unique_name).cloned())
    }
}

#[tauri::command]
pub async fn refresh_remote_db(
    handle: tauri::AppHandle,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let conf = state.config.read().await;
    {
        let mut db = state.remote_db.write().await;
        handle.emit_all("REMOTE-REFRESH", "").ok();
        let remote_db = RemoteDatabase::fetch(&conf.database_url).await;
        *db = remote_db.unwrap_or_else(|_| RemoteDatabase::default());
    }
    Ok(())
}

#[tauri::command]
pub async fn get_remote_mods(
    filter: &str,
    state: tauri::State<'_, State>,
) -> Result<Vec<String>, ()> {
    let db = state.remote_db.read().await;
    let mut mods: Vec<&RemoteMod> = db
        .mods
        .values()
        .filter(|m| m.unique_name != OWML_UNIQUE_NAME)
        .collect();
    if filter.is_empty() {
        mods.sort_by(|a, b| b.download_count.cmp(&a.download_count));
    } else {
        mods = search(mods, &filter.to_ascii_lowercase(), |m| {
            vec![
                m.unique_name.to_ascii_lowercase(),
                m.name.to_ascii_lowercase(),
                m.author.to_ascii_lowercase(),
                m.description.to_ascii_lowercase(),
            ]
        });
    }
    Ok(mods.into_iter().map(|m| m.unique_name.clone()).collect())
}

#[tauri::command]
pub async fn get_remote_mod(
    unique_name: &str,
    state: tauri::State<'_, State>,
) -> Result<Option<RemoteMod>, ()> {
    let db = state.remote_db.read().await;
    if unique_name == OWML_UNIQUE_NAME {
        Ok(db.get_owml().cloned())
    } else {
        Ok(db.get_mod(unique_name).cloned())
    }
}

#[tauri::command]
pub async fn open_mod_folder(
    unique_name: &str,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let db = state.local_db.read().await;
    let conf = state.config.read().await;
    open_shortcut(unique_name, &conf, &db).map_err(e_to_str)?;
    Ok(())
}

#[tauri::command]
pub async fn toggle_mod(
    unique_name: &str,
    enabled: bool,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let db = state.local_db.read().await;
    owmods_core::toggle::toggle_mod(unique_name, &db, enabled, false).map_err(e_to_str)?;
    Ok(())
}

#[tauri::command]
pub async fn toggle_all(enabled: bool, state: tauri::State<'_, State>) -> Result<(), String> {
    let local_db = state.local_db.read().await;
    for local_mod in local_db.mods.values() {
        owmods_core::toggle::toggle_mod(&local_mod.manifest.unique_name, &local_db, enabled, false)
            .map_err(e_to_str)?;
    }
    Ok(())
}

#[tauri::command]
pub async fn install_mod(
    unique_name: &str,
    prerelease: Option<bool>,
    window: tauri::Window,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    let conf = state.config.read().await;
    if let Some(current_mod) = local_db.mods.get(unique_name) {
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
    .await
    .map_err(e_to_str)?;
    Ok(())
}

#[tauri::command]
pub async fn install_url(url: &str, state: tauri::State<'_, State>) -> Result<(), String> {
    let conf = state.config.read().await;
    let db = state.local_db.read().await;
    install_mod_from_url(url, &conf, &db)
        .await
        .map_err(e_to_str)?;
    Ok(())
}

#[tauri::command]
pub async fn install_zip(path: &str, state: tauri::State<'_, State>) -> Result<(), String> {
    let conf = state.config.read().await;
    let db = state.local_db.read().await;
    println!("Installing {}", path);
    install_mod_from_zip(&PathBuf::from(path), &conf, &db).map_err(e_to_str)?;
    Ok(())
}

#[tauri::command]
pub async fn uninstall_mod(
    unique_name: &str,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let db = state.local_db.read().await;
    let local_mod = db
        .get_mod(unique_name)
        .ok_or_else(|| format!("Mod {} not found", unique_name))?;
    remove_mod(local_mod, &db, false).map_err(e_to_str)?;
    Ok(())
}

#[tauri::command]
pub async fn open_mod_readme(
    unique_name: &str,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let db = state.remote_db.read().await;
    open_readme(unique_name, &db).map_err(e_to_str)?;
    Ok(())
}

#[tauri::command]
pub async fn save_config(
    config: Config,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result<(), String> {
    config.save().map_err(e_to_str)?;
    {
        let mut conf_lock = state.config.write().await;
        *conf_lock = config;
    }
    handle.emit_all("CONFIG_RELOAD", "").ok();
    Ok(())
}

#[tauri::command]
pub async fn get_config(state: tauri::State<'_, State>) -> Result<Config, String> {
    Ok(state.config.read().await.clone())
}

#[tauri::command]
pub async fn save_gui_config(
    gui_config: GuiConfig,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result<(), String> {
    gui_config.save().map_err(e_to_str)?;
    {
        let mut conf_lock = state.gui_config.write().await;
        *conf_lock = gui_config;
    }
    handle.emit_all("GUI_CONFIG_RELOAD", "").ok();
    Ok(())
}

#[tauri::command]
pub async fn get_gui_config(state: tauri::State<'_, State>) -> Result<GuiConfig, String> {
    Ok(state.gui_config.read().await.clone())
}

#[tauri::command]
pub async fn save_owml_config(
    owml_config: OWMLConfig,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result<(), String> {
    let config = state.config.read().await;
    owml_config.save(&config).map_err(e_to_str)?;
    handle.emit_all("OWML_CONFIG_RELOAD", "").ok();
    Ok(())
}

#[tauri::command]
pub async fn get_owml_config(state: tauri::State<'_, State>) -> Result<OWMLConfig, String> {
    let config = state.config.read().await;
    OWMLConfig::get(&config).map_err(e_to_str)
}

#[tauri::command]
pub async fn install_owml(
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result<(), String> {
    let config = state.config.read().await;
    let db = state.remote_db.read().await;
    let owml = db.get_owml().ok_or("Couldn't Find OWML In The Database")?;
    download_and_install_owml(&config, owml)
        .await
        .map_err(e_to_str)?;
    handle.emit_all("OWML_CONFIG_RELOAD", "").ok();
    Ok(())
}

#[tauri::command]
pub async fn set_owml(
    path: &str,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result<bool, String> {
    let path = Path::new(path);
    if path.is_dir() && path.join("OWML.Manifest.json").is_file() {
        let mut config = state.config.write().await;
        config.owml_path = path.to_str().unwrap().to_string();
        config.save().map_err(e_to_str)?;
        handle.emit_all("OWML_CONFIG_RELOAD", "").ok();
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn get_updatable_mods(state: tauri::State<'_, State>) -> Result<Vec<String>, String> {
    let mut updates: Vec<String> = vec![];
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    let config = state.config.read().await;
    for local_mod in local_db.mods.values() {
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
pub async fn update_mod(unique_name: &str, state: tauri::State<'_, State>) -> Result<(), String> {
    let config = state.config.read().await;
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    if unique_name == OWML_UNIQUE_NAME {
        download_and_install_owml(&config, remote_db.get_owml().ok_or("OWML Not Found!")?)
            .await
            .map_err(e_to_str)?;
    } else {
        install_mod_from_db(
            &unique_name.to_string(),
            &config,
            &remote_db,
            &local_db,
            false,
            false,
        )
        .await
        .map_err(e_to_str)?;
    }
    Ok(())
}

#[tauri::command]
pub async fn update_all_mods(
    unique_names: Vec<String>,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let config = state.config.read().await;
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    install_mods_parallel(unique_names, &config, &remote_db, &local_db)
        .await
        .map_err(e_to_str)?;
    Ok(())
}

#[tauri::command]
pub async fn start_logs(handle: tauri::AppHandle) -> Result<(), String> {
    make_log_window(&handle).await.map_err(e_to_str)?;
    Ok(())
}

#[tauri::command]
pub async fn active_log(state: tauri::State<'_, State>) -> Result<bool, String> {
    Ok(state.game_log.read().await.is_some())
}

#[tauri::command]
pub async fn run_game(state: tauri::State<'_, State>, window: tauri::Window) -> Result<(), String> {
    // We have to clone here to prevent locking everything else while the game is running
    let config = state.config.read().await.clone();
    {
        let local_db = state.local_db.read().await;
        let new_config = show_warnings(&window, &local_db, &config).map_err(e_to_str)?;
        {
            let mut config = state.config.write().await;
            *config = new_config;
        }
    }
    let log_server = LogServer::new(0).await.map_err(e_to_str)?;
    let port = log_server.port;
    let logs_path = get_app_path()
        .map_err(e_to_str)?
        .join(format!("game_logs/{}.log", Local::now().timestamp()));
    create_all_parents(&logs_path).map_err(e_to_str)?;
    let file = File::options()
        .read(true)
        .append(true)
        .create(true)
        .open(&logs_path)
        .map_err(|e| format!("Couldn't create log file: {:?}", e))?;
    {
        let mut log_file = state.game_log.write().await;
        let writer = BufWriter::new(file);
        *log_file = Some((vec![], writer));
    }
    let handle_log = |msg: &SocketMessage, _: &u16| {
        let msg = msg.clone();
        let logs_map = state.game_log.clone();
        let window_handle = window.app_handle();
        tokio::spawn(async move {
            let mut log_file = logs_map.write().await;
            if let Some((lines, writer)) = log_file.as_mut() {
                write_log(writer, &msg).unwrap();
                lines.push(GameMessage::new(port, msg));
                window_handle
                    .emit_all("LOG-UPDATE", "")
                    .expect("Can't Send Event");
            }
        });
    };
    window.emit("GAME-START", &port).expect("Can't Send Event");
    try_join!(
        log_server.listen(&handle_log, false),
        launch_game(&config, &port)
    )
    .map_err(|e| format!("Can't Start Game: {:?}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn clear_logs(
    handle: tauri::AppHandle,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let mut data = state.game_log.write().await;
    if let Some((lines, _)) = data.as_mut() {
        lines.clear();
        handle
            .emit_all("LOG-UPDATE", "")
            .map_err(|e| format!("Can't Send Event: {:?}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn stop_logging(state: tauri::State<'_, State>) -> Result<(), String> {
    let mut logs = state.game_log.write().await;
    if let Some((_, ref mut writer)) = logs.as_mut() {
        writer
            .flush()
            .map_err(|e| format!("Error flushing buffer: {:?}", e))?;
    }
    *logs = None;
    Ok(())
}

#[tauri::command]
pub async fn get_log_lines(
    filter_port: Option<LogPort>,
    filter_type: Option<SocketMessageType>,
    state: tauri::State<'_, State>,
) -> Result<Vec<usize>, String> {
    let logs = state.game_log.read().await;
    if let Some((lines, _)) = logs.as_ref() {
        let lines = get_logs_indices(lines, filter_port, filter_type).map_err(e_to_str)?;
        Ok(lines)
    } else {
        Err("Log Server Not Running".to_string())
    }
}

#[tauri::command]
pub async fn get_game_message(
    line: usize,
    state: tauri::State<'_, State>,
) -> Result<GameMessage, String> {
    let logs = state.game_log.read().await;
    if let Some((lines, _)) = logs.as_ref() {
        let msg = lines
            .get(line)
            .ok_or_else(|| "Invalid Log Line".to_string())?;
        Ok(msg.clone())
    } else {
        Err("Log Server Not Running".to_string())
    }
}

#[tauri::command]
pub async fn export_mods(path: String, state: tauri::State<'_, State>) -> Result<(), String> {
    let path = PathBuf::from(path);
    let local_db = state.local_db.read().await;
    let output = owmods_core::io::export_mods(&local_db).map_err(e_to_str)?;
    let file = File::create(path).map_err(|e| format!("Error Saving File: {:?}", e))?;
    let mut writer = BufWriter::new(file);
    write!(&mut writer, "{}", output).map_err(|e| format!("Error Saving File: {:?}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn import_mods(path: String, state: tauri::State<'_, State>) -> Result<(), String> {
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    let config = state.config.read().await;
    let path = PathBuf::from(path);
    owmods_core::io::import_mods(&config, &local_db, &remote_db, &path, false)
        .await
        .map_err(e_to_str)?;
    Ok(())
}
