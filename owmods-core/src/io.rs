use std::path::{Path, PathBuf};

use crate::{download::install_mod_from_db, utils::file::deserialize_from_json};

use super::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    toggle::{get_mod_enabled, toggle_mod},
};

pub fn export_mods(db: &LocalDatabase) -> Result<String, anyhow::Error> {
    let mut enabled_mods: Vec<&String> = vec![];
    for local_mod in db.mods.iter() {
        if get_mod_enabled(&PathBuf::from(&local_mod.mod_path))? {
            enabled_mods.push(&local_mod.manifest.unique_name);
        }
    }
    let result = serde_json::to_string_pretty(&enabled_mods)?;
    Ok(result)
}

pub async fn import_mods(
    config: &Config,
    local_db: &LocalDatabase,
    remote_db: &RemoteDatabase,
    file_path: &Path,
    disable_missing: bool,
) -> Result<(), anyhow::Error> {
    let unique_names: Vec<String> = deserialize_from_json(file_path)?;

    if disable_missing {
        for local_mod in local_db.mods.iter() {
            let mod_path = &PathBuf::from(&local_mod.mod_path);
            if get_mod_enabled(&PathBuf::from(&mod_path))? {
                toggle_mod(mod_path, local_db, false, false)?;
            }
        }
    }
    for name in unique_names.iter() {
        let local_mod = local_db.get_mod(name);
        if let Some(local_mod) = local_mod {
            let mod_path = &PathBuf::from(&local_mod.mod_path);
            if !get_mod_enabled(&PathBuf::from(&mod_path))? {
                toggle_mod(mod_path, local_db, true, false)?;
            }
        } else {
            let remote_mod = remote_db.get_mod(name);
            if let Some(remote_mod) = remote_mod {
                install_mod_from_db(&remote_mod.unique_name, config, remote_db, local_db, false)
                    .await?;
            } else {
                println!("{} Not Found In Database, Skipping...", name);
            }
        }
    }
    Ok(())
}
