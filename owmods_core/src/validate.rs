use std::path::PathBuf;

use crate::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    download::install_mod_from_db,
    logging::Logger,
    mods::LocalMod,
    toggle::toggle_mod,
};

/// Returns missing and disabled mod dependencies
pub fn check_deps<'a>(
    local_mod: &'a LocalMod,
    db: &'a LocalDatabase,
) -> (Vec<&'a String>, Vec<&'a LocalMod>) {
    let mut missing: Vec<&String> = vec![];
    let mut disabled: Vec<&LocalMod> = vec![];
    if let Some(deps) = &local_mod.manifest.dependencies {
        for unique_name in deps {
            if let Some(dep_mod) = db.get_mod(unique_name) {
                if !dep_mod.enabled {
                    disabled.push(dep_mod);
                }
            } else {
                missing.push(unique_name);
            }
        }
    }
    (missing, disabled)
}

pub async fn fix_deps(
    log: &Logger,
    config: &Config,
    db: &LocalDatabase,
    remote_db: &RemoteDatabase,
) -> Result<(), anyhow::Error> {
    for local_mod in db.active().iter() {
        let (missing, disabled) = check_deps(local_mod, db);
        for disabled in disabled.iter() {
            toggle_mod(
                log,
                &PathBuf::from(disabled.mod_path.to_owned()),
                db,
                true,
                true,
            )?;
        }
        for missing in missing.iter() {
            install_mod_from_db(log, missing, config, remote_db, db, true).await?;
        }
    }
    Ok(())
}

pub fn check_conflicts<'a>(local_mod: &'a LocalMod, db: &'a LocalDatabase) -> Vec<&'a String> {
    let mut conflicting: Vec<&String> = vec![];
    let active_mods: Vec<&String> = db
        .active()
        .iter()
        .map(|m| &m.manifest.unique_name)
        .collect();
    if let Some(conflicts) = &local_mod.manifest.conflicts {
        for conflict in conflicts.iter() {
            if active_mods.contains(&conflict) {
                conflicting.push(conflict);
            }
        }
    }
    conflicting
}

/// Simply check if there's errors, no details
pub fn has_errors(db: &LocalDatabase) -> bool {
    for local_mod in db.active().iter() {
        let (missing, disabled) = check_deps(local_mod, db);
        let conflicts = check_conflicts(local_mod, db);
        if !missing.is_empty() || !disabled.is_empty() || !conflicts.is_empty() {
            return true;
        }
    }
    false
}
