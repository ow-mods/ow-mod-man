use std::path::{Path, PathBuf};

use anyhow::anyhow;
use glob::glob;
use serde::Deserialize;

use crate::utils::file::{deserialize_from_json, fix_json};

use super::config::Config;
use super::mods::{get_mods_dir, LocalMod, ModManifest, RemoteMod};
use super::toggle::get_mod_enabled;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteDatabase {
    pub releases: Vec<RemoteMod>,
}

pub struct LocalDatabase {
    pub mods: Vec<LocalMod>,
}

impl RemoteDatabase {
    pub fn get_mod(&self, unique_name: &str) -> Option<&RemoteMod> {
        let found_mod = self
            .releases
            .iter()
            .find(|&remote_mod| remote_mod.unique_name == unique_name);
        // Don't treat OWML as a normal mod
        found_mod.filter(|&found_mod| found_mod.unique_name != "Alek.OWML")
    }

    pub fn get_owml(&self) -> Option<&RemoteMod> {
        self.releases
            .iter()
            .find(|&remote_mod| remote_mod.unique_name == "Alek.OWML")
    }
}

impl LocalDatabase {
    pub fn get_mod(&self, unique_name: &str) -> Option<&LocalMod> {
        self.mods
            .iter()
            .find(|&local_mod| local_mod.manifest.unique_name == unique_name)
    }

    pub fn get_owml(&self, config: &Config) -> Option<LocalMod> {
        let manifest_path = PathBuf::from(&config.owml_path).join("OWML.Manifest.json");
        fix_json(&manifest_path).ok();
        let owml_manifest: ModManifest = deserialize_from_json(&manifest_path).ok()?;
        Some(LocalMod {
            enabled: true,
            manifest: owml_manifest,
            mod_path: "".to_string(), // <-- Empty bc the config already has it and also borrow checker angry
            errors: vec![],
        })
    }

    pub fn get_mod_path(&self, unique_name: &str) -> Option<PathBuf> {
        let local_mod = self.get_mod(unique_name)?;
        Some(PathBuf::from(&local_mod.mod_path))
    }
}

pub async fn fetch_remote_db(conf: &Config) -> Result<RemoteDatabase, anyhow::Error> {
    let resp = reqwest::get(&conf.database_url).await?;
    let raw = resp.text().await?;
    let db = serde_json::from_str(&raw)?;
    Ok(db)
}

pub fn read_local_mod(manifest_path: &Path) -> Result<LocalMod, anyhow::Error> {
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

fn get_local_mods(conf: &Config) -> Result<Vec<LocalMod>, anyhow::Error> {
    let mut mods: Vec<LocalMod> = vec![];
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
        let local_mod = read_local_mod(&entry)
            .map_err(|e| anyhow!("Can't Load Mod {}: {:?}", entry.to_str().unwrap(), e))?;
        mods.push(local_mod);
    }
    Ok(mods)
}

pub fn fetch_local_db(conf: &Config) -> Result<LocalDatabase, anyhow::Error> {
    Ok(if get_mods_dir(conf).is_dir() {
        LocalDatabase {
            mods: get_local_mods(conf)?,
        }
    } else {
        LocalDatabase { mods: vec![] }
    })
}
