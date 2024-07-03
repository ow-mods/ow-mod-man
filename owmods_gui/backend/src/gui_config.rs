use std::path::PathBuf;

use owmods_core::file::{deserialize_from_json, get_app_path, serialize_to_json};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Default, Serialize, Deserialize, Clone)]
pub enum Theme {
    Blue,
    Red,
    Pink,
    Purple,
    Blurple,
    OuterWildsOrange,
    GhostlyGreen,
    NomaiBlue,
    NomaiYellow,
    #[default]
    #[serde(other)]
    Green,
}

#[typeshare]
#[derive(Default, Serialize, Deserialize, Clone)]
pub enum Language {
    Japanese,
    Chinese,
    Vietnamese,
    Wario,
    #[default]
    #[serde(other)]
    English,
}

const fn _default_true() -> bool {
    true
}

const fn _default_false() -> bool {
    false
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GuiConfig {
    #[serde(default = "Language::default")]
    language: Language,
    #[serde(default = "Theme::default")]
    theme: Theme,
    #[serde(default = "_default_true")]
    pub watch_fs: bool,
    #[serde(default = "_default_false")]
    no_warning: bool,
    #[serde(default = "_default_false")]
    pub log_multi_window: bool,
    #[serde(default = "_default_false")]
    auto_enable_deps: bool,
    #[serde(default = "_default_false")]
    auto_disable_deps: bool,
    #[serde(default = "_default_false")]
    pub no_log_server: bool,
    #[serde(default = "_default_false")]
    pub hide_installed_in_remote: bool,
    #[serde(default = "_default_false")]
    hide_mod_thumbnails: bool,
    #[serde(default = "_default_false")]
    pub hide_dlc: bool,
    #[serde(default = "_default_false")]
    pub hide_donate: bool,
    #[serde(default = "_default_false")]
    rainbow: bool,
}

impl Default for GuiConfig {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            rainbow: false,
            hide_mod_thumbnails: false,
            language: Language::default(),
            watch_fs: true,
            no_warning: false,
            log_multi_window: false,
            auto_disable_deps: false,
            auto_enable_deps: false,
            hide_installed_in_remote: false,
            hide_donate: false,
            no_log_server: false,
            hide_dlc: false,
        }
    }
}

impl GuiConfig {
    pub fn path() -> Result<PathBuf, anyhow::Error> {
        let path = get_app_path()?.join("gui_settings.json");
        Ok(path)
    }

    fn read() -> Result<Self, anyhow::Error> {
        let conf = deserialize_from_json::<GuiConfig>(&Self::path()?)?;
        Ok(conf)
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
