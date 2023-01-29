use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::utils::fix_version;

use super::config::Config;

pub fn get_mods_dir(conf: &Config) -> PathBuf {
    Path::new(&conf.owml_path).join("Mods")
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteMod {
    pub download_url: String,
    pub download_count: u32,
    pub version: String,
    pub name: String,
    pub unique_name: String,
    pub readme: Option<ModReadMe>,
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
    pub fn get_author(&self) -> &String {
        self.author_display.as_ref().unwrap_or(&self.author)
    }

    pub fn get_version(&self) -> String {
        fix_version(&self.version)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModPrerelease {
    pub download_url: String,
    pub version: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModReadMe {
    pub html_url: String,
    pub download_url: String,
}

#[derive(Clone)]
pub struct LocalMod {
    pub enabled: bool,
    pub errors: Vec<String>,
    pub mod_path: String,
    pub manifest: ModManifest,
}

impl LocalMod {
    pub fn get_version(&self) -> String {
        fix_version(&self.manifest.version)
    }
}

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
    vec![]
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModManifest {
    pub unique_name: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub owml_version: Option<String>,
    pub dependencies: Option<Vec<String>>,
    pub conflicts: Option<Vec<String>>,
    pub paths_to_preserve: Option<Vec<String>>,
}
