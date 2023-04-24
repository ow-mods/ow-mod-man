use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typeshare::typeshare;

use crate::{
    constants::OWML_DEFAULT_CONFIG_NAME,
    file::{deserialize_from_json, serialize_to_json},
    validate::ModValidationError,
};

use super::config::Config;

/// Represents a mod in the remote database
#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteMod {
    pub download_url: String,
    pub download_count: u32,
    pub version: String,
    pub name: String,
    pub unique_name: String,
    pub description: String,
    pub readme: Option<ModReadMe>,
    pub slug: String,
    required: Option<bool>,
    pub repo: String,
    pub author: String,
    pub author_display: Option<String>,
    pub parent: Option<String>,
    pub prerelease: Option<ModPrerelease>,
    alpha: Option<bool>,
    pub tags: Option<Vec<String>>,
}

impl RemoteMod {
    /// Get the author of a mod, first checking `author_display`, then falling back to `author`.
    pub fn get_author(&self) -> &String {
        self.author_display.as_ref().unwrap_or(&self.author)
    }

    #[cfg(test)]
    pub fn get_test(num: u8) -> Self {
        serde_json::from_str(
            &include_str!("../test_files/test_remote_mod.json").replace("$num$", &num.to_string()),
        )
        .unwrap()
    }
}

/// A prerelease for a mod
#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModPrerelease {
    pub download_url: String,
    pub version: String,
}

/// Contains URLs for a mod's README
#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModReadMe {
    pub html_url: String,
    pub download_url: String,
}

/// Represents an installed mod
#[typeshare]
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalMod {
    pub enabled: bool,
    pub errors: Vec<ModValidationError>,
    pub mod_path: String,
    pub manifest: ModManifest,
}

/// Represents a mod that completely failed to load
#[typeshare]
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FailedMod {
    pub error: ModValidationError,
    pub mod_path: String,
    pub display_path: String,
}

/// Represents a `LocalMod` that we aren't sure loaded successfully
#[typeshare]
#[derive(Serialize, Clone)]
#[serde(tag = "loadState", content = "mod", rename_all = "camelCase")]
#[allow(clippy::large_enum_variant)]
pub enum UnsafeLocalMod {
    Valid(LocalMod),
    Invalid(FailedMod),
}

impl UnsafeLocalMod {
    /// Get errors for a mod,
    /// - If this is a [UnsafeLocalMod::Valid] we get all validation errors,
    /// - If it's a [UnsafeLocalMod::Invalid] we get a vec with the error that occurred when loading
    ///
    pub fn get_errs(&self) -> Vec<&ModValidationError> {
        match self {
            Self::Invalid(m) => {
                vec![&m.error]
            }
            Self::Valid(m) => {
                if m.enabled {
                    m.errors.iter().collect()
                } else {
                    vec![]
                }
            }
        }
    }

    /// Get the unique name for a mod,
    /// - If this is a [UnsafeLocalMod::Valid] we get the unique name,
    /// - If it's a [UnsafeLocalMod::Invalid] we get the mod path
    ///
    pub fn get_unique_name(&self) -> &String {
        match self {
            Self::Invalid(m) => &m.mod_path,
            Self::Valid(m) => &m.manifest.unique_name,
        }
    }

    /// Get the name for a mod,
    /// - If this is a [UnsafeLocalMod::Valid] we get the name in the manifest,
    /// - If it's a [UnsafeLocalMod::Invalid] we just get the display path
    ///
    pub fn get_name(&self) -> &String {
        match self {
            Self::Invalid(m) => &m.display_path,
            Self::Valid(m) => &m.manifest.name,
        }
    }

    /// Get enabled for a mod,
    /// - If this is a [UnsafeLocalMod::Valid] we get is the mod is enabled in `config.json`,
    /// - If it's a [UnsafeLocalMod::Invalid] we get false always
    ///
    pub fn get_enabled(&self) -> bool {
        match self {
            Self::Invalid(_) => false,
            Self::Valid(m) => m.enabled,
        }
    }

    /// Gets the path for a mod
    pub fn get_path(&self) -> &str {
        match self {
            Self::Invalid(m) => &m.mod_path,
            Self::Valid(m) => &m.mod_path,
        }
    }
}

#[cfg(test)]
impl LocalMod {
    pub fn get_test(num: u8) -> Self {
        let txt =
            include_str!("../test_files/test_local_mod.json").replace("$num$", &num.to_string());
        let manifest: ModManifest = serde_json::from_str(&txt).unwrap();
        Self {
            manifest,
            mod_path: "".to_string(),
            enabled: true,
            errors: vec![],
        }
    }
}

/// Get the paths to preserve for a mod, if [None] is passed the list will be empty.
pub fn get_paths_to_preserve(local_mod: Option<&LocalMod>) -> Vec<PathBuf> {
    if let Some(local_mod) = local_mod {
        let mut paths: Vec<PathBuf> =
            vec![PathBuf::from("config.json"), PathBuf::from("save.json")];
        if let Some(raw_paths) = local_mod.manifest.paths_to_preserve.to_owned() {
            for path in raw_paths.iter() {
                paths.push(PathBuf::from(path));
            }
        }
        return paths;
    }
    vec![PathBuf::from("config.json")] // We can't trust the mod's config.json that comes with it (look at cheat and debug menu)
}

/// Represents a manifest file for a local mod.
#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModManifest {
    pub unique_name: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub filename: Option<String>,
    pub owml_version: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub conflicts: Option<Vec<String>>,
    pub paths_to_preserve: Option<Vec<String>>,
    pub warning: Option<ModWarning>,
}

/// Represents a warning a mod wants to show to the user on start
#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModWarning {
    pub title: String,
    pub body: String,
}

/// Represents a configuration file for a mod
#[derive(Serialize, Deserialize)]
pub struct ModStubConfig {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<HashMap<String, Value>>,
}

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
    fn load_default(config: &Config) -> Result<OWMLConfig> {
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
    fn load_default(config: &Config) -> Result<OWMLConfig> {
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
            let new_conf = Self::load_default(config)?;
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
