use std::path::PathBuf;

use owmods_core::file::{deserialize_from_json, get_app_path, serialize_to_json};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
pub enum Theme {
    White,
    Blue,
    Green,
    Pink,
    Yellow,
    Orange,
    Blurple,
    GhostlyGreen,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
pub enum Language {
    English,
    Wario,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GuiConfig {
    theme: Theme,
    rainbow: bool,
    language: Language,
    watch_fs: bool,
}

impl Default for GuiConfig {
    fn default() -> Self {
        Self {
            theme: Theme::White,
            rainbow: false,
            language: Language::English,
            watch_fs: true,
        }
    }
}

impl GuiConfig {
    fn path() -> Result<PathBuf, anyhow::Error> {
        let path = get_app_path()?.join("gui_settings.json");
        Ok(path)
    }

    fn read() -> Result<Self, anyhow::Error> {
        deserialize_from_json::<GuiConfig>(&Self::path()?)
    }

    fn write(config: &Self) -> Result<(), anyhow::Error> {
        serialize_to_json(config, &Self::path()?, true)
    }

    pub fn get() -> Result<Self, anyhow::Error> {
        if Self::path()?.is_file() {
            Self::read()
        } else {
            let new = Self::default();
            Self::write(&new)?;
            Ok(new)
        }
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        Self::write(self)
    }
}
