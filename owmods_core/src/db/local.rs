use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use log::{debug, warn};

use crate::{
    file::{deserialize_from_json, fix_json_file},
    mods::local::{FailedMod, LocalMod, ModManifest, UnsafeLocalMod},
    search::search_list,
    toggle::get_mod_enabled,
    updates::check_mod_needs_update,
    validate::{check_mod, ModValidationError},
};

use super::{fix_version, RemoteDatabase};

/// Represents the local (on the local PC) database of mods.
#[derive(Default, Clone)]
pub struct LocalDatabase {
    pub mods: HashMap<String, UnsafeLocalMod>,
}

impl LocalDatabase {
    /// Construct a local database of all installed mods
    ///
    /// ## Returns
    ///
    /// An object containing a hashmap of unique names to mods. If the mods dir doesn't exist or is empty, the database will be empty too.
    ///
    /// ## Errors
    ///
    /// If we can't read the `Mods` directory in `owml_path` (NOT due to it not existing).
    ///
    pub fn fetch(owml_path: &str) -> Result<Self> {
        debug!("Begin construction of local db at {}", owml_path);
        let mods_path = PathBuf::from(owml_path).join("Mods");
        Ok(if mods_path.is_dir() {
            let mut new_db = Self {
                mods: Self::get_local_mods(&mods_path)?,
            };
            new_db.validate();
            new_db
        } else {
            Self::default()
        })
    }

    /// Get a mod from the local database
    ///
    /// ## Returns
    ///
    /// An option of the mod found, set to `None` if the mod isn't there
    ///
    pub fn get_mod(&self, unique_name: &str) -> Option<&LocalMod> {
        let local_mod = self.mods.get(unique_name);
        if let Some(UnsafeLocalMod::Valid(local_mod)) = local_mod {
            Some(local_mod)
        } else {
            None
        }
    }

    /// Get an UnsafeLocalMod from the database, this will also grab mods that failed to load
    ///
    /// ## Returns
    ///
    /// An [UnsafeLocalMod] that may or may not have loaded successfully
    ///
    pub fn get_mod_unsafe(&self, unique_name: &str) -> Option<&UnsafeLocalMod> {
        self.mods.get(unique_name)
    }

    fn get_mod_mut(&mut self, unique_name: &str) -> Option<&mut LocalMod> {
        let local_mod = self.mods.get_mut(unique_name);
        if let Some(UnsafeLocalMod::Valid(local_mod)) = local_mod {
            Some(local_mod)
        } else {
            None
        }
    }

    /// Gets OWML as a LocalMod object
    ///
    /// ## Returns
    ///
    /// OWML as a LocalMod, if it's installed
    ///
    pub fn get_owml(owml_path: &str) -> Option<LocalMod> {
        let manifest_path = PathBuf::from(owml_path).join("OWML.Manifest.json");
        fix_json_file(&manifest_path).ok();
        let mut owml_manifest: ModManifest = deserialize_from_json(&manifest_path).ok()?;
        owml_manifest.version = fix_version(&owml_manifest.version).to_string();
        Some(LocalMod {
            enabled: true,
            manifest: owml_manifest,
            mod_path: owml_path.to_string(),
            errors: vec![],
        })
    }

    /// Read a mod's manifest file and construct a LocalMod from it.
    ///
    /// ## Returns
    ///
    /// The LocalMod object that represents that mod on the disk
    ///
    /// ## Errors
    ///
    /// If we can't read the mod manifest, config, or folder.
    ///
    pub fn read_local_mod(manifest_path: &Path) -> Result<LocalMod> {
        debug!(
            "Loading Mod With Manifest: {}",
            manifest_path.to_str().unwrap()
        );
        let folder_path = manifest_path.parent();
        if folder_path.is_none() {
            return Err(anyhow!("Mod Path Not Found"));
        }
        let folder_path = folder_path.unwrap(); // <- Unwrap is safe, .is_none() check is above
        fix_json_file(manifest_path).ok();
        let mut manifest: ModManifest = deserialize_from_json(manifest_path)?;
        manifest.version = fix_version(&manifest.version).to_string();
        Ok(LocalMod {
            enabled: get_mod_enabled(folder_path)?,
            manifest,
            mod_path: String::from(folder_path.to_str().unwrap()),
            errors: vec![],
        })
    }

    /// Returns an iterator for all enabled mods
    ///
    /// ## Returns
    ///
    /// An Iterator for mods that are installed and enabled.
    ///
    pub fn active(&self) -> impl Iterator<Item = &LocalMod> {
        self.valid().filter(|m| m.enabled)
    }

    /// Returns an iterator for all installed and valid mods
    ///
    /// ## Returns
    ///
    /// An Iterator for all mods that are installed, and have a valid manifest
    ///
    pub fn valid(&self) -> impl Iterator<Item = &LocalMod> {
        self.all().filter_map(|m| match m {
            UnsafeLocalMod::Valid(m) => Some(m),
            _ => None,
        })
    }

    /// Returns an iterator of all mods with validation errors, including [FailedMod]s
    ///
    /// ## Returns
    ///
    /// An iterator containing all mods that failed to load or have validation errors
    ///
    pub fn invalid(&self) -> impl Iterator<Item = &UnsafeLocalMod> {
        self.all().filter(|m| match m {
            UnsafeLocalMod::Invalid(_) => true,
            UnsafeLocalMod::Valid(valid_mod) => valid_mod.enabled && !valid_mod.errors.is_empty(),
        })
    }

    /// Returns an iterator over all mods in the database, valid or no
    ///
    /// ## Returns
    ///
    /// An iterator over all the mods in the database, note how it's [UnsafeLocalMod] and not [LocalMod]
    ///
    pub fn all(&self) -> impl Iterator<Item = &UnsafeLocalMod> {
        self.mods.values()
    }

    /// Returns an iterator over all mods that are dependent on the given mod
    ///
    /// Please note this only checks direct dependence, it doesn't go up the dependency tree and add every parent
    ///
    /// ## Returns
    ///
    /// An iterator over all mods that are dependent on the given mod
    ///
    pub fn dependent<'a>(&'a self, local_mod: &'a LocalMod) -> impl Iterator<Item = &'a LocalMod> {
        self.valid().filter(|m| {
            m.manifest
                .dependencies
                .as_ref()
                .map_or(false, |deps| deps.contains(&local_mod.manifest.unique_name))
        })
    }

    /// Search the database with the given query, pulls from various fields of the mod
    ///
    /// ## Returns
    ///
    /// A Vec of [UnsafeLocalMod]s that exactly or closely match the search query
    ///
    pub fn search(&self, search: &str) -> Vec<&UnsafeLocalMod> {
        let mods: Vec<&UnsafeLocalMod> = self.all().collect();
        search_list(mods, search)
    }

    /// Validates deps, conflicts, etc for all mods in the DB and places errors in each mods' errors Vec
    fn validate(&mut self) {
        let names: Vec<String> = self
            .valid()
            .map(|m| m.manifest.unique_name.clone())
            .collect();
        for name in names {
            // Safe unwrap bc we're iterating over `valid`
            let local_mod = self.get_mod(&name).unwrap();
            let errors = check_mod(local_mod, self);
            self.get_mod_mut(&name).unwrap().errors = errors;
        }
    }

    /// Validates the local database against the remote, checking versions and marking mods as outdated
    pub fn validate_updates(&mut self, db: &RemoteDatabase) {
        for local_mod in self.mods.iter_mut().filter_map(|m| {
            if let UnsafeLocalMod::Valid(m) = m.1 {
                Some(m)
            } else {
                None
            }
        }) {
            let (needs_update, remote) = check_mod_needs_update(local_mod, db);
            if needs_update
                && !local_mod
                    .errors
                    .iter()
                    .any(|e| matches!(e, ModValidationError::Outdated(_)))
            {
                local_mod.errors.push(ModValidationError::Outdated(
                    remote.unwrap().version.clone(),
                ));
            }
        }
    }

    fn get_local_mods(mods_path: &Path) -> Result<HashMap<String, UnsafeLocalMod>> {
        let mut mods: HashMap<String, UnsafeLocalMod> = HashMap::new();
        let glob_matches =
            glob::glob(mods_path.join("**").join("manifest.json").to_str().unwrap())?;
        for entry in glob_matches {
            let entry = entry?;
            let parent = entry.parent().ok_or_else(|| anyhow!("Invalid Manifest!"))?;
            let path = parent.to_str().unwrap().to_string();
            let display_path = parent
                .strip_prefix(mods_path)
                .unwrap_or(parent)
                .to_str()
                .unwrap()
                .to_string();
            let local_mod = Self::read_local_mod(&entry);
            if let Ok(local_mod) = local_mod {
                if let Some(UnsafeLocalMod::Valid(other)) =
                    mods.get(&local_mod.manifest.unique_name)
                {
                    let failed_mod = FailedMod {
                        mod_path: path.to_string(),
                        display_path,
                        error: ModValidationError::DuplicateMod(other.mod_path.to_string()),
                    };
                    mods.insert(path.to_string(), UnsafeLocalMod::Invalid(failed_mod));
                } else {
                    mods.insert(
                        local_mod.manifest.unique_name.to_owned(),
                        UnsafeLocalMod::Valid(local_mod),
                    );
                }
            } else {
                let err = format!("{:?}", local_mod.err().unwrap());
                warn!("Failed to load mod at {}: {:?}", path, err);
                let failed_mod = FailedMod {
                    mod_path: path.to_string(),
                    display_path,
                    error: ModValidationError::InvalidManifest(err),
                };
                mods.insert(path.to_string(), UnsafeLocalMod::Invalid(failed_mod));
            }
        }
        Ok(mods)
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils::get_test_file;

    use super::*;

    #[test]
    fn test_local_db_fetch() {
        let mods_path = get_test_file("");
        let db = LocalDatabase::fetch(mods_path.to_str().unwrap()).unwrap();
        assert_eq!(db.valid().count(), 2);
        assert_eq!(
            db.get_mod("Bwc9876.TimeSaver").unwrap().manifest.name,
            "TimeSaver"
        );
    }

    #[test]
    fn test_local_db_get_owml() {
        let mods_path = get_test_file("");
        let owml = LocalDatabase::get_owml(mods_path.to_str().unwrap());
        assert!(owml.is_some());
        assert_eq!(owml.unwrap().manifest.name, "OWML");
    }

    #[test]
    fn test_local_db_invalid_manifest() {
        let mods_path = get_test_file("Invalid");
        let db = LocalDatabase::fetch(mods_path.to_str().unwrap()).unwrap();
        let bad_mod_path = mods_path.join("Mods").join("Invalid.Manifest");
        let bad_mod = db.get_mod_unsafe(bad_mod_path.to_str().unwrap()).unwrap();
        if let UnsafeLocalMod::Invalid(bad_mod) = bad_mod {
            assert_eq!(bad_mod.mod_path, bad_mod_path.to_str().unwrap());
            if let ModValidationError::InvalidManifest(e) = &bad_mod.error {
                assert!(e.to_ascii_lowercase().contains("string"));
            } else {
                panic!("Wrong Error on bad_mod!");
            }
        } else {
            panic!("Mod valid when it shouldn't be!");
        }
    }

    #[test]
    fn test_local_db_dupe_mods() {
        let mods_path = get_test_file("Invalid");
        let db = LocalDatabase::fetch(mods_path.to_str().unwrap()).unwrap();
        let bad_mod_path = mods_path.join("Mods").join("Dupe.Mod2");
        let other_mod_path = mods_path.join("Mods").join("Dupe.Mod1");
        let bad_mod = db.get_mod_unsafe(bad_mod_path.to_str().unwrap()).unwrap();
        if let UnsafeLocalMod::Invalid(bad_mod) = bad_mod {
            assert_eq!(bad_mod.mod_path, bad_mod_path.to_str().unwrap());
            if let ModValidationError::DuplicateMod(other) = &bad_mod.error {
                assert_eq!(other, other_mod_path.to_str().unwrap());
            } else {
                panic!("Wrong Error on bad_mod!");
            }
        } else {
            panic!("Mod valid when it shouldn't be!");
        }
    }
}
