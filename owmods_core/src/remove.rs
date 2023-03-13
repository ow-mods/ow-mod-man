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

#[cfg(test)]
mod tests {

    use crate::{test_utils::{make_test_dir, get_test_file}, config::Config, download::install_mod_from_zip};

    use super::*;

    #[test]
    fn test_remove_mod() {
        let dir = make_test_dir();
        let test_path = get_test_file("Bwc9876.TimeSaver.zip");
        let mut config = Config::default(None).unwrap();
        config.owml_path = dir.path().join("").to_str().unwrap().to_string();
        let db = LocalDatabase::default();
        install_mod_from_zip(&test_path, &config, &db).unwrap();
        let db = LocalDatabase::fetch(&config.owml_path).unwrap();
        let new_mod = db.get_mod("Bwc9876.TimeSaver").unwrap();
        remove_mod(new_mod, &db, false).unwrap();
        assert_eq!(dir.path().join("Mods/Bwc9876.TimeSaver").is_dir(), false);
        dir.close().unwrap();
    }

    #[test]
    fn test_remove_mod_recursive() {
        let dir = make_test_dir();
        let test_path = get_test_file("Bwc9876.TimeSaver.zip");
        let test_path_2 = get_test_file("Bwc9876.SaveEditor.zip");
        let mut config = Config::default(None).unwrap();
        config.owml_path = dir.path().join("").to_str().unwrap().to_string();
        let db = LocalDatabase::default();
        install_mod_from_zip(&test_path, &config, &db).unwrap();
        install_mod_from_zip(&test_path_2, &config, &db).unwrap();
        let db = LocalDatabase::fetch(&config.owml_path).unwrap();
        let mut new_mod = db.get_mod("Bwc9876.TimeSaver").unwrap().clone();
        new_mod.manifest.dependencies = Some(vec!["Bwc9876.SaveEditor".to_string()]);
        remove_mod(&new_mod, &db, true).unwrap();
        assert_eq!(dir.path().join("Mods/Bwc9876.TimeSaver").is_dir(), false);
        assert_eq!(dir.path().join("Mods/Bwc9876.SaveEditor").is_dir(), false);
        dir.close().unwrap();
    }

    #[test]
    fn test_remove_mod_recursive_cyclical_deps() {
        let dir = make_test_dir();
        let test_path = get_test_file("Bwc9876.TimeSaver.zip");
        let test_path_2 = get_test_file("Bwc9876.SaveEditor.zip");
        let mut config = Config::default(None).unwrap();
        config.owml_path = dir.path().join("").to_str().unwrap().to_string();
        let db = LocalDatabase::default();
        install_mod_from_zip(&test_path, &config, &db).unwrap();
        install_mod_from_zip(&test_path_2, &config, &db).unwrap();
        let mut db = LocalDatabase::fetch(&config.owml_path).unwrap();
        let mut new_mod = db.get_mod("Bwc9876.TimeSaver").unwrap().clone();
        new_mod.manifest.dependencies = Some(vec!["Bwc9876.SaveEditor".to_string()]);
        db.mods.get_mut("Bwc9876.SaveEditor").unwrap().manifest.dependencies = Some(vec!["Bwc9876.TimeSaver".to_string()]);
        remove_mod(&new_mod, &db, true).unwrap();
        assert_eq!(dir.path().join("Mods/Bwc9876.TimeSaver").is_dir(), false);
        assert_eq!(dir.path().join("Mods/Bwc9876.SaveEditor").is_dir(), false);
        dir.close().unwrap();
    }

}
