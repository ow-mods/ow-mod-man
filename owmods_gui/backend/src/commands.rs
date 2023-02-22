use std::path::{Path, PathBuf};

use owmods_core::config::Config;
use owmods_core::db::{LocalDatabase, RemoteDatabase};
use owmods_core::download::{
    download_and_install_owml, install_mod_from_db, install_mod_from_url, install_mod_from_zip,
    install_mods_parallel,
};
use owmods_core::mods::{LocalMod, OWMLConfig, RemoteMod};
use owmods_core::open::{open_readme, open_shortcut};
use owmods_core::remove::remove_mod;
use owmods_core::updates::{check_mod_needs_update, update_all};
use rust_fuzzy_search::fuzzy_compare;
use tauri::Manager;

use crate::State;

use crate::gui_config::GuiConfig;

fn e_to_str(e: anyhow::Error) -> String {
    e.to_string()
}

const SEARCH_THRESHOLD: f32 = 0.04;

#[tauri::command]
pub async fn refresh_local_db(
    handle: tauri::AppHandle,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let conf = state.config.read().await;
    {
        let mut db = state.local_db.write().await;
        handle.emit_all("LOCAL-REFRESH", "").ok();
        let local_db = LocalDatabase::fetch(&conf);
        *db = local_db.unwrap_or_else(|_| LocalDatabase::default());
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
    let conf = state.config.read().await;
    {
        let mut db = state.remote_db.write().await;
        handle.emit_all("REMOTE-REFRESH", "").ok();
        let remote_db = RemoteDatabase::fetch(&conf).await;
        *db = remote_db.unwrap_or_else(|_| RemoteDatabase::default());
    }
    Ok(())
}

#[tauri::command]
pub async fn get_remote_mods(
    filter: &str,
    state: tauri::State<'_, State>,
) -> Result<Vec<String>, ()> {
    let db = state.remote_db.read().await;
    let mut mods: Vec<&RemoteMod> = db
        .mods
        .values()
        .filter(|m| m.unique_name != "Alek.OWML")
        .collect();
    if filter.is_empty() {
        mods.sort_by(|a, b| b.download_count.cmp(&a.download_count));
    } else {
        let mut scores: Vec<(&RemoteMod, f32)> = mods
            .into_iter()
            .filter_map(|m| {
                let score = fuzzy_compare(
                    &format!(
                        "{} {} {}",
                        &m.name.to_ascii_lowercase(),
                        &m.get_author().to_ascii_lowercase(),
                        &m.description.to_ascii_lowercase()
                    ),
                    &filter.to_ascii_lowercase(),
                );
                if score >= SEARCH_THRESHOLD {
                    Some((m, score))
                } else {
                    None
                }
            })
            .collect();
        scores.sort_by(|(_, a), (_, b)| b.total_cmp(a));
        mods = scores.into_iter().map(|(m, _)| m).collect();
    }
    Ok(mods.into_iter().map(|m| m.unique_name.clone()).collect())
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
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    let db = state.local_db.read().await;
    let path = db.get_mod_path(unique_name);
    if let Some(path) = path {
        owmods_core::toggle::toggle_mod(&path, &db, enabled, false).map_err(e_to_str)?;
        Ok(())
    } else {
        Err(format!("Mod {} not found", unique_name))
    }
}

#[tauri::command]
pub async fn install_mod(
    unique_name: &str,
    prerelease: Option<bool>,
    handle: tauri::AppHandle,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    handle.emit_all("INSTALL-START", unique_name).ok();
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    let conf = state.config.read().await;
    install_mod_from_db(
        &unique_name.to_string(),
        &conf,
        &remote_db,
        &local_db,
        true,
        prerelease.unwrap_or(false),
    )
    .await
    .map_err(e_to_str)?;
    handle.emit_all("INSTALL-FINISH", unique_name).ok();
    Ok(())
}

#[tauri::command]
pub async fn install_url(
    url: &str,
    handle: tauri::AppHandle,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    handle.emit_all("INSTALL-START", url).ok();
    let conf = state.config.read().await;
    let db = state.local_db.read().await;
    install_mod_from_url(url, &conf, &db)
        .await
        .map_err(e_to_str)?;
    handle.emit_all("INSTALL-FINISH", url).ok();
    Ok(())
}

#[tauri::command]
pub async fn install_zip(
    path: &str,
    handle: tauri::AppHandle,
    state: tauri::State<'_, State>,
) -> Result<(), String> {
    handle.emit_all("INSTALL-START", path).ok();
    let conf = state.config.read().await;
    let db = state.local_db.read().await;
    println!("Installing {}", path);
    install_mod_from_zip(&PathBuf::from(path), &conf, &db).map_err(e_to_str)?;
    handle.emit_all("INSTALL-FINISH", path).ok();
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

#[tauri::command]
pub async fn save_config(
    config: Config,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result<(), String> {
    config.save().map_err(e_to_str)?;
    {
        let mut conf_lock = state.config.write().await;
        *conf_lock = config;
    }
    handle.emit_all("CONFIG_RELOAD", "").ok();
    Ok(())
}

#[tauri::command]
pub async fn fetch_config(state: tauri::State<'_, State>) -> Result<Config, String> {
    Ok(state.config.read().await.clone())
}

#[tauri::command]
pub async fn save_gui_config(
    gui_config: GuiConfig,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result<(), String> {
    gui_config.save().map_err(e_to_str)?;
    {
        let mut conf_lock = state.gui_config.write().await;
        *conf_lock = gui_config;
    }
    handle.emit_all("GUI_CONFIG_RELOAD", "").ok();
    Ok(())
}

#[tauri::command]
pub async fn get_gui_config(state: tauri::State<'_, State>) -> Result<GuiConfig, String> {
    Ok(state.gui_config.read().await.clone())
}

#[tauri::command]
pub async fn save_owml_config(
    owml_config: OWMLConfig,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result<(), String> {
    let config = state.config.read().await;
    owml_config.save(&config).map_err(e_to_str)?;
    handle.emit_all("OWML_CONFIG_RELOAD", "").ok();
    Ok(())
}

#[tauri::command]
pub async fn get_owml_config(state: tauri::State<'_, State>) -> Result<OWMLConfig, String> {
    let config = state.config.read().await;
    OWMLConfig::get(&config).map_err(e_to_str)
}

#[tauri::command]
pub async fn install_owml(
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result<(), String> {
    let config = state.config.read().await;
    let db = state.remote_db.read().await;
    let owml = db.get_owml().ok_or("Couldn't Find OWML In The Database")?;
    download_and_install_owml(&config, owml)
        .await
        .map_err(e_to_str)?;
    handle.emit_all("OWML_CONFIG_RELOAD", "").ok();
    Ok(())
}

#[tauri::command]
pub async fn set_owml(
    path: &str,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result<bool, String> {
    let path = Path::new(path);
    if path.is_dir() && path.join("OWML.Manifest.json").is_file() {
        let mut config = state.config.write().await;
        config.owml_path = path.to_str().unwrap().to_string();
        config.save().map_err(e_to_str)?;
        handle.emit_all("OWML_CONFIG_RELOAD", "").ok();
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
pub async fn get_updatable_mods(state: tauri::State<'_, State>) -> Result<Vec<String>, String> {
    let mut updates: Vec<String> = vec![];
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    for local_mod in local_db.mods.values() {
        let (needs_update, _) = check_mod_needs_update(local_mod, &remote_db);
        if needs_update {
            updates.push(local_mod.manifest.unique_name.clone());
        }
    }
    Ok(updates)
}

#[tauri::command]
pub async fn update_mod(
    unique_name: &str,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result<(), String> {
    handle.emit_all("INSTALL-START", unique_name).ok();
    let config = state.config.read().await;
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    update_all(&config, &local_db, &remote_db)
        .await
        .map_err(e_to_str)?;
    handle.emit_all("INSTALL-FINISH", unique_name).ok();
    Ok(())
}

#[tauri::command]
pub async fn update_all_mods(
    unique_names: Vec<String>,
    state: tauri::State<'_, State>,
    handle: tauri::AppHandle,
) -> Result<(), String> {
    handle.emit_all("INSTALL-START", "").ok();
    let config = state.config.read().await;
    let local_db = state.local_db.read().await;
    let remote_db = state.remote_db.read().await;
    install_mods_parallel(unique_names, &config, &remote_db, &local_db)
        .await
        .map_err(e_to_str)?;
    handle.emit_all("INSTALL-FINISH", "").ok();
    Ok(())
}
