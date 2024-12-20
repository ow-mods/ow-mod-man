use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use typeshare::typeshare;

use crate::{search::Searchable, validate::ModValidationError};

/// Represents an installed (and valid) mod
#[typeshare]
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalMod {
    /// Whether the mod is enabled
    pub enabled: bool,
    /// Any non-critical errors that occurred when loading the mod
    pub errors: Vec<ModValidationError>,
    /// The path to the mod
    pub mod_path: String,
    /// The manifest for the mod
    pub manifest: ModManifest,
}

/// Represents a mod that completely failed to load
#[typeshare]
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FailedMod {
    /// The error that caused the mod to fail to load
    pub error: ModValidationError,
    /// The path to the mod
    pub mod_path: String,
    /// The path to the mod relative to the mods folder, this usually will match the unique name so it's good for display
    pub display_path: String,
}

/// Represents a `LocalMod` that we aren't sure loaded successfully
#[typeshare]
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "loadState", content = "mod", rename_all = "camelCase")]
pub enum UnsafeLocalMod {
    /// A mod was loaded successfully
    Valid(Box<LocalMod>),
    /// A mod failed to load
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
    /// Get a test LocalMod for use in unit tests
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
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModManifest {
    /// The unique name of the mod
    pub unique_name: String,
    /// The name of the mod
    pub name: String,
    /// The author of the mod
    pub author: String,
    /// The version of the mod, usually in the format `major.minor.patch`
    pub version: String,
    /// The name of the DLL file to load when starting the mod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// The version of OWML this mod was built for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owml_version: Option<String>,
    /// The dependencies of the mod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
    /// The mods this mod will conflict with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<Vec<String>>,
    /// The paths to preserve when updating the mod
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths_to_preserve: Option<Vec<String>>,
    /// A warning the mod wants to show to the user on start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<ModWarning>,
    /// An exe that runs before the game starts, a prepatcher. This is used for mods that need to patch the game before it starts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patcher: Option<String>,
    /// A link to donate to the mod. May only be for Patreon or PayPal. This is deprecated in favor of `donate_links`
    ///
    /// It's recommended you use [ModManifest::migrate_donation_link] to migrate this to `donate_links`
    /// (this automatically handled if you're using [LocalDatabase])
    #[deprecated(since = "0.12.1", note = "please use `donate_links` instead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub donate_link: Option<String>,
    /// A list of links to donate to the mod (this replaced `donate_link`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub donate_links: Option<Vec<String>>,
}

impl ModManifest {
    #[allow(deprecated)]
    /// Migrates the `donate_link` field to `donate_links`
    /// Simply adds the `donate_link` to the `donate_links` vec (or creates it if it doesn't exist)
    pub fn migrate_donation_link(&mut self) {
        if let Some(link) = self.donate_link.to_owned() {
            if let Some(links) = self.donate_links.as_mut() {
                links.push(link);
            } else {
                self.donate_links = Some(vec![link]);
            }
        }
    }
}

/// Represents a warning a mod wants to show to the user on start
#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModWarning {
    /// The title of the warning
    pub title: String,
    /// The body of the warning
    pub body: String,
}

/// Represents a configuration file for a mod
#[derive(Debug, Serialize, Deserialize)]
pub struct ModStubConfig {
    /// Whether the mod is enabled
    pub enabled: bool,
    /// The settings for the mod, this is kept in a Map because the manager doesn't touch it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Map<String, Value>>,
}
