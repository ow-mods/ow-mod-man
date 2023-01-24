use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub owml_path: String,
    pub log_socket: Option<u32>,
    pub database_url: String,
    pub alert_url: String,
}

pub fn get_app_path() -> PathBuf {
    ProjectDirs::from("com", "ow-mods", "ow-mod-man")
        .expect("Couldn't Find App Data Directory")
        .data_dir()
        .to_path_buf()
}

pub fn config_path() -> PathBuf {
    get_app_path().join("settings.json")
}

fn config_exists() -> bool {
    config_path().is_file()
}

fn read_config() -> Config {
    let mut file = File::open(config_path()).expect("Couldn't Open Settings File");
    let mut raw = String::new();
    file.read_to_string(&mut raw)
        .expect("Couldn't Read Settings File");
    serde_json::from_str(&raw).expect("Couldn't Parse Settings Data")
}

pub fn generate_default_config() -> Config {
    let default_config = Config {
        owml_path: String::from(""),
        log_socket: Some(0),
        database_url: String::from(
            "https://raw.githubusercontent.com/ow-mods/ow-mod-db/master/database.json",
        ),
        alert_url: String::from(
            "https://raw.githubusercontent.com/ow-mods/ow-mod-db/source/alert.json",
        ),
    };
    write_config(&default_config);
    default_config
}

pub fn get_config() -> Config {
    if config_exists() {
        read_config()
    } else {
        generate_default_config()
    }
}

pub fn write_config(conf: &Config) {
    let serialized =
        serde_json::to_string_pretty(&conf).expect("Couldn't serialize settings data.");
    let path = config_path();
    create_dir_all(path.parent().unwrap()).expect("Couldn't Create Settings Data");
    let mut file = File::create(&path).expect("Couldn't Create Settings Data");
    file.write_all(serialized.as_bytes())
        .expect("Couldn't Write Settings Data");
}
