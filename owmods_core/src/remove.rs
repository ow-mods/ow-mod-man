use std::{fs::remove_dir_all, path::PathBuf};

use anyhow::Result;

use crate::{db::LocalDatabase, mods::LocalMod};

pub fn remove_mod(local_mod: &LocalMod, db: &LocalDatabase, recursive: bool) -> Result<()> {
    if PathBuf::from(&local_mod.mod_path).is_dir() {
        // In case weird circular dep stuff happens, just don't delete it if it doesn't exist
        remove_dir_all(&local_mod.mod_path)?;
    }
    if recursive {
        let empty: &Vec<String> = &vec![];
        let deps = local_mod.manifest.dependencies.as_ref().unwrap_or(empty);
        for dep in deps.iter() {
            let dep = db.get_mod(dep);
            if let Some(dep) = dep {
                remove_mod(dep, db, true)?;
            }
        }
    }
    Ok(())
}
