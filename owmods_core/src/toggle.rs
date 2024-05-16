use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use log::warn;

use crate::{
    db::LocalDatabase,
    file::{deserialize_from_json, fix_json_file, serialize_to_json},
    mods::local::{LocalMod, ModStubConfig},
};

fn read_config(config_path: &Path) -> Result<ModStubConfig> {
    fix_json_file(config_path).ok();
    deserialize_from_json(config_path)
}

fn write_config(conf: &ModStubConfig, config_path: &Path) -> Result<()> {
    serialize_to_json(&conf, config_path, false)?;
    Ok(())
}

/// Generates an empty, enabled config to the given path.
///
/// Note this doesn't read from default-config.json, it just creates a blank config.
/// OWML will fill in the default values when it loads the config.
///
/// ## Errors
///
/// If we can't create or serialize the config file.
///
pub fn generate_config(path: &Path) -> Result<()> {
    let new_config = ModStubConfig {
        enabled: true,
        settings: None,
    };
    serialize_to_json(&new_config, path, false)
}

/// Gets whether a mod is enabled given its mod path
///
/// ## Returns
///
/// true if the config exists and has `enabled: true`, false if the config doesn't exist or has `enabled: false`.
///
/// ## Errors
///
/// If we can't deserialize the config file (this will happen in the event of malformed JSON, not a missing file).
///
pub fn get_mod_enabled(mod_path: &Path) -> Result<bool> {
    let config_path = mod_path.join("config.json");
    if config_path.is_file() {
        let conf = read_config(&config_path)?;
        Ok(conf.enabled)
    } else {
        generate_config(&config_path)?;
        Ok(true)
    }
}

fn _toggle_mod(local_mod: &LocalMod, enabled: bool) -> Result<bool> {
    let config_path = PathBuf::from(&local_mod.mod_path).join("config.json");

    if config_path.is_file() {
        let mut config = read_config(&config_path)?;
        config.enabled = enabled;
        write_config(&config, &config_path)?;
    } else {
        generate_config(&config_path)?;
        return _toggle_mod(local_mod, enabled);
    }

    Ok(!enabled && local_mod.uses_pre_patcher())
}

/// Toggle a mod to a given enabled value.
/// Also supports applying this action recursively.
///
/// ## Returns
///
/// A list of mod unique names that were disabled and use pre patchers,
/// and therefore **should alert the user to check the mod's README for instructions on how to fully disable it**.
///
/// ## Errors
///
/// If we can't read/save to the config files of the mod or (if recursive is true) any of it's dependents.
///
pub fn toggle_mod(
    unique_name: &str,
    local_db: &LocalDatabase,
    enabled: bool,
    recursive: bool,
) -> Result<Vec<String>> {
    let mut show_warnings_for: Vec<String> = vec![];

    let local_mod = local_db
        .get_mod(unique_name)
        .ok_or_else(|| anyhow!("Mod {} not found in local database.", unique_name))?;
    let show_warning = _toggle_mod(local_mod, enabled)?;

    if show_warning {
        show_warnings_for.push(unique_name.to_string());
    }

    if recursive
        && !local_mod
            .manifest
            .dependencies
            .as_ref()
            .map(|d| d.is_empty())
            .unwrap_or(true)
    {
        let mut to_check: Vec<String> = local_mod.manifest.dependencies.clone().unwrap_or_default();
        let mut toggled_mods: Vec<String> = vec![unique_name.to_string()];
        while !to_check.is_empty() {
            for dep in std::mem::take(&mut to_check) {
                if toggled_mods.contains(&dep) {
                    continue;
                }
                let dep_mod = local_db.get_mod(&dep);
                if let Some(dep_mod) = dep_mod {
                    if dep_mod.enabled == enabled {
                        continue;
                    }
                    let show_warning = if enabled {
                        toggled_mods.push(dep_mod.manifest.unique_name.clone());
                        to_check.extend(dep_mod.manifest.dependencies.clone().unwrap_or_default());
                        _toggle_mod(dep_mod, enabled)
                    } else {
                        let mut flag = true;
                        for dependent_mod in local_db
                            .dependent(&dep_mod.manifest.unique_name)
                            .filter(|m| {
                                m.enabled && !toggled_mods.contains(&m.manifest.unique_name)
                            })
                        {
                            warn!(
                                "Not disabling {} as it's also needed by {}",
                                dep_mod.manifest.name, dependent_mod.manifest.name
                            );
                            flag = false;
                        }
                        if flag {
                            toggled_mods.push(dep_mod.manifest.unique_name.clone());
                            to_check
                                .extend(dep_mod.manifest.dependencies.clone().unwrap_or_default());
                            _toggle_mod(dep_mod, enabled)
                        } else {
                            Ok(false)
                        }
                    }?;
                    if show_warning {
                        show_warnings_for.push(dep_mod.manifest.unique_name.clone());
                    }
                } else {
                    warn!("Dependency {} Was Not Found, Ignoring.", dep);
                }
            }
        }
    }
    Ok(show_warnings_for)
}

#[cfg(test)]
mod tests {

    use std::fs::remove_file;

    use crate::{
        mods::local::{LocalMod, UnsafeLocalMod},
        test_utils::TestContext,
    };

    use super::*;

    #[test]
    fn test_toggle_mod() {
        let mut ctx = TestContext::new();
        ctx.install_test_zip("Bwc9876.TimeSaver.zip", true);
        toggle_mod("Bwc9876.TimeSaver", &ctx.local_db, false, false).unwrap();
        ctx.fetch_local_db();
        assert!(!ctx.local_db.get_mod("Bwc9876.TimeSaver").unwrap().enabled);
        toggle_mod("Bwc9876.TimeSaver", &ctx.local_db, true, false).unwrap();
        ctx.fetch_local_db();
        assert!(ctx.local_db.get_mod("Bwc9876.TimeSaver").unwrap().enabled);
    }

    #[test]
    fn test_toggle_mod_recursive() {
        let mut ctx = TestContext::new();
        let mut new_mod = ctx.install_test_zip("Bwc9876.TimeSaver.zip", false);
        ctx.install_test_zip("Bwc9876.SaveEditor.zip", true);
        new_mod.manifest.dependencies = Some(vec!["Bwc9876.SaveEditor".to_string()]);
        *ctx.local_db
            .mods
            .get_mut(&String::from("Bwc9876.TimeSaver"))
            .unwrap() = UnsafeLocalMod::Valid(Box::new(new_mod));
        toggle_mod("Bwc9876.TimeSaver", &ctx.local_db, false, true).unwrap();
        ctx.fetch_local_db();
        assert!(!ctx.local_db.get_mod("Bwc9876.TimeSaver").unwrap().enabled);
        assert!(!ctx.local_db.get_mod("Bwc9876.SaveEditor").unwrap().enabled);
    }

    #[test]
    fn test_toggle_mod_recursive_cyclical_deps() {
        let mut ctx = TestContext::new();
        let mut new_mod = ctx.install_test_zip("Bwc9876.TimeSaver.zip", false);
        let mut new_mod_2 = ctx.install_test_zip("Bwc9876.SaveEditor.zip", true);
        new_mod.manifest.dependencies = Some(vec!["Bwc9876.SaveEditor".to_string()]);
        *ctx.local_db
            .mods
            .get_mut(&String::from("Bwc9876.TimeSaver"))
            .unwrap() = UnsafeLocalMod::Valid(Box::new(new_mod));
        new_mod_2.manifest.dependencies = Some(vec!["Bwc9876.TimeSaver".to_string()]);
        *ctx.local_db
            .mods
            .get_mut(&String::from("Bwc9876.SaveEditor"))
            .unwrap() = UnsafeLocalMod::Valid(Box::new(new_mod_2));
        toggle_mod("Bwc9876.TimeSaver", &ctx.local_db, false, true).unwrap();
        ctx.fetch_local_db();
        assert!(!ctx.local_db.get_mod("Bwc9876.TimeSaver").unwrap().enabled);
        assert!(!ctx.local_db.get_mod("Bwc9876.SaveEditor").unwrap().enabled);
    }

    #[test]
    fn test_toggle_mod_recursive_other_dependent() {
        let mut ctx = TestContext::new();
        let mut new_mod = ctx.install_test_zip("Bwc9876.TimeSaver.zip", false);
        ctx.install_test_zip("Bwc9876.SaveEditor.zip", true);
        new_mod.manifest.dependencies = Some(vec!["Bwc9876.SaveEditor".to_string()]);
        *ctx.local_db
            .mods
            .get_mut(&String::from("Bwc9876.TimeSaver"))
            .unwrap() = UnsafeLocalMod::Valid(Box::new(new_mod));
        let mut test_mod = LocalMod::get_test(0);
        test_mod.manifest.dependencies = Some(vec![String::from("Bwc9876.SaveEditor")]);
        ctx.insert_test_mod(&test_mod);
        toggle_mod("Bwc9876.TimeSaver", &ctx.local_db, false, true).unwrap();
        ctx.fetch_local_db();
        assert!(!ctx.local_db.get_mod("Bwc9876.TimeSaver").unwrap().enabled);
        assert!(ctx.local_db.get_mod("Bwc9876.SaveEditor").unwrap().enabled);
    }

    #[test]
    fn test_toggle_mod_recursive_other_dependent_but_disabled() {
        let mut ctx = TestContext::new();
        let mut new_mod = ctx.install_test_zip("Bwc9876.TimeSaver.zip", false);
        ctx.install_test_zip("Bwc9876.SaveEditor.zip", true);
        new_mod.manifest.dependencies = Some(vec!["Bwc9876.SaveEditor".to_string()]);
        *ctx.local_db
            .mods
            .get_mut(&String::from("Bwc9876.TimeSaver"))
            .unwrap() = UnsafeLocalMod::Valid(Box::new(new_mod));
        let mut test_mod = LocalMod::get_test(0);
        test_mod.enabled = false;
        test_mod.manifest.dependencies = Some(vec![String::from("Bwc9876.SaveEditor")]);
        ctx.insert_test_mod(&test_mod);
        toggle_mod("Bwc9876.TimeSaver", &ctx.local_db, false, true).unwrap();
        ctx.fetch_local_db();
        assert!(!ctx.local_db.get_mod("Bwc9876.TimeSaver").unwrap().enabled);
        assert!(!ctx.local_db.get_mod("Bwc9876.SaveEditor").unwrap().enabled);
    }

    #[test]
    fn test_mod_toggle_no_config() {
        let mut ctx = TestContext::new();
        ctx.install_test_zip("Bwc9876.TimeSaver.zip", true);
        remove_file(ctx.get_test_path("Bwc9876.TimeSaver").join("config.json")).unwrap();
        toggle_mod("Bwc9876.TimeSaver", &ctx.local_db, false, false).unwrap();
        ctx.fetch_local_db();
        assert!(!ctx.local_db.get_mod("Bwc9876.TimeSaver").unwrap().enabled);
        toggle_mod("Bwc9876.TimeSaver", &ctx.local_db, true, false).unwrap();
        ctx.fetch_local_db();
        assert!(ctx.local_db.get_mod("Bwc9876.TimeSaver").unwrap().enabled);
    }

    #[test]
    fn test_toggle_mod_has_prepatcher() {
        let mut ctx = TestContext::new();
        let mut local_mod = ctx.install_test_zip("Bwc9876.TimeSaver.zip", true);
        local_mod.manifest.patcher = Some("SomePatcher.dll".to_string());
        *ctx.local_db
            .mods
            .get_mut(&String::from("Bwc9876.TimeSaver"))
            .unwrap() = UnsafeLocalMod::Valid(Box::new(local_mod));
        let show_warnings = toggle_mod("Bwc9876.TimeSaver", &ctx.local_db, false, false).unwrap();
        ctx.fetch_local_db();
        assert_eq!(show_warnings[0], "Bwc9876.TimeSaver");
        let show_warnings = toggle_mod("Bwc9876.TimeSaver", &ctx.local_db, true, false).unwrap();
        assert!(show_warnings.is_empty());
        ctx.fetch_local_db();
    }

    #[test]
    fn test_toggle_mod_has_prepatcher_recursive() {
        let mut ctx = TestContext::new();
        let mut local_mod = ctx.install_test_zip("Bwc9876.TimeSaver.zip", true);
        local_mod.manifest.patcher = Some("SomePatcher.dll".to_string());
        *ctx.local_db
            .mods
            .get_mut(&String::from("Bwc9876.TimeSaver"))
            .unwrap() = UnsafeLocalMod::Valid(Box::new(local_mod));
        let mut local_mod_2 = ctx.install_test_zip("Bwc9876.SaveEditor.zip", false);
        local_mod_2.manifest.dependencies = Some(vec!["Bwc9876.TimeSaver".to_string()]);
        local_mod_2.manifest.patcher = Some("SomePatcher.dll".to_string());
        ctx.insert_test_mod(&local_mod_2);
        let show_warnings = toggle_mod("Bwc9876.SaveEditor", &ctx.local_db, false, true).unwrap();
        ctx.fetch_local_db();
        assert!(show_warnings.contains(&"Bwc9876.TimeSaver".to_string()));
        assert!(show_warnings.contains(&"Bwc9876.SaveEditor".to_string()));
        let show_warnings = toggle_mod("Bwc9876.SaveEditor", &ctx.local_db, true, true).unwrap();
        assert!(show_warnings.is_empty());
        ctx.fetch_local_db();
    }
}
