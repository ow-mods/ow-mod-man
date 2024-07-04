
use std::process::Command;

use anyhow::Result;
use serde_json::{Value, from_str};
use toml_edit::{Document, value};

const GUI_TAURI_CONF_PATH: &str = "owmods_gui/backend/tauri.conf.json";
const GUI_CARGO_TOML_PATH: &str = "owmods_gui/backend/Cargo.toml";

pub fn disable_updater() -> Result<()> {
    println!("Disabling updater...");
    // tauri.conf.json
    let tauri_conf = std::fs::read_to_string(GUI_TAURI_CONF_PATH)?;
    let mut tauri_conf: Value = from_str(&tauri_conf)?;
    tauri_conf["tauri"]["updater"]["active"] = false.into();   
    // Cargo.toml
    let cargo_toml = std::fs::read_to_string(GUI_CARGO_TOML_PATH)?;
    let mut cargo_toml = cargo_toml.parse::<Document>()?;
    let mut features = cargo_toml["dependencies"]["tauri"]["features"].as_array_mut().unwrap();
    features.retain(|f| f.as_str().unwrap() != "updater");
    // Write to files
    std::fs::write(GUI_TAURI_CONF_PATH, serde_json::to_string_pretty(&tauri_conf)?)?;
    std::fs::write(GUI_CARGO_TOML_PATH, cargo_toml.to_string())?;

    println!("Refetching dependencies...");

    let cmd = Command::new("cargo")
        .arg("update")
        .current_dir("owmods_gui/backend")
        .output()?;

    println!("Updater disabled.");

    Ok(())
}
