use std::collections::HashMap;

use anyhow::Result;
use log::debug;
use serde::Deserialize;

use crate::{constants::OWML_UNIQUE_NAME, mods::remote::RemoteMod, search::search_list};

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

impl From<RawRemoteDatabase> for RemoteDatabase {
    fn from(raw: RawRemoteDatabase) -> Self {
        // Creating a hash map is O(N) but access is O(1).
        // In a cli context this doesn't rly matter since we usually only get one or two mods in the entire run of the program.
        // But I'm guessing for the GUI this will help out with performance.
        // Same thing for the local DB.
        let mods = raw
            .releases
            .into_iter()
            .map(|m| (m.unique_name.to_owned(), m))
            .collect::<HashMap<_, _>>();
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
        debug!("Success, Constructing Remote Mod Map");
        Ok(Self::from(raw_db))
    }

    /// Fetch the database but block the current thread while doing so
    ///
    /// ## Returns
    ///
    /// An object containing a hashmap of unique names to mods.
    ///
    /// ## Errors
    ///
    /// If we can't fetch the JSON file for whatever reason.
    ///
    pub fn fetch_blocking(url: &str) -> Result<RemoteDatabase> {
        debug!("Fetching Remote DB At {}", url);
        let resp = reqwest::blocking::get(url)?;
        let raw_db: RawRemoteDatabase = resp.json()?;
        debug!("Success, Constructing Remote Mod Map");
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

    /// Search the database with the given query, pulls from various fields of the mod
    ///
    /// ## Returns
    ///
    /// A Vec of [RemoteMod]s that exactly or closely match the search query
    ///
    pub fn search(&self, search: &str) -> Vec<&RemoteMod> {
        let mods: Vec<&RemoteMod> = self.mods.values().collect();
        search_list(mods, search)
    }
}

#[cfg(test)]
mod tests {

    use crate::constants::DEFAULT_DB_URL;

    use super::*;

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
}
