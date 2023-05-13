use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use log::warn;

use crate::{
    db::LocalDatabase,
    file::{deserialize_from_json, fix_json_file, serialize_to_json},
    mods::local::ModStubConfig,
};

fn read_config(config_path: &Path) -> Result<ModStubConfig> {
    fix_json_file(config_path).ok();
    deserialize_from_json(config_path)
}

fn write_config(conf: &ModStubConfig, config_path: &Path) -> Result<()> {
    serialize_to_json(&conf, config_path, false)?;
    Ok(())
}

/// Generates an empty, enabled config to the given path.
///
/// ## Errors
///
/// If we can't create or serialize the config file.
///
pub fn generate_config(path: &Path) -> Result<()> {
    let new_config = ModStubConfig {
        enabled: true,
        settings: None,
    };
    serialize_to_json(&new_config, path, false)
}

/// Gets whether a mod is enabled given its mod path
///
/// ## Returns
///
/// true if the config exists and has `enabled: true`, false if the config doesn't exist or has `enabled: false`.
///
/// ## Errors
///
/// If we can't deserialize the config file (this will happen in the event of malformed JSON, not a missing file).
///
pub fn get_mod_enabled(mod_path: &Path) -> Result<bool> {
    let config_path = mod_path.join("config.json");
    if config_path.is_file() {
        let conf = read_config(&config_path)?;
        Ok(conf.enabled)
    } else {
        Ok(false)
    }
}

/// Toggle a mod to a given enabled value.
/// Also support applying this action recursively.
///
///
/// ## Returns
///
/// A list of mod names that were disabled and use pre patchers, and therefore **should alert the user to check the mod's README for instructions on how to fully disable it**.
///
/// ## Errors
///
/// If we can't read/save to the config files of the mod or (if recursive is true) any of it's dependents.
///
pub fn toggle_mod(
    unique_name: &str,
    local_db: &LocalDatabase,
    enabled: bool,
    recursive: bool,
) -> Result<Vec<String>> {
    let mut show_warnings_for: Vec<String> = vec![];

    let local_mod = local_db
        .get_mod(unique_name)
        .ok_or_else(|| anyhow!("Mod {} not found", unique_name))?;
    let config_path = PathBuf::from(&local_mod.mod_path).join("config.json");

    if !enabled && local_mod.uses_pre_patcher() {
        show_warnings_for.push(local_mod.manifest.name.clone());
    }

    if config_path.is_file() {
        let mut config = read_config(&config_path)?;
        config.enabled = enabled;
        write_config(&config, &config_path)?;
    } else {
        generate_config(&config_path)?;
        show_warnings_for.extend(toggle_mod(unique_name, local_db, enabled, recursive)?);
    }

    if recursive {
        if let Some(deps) = local_mod.manifest.dependencies.as_ref() {
            for dep in deps.iter() {
                let dep_mod = local_db.get_mod(dep);
                if let Some(dep_mod) = dep_mod {
                    if enabled {
                        toggle_mod(&dep_mod.manifest.unique_name, local_db, enabled, recursive)?;
                    } else {
                        let mut flag = true;
                        for dependent_mod in local_db.dependent(dep_mod).filter(|m| m.enabled) {
                            if dependent_mod.manifest.unique_name != local_mod.manifest.unique_name
                            {
                                warn!(
                                    "Not disabling {} as it's also needed by {}",
                                    dep_mod.manifest.name, dependent_mod.manifest.name
                                );
                                flag = false;
                            }
                        }
                        if flag {
                            show_warnings_for.extend(toggle_mod(
                                &dep_mod.manifest.unique_name,
                                local_db,
                                enabled,
                                recursive,
                            )?);
                        }
                    }
                } else {
                    warn!("Dependency {} Was Not Found, Ignoring.", dep);
                }
            }
        }
    }
    Ok(show_warnings_for)
}

#[cfg(test)]
mod tests {

    use std::fs::remove_file;

    use tempfile::TempDir;

    use crate::{
        config::Config,
        download::install_mod_from_zip,
        mods::local::{LocalMod, UnsafeLocalMod},
        test_utils::{get_test_file, make_test_dir},
    };

    use super::*;

    fn setup() -> (TempDir, LocalDatabase, LocalMod) {
        let dir = make_test_dir();
        let test_zip = get_test_file("Bwc9876.TimeSaver.zip");
        let db = LocalDatabase::default();
        let mut config = Config::default(None).unwrap();
        config.owml_path = dir.path().to_str().unwrap().to_string();
        let new_mod = install_mod_from_zip(&test_zip, &config, &db).unwrap();
        let db = LocalDatabase::fetch(&config.owml_path).unwrap();
        (dir, db, new_mod)
    }

    #[test]
    fn test_mod_toggle() {
        let (dir, db, new_mod) = setup();
        let mod_path = PathBuf::from(new_mod.mod_path);
        toggle_mod("Bwc9876.TimeSaver", &db, false, false).unwrap();
        assert!(mod_path.join("config.json").is_file());
        let new_mod = LocalDatabase::read_local_mod(&mod_path.join("manifest.json")).unwrap();
        assert!(!new_mod.enabled);
        toggle_mod("Bwc9876.TimeSaver", &db, true, false).unwrap();
        let new_mod = LocalDatabase::read_local_mod(&mod_path.join("manifest.json")).unwrap();
        assert!(new_mod.enabled);
        dir.close().unwrap();
    }

    #[test]
    fn test_mod_toggle_recursive() {
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
        *db.mods.get_mut(&String::from("Bwc9876.TimeSaver")).unwrap() =
            UnsafeLocalMod::Valid(new_mod);
        toggle_mod("Bwc9876.TimeSaver", &db, false, true).unwrap();
        let new_mod = db.get_mod("Bwc9876.SaveEditor").unwrap().clone();
        let mod_path = PathBuf::from(new_mod.mod_path);
        let new_mod = LocalDatabase::read_local_mod(&mod_path.join("manifest.json")).unwrap();
        assert!(!new_mod.enabled);
        dir.close().unwrap();
    }

    #[test]
    fn test_mod_toggle_recursive_other_dependent() {
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
        *db.mods.get_mut(&String::from("Bwc9876.TimeSaver")).unwrap() =
            UnsafeLocalMod::Valid(new_mod);
        let mut test_mod = LocalMod::get_test(0);
        test_mod.manifest.dependencies = Some(vec![String::from("Bwc9876.SaveEditor")]);
        db.mods.insert(
            test_mod.manifest.unique_name.clone(),
            UnsafeLocalMod::Valid(test_mod),
        );
        toggle_mod("Bwc9876.TimeSaver", &db, false, true).unwrap();
        let new_mod = db.get_mod("Bwc9876.SaveEditor").unwrap().clone();
        let mod_path = PathBuf::from(new_mod.mod_path);
        let new_mod = LocalDatabase::read_local_mod(&mod_path.join("manifest.json")).unwrap();
        assert!(new_mod.enabled);
        dir.close().unwrap();
    }

    #[test]
    fn test_mod_toggle_recursive_other_dependent_but_disabled() {
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
        *db.mods.get_mut(&String::from("Bwc9876.TimeSaver")).unwrap() =
            UnsafeLocalMod::Valid(new_mod);
        let mut test_mod = LocalMod::get_test(0);
        test_mod.manifest.dependencies = Some(vec![String::from("Bwc9876.SaveEditor")]);
        test_mod.enabled = false;
        db.mods.insert(
            test_mod.manifest.unique_name.clone(),
            UnsafeLocalMod::Valid(test_mod),
        );
        toggle_mod("Bwc9876.TimeSaver", &db, false, true).unwrap();
        let new_mod = db.get_mod("Bwc9876.SaveEditor").unwrap().clone();
        let mod_path = PathBuf::from(new_mod.mod_path);
        let new_mod = LocalDatabase::read_local_mod(&mod_path.join("manifest.json")).unwrap();
        assert!(!new_mod.enabled);
        dir.close().unwrap();
    }

    #[test]
    fn test_mod_toggle_no_config() {
        let (dir, db, new_mod) = setup();
        let mod_path = PathBuf::from(new_mod.mod_path);
        remove_file(mod_path.join("config.json")).unwrap();
        assert!(!get_mod_enabled(&mod_path).unwrap());
        toggle_mod("Bwc9876.TimeSaver", &db, false, false).unwrap();
        assert!(mod_path.join("config.json").is_file());
        let new_mod = LocalDatabase::read_local_mod(&mod_path.join("manifest.json")).unwrap();
        assert!(!new_mod.enabled);
        toggle_mod("Bwc9876.TimeSaver", &db, true, false).unwrap();
        let new_mod = LocalDatabase::read_local_mod(&mod_path.join("manifest.json")).unwrap();
        assert!(new_mod.enabled);
        dir.close().unwrap();
    }
}
