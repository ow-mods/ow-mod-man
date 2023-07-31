use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use typeshare::typeshare;

use crate::{search::Searchable, validate::ModValidationError};

/// Represents an installed (and valid) mod
#[typeshare]
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalMod {
    pub enabled: bool,
    pub errors: Vec<ModValidationError>,
    pub mod_path: String,
    pub manifest: ModManifest,
}

impl LocalMod {
    pub fn uses_pre_patcher(&self) -> bool {
        self.manifest.patcher.is_some()
    }
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
    /// This is the same for [UnsafeLocalMod::Valid] and [UnsafeLocalMod::Invalid]
    pub fn get_path(&self) -> &str {
        match self {
            Self::Invalid(m) => &m.mod_path,
            Self::Valid(m) => &m.mod_path,
        }
    }
}

impl Searchable for UnsafeLocalMod {
    fn get_values(&self) -> Vec<String> {
        match self {
            UnsafeLocalMod::Invalid(m) => vec![m.display_path.clone()],
            UnsafeLocalMod::Valid(m) => vec![
                m.manifest.name.clone(),
                m.manifest.unique_name.clone(),
                m.manifest.author.clone(),
            ],
        }
    }
}

#[cfg(test)]
impl LocalMod {
    pub fn get_test(num: u8) -> Self {
        let txt =
            include_str!("../../test_files/test_local_mod.json").replace("$num$", &num.to_string());
        let manifest: ModManifest = serde_json::from_str(&txt).unwrap();
        Self {
            manifest,
            mod_path: "".to_string(),
            enabled: true,
            errors: vec![],
        }
    }
}

/// Get the paths to preserve for a mod, if [None] is passed the list will contain only `config.json`.
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
    pub patcher: Option<String>,
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
    pub settings: Option<Map<String, Value>>,
}
