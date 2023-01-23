use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use colored::Colorize;
use glob::glob;
use serde::Deserialize;

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
        self.releases
            .iter()
            .find(|&remote_mod| remote_mod.unique_name == unique_name)
    }
}

impl LocalDatabase {
    pub fn get_mod(&self, unique_name: &str) -> Option<&LocalMod> {
        self.mods
            .iter()
            .find(|&local_mod| local_mod.manifest.unique_name == unique_name)
    }

    pub fn get_mod_path(&self, unique_name: &str) -> PathBuf {
        let local_mod = self
            .get_mod(unique_name)
            .unwrap_or_else(|| panic!("Mod {} Not Found", unique_name));
        PathBuf::from(&local_mod.mod_path)
    }
}

pub async fn fetch_remote_db(conf: &Config) -> RemoteDatabase {
    let resp = reqwest::get(&conf.database_url)
        .await
        .expect("Couldn't Fetch Database");
    let raw = resp.text().await.expect("Couldn't Fetch Database");
    serde_json::from_str(&raw).expect("Couldn't Parse Database File")
}

pub fn read_local_mod(manifest_path: &PathBuf) -> LocalMod {
    let folder_path = manifest_path.parent().unwrap().to_path_buf();
    let folder_name = folder_path
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap();
    let mut file = File::open(manifest_path)
        .unwrap_or_else(|_| panic!("Couldn't Open Manifest File For: {folder_name}"));
    let mut raw = String::new();
    file.read_to_string(&mut raw)
        .unwrap_or_else(|_| panic!("Couldn't Read Manifest File For: {folder_name}"));
    let manifest: ModManifest = serde_json::from_str(raw.strip_prefix('\u{FEFF}').unwrap_or(&raw))
        .unwrap_or_else(|_| panic!("Couldn't Parse Manifest File For: {folder_name}"));
    LocalMod {
        enabled: get_mod_enabled(&folder_path),
        manifest,
        mod_path: String::from(folder_path.to_str().unwrap()),
        errors: vec![],
    }
}

fn get_local_mods(conf: &Config) -> Vec<LocalMod> {
    let mut mods: Vec<LocalMod> = vec![];
    let glob_matches = glob(
        Path::new(&conf.owml_path)
            .join("Mods")
            .join("*")
            .join("manifest.json")
            .to_str()
            .unwrap(),
    )
    .expect("Couldn't Glob");
    for entry in glob_matches {
        match entry {
            Ok(path) => mods.push(read_local_mod(&path)),
            Err(why) => println!("Error Getting Mod: {:?}", why),
        }
    }
    mods
}

pub fn fetch_local_db(conf: &Config) -> LocalDatabase {
    if get_mods_dir(conf).is_dir() {
        LocalDatabase {
            mods: get_local_mods(conf),
        }
    } else {
        LocalDatabase { mods: vec![] }
    }
}

pub fn local_mod_list_str(conf: &Config) -> String {
    let db = fetch_local_db(conf);
    let mut output = String::new();
    output += &format!(
        "Found {} Installed Mods:\n(+): Enabled\n(-): Disabled\n\n",
        db.mods.len()
    );
    for local_mod in db.mods.iter() {
        output += &format!(
            "{} {} by {} ({})\n",
            if local_mod.enabled { "+" } else { "-" },
            local_mod.manifest.name,
            local_mod.manifest.author,
            &local_mod.manifest.unique_name.to_string().bold()
        );
    }
    output
}

pub async fn remote_mod_list_str(conf: &Config) -> String {
    let db = fetch_remote_db(conf).await;
    let mut output = String::new();
    output += &format!("Found {} Remote Mods:\n", db.releases.len());
    for remote_mod in db.releases.iter() {
        output += &format!(
            "- {} by {} ({})\n",
            remote_mod.name,
            remote_mod
                .author_display
                .as_ref()
                .unwrap_or(&remote_mod.author),
            &remote_mod.unique_name.to_string().bold()
        )
    }
    output
}
