use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::file::{deserialize_from_json, get_app_path, serialize_to_json};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub owml_path: String,
    pub wine_prefix: Option<String>,
    pub database_url: String,
    pub alert_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            owml_path: String::from(""),
            wine_prefix: None,
            database_url: String::from("https://ow-mods.github.io/ow-mod-db/database.json"),
            alert_url: String::from(
                "https://raw.githubusercontent.com/ow-mods/ow-mod-db/source/alert.json",
            ),
        }
    }
}

impl Config {
    fn path() -> Result<PathBuf, anyhow::Error> {
        let app_path = get_app_path()?;
        Ok(app_path.join("settings.json"))
    }

    pub fn save(&self) -> Result<()> {
        debug!("Writing Config To {}", Self::path()?.to_str().unwrap());
        serialize_to_json(self, &Self::path()?, true)?;
        Ok(())
    }

    fn read(path: &Path) -> Result<Self> {
        debug!("Reading Config From {}", path.to_str().unwrap());
        deserialize_from_json(path)
    }

    pub fn get() -> Result<Self> {
        if Self::path()?.is_file() {
            Self::read(&Self::path()?)
        } else {
            let new_config = Self::default();
            new_config.save()?;
            Ok(new_config)
        }
    }
}
