use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use log::{error, info};
use notify::{Event, EventKind, RecursiveMode, Watcher};
use owmods_core::config::Config;
use tauri::{AppHandle, Manager};

use crate::{
    events::{CustomEventEmitterAll, Event as CustomEvent},
    gui_config::GuiConfig,
    State,
};

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
            handle_gui
                .typed_emit_all(&CustomEvent::RequestReload("GUI".to_string()))
                .ok();
        }
    })?);

    let settings_watcher = Mutex::new(notify::recommended_watcher(move |res| {
        if check_res(res) {
            handle_config
                .typed_emit_all(&CustomEvent::RequestReload("CONFIG".to_string()))
                .ok();
        }
    })?);

    let mods_watcher = Arc::new(Mutex::new(notify::recommended_watcher(move |res| {
        if check_res(res) {
            handle_local
                .typed_emit_all(&CustomEvent::RequestReload("LOCAL".to_string()))
                .ok();
        }
    })?));

    let gui_path = GuiConfig::path()?;
    let config_path = Config::default_path()?;

    let watch_enabled = Mutex::new(false);

    let e_handle = handle.clone();

    handle.listen_global("owmods://events/invoke", move |e| {
        let payload = e.payload().map(serde_json::from_str);

        if let Some(Ok(payload)) = payload {
            let payload: CustomEvent = payload;

            if let CustomEvent::GuiConfigReload(watch_fs) = payload {
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
                            let manifest_path =
                                PathBuf::from(&config.owml_path).join("OWML.Manifest.json");

                            let mut local_db_watcher = mods_watcher_ref.lock().unwrap();

                            let res = local_db_watcher.watch(&mods_path, RecursiveMode::Recursive);
                            let res2 =
                                local_db_watcher.watch(&manifest_path, RecursiveMode::NonRecursive);
                            if let Err(why) = res {
                                error!("Error starting Mods watcher: {:?}", why);
                            }
                            if let Err(why) = res2 {
                                error!("Error starting Mods watcher: {:?}", why);
                            }
                        });
                    } else if *watch_enabled {
                        info!("File Watcher: Unwatching Filesystem");
                        // GUI
                        let res = gui_watcher.unwatch(&gui_path);
                        if let Err(why) = res {
                            error!("Error stopping GUI watcher: {:?}", why);
                        }

                        // SETTINGS
                        let res = settings_watcher.unwatch(&config_path);
                        if let Err(why) = res {
                            error!("Error stopping Settings watcher: {:?}", why);
                        }

                        // LOCAL DB
                        tokio::spawn(async move {
                            let state = db_handle.state::<State>();
                            let config = state.config.read().await;

                            let mods_path = PathBuf::from(&config.owml_path).join("Mods");
                            let manifest_path =
                                PathBuf::from(&config.owml_path).join("OWML.Manifest.json");

                            let mut local_db_watcher = mods_watcher_ref.lock().unwrap();

                            let res = local_db_watcher.unwatch(&mods_path);
                            let res2 = local_db_watcher.unwatch(&manifest_path);
                            if let Err(why) = res {
                                error!("Error stopping Mods watcher: {:?}", why);
                            }
                            if let Err(why) = res2 {
                                error!("Error stopping Mods watcher: {:?}", why);
                            }
                        });
                    }
                }
                *watch_enabled = watch_fs;
            }
        }
    });

    info!("File Watcher Setup On Standby");

    Ok(())
}
