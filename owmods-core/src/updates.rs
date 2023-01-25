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
    let remote_mod = if local_mod.manifest.unique_name == "Alek.OWML" {
        remote_db.get_owml()
    } else {
        remote_db.get_mod(&local_mod.manifest.unique_name)
    };
    if let Some(remote_mod) = remote_mod {
        (
            version_compare::compare(remote_mod.get_version(), local_mod.get_version())
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
) -> Result<(), anyhow::Error> {
    let mut needs_updates: Vec<&RemoteMod> = vec![];

    println!("Checking For Updates...");

    let owml_local = local_db.get_owml(&config);
    if owml_local.is_some() {
        let (owml_update, owml_remote) =
            check_mod_needs_update(&owml_local.as_ref().unwrap(), &remote_db);
        if owml_update {
            println!(
                "- OWML (v{}->v{})",
                owml_local.as_ref().unwrap().get_version(),
                owml_remote.unwrap().get_version()
            );
            needs_updates.push(&owml_remote.unwrap());
        } else if let Some(owml_remote) = owml_remote {
            println!(
                "- Skipping OWML (v{} >= v{})",
                owml_local.as_ref().unwrap().get_version(),
                owml_remote.get_version()
            );
        } else {
            println!("- Skipping OWML (No Remote) (What!?)")
        }
    }

    for local_mod in local_db.mods.iter() {
        let (update, new_mod) = check_mod_needs_update(&local_mod, remote_db);
        if update {
            println!(
                "- {} (v{} -> v{})",
                local_mod.manifest.name,
                local_mod.get_version(),
                new_mod.as_ref().unwrap().get_version()
            );
            needs_updates.push(new_mod.unwrap());
        } else if let Some(new_mod) = new_mod {
            println!(
                "- Skipping {} (v{} >= v{})",
                local_mod.manifest.name,
                local_mod.get_version(),
                new_mod.get_version()
            );
        } else {
            println!("- Skipping {} (No Remote)", local_mod.manifest.name);
        }
    }

    if !needs_updates.is_empty() {
        for update_mod in needs_updates.iter() {
            if update_mod.unique_name == "Alek.OWML" {
                download_owml(config, update_mod).await?;
            } else {
                download_mod(config, local_db, remote_db, update_mod, false).await?;
            }
        }
        println!("Update Complete!");
    } else {
        println!("No Updates Available!");
    }
    Ok(())
}
