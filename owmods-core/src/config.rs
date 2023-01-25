use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::utils::file::{deserialize_from_json, get_app_path, serialize_to_json};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub owml_path: String,
    pub log_socket: Option<u32>,
    pub database_url: String,
    pub alert_url: String,
}

pub fn config_path() -> Result<PathBuf, anyhow::Error> {
    let app_path = get_app_path()?;
    Ok(app_path.join("settings.json"))
}

pub fn generate_default_config() -> Result<Config, anyhow::Error> {
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
    write_config(&default_config)?;
    Ok(default_config)
}

pub fn get_config() -> Result<Config, anyhow::Error> {
    if config_exists() {
        read_config(&config_path()?)
    } else {
        generate_default_config()
    }
}

pub fn write_config(conf: &Config) -> Result<(), anyhow::Error> {
    serialize_to_json(&conf, &config_path()?, true)
}

pub fn read_config(path: &Path) -> Result<Config, anyhow::Error> {
    deserialize_from_json(path)
}

fn config_exists() -> bool {
    let config_path = config_path();
    match config_path {
        Ok(config_path) => config_path.is_file(),
        Err(_) => false,
    }
}
