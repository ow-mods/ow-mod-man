#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, error::Error, fs::File, io::BufWriter, sync::Arc};

use commands::*;
use game::GameMessage;
use gui_config::GuiConfig;
use log::{debug, set_boxed_logger, set_max_level, warn};
use logging::Logger;
use owmods_core::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
};

use protocol::{ProtocolInstallType, ProtocolPayload};
use tauri::Manager;
use tokio::sync::RwLock as TokioLock;

mod commands;
mod game;
mod gui_config;
mod logging;
mod protocol;

type StatePart<T> = Arc<TokioLock<T>>;
type LogPort = u16;
type LogMessages = HashMap<LogPort, (Vec<GameMessage>, BufWriter<File>)>;

fn manage<T>(obj: T) -> StatePart<T> {
    Arc::new(TokioLock::new(obj))
}

pub struct State {
    local_db: StatePart<LocalDatabase>,
    remote_db: StatePart<RemoteDatabase>,
    config: StatePart<Config>,
    gui_config: StatePart<GuiConfig>,
    game_log: StatePart<LogMessages>,
    protocol_url: StatePart<Option<ProtocolPayload>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::get(None).unwrap_or(Config::default(None)?);
    let gui_config = GuiConfig::get().unwrap_or_default();
    let local_db = LocalDatabase::fetch(&config.owml_path).unwrap_or_default();
    let remote_db = RemoteDatabase::default();

    tauri_plugin_deep_link::prepare("com.bwc9876.owmods-gui");

    let url = std::env::args().nth(1).map(|s| ProtocolPayload::parse(&s));

    tauri::Builder::default()
        .manage(State {
            local_db: manage(local_db),
            remote_db: manage(remote_db),
            config: manage(config),
            gui_config: manage(gui_config),
            game_log: manage(HashMap::new()),
            protocol_url: manage(url),
        })
        .setup(move |app| {
            let logger = Logger::new(app.handle());
            logger
                .write_log_to_file(
                    log::Level::Info,
                    &format!(
                        "Start of Outer Wilds Mod Manager v{}",
                        env!("CARGO_PKG_VERSION")
                    ),
                )
                .ok();
            set_boxed_logger(Box::new(logger)).map(|_| set_max_level(log::LevelFilter::Debug))?;

            let handle = app.handle();

            let res = tauri_plugin_deep_link::register("owmods", move |request| {
                let protocol_payload = ProtocolPayload::parse(&request);
                match protocol_payload.install_type {
                    ProtocolInstallType::Unknown => {}
                    _ => {
                        debug!(
                            "Invoking {:?} with {} from protocol",
                            protocol_payload.install_type, protocol_payload.payload
                        );
                        handle.emit_all("PROTOCOL_INVOKE", protocol_payload).ok();
                    }
                }
            });

            if let Err(why) = res {
                warn!("Failed to register URI handler: {:?}", why);
            }

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
            uninstall_broken_mod,
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
            active_log,
            start_logs,
            run_game,
            clear_logs,
            stop_logging,
            get_log_lines,
            get_game_message,
            export_mods,
            import_mods,
            fix_mod_deps,
            db_has_issues,
            get_alert,
            get_watcher_paths,
            pop_protocol_url
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_fs_watch::init())
        .run(tauri::generate_context!())
        .expect("Error while running tauri application.");
    Ok(())
}
