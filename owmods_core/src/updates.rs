use version_compare::Cmp;

use crate::{download::install_mods_parallel, log, logging::Logger};

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

pub async fn update_all(
    log: &Logger,
    config: &Config,
    local_db: &LocalDatabase,
    remote_db: &RemoteDatabase,
) -> Result<bool, anyhow::Error> {
    let mut needs_update: Vec<&RemoteMod> = vec![];

    for local_mod in local_db.mods.values() {
        let (update, remote_mod) = check_mod_needs_update(local_mod, remote_db);
        if update {
            log!(
                log,
                info,
                "{}: {} -> {}",
                local_mod.manifest.name,
                local_mod.get_version(),
                remote_mod.unwrap().get_version()
            );
            needs_update.push(remote_mod.unwrap());
        }
    }

    let owml = local_db.get_owml(config);

    if owml.is_some() {
        let (update, remote_owml) = check_mod_needs_update(owml.as_ref().unwrap(), remote_db);
        if update {
            log!(
                log,
                info,
                "OWML: {} -> {}",
                owml.as_ref().unwrap().get_version(),
                remote_owml.unwrap().get_version()
            );
            download_and_install_owml(log, config, remote_owml.unwrap()).await?;
        }
    }

    if needs_update.is_empty() {
        Ok(false)
    } else {
        let mod_names = needs_update.into_iter()
            .map(|m| m.unique_name.clone())
            .collect();
        install_mods_parallel(log, mod_names, config, remote_db, local_db).await?;
        Ok(true)
    }
}
