use std::{
    fs::{remove_dir_all, remove_file},
    path::PathBuf,
};

use anyhow::Result;
use glob::glob;

use crate::{
    db::LocalDatabase,
    file::check_file_matches_paths,
    mods::local::{get_paths_to_preserve, FailedMod, LocalMod},
};

/// Uninstall a mod
///
/// ## Returns
///
/// A `Vec<String>` of mods that have pre-patchers
/// and thus **should have a warning shown to the user telling them to check the mod's README for instructions**
///
/// ## Errors
///
/// If we can't delete the mod's folder.
///
pub fn remove_mod(
    local_mod: &LocalMod,
    db: &LocalDatabase,
    recursive: bool,
) -> Result<Vec<String>> {
    let mut show_warnings_for: Vec<String> = vec![];

    if PathBuf::from(&local_mod.mod_path).is_dir() {
        // In case weird circular dep stuff happens, just don't delete it if it doesn't exist
        remove_dir_all(&local_mod.mod_path)?;
        if local_mod.manifest.patcher.is_some() {
            show_warnings_for.push(local_mod.manifest.name.clone());
        }
    }

    if recursive {
        let empty: &Vec<String> = &vec![];
        let deps = local_mod.manifest.dependencies.as_ref().unwrap_or(empty);
        for dep in deps.iter() {
            let dep = db.get_mod(dep);
            if let Some(dep) = dep {
                show_warnings_for.extend(remove_mod(dep, db, true)?);
            }
        }
    }

    Ok(show_warnings_for)
}

/// Removes a [FailedMod]
///
/// ## Errors
///
/// If we can't delete the folder the mod was in.
///
pub fn remove_failed_mod(failed_mod: &FailedMod) -> Result<()> {
    remove_dir_all(PathBuf::from(&failed_mod.mod_path))?;
    Ok(())
}

/// Removes all files not specified in `pathsToPreserve`
///
/// See [get_paths_to_preserve] to see implicit paths to preserve
///
/// ## Errors
///
/// If we can't delete the files
///
pub fn remove_old_mod_files(local_mod: &LocalMod) -> Result<()> {
    let glob_matches = glob(
        PathBuf::from(&local_mod.mod_path)
            .join("**")
            .join("*")
            .to_str()
            .unwrap(),
    )?;
    let preserve_paths = get_paths_to_preserve(Some(local_mod));
    for glob_match in glob_matches {
        let path = glob_match?;
        let relative_path = path.strip_prefix(&local_mod.mod_path)?;
        if !check_file_matches_paths(relative_path, &preserve_paths) {
            if path.is_file() {
                remove_file(&path)?;
            } else if path.is_dir() {
                remove_dir_all(&path)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use std::fs;

    use crate::{file::create_all_parents, mods::local::UnsafeLocalMod, test_utils::TestContext};

    use super::*;

    #[test]
    fn test_remove_mod() {
        let mut ctx = TestContext::new();
        let new_mod = ctx.install_test_zip("Bwc9876.TimeSaver.zip", true);
        remove_mod(&new_mod, &ctx.local_db, false).unwrap();
        ctx.fetch_local_db();
        assert!(!ctx.get_test_path("Bwc9876.TimeSaver").is_dir());
        assert!(ctx.local_db.get_mod("Bwc9876.TimeSaver").is_none());
    }

    #[test]
    fn test_remove_mod_recursive() {
        let mut ctx = TestContext::new();
        let mut new_mod = ctx.install_test_zip("Bwc9876.TimeSaver.zip", false);
        ctx.install_test_zip("Bwc9876.SaveEditor.zip", true);
        new_mod.manifest.dependencies = Some(vec!["Bwc9876.SaveEditor".to_string()]);
        remove_mod(&new_mod, &ctx.local_db, true).unwrap();
        ctx.fetch_local_db();
        assert!(!ctx.get_test_path("Bwc9876.TimeSaver").is_dir());
        assert!(ctx.local_db.get_mod("Bwc9876.TimeSaver").is_none());
        assert!(!ctx.get_test_path("Bwc9876.SaveEditor").is_dir());
        assert!(ctx.local_db.get_mod("Bwc9876.SaveEditor").is_none());
    }

    #[test]
    fn test_remove_mod_recursive_cyclical_deps() {
        let mut ctx = TestContext::new();
        let mut new_mod = ctx.install_test_zip("Bwc9876.TimeSaver.zip", false);
        ctx.install_test_zip("Bwc9876.SaveEditor.zip", true);
        new_mod.manifest.dependencies = Some(vec!["Bwc9876.SaveEditor".to_string()]);
        ctx.local_db
            .mods
            .get_mut("Bwc9876.SaveEditor")
            .map(|m| match m {
                UnsafeLocalMod::Valid(m) => m,
                _ => {
                    panic!("Mod Install Failed!")
                }
            })
            .unwrap()
            .manifest
            .dependencies = Some(vec!["Bwc9876.TimeSaver".to_string()]);
        remove_mod(&new_mod, &ctx.local_db, true).unwrap();
        ctx.fetch_local_db();
        assert!(!ctx.get_test_path("Bwc9876.TimeSaver").is_dir());
        assert!(ctx.local_db.get_mod("Bwc9876.TimeSaver").is_none());
        assert!(!ctx.get_test_path("Bwc9876.SaveEditor").is_dir());
        assert!(ctx.local_db.get_mod("Bwc9876.SaveEditor").is_none());
    }

    #[test]
    fn test_remove_old_mod_files() {
        let mut ctx = TestContext::new();
        let new_mod = ctx.install_test_zip("Bwc9876.TimeSaver.zip", true);
        let target_path = ctx.get_test_path("Bwc9876.TimeSaver");
        remove_old_mod_files(&new_mod).unwrap();
        assert!(target_path.is_dir());
        assert!(!target_path.join("TimeSaver.dll").is_file());
        assert!(target_path.join("config.json").is_file());
    }

    #[test]
    fn test_remove_old_mod_files_folder() {
        let mut ctx = TestContext::new();
        let new_mod = ctx.install_test_zip("Bwc9876.TimeSaver.zip", true);
        let target_path = ctx.get_test_path("Bwc9876.TimeSaver");
        let unimportant_path = target_path
            .join("UnimportantFolder")
            .join("unimportant.json");
        create_all_parents(&unimportant_path).unwrap();
        fs::write(&unimportant_path, "{{}}").unwrap();
        remove_old_mod_files(&new_mod).unwrap();
        assert!(target_path.is_dir());
        assert!(!target_path.join("TimeSaver.dll").is_file());
        assert!(target_path.join("config.json").is_file());
    }

    #[test]
    fn test_remove_old_mod_files_paths_to_preserve() {
        let mut ctx = TestContext::new();
        let mut new_mod = ctx.install_test_zip("Bwc9876.TimeSaver.zip", true);
        let target_path = ctx.get_test_path("Bwc9876.TimeSaver");
        let important_path = target_path.join("important.json");
        new_mod.manifest.paths_to_preserve = Some(vec!["important.json".to_string()]);
        fs::write(&important_path, "{{}}").unwrap();
        remove_old_mod_files(&new_mod).unwrap();
        assert!(target_path.is_dir());
        assert!(!target_path.join("TimeSaver.dll").is_file());
        assert!(important_path.is_file());
        assert!(target_path.join("config.json").is_file());
    }

    #[test]
    fn test_remove_old_mod_files_paths_to_preserve_folder() {
        let mut ctx = TestContext::new();
        let mut new_mod = ctx.install_test_zip("Bwc9876.TimeSaver.zip", true);
        let target_path = ctx.get_test_path("Bwc9876.TimeSaver");
        let important_path = target_path.join("ImportantFolder").join("important.json");
        new_mod.manifest.paths_to_preserve = Some(vec!["ImportantFolder".to_string()]);
        create_all_parents(&important_path).unwrap();
        fs::write(&important_path, "{{}}").unwrap();
        remove_old_mod_files(&new_mod).unwrap();
        assert!(target_path.is_dir());
        assert!(!target_path.join("TimeSaver.dll").is_file());
        assert!(important_path.is_file());
        assert!(target_path.join("config.json").is_file());
    }
}
