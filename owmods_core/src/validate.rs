use anyhow::Result;

use crate::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    download::install_mods_parallel,
    mods::LocalMod,
    toggle::toggle_mod,
};

/// Check for missing and disabled mod dependencies.
///
/// ## Returns
///
/// A tuple containing:
/// - A Vec<String> of the missing dependencies' unique names.
/// - A Vec<&LocalMod> of the disabled dependencies.
///
pub fn check_deps<'a>(
    local_mod: &'a LocalMod,
    db: &'a LocalDatabase,
) -> (Vec<&'a String>, Vec<&'a LocalMod>) {
    let mut missing: Vec<&String> = vec![];
    let mut disabled: Vec<&LocalMod> = vec![];
    if let Some(deps) = &local_mod.manifest.dependencies {
        for unique_name in deps {
            if let Some(dep_mod) = db.get_mod(unique_name) {
                if !dep_mod.enabled {
                    disabled.push(dep_mod);
                }
            } else {
                missing.push(unique_name);
            }
        }
    }
    (missing, disabled)
}

/// Auto-fix dependency issues.
/// Enables the disabled dependencies and installs missing ones.
///
/// ## Errors
///
/// If we can't install/enable the dependencies.
///
pub async fn fix_deps(
    config: &Config,
    db: &LocalDatabase,
    remote_db: &RemoteDatabase,
) -> Result<()> {
    for local_mod in db.active() {
        let (missing, disabled) = check_deps(local_mod, db);
        for disabled in disabled.iter() {
            toggle_mod(&disabled.manifest.unique_name, db, true, true)?;
        }
        install_mods_parallel(
            missing.into_iter().cloned().collect(),
            config,
            remote_db,
            db,
        )
        .await?;
    }
    Ok(())
}

/// Check for mods that conflict with eachother.
///
/// ## Returns
///
/// A Vec<String> containing all mods that are enabled and conflicting with this mod.
///
pub fn check_conflicts<'a>(local_mod: &'a LocalMod, db: &'a LocalDatabase) -> Vec<&'a String> {
    let mut conflicting: Vec<&String> = vec![];
    let active_mods: Vec<&String> = db.active().map(|m| &m.manifest.unique_name).collect();
    if let Some(conflicts) = &local_mod.manifest.conflicts {
        for conflict in conflicts.iter() {
            if active_mods.contains(&conflict) {
                conflicting.push(conflict);
            }
        }
    }
    conflicting
}

/// Check if there are any dependency or conflict errors
///
/// ## Returns
///
/// A bool signifying if there's any errors
///
pub fn has_errors(db: &LocalDatabase) -> bool {
    for local_mod in db.active() {
        let (missing, disabled) = check_deps(local_mod, db);
        let conflicts = check_conflicts(local_mod, db);
        if !missing.is_empty() || !disabled.is_empty() || !conflicts.is_empty() {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_check_deps_valid() {
        let mut mod_a = LocalMod::get_test(0);
        mod_a.manifest.dependencies = Some(vec!["Example.TestMod1".to_string()]);
        let mod_b = LocalMod::get_test(1);
        let mut db = LocalDatabase::default();
        db.mods
            .insert(mod_a.manifest.unique_name.to_string(), mod_a);
        db.mods
            .insert(mod_b.manifest.unique_name.to_string(), mod_b);
        let mod_a = db.get_mod("Example.TestMod0").unwrap();
        let (missing, disabled) = check_deps(mod_a, &db);
        assert!(missing.is_empty());
        assert!(disabled.is_empty());
    }

    #[test]
    fn test_check_deps_missing() {
        let mut mod_a = LocalMod::get_test(0);
        mod_a.manifest.dependencies = Some(vec!["Missing.Mod".to_string()]);
        let mut db = LocalDatabase::default();
        db.mods
            .insert(mod_a.manifest.unique_name.to_string(), mod_a);
        let mod_a = db.get_mod("Example.TestMod0").unwrap();
        let (missing, disabled) = check_deps(mod_a, &db);
        assert_eq!(missing.len(), 1);
        assert_eq!(*missing.get(0).unwrap(), "Missing.Mod");
        assert!(disabled.is_empty());
    }

    #[test]
    fn test_check_deps_disabled() {
        let mut mod_a = LocalMod::get_test(0);
        mod_a.manifest.dependencies = Some(vec!["Example.TestMod1".to_string()]);
        let mut mod_b = LocalMod::get_test(1);
        mod_b.enabled = false;
        let mut db = LocalDatabase::default();
        db.mods
            .insert(mod_a.manifest.unique_name.to_string(), mod_a);
        db.mods
            .insert(mod_b.manifest.unique_name.to_string(), mod_b);
        let mod_a = db.get_mod("Example.TestMod0").unwrap();
        let (missing, disabled) = check_deps(mod_a, &db);
        assert!(missing.is_empty());
        assert_eq!(disabled.len(), 1);
        assert_eq!(
            disabled.get(0).unwrap().manifest.unique_name.clone(),
            "Example.TestMod1"
        );
    }

    #[test]
    fn test_check_conflicts_valid() {
        let mut mod_a = LocalMod::get_test(0);
        mod_a.manifest.conflicts = Some(vec!["Example.TestMod1".to_string()]);
        let mut db = LocalDatabase::default();
        db.mods
            .insert(mod_a.manifest.unique_name.to_string(), mod_a);
        let mod_a = db.get_mod("Example.TestMod0").unwrap();
        let conflicts = check_conflicts(mod_a, &db);
        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_check_conflicts_valid_with_disabled() {
        let mut mod_a = LocalMod::get_test(0);
        mod_a.manifest.conflicts = Some(vec!["Example.TestMod1".to_string()]);
        let mut mod_b = LocalMod::get_test(1);
        mod_b.enabled = false;
        let mut db = LocalDatabase::default();
        db.mods
            .insert(mod_a.manifest.unique_name.to_string(), mod_a);
        db.mods
            .insert(mod_b.manifest.unique_name.to_string(), mod_b);
        let mod_a = db.get_mod("Example.TestMod0").unwrap();
        let conflicts = check_conflicts(mod_a, &db);
        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_check_conflicts_invalid() {
        let mut mod_a = LocalMod::get_test(0);
        mod_a.manifest.conflicts = Some(vec!["Example.TestMod1".to_string()]);
        let mod_b = LocalMod::get_test(1);
        let mut db = LocalDatabase::default();
        db.mods
            .insert(mod_a.manifest.unique_name.to_string(), mod_a);
        db.mods
            .insert(mod_b.manifest.unique_name.to_string(), mod_b);
        let mod_a = db.get_mod("Example.TestMod0").unwrap();
        let conflicts = check_conflicts(mod_a, &db);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(*conflicts.get(0).unwrap(), "Example.TestMod1");
    }
}
