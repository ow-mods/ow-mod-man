use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use glob::glob;
use serde::Deserialize;

use crate::file::{deserialize_from_json, fix_json};
use crate::log;
use crate::logging::Logger;

use super::config::Config;
use super::mods::{get_mods_dir, LocalMod, ModManifest, RemoteMod};
use super::toggle::get_mod_enabled;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawRemoteDatabase {
    pub releases: Vec<RemoteMod>,
}

pub struct RemoteDatabase {
    pub mods: HashMap<String, RemoteMod>,
}

#[derive(Clone)]
pub struct LocalDatabase {
    pub mods: HashMap<String, LocalMod>,
}

impl RemoteDatabase {
    pub fn empty() -> RemoteDatabase {
        RemoteDatabase {
            mods: HashMap::new(),
        }
    }

    pub async fn fetch(log: &Logger, conf: &Config) -> Result<RemoteDatabase, anyhow::Error> {
        log!(log, debug, "Fetching Remote DB At {}", conf.database_url);
        let resp = reqwest::get(&conf.database_url).await?;
        let raw = resp.text().await?;
        let raw_db: RawRemoteDatabase = serde_json::from_str(&raw)?;
        log.debug("Success, Constructing Mod Map");
        // Creating a hash map is O(N) but access is O(1).
        // In a cli context this doesn't rly matter since we usually only get one or two mods in the entire run of the program.
        // But I'm guessing for the GUI this will help out with performance.
        // Same thing for the local DB.
        Ok(RemoteDatabase {
            mods: raw_db
                .releases
                .into_iter()
                .map(|m| (m.unique_name.to_owned(), m))
                .collect::<HashMap<_, _>>(),
        })
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
    pub fn empty() -> LocalDatabase {
        LocalDatabase {
            mods: HashMap::new(),
        }
    }

    pub fn get_mod(&self, unique_name: &str) -> Option<&LocalMod> {
        self.mods.get(unique_name)
    }

    pub fn get_owml(&self, config: &Config) -> Option<LocalMod> {
        let manifest_path = PathBuf::from(&config.owml_path).join("OWML.Manifest.json");
        fix_json(&manifest_path).ok();
        let owml_manifest: ModManifest = deserialize_from_json(&manifest_path).ok()?;
        Some(LocalMod {
            enabled: true,
            manifest: owml_manifest,
            mod_path: "".to_string(), // <-- Empty bc the config already has it and also less copies
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
}

pub async fn fetch_remote_db(log: &Logger, conf: &Config) -> Result<RemoteDatabase, anyhow::Error> {
    RemoteDatabase::fetch(log, conf).await
}

pub fn read_local_mod(log: &Logger, manifest_path: &Path) -> Result<LocalMod, anyhow::Error> {
    log!(
        log,
        debug,
        "Loading Mod With Manifest: {}",
        manifest_path.to_str().unwrap()
    );
    let folder_path = manifest_path.parent();
    if folder_path.is_none() {
        return Err(anyhow!("Mod Path Not Found"));
    }
    let folder_path = folder_path.unwrap(); // <- Unwrap is safe, .is_none() check is above
    fix_json(manifest_path).ok();
    let manifest: ModManifest = deserialize_from_json(manifest_path)?;
    Ok(LocalMod {
        enabled: get_mod_enabled(folder_path)?,
        manifest,
        mod_path: String::from(folder_path.to_str().unwrap()),
        errors: vec![],
    })
}

fn get_local_mods(log: &Logger, conf: &Config) -> Result<HashMap<String, LocalMod>, anyhow::Error> {
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
        let local_mod = read_local_mod(log, &entry);
        if let Ok(local_mod) = local_mod {
            mods.insert(local_mod.manifest.unique_name.to_owned(), local_mod);
        } else {
            log!(
                log,
                error,
                "Error loading mod {}: {:?}",
                entry.to_str().unwrap(),
                local_mod.err().unwrap()
            );
        }
    }
    Ok(mods)
}

pub fn fetch_local_db(log: &Logger, conf: &Config) -> Result<LocalDatabase, anyhow::Error> {
    log!(log, debug, "Begin construction of local db");
    Ok(if get_mods_dir(conf).is_dir() {
        LocalDatabase {
            mods: get_local_mods(log, conf)?,
        }
    } else {
        LocalDatabase {
            mods: HashMap::new(),
        }
    })
}
