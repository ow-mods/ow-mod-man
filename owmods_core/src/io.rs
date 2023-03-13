use crate::{download::install_mods_parallel, file::deserialize_from_json};
use anyhow::Result;
use std::path::{Path, PathBuf};

use super::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    toggle::{get_mod_enabled, toggle_mod},
};

pub fn export_mods(db: &LocalDatabase) -> Result<String> {
    let enabled_mods: Vec<&String> = db
        .active()
        .iter()
        .filter_map(|m| {
            if m.enabled {
                Some(&m.manifest.unique_name)
            } else {
                None
            }
        })
        .collect();
    let result = serde_json::to_string_pretty(&enabled_mods)?;
    Ok(result)
}

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
            if get_mod_enabled(&PathBuf::from(&mod_path))? {
                toggle_mod(mod_path, local_db, false, false)?;
            }
        }
    }
    for name in unique_names.iter() {
        let local_mod = local_db.get_mod(name);
        if let Some(local_mod) = local_mod {
            let mod_path = &PathBuf::from(&local_mod.mod_path);
            if !get_mod_enabled(&PathBuf::from(&mod_path))? {
                toggle_mod(mod_path, local_db, true, false)?;
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
        assert_eq!(result.contains("Bwc9876.SaveEditor"), false);
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
            write!(file, "{}", "[\"Bwc9876.TimeSaver\"]").unwrap();
            drop(file);
            let local_db = LocalDatabase::default();
            import_mods(&config, &local_db, &remote_db, &list_path, false)
                .await
                .unwrap();
            assert!(dir.path().join("Mods/Bwc9876.TimeSaver").is_dir());
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
            let new_mod = install_mod_from_zip(&zip_path, &config, &local_db).unwrap();
            toggle_mod(&PathBuf::from(new_mod.mod_path), &local_db, false, false).unwrap();
            let list_path = dir.path().join("list.json");
            let mut file = File::create(&list_path).unwrap();
            write!(file, "{}", "[\"Bwc9876.TimeSaver\"]").unwrap();
            drop(file);
            let local_db = LocalDatabase::fetch(dir.path().to_str().unwrap()).unwrap();
            import_mods(&config, &local_db, &remote_db, &list_path, false)
                .await
                .unwrap();
            let new_mod = LocalDatabase::read_local_mod(
                &dir.path().join("Mods/Bwc9876.TimeSaver/manifest.json"),
            )
            .unwrap();
            assert_eq!(new_mod.enabled, true);
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
            write!(file, "{}", "[]").unwrap();
            drop(file);
            let local_db = LocalDatabase::fetch(dir.path().to_str().unwrap()).unwrap();
            import_mods(&config, &local_db, &remote_db, &list_path, true)
                .await
                .unwrap();
            let new_mod = LocalDatabase::read_local_mod(
                &dir.path().join("Mods/Bwc9876.TimeSaver/manifest.json"),
            )
            .unwrap();
            assert_eq!(new_mod.enabled, false);
            dir.close().unwrap();
        });
    }
}
