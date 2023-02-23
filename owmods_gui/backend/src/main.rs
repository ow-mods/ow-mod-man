#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{error::Error, sync::Arc};

use commands::*;
use gui_config::GuiConfig;
use log::{set_boxed_logger, set_max_level};
use logging::Logger;
use owmods_core::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
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
    let config = Config::get()?;
    let gui_config = GuiConfig::get()?;

    tauri::Builder::default()
        .manage(State {
            local_db: Arc::new(TokioLock::new(LocalDatabase::default())),
            remote_db: Arc::new(TokioLock::new(RemoteDatabase::default())),
            config: Arc::new(TokioLock::new(config)),
            gui_config: Arc::new(TokioLock::new(gui_config)),
        })
        .setup(move |app| {
            set_boxed_logger(Box::new(Logger::new(app.handle())))
                .map(|_| set_max_level(log::LevelFilter::Debug))?;
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
            update_all_mods
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application.");
    Ok(())
}
