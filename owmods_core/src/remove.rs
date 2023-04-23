use std::{
    fs::{remove_dir_all, remove_file},
    path::PathBuf,
};

use anyhow::Result;
use glob::glob;

use crate::{
    db::LocalDatabase,
    file::check_file_matches_paths,
    mods::{get_paths_to_preserve, FailedMod, LocalMod},
};

/// Uninstall a mod
///
/// ## Errors
///
/// If we can't delete the mod's folder.
///
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

/// Removes a [FailedMod]
///
/// ## Errors
///
/// If we can't delete the folder
///
pub fn remove_failed_mod(failed_mod: &FailedMod) -> Result<()> {
    remove_dir_all(PathBuf::from(&failed_mod.mod_path))?;
    Ok(())
}

/// Removes all files not specified in `pathsToPreserve`
pub fn remove_old_mod_files(local_mod: &LocalMod) -> Result<()> {
    let glob_matches = glob(
        PathBuf::from(&local_mod.mod_path)
            .join("**")
            .join("*")
            .to_str()
            .unwrap(),
    )?;
    let preserve_paths = get_paths_to_preserve(Some(local_mod));
    for glob_match in glob_matches {
        let path = glob_match?;
        let relative_path = path.strip_prefix(&local_mod.mod_path)?;
        if !check_file_matches_paths(relative_path, &preserve_paths) {
            if path.is_file() {
                remove_file(&path)?;
            } else if path.is_dir() {
                remove_dir_all(&path)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use std::{fs::File, io::Write};

    use crate::{
        config::Config,
        download::install_mod_from_zip,
        file::create_all_parents,
        mods::UnsafeLocalMod,
        test_utils::{get_test_file, make_test_dir},
    };

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
        assert!(!dir.path().join("Mods").join("Bwc9876.TimeSaver").is_dir());
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
        assert!(!dir.path().join("Mods").join("Bwc9876.TimeSaver").is_dir());
        assert!(!dir.path().join("Mods").join("Bwc9876.SaveEditor").is_dir());
        dir.close().unwrap();
    }

    #[test]
    fn test_remove_old_mod_files() {
        let dir = make_test_dir();
        let test_path = get_test_file("Bwc9876.TimeSaver.zip");
        let target_path = dir.path().join("Mods").join("Bwc9876.TimeSaver");
        let mut config = Config::default(None).unwrap();
        config.owml_path = dir.path().to_str().unwrap().to_string();
        let db = LocalDatabase::default();
        let new_mod = install_mod_from_zip(&test_path, &config, &db).unwrap();
        remove_old_mod_files(&new_mod).unwrap();
        assert!(target_path.is_dir());
        assert!(!target_path.join("TimeSaver.dll").is_file());
        assert!(target_path.join("config.json").is_file());
        dir.close().unwrap();
    }

    #[test]
    fn test_remove_old_mod_files_folder() {
        let dir = make_test_dir();
        let test_path = get_test_file("Bwc9876.TimeSaver.zip");
        let target_path = dir.path().join("Mods").join("Bwc9876.TimeSaver");
        let unimportant_path = target_path
            .join("UnimportantFolder")
            .join("unimportant.json");
        let mut config = Config::default(None).unwrap();
        config.owml_path = dir.path().to_str().unwrap().to_string();
        let db = LocalDatabase::default();
        let new_mod = install_mod_from_zip(&test_path, &config, &db).unwrap();
        create_all_parents(&unimportant_path).unwrap();
        let mut file = File::create(&unimportant_path).unwrap();
        write!(file, "{{}}").unwrap();
        drop(file);
        remove_old_mod_files(&new_mod).unwrap();
        assert!(target_path.is_dir());
        assert!(!target_path.join("TimeSaver.dll").is_file());
        assert!(target_path.join("config.json").is_file());
        dir.close().unwrap();
    }

    #[test]
    fn test_remove_old_mod_files_paths_to_preserve() {
        let dir = make_test_dir();
        let test_path = get_test_file("Bwc9876.TimeSaver.zip");
        let target_path = dir.path().join("Mods").join("Bwc9876.TimeSaver");
        let important_path = target_path.join("important.json");
        let mut config = Config::default(None).unwrap();
        config.owml_path = dir.path().join("").to_str().unwrap().to_string();
        let db = LocalDatabase::default();
        let mut new_mod = install_mod_from_zip(&test_path, &config, &db).unwrap();
        new_mod.manifest.paths_to_preserve = Some(vec!["important.json".to_string()]);
        let mut file = File::create(&important_path).unwrap();
        write!(file, "{{}}").unwrap();
        drop(file);
        remove_old_mod_files(&new_mod).unwrap();
        assert!(target_path.is_dir());
        assert!(!target_path.join("TimeSaver.dll").is_file());
        assert!(important_path.is_file());
        assert!(target_path.join("config.json").is_file());
        dir.close().unwrap();
    }

    #[test]
    fn test_remove_old_mod_files_paths_to_preserve_folder() {
        let dir = make_test_dir();
        let test_path = get_test_file("Bwc9876.TimeSaver.zip");
        let target_path = dir.path().join("Mods").join("Bwc9876.TimeSaver");
        let important_path = target_path.join("ImportantFolder").join("important.json");
        let mut config = Config::default(None).unwrap();
        config.owml_path = dir.path().join("").to_str().unwrap().to_string();
        let db = LocalDatabase::default();
        let mut new_mod = install_mod_from_zip(&test_path, &config, &db).unwrap();
        new_mod.manifest.paths_to_preserve = Some(vec!["ImportantFolder".to_string()]);
        create_all_parents(&important_path).unwrap();
        let mut file = File::create(&important_path).unwrap();
        write!(file, "{{}}").unwrap();
        drop(file);
        remove_old_mod_files(&new_mod).unwrap();
        assert!(target_path.is_dir());
        assert!(!target_path.join("TimeSaver.dll").is_file());
        assert!(important_path.is_file());
        assert!(target_path.join("config.json").is_file());
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
        db.mods
            .get_mut("Bwc9876.SaveEditor")
            .map(|m| match m {
                UnsafeLocalMod::Valid(m) => m,
                _ => {
                    panic!("Mod Install Failed!")
                }
            })
            .unwrap()
            .manifest
            .dependencies = Some(vec!["Bwc9876.TimeSaver".to_string()]);
        remove_mod(&new_mod, &db, true).unwrap();
        assert!(!dir.path().join("Mods").join("Bwc9876.TimeSaver").is_dir());
        assert!(!dir.path().join("Mods").join("Bwc9876.SaveEditor").is_dir());
        dir.close().unwrap();
    }
}
