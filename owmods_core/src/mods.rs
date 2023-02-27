use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typeshare::typeshare;

use crate::{
    file::{deserialize_from_json, serialize_to_json},
    utils::fix_version,
};

use super::config::Config;

pub fn get_mods_dir(conf: &Config) -> PathBuf {
    Path::new(&conf.owml_path).join("Mods")
}

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
    pub fn get_author(&self) -> &String {
        self.author_display.as_ref().unwrap_or(&self.author)
    }

    pub fn get_version(&self) -> String {
        fix_version(&self.version)
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModPrerelease {
    pub download_url: String,
    pub version: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModReadMe {
    pub html_url: String,
    pub download_url: String,
}

#[typeshare]
#[derive(Clone, Serialize)]
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

#[typeshare]
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
    pub warning: Option<ModWarning>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModWarning {
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct ModStubConfig {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<HashMap<String, Value>>,
}

// Have to allow non_snake_case here because OWML's config uses incrementalGC, which isn't proper camelCase
#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
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

    pub fn get_from_path(path: &Path) -> Result<OWMLConfig> {
        deserialize_from_json(path)
    }

    pub fn save_to_path(&self, path: &Path) -> Result<()> {
        serialize_to_json(self, path, true)
    }

    fn load_default(config: &Config) -> Result<OWMLConfig> {
        deserialize_from_json(&Path::new(&config.owml_path).join("OWML.DefaultConfig.json"))
    }

    fn write(owml_config: &OWMLConfig, config: &Config) -> Result<()> {
        serialize_to_json(owml_config, &Self::path(config), true)?;
        Ok(())
    }

    pub fn get(config: &Config) -> Result<OWMLConfig> {
        if Self::path(config).is_file() {
            Self::read(config)
        } else {
            let new_conf = Self::load_default(config)?;
            new_conf.save(config)?;
            Ok(new_conf)
        }
    }

    pub fn save(&self, config: &Config) -> Result<()> {
        Self::write(self, config)
    }
}
