use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use glob::glob;
use serde::Deserialize;

use crate::log;
use crate::logging::Logger;
use crate::utils::file::{deserialize_from_json, fix_json};

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

pub struct LocalDatabase {
    pub mods: HashMap<String, LocalMod>,
}

impl RemoteDatabase {
    pub async fn fetch(conf: &Config) -> Result<RemoteDatabase, anyhow::Error> {
        let resp = reqwest::get(&conf.database_url).await?;
        let raw = resp.text().await?;
        let mut raw_db: RawRemoteDatabase = serde_json::from_str(&raw)?;
        let mut map = HashMap::new();
        while let Some(r_mod) = raw_db.releases.pop() {
            // Clones the string but at least not the entire mod
            map.insert(r_mod.unique_name.to_owned(), r_mod);
        }
        Ok(RemoteDatabase { mods: map })
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

pub async fn fetch_remote_db(conf: &Config) -> Result<RemoteDatabase, anyhow::Error> {
    RemoteDatabase::fetch(conf).await
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
