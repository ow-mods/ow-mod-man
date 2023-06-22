use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use log::{error, info};
use notify::{Event, EventKind, RecursiveMode, Watcher};
use owmods_core::config::Config;
use tauri::{AppHandle, Manager};

use crate::{gui_config::GuiConfig, State};

fn check_res(res: Result<Event, notify::Error>) -> bool {
    if let Ok(e) = res {
        matches!(
            e.kind,
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
        )
    } else {
        false
    }
}

pub fn setup_fs_watch(handle: AppHandle) -> Result<()> {
    let handle_gui = handle.clone();
    let handle_config = handle.clone();
    let handle_local = handle.clone();

    let gui_settings_watcher = Mutex::new(notify::recommended_watcher(move |res| {
        if check_res(res) {
            handle_gui.emit_all("REQUEST-RELOAD", "GUI").ok();
        }
    })?);

    let settings_watcher = Mutex::new(notify::recommended_watcher(move |res| {
        if check_res(res) {
            handle_config.emit_all("REQUEST-RELOAD", "CONFIG").ok();
        }
    })?);

    let mods_watcher = Arc::new(Mutex::new(notify::recommended_watcher(move |res| {
        if check_res(res) {
            handle_local.emit_all("REQUEST-RELOAD", "LOCAL").ok();
        }
    })?));

    let gui_path = GuiConfig::path()?;
    let config_path = Config::default_path()?;

    let watch_enabled = Mutex::new(false);

    let e_handle = handle.clone();

    handle.listen_global("GUI_CONFIG_RELOAD", move |e| {
        let watch_fs = e
            .payload()
            .map(|v| serde_json::from_str(v).unwrap())
            .unwrap_or(false);

        let mut gui_watcher = gui_settings_watcher.lock().unwrap();
        let mut settings_watcher = settings_watcher.lock().unwrap();
        let db_handle = e_handle.clone();
        let mods_watcher_ref = mods_watcher.clone();

        let mut watch_enabled = watch_enabled.lock().unwrap();
        if *watch_enabled != watch_fs {
            if watch_fs {
                info!("File Watcher: Watching Filesystem");
                // GUI
                let res = gui_watcher.watch(&gui_path, RecursiveMode::NonRecursive);
                if let Err(why) = res {
                    error!("Error starting GUI settings watcher: {:?}", why);
                }

                // SETTINGS
                let res = settings_watcher.watch(&config_path, RecursiveMode::NonRecursive);
                if let Err(why) = res {
                    error!("Error starting Settings watcher: {:?}", why);
                }

                // LOCAL DB
                tokio::spawn(async move {
                    let state = db_handle.state::<State>();
                    let config = state.config.read().await;

                    let mods_path = PathBuf::from(&config.owml_path).join("Mods");

                    let mut local_db_watcher = mods_watcher_ref.lock().unwrap();

                    let res = local_db_watcher.watch(&mods_path, RecursiveMode::Recursive);
                    if let Err(why) = res {
                        error!("Error starting Mods watcher: {:?}", why);
                    }
                });
            } else if *watch_enabled {
                info!("File Watcher: Unwatching Filesystem");
                // GUI
                let res = gui_watcher.unwatch(&gui_path);
                if let Err(why) = res {
                    if !matches!(why.kind, notify::ErrorKind::WatchNotFound) {
                        error!("Error stopping GUI watcher: {:?}", why);
                    }
                }

                // SETTINGS
                let res = settings_watcher.unwatch(&config_path);
                if let Err(why) = res {
                    if !matches!(why.kind, notify::ErrorKind::WatchNotFound) {
                        error!("Error stopping Settings watcher: {:?}", why);
                    }
                }

                // LOCAL DB
                tokio::spawn(async move {
                    let state = db_handle.state::<State>();
                    let config = state.config.read().await;

                    let mods_path = PathBuf::from(&config.owml_path).join("Mods");

                    let mut local_db_watcher = mods_watcher_ref.lock().unwrap();

                    let res = local_db_watcher.unwatch(&mods_path);
                    if let Err(why) = res {
                        if !matches!(why.kind, notify::ErrorKind::WatchNotFound) {
                            error!("Error stopping Mods watcher: {:?}", why);
                        }
                    }
                });
            }
        }
        *watch_enabled = watch_fs;
    });

    info!("File Watcher Setup On Standby");

    Ok(())
}
