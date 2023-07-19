use std::collections::HashMap;

use anyhow::Result;
use log::debug;
use serde::Deserialize;

use crate::{constants::OWML_UNIQUE_NAME, mods::remote::RemoteMod, search::search_list};

use super::fix_version;

/// Used internally to construct an actual [RemoteDatabase]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawRemoteDatabase {
    pub releases: Vec<RemoteMod>,
}

/// Represents the remote (on the website) database of mods.
#[derive(Default, Clone)]
pub struct RemoteDatabase {
    pub mods: HashMap<String, RemoteMod>,
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

    /// Get all the tags of all mods in the database, sorted by how often they appear
    ///
    /// ## Returns
    ///
    /// A Vec<String> of tags sorted by the amount of times they appear in the database (highest -> lowest)
    ///
    pub fn get_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = self
            .mods
            .values()
            .filter_map(|m| m.tags.clone())
            .flatten()
            .collect();

        tags.sort();

        let mut tag_counts: Vec<(String, u32)> = vec![];

        for tag in tags.into_iter() {
            let last_item = tag_counts.last_mut();
            if let Some(last_item) = last_item {
                if last_item.0 == tag {
                    last_item.1 += 1;
                } else {
                    tag_counts.push((tag, 1));
                }
            } else {
                tag_counts.push((tag, 1));
            }
        }

        tag_counts.sort_unstable_by_key(|i| i.1);

        tag_counts.dedup_by_key(|i| i.0.clone());

        tag_counts.into_iter().map(|i| i.0).rev().collect()
    }

    pub fn filter_by_tags<'a>(
        mods: impl Iterator<Item = &'a RemoteMod>,
        tags: Vec<String>,
    ) -> impl Iterator<Item = &'a RemoteMod> {
        mods.filter(move |m| {
            m.tags
                .as_ref()
                .map(|mod_tags| mod_tags.iter().any(|t| tags.contains(t)))
                .unwrap_or(false)
        })
    }

    /// Get all mods in the db that match the given list of tags
    ///
    /// * Note this performs an OR on the tags, meaning if it matches one of them it passes (reflects website logic)
    ///
    /// ## Returns
    ///
    /// An iterator over the mods that match the given list of tags
    ///
    pub fn matches_tags(&self, tags: Vec<String>) -> impl Iterator<Item = &RemoteMod> {
        Self::filter_by_tags(self.mods.values(), tags)
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
    fn test_remote_db_get_tags() {
        let mut mod1 = RemoteMod::get_test(1);
        mod1.tags = Some(vec!["story".to_string()]);
        let mut mod2 = RemoteMod::get_test(2);
        mod2.tags = Some(vec!["story".to_string(), "gameplay".to_string()]);
        let mut mod3 = RemoteMod::get_test(3);
        mod3.tags = Some(vec!["story".to_string(), "gameplay".to_string()]);
        let mut mod4 = RemoteMod::get_test(4);
        mod4.tags = Some(vec!["other".to_string()]);
        let raw_db = RawRemoteDatabase {
            releases: vec![mod1, mod2, mod3, mod4],
        };
        let db = RemoteDatabase::from(raw_db);
        let tags = db.get_tags();

        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0], "story");
        assert_eq!(tags[1], "gameplay");
        assert_eq!(tags[2], "other");
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
