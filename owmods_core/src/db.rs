use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use anyhow::Result;
use glob::glob;
use log::{debug, error};
use serde::Deserialize;

use crate::file::{deserialize_from_json, fix_json};

use super::config::Config;
use super::mods::{get_mods_dir, LocalMod, ModManifest, RemoteMod};
use super::toggle::get_mod_enabled;

fn fix_version(version: &str) -> String {
    let mut str = version.to_owned();
    while str.starts_with('v') {
        str = str.strip_prefix('v').unwrap_or(&str).to_string();
    }
    str
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawRemoteDatabase {
    pub releases: Vec<RemoteMod>,
}

#[derive(Default)]
pub struct RemoteDatabase {
    pub mods: HashMap<String, RemoteMod>,
}

#[derive(Default)]
pub struct LocalDatabase {
    pub mods: HashMap<String, LocalMod>,
}

impl RemoteDatabase {
    pub async fn fetch(conf: &Config) -> Result<RemoteDatabase> {
        debug!("Fetching Remote DB At {}", conf.database_url);
        let resp = reqwest::get(&conf.database_url).await?;
        let raw = resp.text().await?;
        let raw_db: RawRemoteDatabase = serde_json::from_str(&raw)?;
        debug!("Success, Constructing Mod Map");

        let mut mods = raw_db
            .releases
            .into_iter()
            .map(|m| (m.unique_name.to_owned(), m))
            .collect::<HashMap<_, _>>();

        for remote_mod in mods.values_mut() {
            remote_mod.version = fix_version(&remote_mod.version);
        }

        // Creating a hash map is O(N) but access is O(1).
        // In a cli context this doesn't rly matter since we usually only get one or two mods in the entire run of the program.
        // But I'm guessing for the GUI this will help out with performance.
        // Same thing for the local DB.
        Ok(RemoteDatabase { mods })
    }

    pub fn get_mod(&self, unique_name: &str) -> Option<&RemoteMod> {
        if unique_name == "Alek.OWML" {
            return None;
        }
        self.mods.get(unique_name)
    }

    pub fn get_owml(&self) -> Option<&RemoteMod> {
        self.mods.get("Alek.OWML")
    }
}

impl LocalDatabase {
    pub fn fetch(conf: &Config) -> Result<Self> {
        debug!("Begin construction of local db at {}", conf.owml_path);
        Ok(if get_mods_dir(conf).is_dir() {
            Self {
                mods: Self::get_local_mods(conf)?,
            }
        } else {
            Self::default()
        })
    }

    pub fn get_mod(&self, unique_name: &str) -> Option<&LocalMod> {
        self.mods.get(unique_name)
    }

    pub fn get_owml(&self, config: &Config) -> Option<LocalMod> {
        let manifest_path = PathBuf::from(&config.owml_path).join("OWML.Manifest.json");
        fix_json(&manifest_path).ok();
        let mut owml_manifest: ModManifest = deserialize_from_json(&manifest_path).ok()?;
        owml_manifest.version = fix_version(&owml_manifest.version);
        Some(LocalMod {
            enabled: true,
            manifest: owml_manifest,
            mod_path: "".to_string(), // <-- Empty bc the config already has it and also less copies
            errors: vec![],
        })
    }

    pub fn read_local_mod(manifest_path: &Path) -> Result<LocalMod> {
        debug!(
            "Loading Mod With Manifest: {}",
            manifest_path.to_str().unwrap()
        );
        let folder_path = manifest_path.parent();
        if folder_path.is_none() {
            return Err(anyhow!("Mod Path Not Found"));
        }
        let folder_path = folder_path.unwrap(); // <- Unwrap is safe, .is_none() check is above
        fix_json(manifest_path).ok();
        let mut manifest: ModManifest = deserialize_from_json(manifest_path)?;
        manifest.version = fix_version(&manifest.version);
        Ok(LocalMod {
            enabled: get_mod_enabled(folder_path)?,
            manifest,
            mod_path: String::from(folder_path.to_str().unwrap()),
            errors: vec![],
        })
    }

    pub fn get_mod_path(&self, unique_name: &str) -> Option<PathBuf> {
        let local_mod = self.get_mod(unique_name)?;
        Some(PathBuf::from(&local_mod.mod_path))
    }

    pub fn active(&self) -> Vec<&LocalMod> {
        self.mods
            .values()
            .into_iter()
            .filter(|m| m.enabled)
            .collect()
    }

    fn get_local_mods(conf: &Config) -> Result<HashMap<String, LocalMod>> {
        let mut mods: HashMap<String, LocalMod> = HashMap::new();
        let glob_matches = glob(
            Path::new(&conf.owml_path)
                .join("Mods")
                .join("*")
                .join("manifest.json")
                .to_str()
                .unwrap(),
        )?;
        for entry in glob_matches {
            let entry = entry?;
            let local_mod = Self::read_local_mod(&entry);
            if let Ok(local_mod) = local_mod {
                mods.insert(local_mod.manifest.unique_name.to_owned(), local_mod);
            } else {
                error!(
                    "Error loading mod {}: {:?}",
                    entry.to_str().unwrap(),
                    local_mod.err().unwrap()
                );
            }
        }
        Ok(mods)
    }
}
