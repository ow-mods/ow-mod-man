use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use typeshare::typeshare;

use crate::{
    config::Config,
    constants::OWML_DEFAULT_CONFIG_NAME,
    file::{deserialize_from_json, serialize_to_json},
};

const fn _default_true() -> bool {
    true
}

const fn _default_false() -> bool {
    false
}

/// Represents the configuration for OWML
#[typeshare]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)] // Have to allow non_snake_case here because OWML's config uses "incrementalGC", which isn't proper camelCase
pub struct OWMLConfig {
    /// The path to the game
    pub game_path: String,
    #[serde(default = "_default_false")]
    debug_mode: bool,
    /// Whether to launch the game directly
    #[serde(default = "_default_false")]
    pub force_exe: bool,
    #[serde(default = "_default_true")]
    incremental_GC: bool,
    /// The path to OWML
    #[serde(skip_serializing_if = "Option::is_none")]
    owml_path: Option<String>,
    /// Mods that OWML has run a prepatcher for
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prepatched_mods: Vec<String>,
    /// The port to use for sending logs to
    pub socket_port: u16,
    #[typeshare(skip)]
    #[serde(flatten)]
    extra: Map<String, Value>,
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

    /// Get the default OWML config (OWML.DefaultConfig.json)
    ///
    /// ## Errors
    ///
    /// If we can't read the default config or can't get the user data dir. (Linux only)
    ///
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

    /// Get the default OWML config (OWML.DefaultConfig.json)
    ///
    /// ## Errors
    ///
    /// If we can't read the default config or can't get the user data dir. (Linux only)
    ///
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
    /// ## Examples
    ///
    /// ```no_run
    /// use owmods_core::config::Config;
    /// use owmods_core::owml::OWMLConfig;
    ///
    /// let config = Config::get(None).unwrap();
    /// let owml_config = OWMLConfig::get(&config).unwrap();
    /// println!("Game Path: {}", owml_config.game_path);
    /// ```
    ///
    /// ```no_run
    /// use owmods_core::config::Config;
    /// use owmods_core::owml::OWMLConfig;
    ///
    /// let config = Config::get(None).unwrap();
    /// std::fs::remove_file(&config.owml_path).unwrap();
    ///
    /// let owml_config = OWMLConfig::get(&config).unwrap();
    /// println!("Game Path: {}", owml_config.game_path);
    /// assert!(std::path::Path::new(&config.owml_path).is_file());
    /// ```
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
    /// ## Examples
    ///
    /// ```no_run
    /// use owmods_core::config::Config;
    /// use owmods_core::owml::OWMLConfig;
    ///
    /// let config = Config::get(None).unwrap();
    /// let mut owml_config = OWMLConfig::get(&config).unwrap();
    ///
    /// owml_config.force_exe = true;
    /// owml_config.save(&config).unwrap();
    /// ```
    ///
    pub fn save(&self, config: &Config) -> Result<()> {
        Self::write(self, config)
    }
}

#[cfg(test)]
mod tests {

    use std::fs;

    use crate::test_utils::{get_test_file, TestContext};

    use super::*;

    fn setup_default_conf(ctx: &TestContext) {
        fs::create_dir_all(ctx.temp_dir.path().join("OWML")).unwrap();
        fs::write(
            ctx.temp_dir
                .path()
                .join("OWML")
                .join("OWML.DefaultConfig.json"),
            include_str!("../test_files/OWML.Config.json"),
        )
        .unwrap();
    }

    #[test]
    fn test_owml_config_read() {
        let mut ctx = TestContext::new();
        ctx.config.owml_path = get_test_file("").to_str().unwrap().to_string();
        let conf = OWMLConfig::read(&ctx.config).unwrap();
        assert!(conf.debug_mode);
        assert!(conf.force_exe);
        assert!(conf.incremental_GC);
    }

    #[test]
    fn test_owml_config_save() {
        let ctx = TestContext::new();
        let owml_conf: OWMLConfig =
            serde_json::from_str(include_str!("../test_files/OWML.Config.json")).unwrap();
        owml_conf.save(&ctx.config).unwrap();
        assert!(ctx
            .temp_dir
            .path()
            .join("OWML")
            .join("OWML.Config.json")
            .is_file());
    }

    #[test]
    fn test_owml_config_get() {
        let ctx = TestContext::new();
        setup_default_conf(&ctx);
        let mut conf = OWMLConfig::get(&ctx.config).unwrap();
        conf.debug_mode = true;
        conf.save(&ctx.config).unwrap();
        let conf = OWMLConfig::get(&ctx.config).unwrap();
        assert!(conf.debug_mode);
    }

    #[test]
    fn test_owml_config_get_default() {
        let ctx = TestContext::new();
        setup_default_conf(&ctx);
        let conf = OWMLConfig::get(&ctx.config).unwrap();
        assert!(conf.debug_mode);
        assert!(ctx
            .temp_dir
            .path()
            .join("OWML")
            .join("OWML.Config.json")
            .is_file());
    }
}
