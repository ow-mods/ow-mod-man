use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use anyhow::Result;
use glob::glob;
use log::{debug, warn};
use serde::Deserialize;

use crate::constants::OWML_UNIQUE_NAME;
use crate::file::{deserialize_from_json, fix_json_file};
use crate::mods::{FailedMod, UnsafeLocalMod};
use crate::validate::{check_mod, ModValidationError};

use super::mods::{LocalMod, ModManifest, RemoteMod};
use super::toggle::get_mod_enabled;

fn fix_version(version: &str) -> &str {
    version.trim_start_matches('v')
}

/// Used internally to construct an actual [RemoteDatabase]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawRemoteDatabase {
    pub releases: Vec<RemoteMod>,
}

/// Represents the remote (on the website) database of mods.
#[derive(Default)]
pub struct RemoteDatabase {
    pub mods: HashMap<String, RemoteMod>,
}

/// Represents the local (on the local PC) database of mods.
#[derive(Default)]
pub struct LocalDatabase {
    pub mods: HashMap<String, UnsafeLocalMod>,
}

impl From<RawRemoteDatabase> for RemoteDatabase {
    fn from(raw: RawRemoteDatabase) -> Self {
        // Creating a hash map is O(N) but access is O(1).
        // In a cli context this doesn't rly matter since we usually only get one or two mods in the entire run of the program.
        // But I'm guessing for the GUI this will help out with performance.
        // Same thing for the local DB.
        let mut mods = raw
            .releases
            .into_iter()
            .map(|m| (m.unique_name.to_owned(), m))
            .collect::<HashMap<_, _>>();

        for remote_mod in mods.values_mut() {
            remote_mod.version = fix_version(&remote_mod.version).to_string();
        }
        Self { mods }
    }
}

impl RemoteDatabase {
    /// Fetch the database of remote mods.
    ///
    /// ## Returns
    ///
    /// An object containing a hashmap of unique names to mods.
    ///
    /// ## Errors
    ///
    /// If we can't fetch the JSON file for whatever reason.
    ///
    pub async fn fetch(url: &str) -> Result<RemoteDatabase> {
        debug!("Fetching Remote DB At {}", url);
        let resp = reqwest::get(url).await?;
        let raw_db: RawRemoteDatabase = resp.json().await?;
        debug!("Success, Constructing Mod Map");
        Ok(Self::from(raw_db))
    }

    /// Get a mod by unique name, **will not return OWML**.
    ///
    /// ## Returns
    ///
    /// A reference to the requested mod in the database, or `None` if it doesn't exist.
    ///
    pub fn get_mod(&self, unique_name: &str) -> Option<&RemoteMod> {
        if unique_name == OWML_UNIQUE_NAME {
            return None;
        }
        self.mods.get(unique_name)
    }

    /// Gets OWML from the database
    ///
    /// ## Returns
    ///
    /// A reference to OWML if it's in the database
    ///
    pub fn get_owml(&self) -> Option<&RemoteMod> {
        self.mods.get(OWML_UNIQUE_NAME)
    }
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
            UnsafeLocalMod::Valid(valid_mod) => !valid_mod.errors.is_empty(),
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

    fn get_local_mods(mods_path: &Path) -> Result<HashMap<String, UnsafeLocalMod>> {
        let mut mods: HashMap<String, UnsafeLocalMod> = HashMap::new();
        let glob_matches = glob(mods_path.join("**").join("manifest.json").to_str().unwrap())?;
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
    // TODO: Tests for invalid/duplicate mods

    use crate::{constants::DEFAULT_DB_URL, test_utils::get_test_file};

    use super::*;

    #[test]
    fn test_fix_version() {
        assert_eq!(fix_version("v0.1.0"), "0.1.0");
        assert_eq!(fix_version("vvvvv0.1.0"), "0.1.0");
        assert_eq!(fix_version("0.1.0"), "0.1.0");
        assert_eq!(fix_version("asdf"), "asdf");
    }

    #[test]
    fn test_remote_db_fetch() {
        tokio_test::block_on(async {
            let db = RemoteDatabase::fetch(DEFAULT_DB_URL).await.unwrap();
            // Yes this will make all tests depend on my mod existing, I win!
            assert!(db.get_mod("Bwc9876.TimeSaver").is_some());
        });
    }

    #[test]
    fn test_remote_db_construction() {
        let mod1 = RemoteMod::get_test(1);
        let mod2 = RemoteMod::get_test(2);
        let raw_db = RawRemoteDatabase {
            releases: vec![mod1, mod2],
        };
        let db = RemoteDatabase::from(raw_db);
        assert_eq!(db.mods.len(), 2);
        assert!(db.get_mod("Example.TestMod1").is_some());
        assert!(db.get_mod("Example.TestMod2").is_some());
    }

    #[test]
    fn test_remote_db_get_owml() {
        let mut mod1 = RemoteMod::get_test(1);
        mod1.unique_name = OWML_UNIQUE_NAME.to_string();
        let db = RemoteDatabase::from(RawRemoteDatabase {
            releases: vec![mod1],
        });
        assert!(db.get_mod(OWML_UNIQUE_NAME).is_none());
    }

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
