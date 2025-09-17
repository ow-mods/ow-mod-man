use std::path::Path;

use anyhow::Result;

use crate::{
    analytics::{send_analytics_deferred, AnalyticsEventName},
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    download::install_mods_parallel,
    file::deserialize_from_json,
    toggle::toggle_mod,
};

/// Export all installed **and enabled** mods in the database
///
/// ## Returns
///
/// A JSON array of unique names of the mods
///
/// ## Errors
///
/// If we can't serialize to JSON
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::io::export_mods;
/// use owmods_core::db::LocalDatabase;
/// use owmods_core::config::Config;
///
/// let config = Config::get(None).unwrap();
/// let local_db = LocalDatabase::fetch(&config.owml_path).unwrap();
/// let result = export_mods(&local_db).unwrap();
///
/// println!("Enabled Mods: {}", result);
/// ```
///
pub fn export_mods(db: &LocalDatabase) -> Result<String> {
    let enabled_mods: Vec<&String> = db.active().map(|m| &m.manifest.unique_name).collect();
    let result = serde_json::to_string_pretty(&enabled_mods)?;
    Ok(result)
}

/// Import mods from a JSON file that contains an array or unique name (like the one exported by [export_mods]).
/// Mods that aren't in the remote database will be ignored and will only log a warning.
/// Optionally, this can also disable all current mods not found in this list.
///
/// ## Errors
///
/// If we can't install any mods (that are in the remote database) for whatever reason.
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::io::{import_mods, export_mods};
/// use owmods_core::db::{LocalDatabase, RemoteDatabase};
/// use owmods_core::toggle::toggle_mod;
/// use owmods_core::config::Config;
/// use std::path::PathBuf;
///
/// # tokio_test::block_on(async {
/// let config = Config::get(None).unwrap();
/// let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
///
/// let local_db = LocalDatabase::fetch(&config.owml_path).unwrap();
/// let exported_list = export_mods(&local_db).unwrap();
///
/// std::fs::write("exported_list.json", exported_list).unwrap();
///
/// for local_mod in local_db.valid() {
///     if local_mod.enabled {
///         toggle_mod(&local_mod.manifest.unique_name, &local_db, false, false).unwrap();
///     }
/// }
///
/// import_mods(&config, &local_db, &remote_db, &PathBuf::from("exported_list.json"), false).await.unwrap();
/// # });
/// ```
///
/// ```no_run
/// use owmods_core::io::{import_mods, export_mods};
/// use owmods_core::db::{LocalDatabase, RemoteDatabase};
/// use owmods_core::toggle::toggle_mod;
/// use owmods_core::config::Config;
/// use std::path::PathBuf;
///
/// # tokio_test::block_on(async {
/// let config = Config::get(None).unwrap();
/// let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
///
/// let local_db = LocalDatabase::fetch(&config.owml_path).unwrap();
/// toggle_mod("Bwc9876.TimeSaver", &local_db, false, false).unwrap();
/// let exported_list = export_mods(&local_db).unwrap();
///
/// std::fs::write("exported_list.json", exported_list).unwrap();
///
/// for local_mod in local_db.valid() {
///    if local_mod.enabled {
///       toggle_mod(&local_mod.manifest.unique_name, &local_db, false, false).unwrap();
///    }
/// }
///
/// toggle_mod("Bwc9876.TimeSaver", &local_db, true, false).unwrap();
///
/// import_mods(&config, &local_db, &remote_db, &PathBuf::from("exported_list.json"), true).await.unwrap();
///
/// assert!(!local_db.get_mod("Bwc9876.TimeSaver").unwrap().enabled);
/// # });
/// ```
///
///
pub async fn import_mods(
    config: &Config,
    local_db: &LocalDatabase,
    remote_db: &RemoteDatabase,
    file_path: &Path,
    disable_missing: bool,
) -> Result<()> {
    let unique_names: Vec<String> = deserialize_from_json(file_path)?;
    let mut needed_install: Vec<String> = vec![];

    if disable_missing {
        for local_mod in local_db.valid() {
            if local_mod.enabled {
                toggle_mod(&local_mod.manifest.unique_name, local_db, false, false)?;
            }
        }
    }
    for name in unique_names.iter() {
        let local_mod = local_db.get_mod(name);
        if let Some(local_mod) = local_mod {
            toggle_mod(&local_mod.manifest.unique_name, local_db, true, false)?;
        } else {
            needed_install.push(name.to_string());
        }
    }

    install_mods_parallel(needed_install.clone(), config, remote_db, local_db).await?;

    for unique_name in needed_install {
        send_analytics_deferred(AnalyticsEventName::ModInstall, &unique_name, config).await;
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use std::{fs, path::PathBuf};

    use crate::test_utils::{get_test_file, TestContext};

    use super::*;

    fn make_list_json(ctx: &TestContext, list: &str) -> PathBuf {
        let path = ctx.temp_dir.path().join("list.json");
        fs::write(&path, list).unwrap();
        path
    }

    #[test]
    fn test_export_mods() {
        let test_dir = get_test_file("");
        let db = LocalDatabase::fetch(test_dir.to_str().unwrap()).unwrap();
        let result = export_mods(&db).unwrap();
        assert!(result.contains("Bwc9876.TimeSaver"));
        assert!(!result.contains("Bwc9876.SaveEditor"));
    }

    #[test]
    fn test_import_mods() {
        tokio_test::block_on(async {
            let mut ctx = TestContext::new();
            ctx.fetch_remote_db().await;
            let list_path = make_list_json(&ctx, "[\"Bwc9876.TimeSaver\"]");
            import_mods(
                &ctx.config,
                &ctx.local_db,
                &ctx.remote_db,
                &list_path,
                false,
            )
            .await
            .unwrap();
            assert!(ctx.get_test_path("Bwc9876.TimeSaver").is_dir());
        });
    }

    #[test]
    fn test_import_mods_with_disabled() {
        tokio_test::block_on(async {
            let mut ctx = TestContext::new();
            ctx.fetch_remote_db().await;
            ctx.install_test_zip("Bwc9876.TimeSaver.zip", true);
            toggle_mod("Bwc9876.TimeSaver", &ctx.local_db, false, false).unwrap();
            let list_path = make_list_json(&ctx, "[\"Bwc9876.TimeSaver\"]");
            import_mods(
                &ctx.config,
                &ctx.local_db,
                &ctx.remote_db,
                &list_path,
                false,
            )
            .await
            .unwrap();
            ctx.fetch_local_db();
            let new_mod = ctx.local_db.get_mod("Bwc9876.TimeSaver").unwrap();
            assert!(new_mod.enabled);
        });
    }

    #[test]
    fn test_import_mods_disable_missing() {
        tokio_test::block_on(async {
            let mut ctx = TestContext::new();
            ctx.fetch_remote_db().await;
            ctx.install_test_zip("Bwc9876.TimeSaver.zip", true);
            let list_path = make_list_json(&ctx, "[]");
            import_mods(&ctx.config, &ctx.local_db, &ctx.remote_db, &list_path, true)
                .await
                .unwrap();
            ctx.fetch_local_db();
            let new_mod = ctx.local_db.get_mod("Bwc9876.TimeSaver").unwrap();
            assert!(!new_mod.enabled);
        });
    }

    #[test]
    fn test_import_mods_disable_missing_already_enabled() {
        tokio_test::block_on(async {
            let mut ctx = TestContext::new();
            ctx.fetch_remote_db().await;
            ctx.install_test_zip("Bwc9876.TimeSaver.zip", true);
            let list_path = make_list_json(&ctx, "[\"Bwc9876.TimeSaver\"]");
            import_mods(&ctx.config, &ctx.local_db, &ctx.remote_db, &list_path, true)
                .await
                .unwrap();
            ctx.fetch_local_db();
            let new_mod = ctx.local_db.get_mod("Bwc9876.TimeSaver").unwrap();
            assert!(new_mod.enabled);
        });
    }
}
