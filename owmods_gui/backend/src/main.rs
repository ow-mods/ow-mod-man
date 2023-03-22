#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, error::Error, fs::File, io::BufWriter, sync::Arc};

use commands::*;
use gui_config::GuiConfig;
use log::{set_boxed_logger, set_max_level};
use logging::Logger;
use owmods_core::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    socket::SocketMessage,
};

use tokio::sync::RwLock as TokioLock;

mod commands;
mod game;
mod gui_config;
mod logging;

type StatePart<T> = Arc<TokioLock<T>>;
type LogPort = u16;
type LogMessages = HashMap<LogPort, (Vec<SocketMessage>, BufWriter<File>)>;

pub struct State {
    local_db: StatePart<LocalDatabase>,
    remote_db: StatePart<RemoteDatabase>,
    config: StatePart<Config>,
    gui_config: StatePart<GuiConfig>,
    log_files: StatePart<LogMessages>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::default(None)?;
    let gui_config = GuiConfig::default();

    tauri::Builder::default()
        .manage(State {
            local_db: Arc::new(TokioLock::new(LocalDatabase::default())),
            remote_db: Arc::new(TokioLock::new(RemoteDatabase::default())),
            config: Arc::new(TokioLock::new(config)),
            gui_config: Arc::new(TokioLock::new(gui_config)),
            log_files: Arc::new(TokioLock::new(HashMap::new())),
        })
        .setup(move |app| {
            set_boxed_logger(Box::new(Logger::new(app.handle())))
                .map(|_| set_max_level(log::LevelFilter::Debug))?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            initial_setup,
            refresh_local_db,
            get_local_mods,
            get_local_mod,
            refresh_remote_db,
            get_remote_mods,
            get_remote_mod,
            open_mod_folder,
            toggle_mod,
            toggle_all,
            uninstall_mod,
            install_mod,
            install_url,
            install_zip,
            open_mod_readme,
            save_config,
            get_config,
            save_gui_config,
            get_gui_config,
            save_owml_config,
            get_owml_config,
            install_owml,
            set_owml,
            get_updatable_mods,
            update_mod,
            update_all_mods,
            run_game,
            clear_logs,
            stop_logging,
            get_log_lines,
            get_game_message,
            export_mods,
            import_mods
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application.");
    Ok(())
}
