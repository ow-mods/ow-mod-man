use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typeshare::typeshare;

use crate::{
    config::Config,
    constants::OWML_DEFAULT_CONFIG_NAME,
    file::{deserialize_from_json, serialize_to_json},
};

/// Represents the configuration for OWML
#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)] // Have to allow non_snake_case here because OWML's config uses incrementalGC, which isn't proper camelCase
pub struct OWMLConfig {
    pub game_path: String,
    debug_mode: bool,
    pub force_exe: bool,
    incremental_GC: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    owml_path: Option<String>,
    pub socket_port: u16,
    #[typeshare(skip)]
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl OWMLConfig {
    fn path(config: &Config) -> PathBuf {
        Path::new(&config.owml_path).join("OWML.Config.json")
    }

    fn read(config: &Config) -> Result<OWMLConfig> {
        deserialize_from_json(&Self::path(config))
    }

    /// Read the config from a specific path
    ///
    /// ## Returns
    ///
    /// The configuration at that path
    ///
    /// ## Errors
    ///
    /// If we can't deserialize the object or can't access the file.
    ///
    pub fn get_from_path(path: &Path) -> Result<OWMLConfig> {
        deserialize_from_json(path)
    }

    /// Save the config at the given path
    ///
    /// ## Errors
    ///
    /// If we can't save to the file.
    ///
    pub fn save_to_path(&self, path: &Path) -> Result<()> {
        serialize_to_json(self, path, true)
    }

    #[cfg(not(windows))]
    pub fn default(config: &Config) -> Result<OWMLConfig> {
        use anyhow::anyhow;
        use directories::UserDirs;

        const LINUX_GAME_PATH: &str = ".steam/steam/steamapps/common/Outer Wilds/";

        let path = Path::new(&config.owml_path).join(OWML_DEFAULT_CONFIG_NAME);
        let mut conf: OWMLConfig = deserialize_from_json(&path)?;
        let dirs = UserDirs::new().ok_or_else(|| anyhow!("Can't get user data dir"))?;
        conf.game_path = dirs
            .home_dir()
            .join(LINUX_GAME_PATH)
            .to_str()
            .unwrap()
            .to_string();
        Ok(conf)
    }

    #[cfg(windows)]
    pub fn default(config: &Config) -> Result<OWMLConfig> {
        deserialize_from_json(&Path::new(&config.owml_path).join(OWML_DEFAULT_CONFIG_NAME))
    }

    fn write(owml_config: &OWMLConfig, config: &Config) -> Result<()> {
        serialize_to_json(owml_config, &Self::path(config), true)?;
        Ok(())
    }

    /// Get the OWML config located in `config.owml_path`.
    /// This will copy the default config if it doesn't exist.
    ///
    /// ## Returns
    ///
    /// The configuration for OWML
    ///
    /// ## Errors
    ///
    /// If we can't read the current config or copy the default one.
    ///
    pub fn get(config: &Config) -> Result<OWMLConfig> {
        if Self::path(config).is_file() {
            Self::read(config)
        } else {
            let new_conf = Self::default(config)?;
            new_conf.save(config)?;
            Ok(new_conf)
        }
    }

    /// Save this config to the path specified in `config.owml_path`
    ///
    /// ## Errors
    ///
    /// If we can't save the file or serialize the object.
    ///
    pub fn save(&self, config: &Config) -> Result<()> {
        Self::write(self, config)
    }
}

#[cfg(test)]
mod tests {

    use std::{fs::File, io::Write};

    use crate::test_utils::{get_test_file, make_test_dir};

    use super::*;

    #[test]
    fn test_owml_config_read() {
        let mut config = Config::default(None).unwrap();
        config.owml_path = get_test_file("").to_str().unwrap().to_string();
        let conf = OWMLConfig::read(&config).unwrap();
        assert!(conf.debug_mode);
        assert!(conf.force_exe);
        assert!(conf.incremental_GC);
    }

    #[test]
    fn test_owml_config_save() {
        let dir = make_test_dir();
        let mut config = Config::default(None).unwrap();
        config.owml_path = get_test_file("").to_str().unwrap().to_string();
        let conf = OWMLConfig::read(&config).unwrap();
        config.owml_path = dir.path().to_str().unwrap().to_string();
        conf.save(&config).unwrap();
        assert!(dir.path().join("OWML.Config.json").is_file());
        dir.close().unwrap();
    }

    #[test]
    fn test_owml_config_get() {
        let dir = make_test_dir();
        let mut file = File::create(dir.path().join("OWML.Config.json")).unwrap();
        write!(file, "{}", include_str!("../test_files/OWML.Config.json")).unwrap();
        drop(file);
        let mut config = Config::default(None).unwrap();
        config.owml_path = dir.path().to_str().unwrap().to_string();
        let conf = OWMLConfig::get(&config).unwrap();
        assert!(conf.debug_mode);
        dir.close().unwrap();
    }

    #[test]
    fn test_owml_config_get_default() {
        let dir = make_test_dir();
        let mut file = File::create(dir.path().join("OWML.DefaultConfig.json")).unwrap();
        write!(file, "{}", include_str!("../test_files/OWML.Config.json")).unwrap();
        drop(file);
        let mut config = Config::default(None).unwrap();
        config.owml_path = dir.path().to_str().unwrap().to_string();
        let conf = OWMLConfig::get(&config).unwrap();
        assert!(dir.path().join("OWML.Config.json").is_file());
        assert!(conf.debug_mode);
        dir.close().unwrap();
    }
}
