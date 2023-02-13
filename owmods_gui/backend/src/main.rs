#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{error::Error, sync::Arc};

use commands::*;
use gui_config::GuiConfig;
use logging::get_logger;
use owmods_core::{
    config::{get_config, Config},
    db::{LocalDatabase, RemoteDatabase},
    logging::{BasicConsoleBackend, Logger},
};

use tokio::sync::RwLock as TokioLock;

mod commands;
mod gui_config;
mod logging;

pub struct State {
    local_db: Arc<TokioLock<LocalDatabase>>,
    remote_db: Arc<TokioLock<RemoteDatabase>>,
    config: Arc<TokioLock<Config>>,
    gui_config: Arc<TokioLock<GuiConfig>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let basic_console = BasicConsoleBackend;
    let temp_logger = Logger::new(Box::new(basic_console));

    let config = get_config(&temp_logger)?;
    let gui_config = GuiConfig::get()?;

    tauri::Builder::default()
        .manage(State {
            local_db: Arc::new(TokioLock::new(LocalDatabase::empty())),
            remote_db: Arc::new(TokioLock::new(RemoteDatabase::empty())),
            config: Arc::new(TokioLock::new(config)),
            gui_config: Arc::new(TokioLock::new(gui_config)),
        })
        .setup(move |app| {
            get_logger(app.handle()).debug("Starting App");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            refresh_local_db,
            get_local_mods,
            get_local_mod,
            refresh_remote_db,
            get_remote_mods,
            get_remote_mod,
            open_mod_folder,
            toggle_mod,
            uninstall_mod,
            install_mod,
            open_mod_readme,
            save_config,
            fetch_config,
            save_gui_config,
            get_gui_config,
            save_owml_config,
            get_owml_config
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application.");

    Ok(())
}
