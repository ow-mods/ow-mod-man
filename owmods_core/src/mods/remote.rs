use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::search::Searchable;

/// Represents a mod in the remote database
#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteMod {
    pub download_url: String,
    pub download_count: u32,
    pub version: String,
    pub name: String,
    pub unique_name: String,
    pub description: String,
    pub readme: Option<ModReadMe>,
    pub slug: String,
    required: Option<bool>,
    pub repo: String,
    pub author: String,
    pub author_display: Option<String>,
    pub parent: Option<String>,
    pub prerelease: Option<ModPrerelease>,
    alpha: Option<bool>,
    pub tags: Option<Vec<String>>,
}

impl RemoteMod {
    /// Get the author of a mod, first checking `author_display`, then falling back to `author`.
    pub fn get_author(&self) -> &String {
        self.author_display.as_ref().unwrap_or(&self.author)
    }

    #[cfg(test)]
    pub fn get_test(num: u8) -> Self {
        serde_json::from_str(
            &include_str!("../../test_files/test_remote_mod.json")
                .replace("$num$", &num.to_string()),
        )
        .unwrap()
    }
}

impl Searchable for RemoteMod {
    fn get_values(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.unique_name.clone(),
            self.get_author().clone(),
            self.description.clone(),
        ]
    }
}

/// A prerelease for a mod
#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModPrerelease {
    pub download_url: String,
    pub version: String,
}

/// Contains URLs for a mod's README
#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModReadMe {
    pub html_url: String,
    pub download_url: String,
}
