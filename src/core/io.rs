use std::{fs::File, io::Read, path::PathBuf};

use super::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    download::download_mod,
    toggle::{get_mod_enabled, toggle_mod},
};

pub fn export_mods(db: &LocalDatabase) -> String {
    let mut enabled_mods: Vec<&String> = vec![];
    for local_mod in db.mods.iter() {
        if get_mod_enabled(&PathBuf::from(&local_mod.mod_path)) {
            enabled_mods.push(&local_mod.manifest.unique_name);
        }
    }
    serde_json::to_string_pretty(&enabled_mods).expect("Couldn't Export Mods")
}

pub async fn import_mods(
    config: &Config,
    local_db: &LocalDatabase,
    remote_db: &RemoteDatabase,
    file_path: &PathBuf,
    disable_missing: bool,
) {
    let mut file = File::open(file_path).expect("Couldn't Open File To Import");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Couldn't Read File");
    let unique_names: Vec<String> = serde_json::from_str(&buf).expect("Couldn't Parse File");

    if disable_missing {
        for local_mod in local_db.mods.iter() {
            let mod_path = &PathBuf::from(&local_mod.mod_path);
            if get_mod_enabled(&PathBuf::from(&mod_path)) {
                toggle_mod(mod_path, local_db, false, false);
            }
        }
    }
    for name in unique_names.iter() {
        let local_mod = local_db.get_mod(name);
        if let Some(local_mod) = local_mod {
            let mod_path = &PathBuf::from(&local_mod.mod_path);
            if !get_mod_enabled(&PathBuf::from(&mod_path)) {
                toggle_mod(mod_path, local_db, true, false);
            }
        } else {
            let remote_mod = remote_db.get_mod(name);
            if let Some(remote_mod) = remote_mod {
                download_mod(config, local_db, remote_db, remote_mod, false).await;
            } else {
                println!("{} Not Found In Database, Skipping...", name);
            }
        }
    }
}
