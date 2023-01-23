use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::config::Config;

pub fn get_mods_dir(conf: &Config) -> PathBuf {
    Path::new(&conf.owml_path).join("Mods")
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteMod {
    pub download_url: String,
    download_count: u32,
    pub version: String,
    pub name: String,
    pub unique_name: String,
    required: Option<bool>,
    repo: String,
    pub author: String,
    pub author_display: Option<String>,
    parent: Option<String>,
    prerelease: Option<ModPrerelease>,
    alpha: Option<bool>,
    tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModPrerelease {
    download_url: String,
    version: String,
}

pub struct LocalMod {
    pub enabled: bool,
    pub errors: Vec<String>,
    pub mod_path: String,
    pub manifest: ModManifest,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModManifest {
    pub unique_name: String,
    pub name: String,
    pub author: String,
    pub version: String,
    owml_version: Option<String>,
    pub dependencies: Option<Vec<String>>,
    conflicts: Option<Vec<String>>,
    pub paths_to_preserve: Option<Vec<String>>,
}
