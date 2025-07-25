#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashMap, error::Error, fs::File, io::Write, sync::Arc};

use commands::*;
use fs_watch::setup_fs_watch;
use game::LogData;
use gui_config::GuiConfig;
use log::{debug, error, set_boxed_logger, set_max_level, warn};
use logging::Logger;
use owmods_core::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    file::get_app_path,
    progress::bars::ProgressBars,
    protocol::ProtocolPayload,
};
use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;

use anyhow::anyhow;
use time::macros::format_description;
use tokio::sync::RwLock as TokioLock;

mod commands;
mod error;
mod events;
mod fs_watch;
mod game;
mod gui_config;
mod logging;
mod protocol;

pub type StatePart<T> = Arc<TokioLock<T>>;
type LogPort = u16;
type LogMessages = HashMap<LogPort, LogData>;

fn manage<T>(obj: T) -> StatePart<T> {
    Arc::new(TokioLock::new(obj))
}

#[derive(Clone)]
pub enum RemoteDatabaseOption {
    PreInit,
    Loading,
    Connected(Box<RemoteDatabase>),
    Error(error::Error),
}

impl RemoteDatabaseOption {
    pub fn is_pending(&self) -> bool {
        matches!(
            self,
            RemoteDatabaseOption::PreInit | RemoteDatabaseOption::Loading
        )
    }

    pub fn get(&self) -> Option<&RemoteDatabase> {
        match self {
            RemoteDatabaseOption::Connected(db) => Some(db),
            _ => None,
        }
    }

    pub fn try_get(&self) -> Result<&RemoteDatabase, error::Error> {
        match self {
            RemoteDatabaseOption::Connected(db) => Ok(db),
            RemoteDatabaseOption::Error(err) => Err(err.clone()),
            _ => Err(anyhow!("Remote database not loaded yet".to_string()).into()),
        }
    }
}

pub struct State {
    /// The local database
    local_db: StatePart<LocalDatabase>,
    /// The remote database
    remote_db: StatePart<RemoteDatabaseOption>,
    /// The current core configuration
    config: StatePart<Config>,
    /// The current GUI configuration
    gui_config: StatePart<GuiConfig>,
    /// A map of ports to the log messages sent to that port
    game_log: StatePart<LogMessages>,
    /// The protocol url used to invoke the program, if any. This is should only be gotten once and removed after
    protocol_url: StatePart<Option<ProtocolPayload>>,
    /// How many protocol listeners are currently active
    protocol_listeners: StatePart<Vec<String>>,
    /// The progress bars of installs/updates/downloads/etc.
    progress_bars: StatePart<ProgressBars>,
    /// A list of unique names of mods that currently have an operation being performed on them
    mods_in_progress: StatePart<Vec<String>>,
}

async fn update(app: AppHandle) -> tauri_plugin_updater::Result<()> {
    log::debug!("Checking for Manager Updates");
    if let Some(update) = app.updater()?.check().await? {
        let mut bytes: usize = 0;
        let mut last_debounce: usize = 0;
        log::info!("Manager Update Found! (${})", update.version);
        update
            .download_and_install(
                |recv, tot| {
                    bytes = bytes.saturating_add(recv);
                    let debounce = bytes / 1000000;
                    if last_debounce != debounce {
                        last_debounce = debounce;
                        log::debug!(
                            "Download Update ({}/{})",
                            bytes / 1000,
                            tot.unwrap_or(u64::MAX) / 1000
                        );
                    }
                },
                || {
                    log::debug!("Download complete! Installing");
                },
            )
            .await?;
        log::info!("Manager Update Complete");
        Ok(())
    } else {
        log::debug!("No Manager Updates");
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::get(None).unwrap_or(Config::default(None)?);
    let gui_config = GuiConfig::get().unwrap_or_default();
    let local_db = LocalDatabase::fetch(&config.owml_path).unwrap_or_default();

    let log_file = gui_config.manager_logs;

    let url = std::env::args().nth(1).map(|s| ProtocolPayload::parse(&s));

    let res = tauri::Builder::default()
        .manage(State {
            local_db: manage(local_db),
            remote_db: manage(RemoteDatabaseOption::PreInit),
            config: manage(config),
            gui_config: manage(gui_config),
            game_log: manage(HashMap::new()),
            protocol_url: manage(url),
            protocol_listeners: manage(Vec::with_capacity(2)),
            progress_bars: manage(ProgressBars::new()),
            mods_in_progress: manage(Vec::with_capacity(4)),
        })
        .plugin(tauri_plugin_single_instance::init(|_app, _argv, _cwd| {
            println!("New app instance opened, invoked URI handler.");
        }))
        .plugin(tauri_plugin_deep_link::init())
        .setup(move |app| {
            // Logger Setup

            let logger = Logger::new(app.handle().clone(), log_file);
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

            // Window State Setup
            let res = app
                .handle()
                .plugin(tauri_plugin_window_state::Builder::default().build());

            if let Err(why) = res {
                warn!("Failed to setup window state plugin: {why:?}");
            } else {
                debug!("Setup window state");
            }

            // Update Setup
            let update_handle = app.handle().clone();

            tauri::async_runtime::spawn(async {
                update_handle
                    .plugin(tauri_plugin_updater::Builder::new().build())
                    .unwrap();
                if let Err(why) = update(update_handle).await {
                    warn!("Failed to update: {why:?}");
                }
            });

            // Protocol Listener Setup

            protocol::prep_protocol(app.handle().clone());

            // File System Watch Setup

            let handle = app.handle();

            let res = setup_fs_watch(handle.clone());

            if let Err(why) = res {
                error!("Failed to setup file watching: {why:?}");
            }

            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
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
            dismiss_alert,
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
            open_mod_github,
            force_log_update,
            show_log_folder
        ])
        .run(tauri::generate_context!());

    if let Err(why) = res {
        eprintln!("Error: {why:?}");
        let app_path = get_app_path()?;
        let now = time::OffsetDateTime::now_utc();
        let timestamp_str = now
            .format(format_description!(
                "[year]-[month]-[day]_[hour]-[minute]-[second]"
            ))
            .unwrap();
        let log_path = app_path.join(format!("crash_log_{timestamp_str}.txt"));
        let mut file = File::create(&log_path)?;
        file.write_all(
            format!("The manager encountered a fatal error when starting: {why:?}").as_bytes(),
        )?;
        drop(file);
        opener::open(&log_path)?;
        std::process::exit(1);
    }

    Ok(())
}
