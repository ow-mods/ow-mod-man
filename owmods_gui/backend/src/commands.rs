use owmods_core::db::{fetch_local_db, fetch_remote_db};
use owmods_core::mods::{LocalMod, RemoteMod};
use tauri::Manager;

use crate::State;

use crate::logging::get_logger;

#[tauri::command]
pub async fn refresh_local_db(
    handle: tauri::AppHandle,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let log = get_logger(handle.clone());
    let conf = state.config.read().await;
    {
        let mut db = state.local_db.write().await;
        handle.emit_all("LOCAL-REFRESH", "").ok();
        let local_db = fetch_local_db(&log, &conf).map_err(|err| err.to_string())?;
        *db = local_db;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_local_mods(state: tauri::State<'_, State>) -> Result<Vec<String>, ()> {
    let db = state.local_db.read().await;
    let mut mods: Vec<&LocalMod> = db.mods.values().collect();
    mods.sort_by(|a, b| a.manifest.name.cmp(&b.manifest.name));
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
    Ok(state.local_db.read().await.get_mod(unique_name).cloned())
}

#[tauri::command]
pub async fn refresh_remote_db(
    handle: tauri::AppHandle,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let log = get_logger(handle.clone());
    let conf = state.config.read().await;
    {
        let mut db = state.remote_db.write().await;
        handle.emit_all("REMOTE-REFRESH", "").ok();
        let remote_db = fetch_remote_db(&log, &conf)
            .await
            .map_err(|e| e.to_string())?;
        *db = remote_db;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_remote_mods(state: tauri::State<'_, State>) -> Result<Vec<String>, ()> {
    let db = state.remote_db.read().await;
    let mut mods: Vec<&RemoteMod> = db.mods.values().collect();
    mods.sort_by(|a, b| b.download_count.cmp(&a.download_count));
    Ok(mods
        .into_iter()
        .map(|m| m.unique_name.clone())
        .filter(|m| m != "Alek.OWML")
        .collect::<Vec<String>>())
}

#[tauri::command]
pub async fn get_remote_mod(
    unique_name: &str,
    state: tauri::State<'_, State>,
) -> Result<Option<RemoteMod>, ()> {
    Ok(state.remote_db.read().await.get_mod(unique_name).cloned())
}
