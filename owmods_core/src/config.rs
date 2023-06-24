use std::path::{Path, PathBuf};

use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    constants::{
        CONFIG_FILE_NAME, DEFAULT_ALERT_URL, DEFAULT_DB_URL, OLD_ALERT_URL,
        OWML_DEFAULT_CONFIG_NAME, OWML_EXE_NAME, OWML_MANIFEST_NAME,
    },
    file::{deserialize_from_json, get_app_path, get_default_owml_path, serialize_to_json},
};

/// Represents the core config, contains critical info needed by the core API
#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub owml_path: String,
    pub database_url: String,
    pub alert_url: String,
    pub viewed_alerts: Vec<String>,
    #[serde(skip)]
    pub path: PathBuf,
}

impl Config {
    /// Create a new config object with defaults set and optionally set to save at a specified: `path`.
    ///
    /// ## Errors
    ///
    /// Only error that could be thrown is if we can't get the local app data directory of the user, if a custom path is specified this error will not happen.
    ///
    pub fn default(path: Option<PathBuf>) -> Result<Self> {
        let path = path.unwrap_or(Self::default_path()?);
        let owml_path = get_default_owml_path()?;
        Ok(Self {
            owml_path: String::from(owml_path.to_str().unwrap()),
            database_url: String::from(DEFAULT_DB_URL),
            alert_url: String::from(DEFAULT_ALERT_URL),
            viewed_alerts: vec![],
            path,
        })
    }

    /// Get the default path settings should save to, derived from user's local app data dir
    ///
    /// `Config::get` uses this internally
    ///
    /// ## Returns
    ///
    /// The default path the settings file should be saved to.
    ///
    /// ## Errors
    ///
    /// If we can't get the user's local app data
    ///
    pub fn default_path() -> Result<PathBuf> {
        let app_path = get_app_path()?;
        Ok(app_path.join(CONFIG_FILE_NAME))
    }

    /// Save the config
    ///
    /// ## Errors
    ///
    /// If we can't save the config file
    ///
    pub fn save(&self) -> Result<()> {
        debug!("Writing Config To {}", self.path.to_str().unwrap());
        serialize_to_json(self, &self.path, true)?;
        Ok(())
    }

    /// Set that a specific mod's warning was shown.
    pub fn set_warning_shown(&mut self, unique_name: &str) {
        self.viewed_alerts.push(unique_name.to_string());
    }

    fn read(path: &Path) -> Result<Self> {
        debug!("Reading Config From {}", path.to_str().unwrap());
        let mut new_conf: Config = deserialize_from_json(path)?;
        new_conf.path = path.to_path_buf();
        Ok(new_conf.migrate())
    }

    // Migrate a config from older versions
    fn migrate(mut self) -> Self {
        if self.alert_url == OLD_ALERT_URL {
            self.alert_url = DEFAULT_ALERT_URL.to_string();
        }
        self
    }

    /// Get the config from the provided path (or default one), creating a default file if it doesn't exist.
    ///
    /// ## Returns
    ///
    /// The newly created or loaded config.
    ///
    /// ## Errors
    ///
    /// If we can't read the current config or create a new one.
    ///
    pub fn get(path: Option<PathBuf>) -> Result<Self> {
        let path = path.unwrap_or(Self::default_path()?);
        if path.is_file() {
            Self::read(&path)
        } else {
            let new_config = Self::default(Some(path))?;
            new_config.save()?;
            Ok(new_config)
        }
    }

    /// Checks that the path in `owml_path` is a valid OWML install (at least for our uses)
    ///
    /// ## Returns
    ///
    /// If the given folder contains the files needed by the manager to use OWML.
    /// Other files that make OWML work are not checked, only the ones the manager needs.
    ///
    pub fn check_owml(&self) -> bool {
        if self.owml_path.trim().is_empty() {
            false
        } else {
            let path = PathBuf::from(&self.owml_path);
            path.is_dir()
                && path.join(OWML_DEFAULT_CONFIG_NAME).is_file()
                && path.join(OWML_EXE_NAME).is_file()
                && path.join(OWML_MANIFEST_NAME).is_file()
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils::make_test_dir;

    use super::*;

    #[test]
    pub fn test_config_default() {
        let path = PathBuf::from("/test/path");
        let config = Config::default(Some(path)).unwrap();
        assert_eq!(config.database_url, DEFAULT_DB_URL);
    }

    #[test]
    pub fn test_config_save() {
        let dir = make_test_dir();
        let path = dir.path().join("settings.json");
        let mut config = Config::default(Some(path.clone())).unwrap();
        config.database_url = "test".to_string();
        config.save().unwrap();
        assert!(path.is_file());
        let new_config = Config::read(&path).unwrap();
        assert_eq!(config.database_url, new_config.database_url);
        dir.close().unwrap();
    }

    #[test]
    pub fn test_config_get_new() {
        let dir = make_test_dir();
        let path = dir.path().join("settings.json");
        let config = Config::get(Some(path.clone())).unwrap();
        assert!(path.is_file());
        assert_eq!(config.database_url, DEFAULT_DB_URL.to_string());
        dir.close().unwrap();
    }

    #[test]
    pub fn test_config_get_existing() {
        let dir = make_test_dir();
        let path = dir.path().join("settings.json");
        let mut config = Config::default(Some(path.clone())).unwrap();
        config.owml_path = "/different/path".to_string();
        config.save().unwrap();
        let config = Config::get(Some(path)).unwrap();
        assert_eq!(config.owml_path, "/different/path");
        dir.close().unwrap();
    }

    #[test]
    pub fn test_config_migrate_alert() {
        let dir = make_test_dir();
        let path = dir.path().join("settings.json");
        let mut config = Config::default(Some(path.clone())).unwrap();
        config.alert_url = OLD_ALERT_URL.to_string();
        config.save().unwrap();
        let config = Config::get(Some(path)).unwrap();
        assert_eq!(config.alert_url, DEFAULT_ALERT_URL);
        dir.close().unwrap();
    }

    #[test]
    pub fn test_check_owml_no_folder() {
        let dir = make_test_dir();
        let path = dir.path().join("settings.json");
        let mut config = Config::default(Some(path)).unwrap();
        config.owml_path = "/different/path".to_string();
        assert!(!config.check_owml());
        dir.close().unwrap();
    }
}
