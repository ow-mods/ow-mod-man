use version_compare::Cmp;

use super::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    download::{download_mod, download_owml},
    mods::{LocalMod, RemoteMod},
};

fn check_mod_needs_update<'a>(
    local_mod: &'a LocalMod,
    remote_db: &'a RemoteDatabase,
) -> (bool, Option<&'a RemoteMod>) {
    let remote_mod = remote_db.get_mod(&local_mod.manifest.unique_name);
    if let Some(remote_mod) = remote_mod {
        (
            version_compare::compare(
                remote_mod.version.replace('v', ""),
                local_mod.manifest.version.replace('v', ""),
            )
            .unwrap_or(Cmp::Eq)
                == Cmp::Gt,
            Some(remote_mod),
        )
    } else {
        (false, None)
    }
}

pub async fn check_for_updates(
    config: &Config,
    local_db: &LocalDatabase,
    remote_db: &RemoteDatabase,
) {
    let mut needs_updates: Vec<&RemoteMod> = vec![];

    println!("Checking For Updates...");
    for local_mod in local_db.mods.iter() {
        let (update, new_mod) = check_mod_needs_update(local_mod, remote_db);
        if update {
            println!(
                "- {} (v{} -> {})",
                local_mod.manifest.name,
                local_mod.manifest.version,
                new_mod.as_ref().unwrap().version
            );
            needs_updates.push(new_mod.unwrap());
        } else if let Some(new_mod) = new_mod {
            println!(
                "- Skipping {} (v{} >= {})",
                local_mod.manifest.name, local_mod.manifest.version, new_mod.version
            );
        } else {
            println!("- Skipping {} (No Remote)", local_mod.manifest.name);
        }
    }

    if !needs_updates.is_empty() {
        for update_mod in needs_updates.iter() {
            if update_mod.unique_name == "Alek.OWML" {
                download_owml(config, update_mod).await;
            } else {
                download_mod(config, local_db, remote_db, update_mod, false).await;
            }
        }
        println!("Update Complete!");
    } else {
        println!("No Updates Available!");
    }
}
