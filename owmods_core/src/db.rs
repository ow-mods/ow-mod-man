use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::anyhow;
use anyhow::Result;
use glob::glob;
use log::{debug, error};
use serde::Deserialize;

use crate::constants::OWML_UNIQUE_NAME;
use crate::file::{deserialize_from_json, fix_json_file};

use super::mods::{LocalMod, ModManifest, RemoteMod};
use super::toggle::get_mod_enabled;

fn fix_version(version: &str) -> &str {
    let mut str = version;
    while str.starts_with('v') {
        str = str.strip_prefix('v').unwrap_or(str);
    }
    str
}

/// Used intermittently to construct an actual `RemoteDatabase`
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
    pub mods: HashMap<String, LocalMod>,
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
        let raw = resp.text().await?;
        let raw_db: RawRemoteDatabase = serde_json::from_str(&raw)?;
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
            Self {
                mods: Self::get_local_mods(&mods_path)?,
            }
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
        self.mods.get(unique_name)
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

    /// Filters for only active mods in the DB
    pub fn active(&self) -> Vec<&LocalMod> {
        self.mods
            .values()
            .into_iter()
            .filter(|m| m.enabled)
            .collect()
    }

    fn get_local_mods(mods_path: &Path) -> Result<HashMap<String, LocalMod>> {
        let mut mods: HashMap<String, LocalMod> = HashMap::new();
        let glob_matches = glob(mods_path.join("*").join("manifest.json").to_str().unwrap())?;
        for entry in glob_matches {
            let entry = entry?;
            let local_mod = Self::read_local_mod(&entry);
            if let Ok(local_mod) = local_mod {
                mods.insert(local_mod.manifest.unique_name.to_owned(), local_mod);
            } else {
                error!(
                    "Error loading mod {}: {:?}",
                    entry.to_str().unwrap(),
                    local_mod.err().unwrap()
                );
            }
        }
        Ok(mods)
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::DEFAULT_DB_URL;

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
        assert!(db.get_mod("Example.Mod1").is_some());
        assert!(db.get_mod("Example.Mod2").is_some());
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
        let mods_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("test_files");
        let db = LocalDatabase::fetch(mods_path.to_str().unwrap()).unwrap();
        assert_eq!(db.mods.len(), 2);
        assert_eq!(
            db.get_mod("Bwc9876.TimeSaver").unwrap().manifest.name,
            "TimeSaver"
        );
    }

    #[test]
    fn test_local_db_get_owml() {
        let mods_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("test_files");
        let owml = LocalDatabase::get_owml(mods_path.to_str().unwrap());
        assert!(owml.is_some());
        assert_eq!(owml.unwrap().manifest.name, "OWML");
    }
}
