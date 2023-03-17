use crate::{download::install_mods_parallel, file::deserialize_from_json};
use anyhow::Result;
use std::path::{Path, PathBuf};

use super::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    toggle::{get_mod_enabled, toggle_mod},
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
pub fn export_mods(db: &LocalDatabase) -> Result<String> {
    let enabled_mods: Vec<&String> = db.active().map(|m| &m.manifest.unique_name).collect();
    let result = serde_json::to_string_pretty(&enabled_mods)?;
    Ok(result)
}

/// Import mods from a JSON file that contains an array or unique name (like the one exported by `export_mods`).
/// Mods that aren't in the remote database will be ignored and will only log a warning.
/// Optionally this can also disable all current mods not found in this list as well.
///
/// ## Errors
///
/// If we can't install any mods (that are in the remote database) for whatever reason.
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
        for local_mod in local_db.mods.values() {
            let mod_path = &PathBuf::from(&local_mod.mod_path);
            if get_mod_enabled(mod_path)? {
                toggle_mod(&local_mod.manifest.unique_name, local_db, false, false)?;
            }
        }
    }
    for name in unique_names.iter() {
        let local_mod = local_db.get_mod(name);
        if let Some(local_mod) = local_mod {
            let mod_path = &PathBuf::from(&local_mod.mod_path);
            if !get_mod_enabled(&PathBuf::from(&mod_path))? {
                toggle_mod(&local_mod.manifest.unique_name, local_db, true, false)?;
            }
        } else {
            needed_install.push(name.to_string());
        }
    }

    install_mods_parallel(needed_install, config, remote_db, local_db).await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::Write;

    use crate::{
        download::install_mod_from_zip,
        test_utils::{get_test_file, make_test_dir},
    };

    use super::*;

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
            let dir = make_test_dir();
            let mut config = Config::default(Some(dir.path().join("settings.json"))).unwrap();
            config.owml_path = dir.path().to_str().unwrap().to_string();
            let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
            let list_path = dir.path().join("list.json");
            let mut file = File::create(&list_path).unwrap();
            write!(file, "[\"Bwc9876.TimeSaver\"]").unwrap();
            drop(file);
            let local_db = LocalDatabase::default();
            import_mods(&config, &local_db, &remote_db, &list_path, false)
                .await
                .unwrap();
            assert!(dir.path().join("Mods").join("Bwc9876.TimeSaver").is_dir());
            dir.close().unwrap();
        });
    }

    #[test]
    fn test_import_mods_with_disabled() {
        tokio_test::block_on(async {
            let dir = make_test_dir();
            let zip_path = get_test_file("Bwc9876.TimeSaver.zip");
            let mut config = Config::default(Some(dir.path().join("settings.json"))).unwrap();
            config.owml_path = dir.path().to_str().unwrap().to_string();
            let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
            let local_db = LocalDatabase::default();
            install_mod_from_zip(&zip_path, &config, &local_db).unwrap();
            let local_db = LocalDatabase::fetch(&config.owml_path).unwrap();
            toggle_mod("Bwc9876.TimeSaver", &local_db, false, false).unwrap();
            let list_path = dir.path().join("list.json");
            let mut file = File::create(&list_path).unwrap();
            write!(file, "[\"Bwc9876.TimeSaver\"]").unwrap();
            drop(file);
            let local_db = LocalDatabase::fetch(dir.path().to_str().unwrap()).unwrap();
            import_mods(&config, &local_db, &remote_db, &list_path, false)
                .await
                .unwrap();
            let new_mod = LocalDatabase::read_local_mod(
                &dir.path()
                    .join("Mods")
                    .join("Bwc9876.TimeSaver")
                    .join("manifest.json"),
            )
            .unwrap();
            assert!(new_mod.enabled);
            dir.close().unwrap();
        });
    }

    #[test]
    fn test_import_mods_disable_missing() {
        tokio_test::block_on(async {
            let dir = make_test_dir();
            let zip_path = get_test_file("Bwc9876.TimeSaver.zip");
            let mut config = Config::default(Some(dir.path().join("settings.json"))).unwrap();
            config.owml_path = dir.path().to_str().unwrap().to_string();
            let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
            let local_db = LocalDatabase::default();
            install_mod_from_zip(&zip_path, &config, &local_db).unwrap();
            let list_path = dir.path().join("list.json");
            let mut file = File::create(&list_path).unwrap();
            write!(file, "[]").unwrap();
            drop(file);
            let local_db = LocalDatabase::fetch(dir.path().to_str().unwrap()).unwrap();
            import_mods(&config, &local_db, &remote_db, &list_path, true)
                .await
                .unwrap();
            let new_mod = LocalDatabase::read_local_mod(
                &dir.path()
                    .join("Mods")
                    .join("Bwc9876.TimeSaver")
                    .join("manifest.json"),
            )
            .unwrap();
            assert!(!new_mod.enabled);
            dir.close().unwrap();
        });
    }
}
