use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use serde_json::Map;

use super::db::{read_local_mod, LocalDatabase};

#[derive(Serialize, Deserialize)]
struct ModStubConfig {
    enabled: bool,
    settings: Option<Map<String, serde_json::Value>>,
}

fn get_mod_config_path(mod_path: &Path) -> PathBuf {
    mod_path.join("config.json")
}

fn read_config(config_path: &Path) -> ModStubConfig {
    let mut file = File::open(config_path).expect("Couldn't Read Config");
    let mut raw = String::new();
    file.read_to_string(&mut raw).expect("Couldn't Read Config");
    serde_json::from_str(
        &raw.strip_prefix('\u{FEFF}')
            .unwrap_or(&raw)
            .replace("\"true\"", "true"),
    )
    .expect("Couldn't Parse config")
}

fn write_config(conf: &ModStubConfig, config_path: &Path) {
    let serialized = serde_json::to_string_pretty(&conf)
        .expect("Couldn't Serialize Config data.")
        .replace("null", "{}");
    let mut file = File::create(config_path).expect("Couldn't Create Config Data");
    file.write_all(serialized.as_bytes())
        .expect("Couldn't Write Config Data");
}

pub fn copy_default_config(mod_path: &Path) {
    let default_config_path = mod_path.join("default-config.json");
    if default_config_path.is_file() {
        let default_config = read_config(&default_config_path);
        let config_path = get_mod_config_path(mod_path);
        write_config(&default_config, &config_path)
    } else {
        panic!(
            "No Default Config File Found For {}",
            &mod_path.to_str().unwrap()
        );
    }
}

pub fn get_mod_enabled(mod_path: &Path) -> bool {
    let config_path = get_mod_config_path(mod_path);
    if config_path.is_file() {
        read_config(&config_path).enabled
    } else {
        false
    }
}

pub fn toggle_mod(mod_path: &Path, local_db: &LocalDatabase, enabled: bool, recursive: bool) {
    let config_path = get_mod_config_path(mod_path);
    if config_path.is_file() {
        let config_path = get_mod_config_path(mod_path);
        let mut config = read_config(&config_path);
        config.enabled = enabled;
        write_config(&config, &config_path);
    } else {
        copy_default_config(mod_path);
        toggle_mod(mod_path, local_db, enabled, recursive);
    }
    if recursive {
        let local_mod = read_local_mod(&mod_path.join("manifest.json"));
        if let Some(deps) = local_mod.manifest.dependencies {
            for dep in deps.iter() {
                let dep_mod = local_db.get_mod(dep);
                if let Some(dep_mod) = dep_mod {
                    toggle_mod(
                        &PathBuf::from(&dep_mod.mod_path),
                        local_db,
                        enabled,
                        recursive,
                    );
                }
            }
        }
    }
}
