use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::{
    log,
    logging::Logger,
    utils::file::{deserialize_from_json, get_app_path, serialize_to_json},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub owml_path: String,
    pub wine_prefix: Option<String>,
    pub database_url: String,
    pub alert_url: String,
}

pub fn config_path() -> Result<PathBuf, anyhow::Error> {
    let app_path = get_app_path()?;
    Ok(app_path.join("settings.json"))
}

pub fn generate_default_config(log: &Logger) -> Result<Config, anyhow::Error> {
    let default_config = Config {
        owml_path: String::from(""),
        wine_prefix: None,
        database_url: String::from(
            "https://raw.githubusercontent.com/ow-mods/ow-mod-db/master/database.json",
        ),
        alert_url: String::from(
            "https://raw.githubusercontent.com/ow-mods/ow-mod-db/source/alert.json",
        ),
    };
    write_config(log, &default_config)?;
    Ok(default_config)
}

pub fn get_config(log: &Logger) -> Result<Config, anyhow::Error> {
    if config_exists() {
        read_config(log, &config_path()?)
    } else {
        generate_default_config(log)
    }
}

pub fn write_config(log: &Logger, conf: &Config) -> Result<(), anyhow::Error> {
    log!(
        log,
        debug,
        "Writing Config To {}",
        config_path()?.to_str().unwrap()
    );
    serialize_to_json(&conf, &config_path()?, true)
}

pub fn read_config(log: &Logger, path: &Path) -> Result<Config, anyhow::Error> {
    log!(log, debug, "Reading Config From {}", path.to_str().unwrap());
    deserialize_from_json(path)
}

fn config_exists() -> bool {
    let config_path = config_path();
    match config_path {
        Ok(config_path) => config_path.is_file(),
        Err(_) => false,
    }
}
