use owmods_core::db::{fetch_local_db, fetch_remote_db};
use owmods_core::download::install_mod_from_db;
use owmods_core::mods::{LocalMod, RemoteMod};
use owmods_core::open::{open_readme, open_shortcut};
use owmods_core::remove::remove_mod;
use tauri::Manager;

use crate::State;

use crate::logging::get_logger;

fn e_to_str(e: anyhow::Error) -> String {
    e.to_string()
}

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
        let local_db = fetch_local_db(&log, &conf).map_err(e_to_str)?;
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
        let remote_db = fetch_remote_db(&log, &conf).await.map_err(e_to_str)?;
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

#[tauri::command]
pub async fn open_mod_folder(
    unique_name: &str,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let db = state.local_db.read().await;
    let conf = state.config.read().await;
    open_shortcut(unique_name, &conf, &db).map_err(e_to_str)?;
    Ok(())
}

#[tauri::command]
pub async fn toggle_mod(
    unique_name: &str,
    enabled: bool,
    handle: tauri::AppHandle,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let db = state.local_db.read().await;
    let log = get_logger(handle);
    let path = db.get_mod_path(unique_name);
    if let Some(path) = path {
        owmods_core::toggle::toggle_mod(&log, &path, &db, enabled, false).map_err(e_to_str)?;
        Ok(())
    } else {
        Err(format!("Mod {} not found", unique_name))
    }
}

#[tauri::command]
pub async fn install_mod(
    unique_name: &str,
    handle: tauri::AppHandle,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    handle.emit_all("INSTALL-START", unique_name).ok();
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    let conf = state.config.read().await;
    let log = get_logger(handle.clone());
    install_mod_from_db(
        &log,
        &unique_name.to_string(),
        &conf,
        &remote_db,
        &local_db,
        true,
    )
    .await
    .map_err(e_to_str)?;
    handle.emit_all("INSTALL-FINISH", unique_name).ok();
    Ok(())
}

#[tauri::command]
pub async fn uninstall_mod(
    unique_name: &str,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let db = state.local_db.read().await;
    let local_mod = db
        .get_mod(unique_name)
        .ok_or_else(|| format!("Mod {} not found", unique_name))?;
    remove_mod(local_mod, &db, false).map_err(e_to_str)?;
    Ok(())
}

#[tauri::command]
pub async fn open_mod_readme(
    unique_name: &str,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let db = state.remote_db.read().await;
    open_readme(unique_name, &db).map_err(e_to_str)?;
    Ok(())
}
