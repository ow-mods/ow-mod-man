use anyhow::anyhow;
use anyhow::Result;
use log::warn;
use std::path::{Path, PathBuf};

use crate::{
    file::{deserialize_from_json, fix_json_file, serialize_to_json},
    mods::ModStubConfig,
};

use super::db::LocalDatabase;

fn read_config(config_path: &Path) -> Result<ModStubConfig> {
    fix_json_file(config_path).ok();
    deserialize_from_json(config_path)
}

fn write_config(conf: &ModStubConfig, config_path: &Path) -> Result<()> {
    serialize_to_json(&conf, config_path, false)?;
    Ok(())
}

// OWML will copy settings for us, so no need to read from default-config.json, just generate an empty config
pub fn generate_config(path: &Path) -> Result<()> {
    let new_config = ModStubConfig {
        enabled: true,
        settings: None,
    };
    serialize_to_json(&new_config, &path, false)
}

pub fn get_mod_enabled(mod_path: &Path) -> Result<bool> {
    let config_path = mod_path.join("config.json");
    if config_path.is_file() {
        let conf = read_config(&config_path)?;
        Ok(conf.enabled)
    } else {
        Ok(false)
    }
}

pub fn toggle_mod(
    unique_name: &str,
    local_db: &LocalDatabase,
    enabled: bool,
    recursive: bool,
) -> Result<()> {
    let local_mod = local_db
        .get_mod(unique_name)
        .ok_or_else(|| anyhow!("Mod {} not found", unique_name))?;
    let config_path = PathBuf::from(&local_mod.mod_path).join("config.json");
    if config_path.is_file() {
        let mut config = read_config(&config_path)?;
        config.enabled = enabled;
        write_config(&config, &config_path)?;
    } else {
        generate_config(&config_path)?;
        toggle_mod(unique_name, local_db, enabled, recursive)?;
    }
    if recursive {
        if let Some(deps) = local_mod.manifest.dependencies.as_ref() {
            for dep in deps.iter() {
                let dep_mod = local_db.get_mod(dep);
                if let Some(dep_mod) = dep_mod {
                    toggle_mod(&dep_mod.manifest.unique_name, local_db, enabled, recursive)?;
                } else {
                    warn!("Dependency {} Was Not Found, Ignoring.", dep);
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use std::fs::remove_file;

    use tempdir::TempDir;

    use crate::{
        config::Config,
        download::install_mod_from_zip,
        mods::LocalMod,
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
        assert_eq!(new_mod.enabled, false);
        toggle_mod("Bwc9876.TimeSaver", &db, true, false).unwrap();
        let new_mod = LocalDatabase::read_local_mod(&mod_path.join("manifest.json")).unwrap();
        assert!(new_mod.enabled);
        dir.close().unwrap();
    }

    #[test]
    fn test_mod_toggle_no_config() {
        let (dir, db, new_mod) = setup();
        let mod_path = PathBuf::from(new_mod.mod_path);
        remove_file(mod_path.join("config.json")).unwrap();
        assert_eq!(get_mod_enabled(&mod_path).unwrap(), false);
        toggle_mod("Bwc9876.TimeSaver", &db, false, false).unwrap();
        assert!(mod_path.join("config.json").is_file());
        let new_mod = LocalDatabase::read_local_mod(&mod_path.join("manifest.json")).unwrap();
        assert_eq!(new_mod.enabled, false);
        toggle_mod("Bwc9876.TimeSaver", &db, true, false).unwrap();
        let new_mod = LocalDatabase::read_local_mod(&mod_path.join("manifest.json")).unwrap();
        assert!(new_mod.enabled);
        dir.close().unwrap();
    }

}
