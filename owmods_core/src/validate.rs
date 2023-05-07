use std::path::PathBuf;

use anyhow::Result;
use log::info;
use serde::Serialize;
use typeshare::typeshare;

use crate::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    download::install_mods_parallel,
    mods::local::LocalMod,
    toggle::toggle_mod,
};

/// Represents an error with a [LocalMod]
#[typeshare]
#[derive(Serialize, Clone)]
#[serde(tag = "errorType", content = "payload")]
pub enum ModValidationError {
    /// The mod's manifest was invalid, contains the error encountered when loading it
    InvalidManifest(String),
    /// The mod is missing a dependency that needs to be installed, contains the unique name of the missing dep
    MissingDep(String),
    /// A dependency of the mod is disabled, contains the unique name of the disabled dep
    DisabledDep(String),
    /// There's another enabled mod that conflicts with this one, contains the conflicting mod
    ConflictingMod(String),
    /// The DLL the mod specifies in its `manifest.json` doesn't exist, contains the path (if even present) to the DLL specified by the mod
    MissingDLL(Option<String>),
    /// There's another mod already in the DB with this mod's unique name, contains the path of the other mod that has the same unique name
    DuplicateMod(String),
}

fn check_mod_dll(local_mod: &LocalMod) -> Option<ModValidationError> {
    if let Some(dll_name) = local_mod.manifest.filename.as_ref() {
        let dll_path = PathBuf::from(local_mod.mod_path.clone()).join(dll_name);
        if dll_path.is_file() {
            None
        } else {
            Some(ModValidationError::MissingDLL(Some(dll_name.to_string())))
        }
    } else {
        Some(ModValidationError::MissingDLL(None))
    }
}

fn check_mod_deps(local_mod: &LocalMod, db: &LocalDatabase) -> Vec<ModValidationError> {
    let mut errors: Vec<ModValidationError> = vec![];
    if let Some(deps) = &local_mod.manifest.dependencies {
        for unique_name in deps {
            if let Some(dep_mod) = db.get_mod(unique_name) {
                if !dep_mod.enabled {
                    errors.push(ModValidationError::DisabledDep(unique_name.clone()))
                }
            } else {
                errors.push(ModValidationError::MissingDep(unique_name.clone()))
            }
        }
    }
    errors
}

fn check_mod_conflicts(local_mod: &LocalMod, db: &LocalDatabase) -> Vec<ModValidationError> {
    let mut errors: Vec<ModValidationError> = vec![];
    let active_mods: Vec<&String> = db.active().map(|m| &m.manifest.unique_name).collect();
    if let Some(conflicts) = &local_mod.manifest.conflicts {
        for conflict in conflicts.iter() {
            if active_mods.contains(&conflict) {
                errors.push(ModValidationError::ConflictingMod(conflict.clone()));
            }
        }
    }
    errors
}

/// Check a local mod for issues such as:
/// - Missing/Disabled Dependencies
/// - Conflicting Mods
/// - Missing DLL File
///
/// ## Returns
///
/// A Vec of [ModValidationError] that contains all errors we found.
///
pub fn check_mod(local_mod: &LocalMod, db: &LocalDatabase) -> Vec<ModValidationError> {
    let mut errors: Vec<ModValidationError> = vec![];
    errors.extend(check_mod_deps(local_mod, db).into_iter());
    errors.extend(check_mod_conflicts(local_mod, db).into_iter());
    if let Some(dll_error) = check_mod_dll(local_mod) {
        errors.push(dll_error);
    }
    errors
}

/// Auto-fix dependency issues.
/// Enables the disabled dependencies and installs missing ones.
///
/// ## Errors
///
/// If we can't install/enable the dependencies.
///
pub async fn fix_deps(
    local_mod: &LocalMod,
    config: &Config,
    db: &LocalDatabase,
    remote_db: &RemoteDatabase,
) -> Result<()> {
    let errors = check_mod_deps(local_mod, db);
    let mut missing: Vec<String> = vec![];
    for error in errors {
        match error {
            ModValidationError::DisabledDep(unique_name) => {
                info!("Enabling {}", unique_name);
                toggle_mod(&unique_name, db, true, true)?;
            }
            ModValidationError::MissingDep(unique_name) => {
                info!("Marking {} For Install", unique_name);
                missing.push(unique_name);
            }
            _ => {}
        }
    }
    if !missing.is_empty() {
        info!("Installing {} Missing Dependencies", missing.len());
    }
    install_mods_parallel(missing, config, remote_db, db).await?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::mods::local::UnsafeLocalMod;

    use super::*;

    #[test]
    fn test_check_deps_valid() {
        let mut mod_a = LocalMod::get_test(0);
        mod_a.manifest.dependencies = Some(vec!["Example.TestMod1".to_string()]);
        let mod_b = LocalMod::get_test(1);
        let mut db = LocalDatabase::default();
        db.mods.insert(
            mod_a.manifest.unique_name.to_string(),
            UnsafeLocalMod::Valid(mod_a),
        );
        db.mods.insert(
            mod_b.manifest.unique_name.to_string(),
            UnsafeLocalMod::Valid(mod_b),
        );
        let mod_a = db.get_mod("Example.TestMod0").unwrap();
        let errors = check_mod_deps(mod_a, &db);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_check_deps_missing() {
        let mut mod_a = LocalMod::get_test(0);
        mod_a.manifest.dependencies = Some(vec!["Missing.Mod".to_string()]);
        let mut db = LocalDatabase::default();
        db.mods.insert(
            mod_a.manifest.unique_name.to_string(),
            UnsafeLocalMod::Valid(mod_a),
        );
        let mod_a = db.get_mod("Example.TestMod0").unwrap();
        let errors = check_mod_deps(mod_a, &db);
        assert_eq!(errors.len(), 1);
        match errors.get(0).unwrap() {
            ModValidationError::MissingDep(unique_name) => {
                assert_eq!(unique_name, "Missing.Mod");
            }
            _ => {
                panic!("Invalid Error Variant Passed!");
            }
        }
    }

    #[test]
    fn test_check_deps_disabled() {
        let mut mod_a = LocalMod::get_test(0);
        mod_a.manifest.dependencies = Some(vec!["Example.TestMod1".to_string()]);
        let mut mod_b = LocalMod::get_test(1);
        mod_b.enabled = false;
        let mut db = LocalDatabase::default();
        db.mods.insert(
            mod_a.manifest.unique_name.to_string(),
            UnsafeLocalMod::Valid(mod_a),
        );
        db.mods.insert(
            mod_b.manifest.unique_name.to_string(),
            UnsafeLocalMod::Valid(mod_b),
        );
        let mod_a = db.get_mod("Example.TestMod0").unwrap();
        let errors = check_mod_deps(mod_a, &db);
        assert_eq!(errors.len(), 1);
        match errors.get(0).unwrap() {
            ModValidationError::DisabledDep(unique_name) => {
                assert_eq!(unique_name, "Example.TestMod1");
            }
            _ => {
                panic!("Invalid Error Variant Passed!");
            }
        }
    }

    #[test]
    fn test_check_conflicts_valid() {
        let mut mod_a = LocalMod::get_test(0);
        mod_a.manifest.conflicts = Some(vec!["Example.TestMod1".to_string()]);
        let mut db = LocalDatabase::default();
        db.mods.insert(
            mod_a.manifest.unique_name.to_string(),
            UnsafeLocalMod::Valid(mod_a),
        );
        let mod_a = db.get_mod("Example.TestMod0").unwrap();
        let errors = check_mod_conflicts(mod_a, &db);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_check_conflicts_valid_with_disabled() {
        let mut mod_a = LocalMod::get_test(0);
        mod_a.manifest.conflicts = Some(vec!["Example.TestMod1".to_string()]);
        let mut mod_b = LocalMod::get_test(1);
        mod_b.enabled = false;
        let mut db = LocalDatabase::default();
        db.mods.insert(
            mod_a.manifest.unique_name.to_string(),
            UnsafeLocalMod::Valid(mod_a),
        );
        db.mods.insert(
            mod_b.manifest.unique_name.to_string(),
            UnsafeLocalMod::Valid(mod_b),
        );
        let mod_a = db.get_mod("Example.TestMod0").unwrap();
        let errors = check_mod_conflicts(mod_a, &db);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_check_conflicts_invalid() {
        let mut mod_a = LocalMod::get_test(0);
        mod_a.manifest.conflicts = Some(vec!["Example.TestMod1".to_string()]);
        let mod_b = LocalMod::get_test(1);
        let mut db = LocalDatabase::default();
        db.mods.insert(
            mod_a.manifest.unique_name.to_string(),
            UnsafeLocalMod::Valid(mod_a),
        );
        db.mods.insert(
            mod_b.manifest.unique_name.to_string(),
            UnsafeLocalMod::Valid(mod_b),
        );
        let mod_a = db.get_mod("Example.TestMod0").unwrap();
        let errors = check_mod_conflicts(mod_a, &db);
        assert_eq!(errors.len(), 1);
        match errors.get(0).unwrap() {
            ModValidationError::ConflictingMod(unique_name) => {
                assert_eq!(unique_name, "Example.TestMod1");
            }
            _ => {
                panic!("Invalid Error Variant Passed!");
            }
        }
    }

    #[test]
    fn test_check_mod_dll_not_specified() {
        let mut mod_a = LocalMod::get_test(0);
        mod_a.manifest.filename = None;
        let error = check_mod_dll(&mod_a);
        assert!(error.is_some());
        match error.unwrap() {
            ModValidationError::MissingDLL(path) => {
                assert!(path.is_none());
            }
            _ => {
                panic!("Wrong Error Thrown!");
            }
        }
    }

    #[test]
    fn test_check_mod_dll_not_found() {
        let mut mod_a = LocalMod::get_test(0);
        mod_a.mod_path = "/not/real/".to_string();
        let error = check_mod_dll(&mod_a);
        assert!(error.is_some());
        match error.unwrap() {
            ModValidationError::MissingDLL(path) => {
                assert!(path.is_some());
                assert_eq!(path.unwrap(), "Test.dll");
            }
            _ => {
                panic!("Wrong Error Thrown!");
            }
        }
    }
}
