use owmods_core::db::{fetch_local_db, fetch_remote_db};
use owmods_core::mods::{LocalMod, RemoteMod};
use tauri::Manager;

use crate::State;

use crate::logging::get_logger;

#[tauri::command]
pub fn refresh_local_db(
    handle: tauri::AppHandle,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let log = get_logger(handle.clone());
    let conf = state.config.read().unwrap();
    let local_db = fetch_local_db(&log, &conf).map_err(|err| err.to_string())?;
    {
        let mut db = state.local_db.write().unwrap();
        *db = local_db;
    }
    handle.emit_all("LOCAL-REFRESH", "").ok();
    Ok(())
}

#[tauri::command]
pub fn get_local_mods(state: tauri::State<'_, State>) -> Vec<String> {
    state
        .local_db
        .read()
        .unwrap()
        .mods
        .values()
        .map(|m| m.manifest.unique_name.clone())
        .collect()
}

#[tauri::command]
pub fn get_local_mod(unique_name: &str, state: tauri::State<'_, State>) -> Option<LocalMod> {
    state
        .local_db
        .read()
        .unwrap()
        .get_mod(unique_name).cloned()
}

#[tauri::command]
pub async fn refresh_remote_db(
    handle: tauri::AppHandle,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let log = get_logger(handle.clone());
    // Clones to release lock
    let conf = state.config.read().unwrap().clone();
    let remote_db = fetch_remote_db(&log, &conf)
        .await
        .map_err(|e| e.to_string())?;
    {
        let mut db = state.remote_db.write().unwrap();
        *db = remote_db;
    }
    handle.emit_all("REMOTE-REFRESH", "").ok();
    Ok(())
}

#[tauri::command]
pub fn get_remote_mods(state: tauri::State<'_, State>) -> Vec<String> {
    let db = state.remote_db.read().unwrap();
    let mut mods: Vec<&RemoteMod> = db.mods.values().collect();
    mods.sort_by(|a, b| b.download_count.cmp(&a.download_count));
    mods.into_iter()
        .map(|m| m.unique_name.clone())
        .filter(|m| m != "Alek.OWML")
        .collect::<Vec<String>>()
}

#[tauri::command]
pub fn get_remote_mod(unique_name: &str, state: tauri::State<'_, State>) -> Option<RemoteMod> {
    state
        .remote_db
        .read()
        .unwrap()
        .get_mod(unique_name).cloned()
}
