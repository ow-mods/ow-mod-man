use log::info;
use version_compare::Cmp;

use anyhow::Result;

use crate::{
    analytics::{send_analytics_event, AnalyticsEventName},
    constants::OWML_UNIQUE_NAME,
    download::install_mods_parallel,
};

use super::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    download::download_and_install_owml,
    mods::{LocalMod, RemoteMod},
};

pub fn check_mod_needs_update<'a>(
    local_mod: &'a LocalMod,
    remote_db: &'a RemoteDatabase,
) -> (bool, Option<&'a RemoteMod>) {
    let remote_mod = if local_mod.manifest.unique_name == OWML_UNIQUE_NAME {
        remote_db.get_owml()
    } else {
        remote_db.get_mod(&local_mod.manifest.unique_name)
    };
    if let Some(remote_mod) = remote_mod {
        (
            version_compare::compare(&remote_mod.version, &local_mod.manifest.version)
                .unwrap_or(Cmp::Eq)
                == Cmp::Gt,
            Some(remote_mod),
        )
    } else {
        (false, None)
    }
}

pub async fn update_all(
    config: &Config,
    local_db: &LocalDatabase,
    remote_db: &RemoteDatabase,
) -> Result<bool> {
    let mut needs_update: Vec<&RemoteMod> = vec![];

    for local_mod in local_db.mods.values() {
        let (update, remote_mod) = check_mod_needs_update(local_mod, remote_db);
        if update {
            info!(
                "{}: {} -> {}",
                local_mod.manifest.name,
                local_mod.manifest.version,
                remote_mod.unwrap().version
            );
            needs_update.push(remote_mod.unwrap());
        }
    }

    let owml = local_db.get_owml(config);

    if owml.is_some() {
        let (update, remote_owml) = check_mod_needs_update(owml.as_ref().unwrap(), remote_db);
        if update {
            info!(
                "OWML: {} -> {}",
                owml.as_ref().unwrap().manifest.version,
                remote_owml.unwrap().version
            );
            download_and_install_owml(config, remote_owml.unwrap()).await?;
        }
    }

    if needs_update.is_empty() {
        Ok(false)
    } else {
        let mod_names = needs_update
            .into_iter()
            .map(|m| m.unique_name.clone())
            .collect();
        let updated = install_mods_parallel(mod_names, config, remote_db, local_db).await?;
        for updated_mod in updated {
            send_analytics_event(
                AnalyticsEventName::ModUpdate,
                &updated_mod.manifest.unique_name,
            )
            .await?;
        }
        Ok(true)
    }
}
