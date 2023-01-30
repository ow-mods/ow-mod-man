use version_compare::Cmp;

use crate::{download::install_mod_from_db, log, logging::Logger};

use super::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    download::download_and_install_owml,
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
    log: &Logger,
    config: &Config,
    local_db: &LocalDatabase,
    remote_db: &RemoteDatabase,
) -> Result<(), anyhow::Error> {
    let mut needs_updates: Vec<&RemoteMod> = vec![];

    log.info("Checking For Updates...");

    let owml_local = local_db.get_owml(config);
    if owml_local.is_some() {
        let (owml_update, owml_remote) =
            check_mod_needs_update(owml_local.as_ref().unwrap(), remote_db);
        if owml_update {
            log!(
                log,
                info,
                "- OWML (v{}->v{})",
                owml_local.as_ref().unwrap().get_version(),
                owml_remote.unwrap().get_version()
            );
            needs_updates.push(owml_remote.unwrap());
        } else if let Some(owml_remote) = owml_remote {
            log!(
                log,
                info,
                "- Skipping OWML (v{} >= v{})",
                owml_local.as_ref().unwrap().get_version(),
                owml_remote.get_version()
            );
        } else {
            log.info("- Skipping OWML (No Remote)... (Guh??)")
        }
    }

    for local_mod in local_db.mods.values().into_iter() {
        let (update, new_mod) = check_mod_needs_update(local_mod, remote_db);
        if update {
            log!(
                log,
                info,
                "- {} (v{} -> v{})",
                local_mod.manifest.name,
                local_mod.get_version(),
                new_mod.as_ref().unwrap().get_version()
            );
            needs_updates.push(new_mod.unwrap());
        } else if let Some(new_mod) = new_mod {
            log!(
                log,
                info,
                "- Skipping {} (v{} >= v{})",
                local_mod.manifest.name,
                local_mod.get_version(),
                new_mod.get_version()
            );
        } else {
            log!(
                log,
                info,
                "- Skipping {} (No Remote)",
                local_mod.manifest.name
            );
        }
    }

    if !needs_updates.is_empty() {
        for update_mod in needs_updates.iter() {
            if update_mod.unique_name == "Alek.OWML" {
                download_and_install_owml(log, config, update_mod).await?;
            } else {
                install_mod_from_db(
                    log,
                    &update_mod.unique_name,
                    config,
                    remote_db,
                    local_db,
                    false,
                )
                .await?;
            }
        }
        log.success("Update Complete!");
    } else {
        log.success("No Updates Available");
    }
    Ok(())
}
