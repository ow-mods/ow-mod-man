#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufWriter, Write},
    sync::Arc,
};

use commands::*;
use fs_watch::setup_fs_watch;
use game::GameMessage;
use gui_config::GuiConfig;
use log::{debug, error, set_boxed_logger, set_max_level, warn};
use logging::Logger;
use owmods_core::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    file::get_app_path,
    progress::bars::ProgressBars,
    protocol::{ProtocolInstallType, ProtocolPayload},
};

use time::macros::format_description;
use tokio::sync::RwLock as TokioLock;

use crate::events::{CustomEventEmitterAll, Event};

mod commands;
mod events;
mod fs_watch;
mod game;
mod gui_config;
mod logging;

type StatePart<T> = Arc<TokioLock<T>>;
type LogPort = u16;
type LogMessages = HashMap<LogPort, (Vec<GameMessage>, BufWriter<File>)>;

fn manage<T>(obj: T) -> StatePart<T> {
    Arc::new(TokioLock::new(obj))
}

pub struct State {
    /// The local database
    local_db: StatePart<LocalDatabase>,
    /// The remote database
    remote_db: StatePart<RemoteDatabase>,
    /// The current core configuration
    config: StatePart<Config>,
    /// The current GUI configuration
    gui_config: StatePart<GuiConfig>,
    /// A map of ports to the log messages sent to that port
    game_log: StatePart<LogMessages>,
    /// The protocol url used to invoke the program, if any. This is should only be gotten once and removed after
    protocol_url: StatePart<Option<ProtocolPayload>>,
    /// The progress bars of installs/updates/downloads/etc.
    progress_bars: StatePart<ProgressBars>,
    /// A list of unique names of mods that currently have an operation being performed on them
    mods_in_progress: StatePart<Vec<String>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::get(None).unwrap_or(Config::default(None)?);
    let gui_config = GuiConfig::get().unwrap_or_default();
    let local_db = LocalDatabase::fetch(&config.owml_path).unwrap_or_default();
    let remote_db = RemoteDatabase::default();

    tauri_plugin_deep_link::prepare("com.bwc9876.owmods-gui");

    let url = std::env::args().nth(1).map(|s| ProtocolPayload::parse(&s));

    let res = tauri::Builder::default()
        .manage(State {
            local_db: manage(local_db),
            remote_db: manage(remote_db),
            config: manage(config),
            gui_config: manage(gui_config),
            game_log: manage(HashMap::new()),
            protocol_url: manage(url),
            progress_bars: manage(ProgressBars::new()),
            mods_in_progress: manage(vec![]),
        })
        .setup(move |app| {
            // Logger Setup

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

            // Protocol Listener Setup

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
                        handle
                            .typed_emit_all(&Event::ProtocolInvoke(protocol_payload))
                            .ok();
                    }
                }
            });

            if let Err(why) = res {
                warn!("Failed to register URI handler: {:?}", why);
            }

            // File System Watch Setup

            let handle = app.handle();

            let res = setup_fs_watch(handle);

            if let Err(why) = res {
                error!("Failed to setup file watching: {:?}", why);
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
            open_owml,
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
            get_log_lines,
            get_game_message,
            export_mods,
            import_mods,
            fix_mod_deps,
            db_has_issues,
            get_alert,
            pop_protocol_url,
            check_owml,
            get_defaults,
            get_downloads,
            clear_downloads,
            get_busy_mods,
            get_mod_busy,
            has_disabled_deps,
            log_error,
            get_bar_by_unique_name,
            register_drop_handler,
            get_db_tags,
            open_mod_github
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .run(tauri::generate_context!());

    if let Err(why) = res {
        eprintln!("Error: {:?}", why);
        let app_path = get_app_path()?;
        let now = time::OffsetDateTime::now_utc();
        let timestamp_str = now
            .format(format_description!(
                "[year]-[month]-[day]_[hour]-[minute]-[second]"
            ))
            .unwrap();
        let log_path = app_path.join(format!("crash_log_{}.txt", timestamp_str));
        let mut file = File::create(&log_path)?;
        file.write_all(
            format!(
                "The manager encountered a fatal error when starting: {:?}",
                why
            )
            .as_bytes(),
        )?;
        drop(file);
        opener::open(&log_path)?;
        std::process::exit(1);
    }

    Ok(())
}
