use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{constants::REQUIRES_DLC_TAG, search::Searchable};

/// Represents a mod in the remote database
#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteMod {
    /// The URL to download the mod from, always GitHub
    pub download_url: String,
    /// The number of times the mod has been downloaded, this uses GitHub releases
    pub download_count: u32,
    /// The version of the mod, usually in the format `major.minor.patch`
    pub version: String,
    /// The name of the mod
    pub name: String,
    /// The unique name of the mod
    pub unique_name: String,
    /// The description of the mod
    pub description: String,
    /// The mod's README file, if it has one
    pub readme: Option<ModReadMe>,
    /// The slug of the mod, this is used for the URL on the website
    pub slug: String,
    /// Whether the mod is "required" this is an artifact of old manager as it treated OWML (and the manager itself) as a mod and required it to be installed
    required: Option<bool>,
    /// A link to the mod's repository on GitHub
    pub repo: String,
    /// The author of the mod, based on GitHub author name
    pub author: String,
    /// The display name of the author of the mod, manually set in the database
    pub author_display: Option<String>,
    /// The parent of the mod if this mod is an addon, e.g. NH
    pub parent: Option<String>,
    /// The prerelease for the mod, if it has one
    pub prerelease: Option<ModPrerelease>,
    /// The thumbnail for the mod
    pub thumbnail: ModThumbnail,
    /// Whether the mod is for the alpha version of the game, currently alpha support is not implemented
    alpha: Option<bool>,
    /// The tags for the mod, these are manually set in the database
    pub tags: Option<Vec<String>>,
}

impl RemoteMod {
    /// Get the author of a mod, first checking `author_display`, then falling back to `author`.
    pub fn get_author(&self) -> &String {
        self.author_display.as_ref().unwrap_or(&self.author)
    }

    /// Whether this mod requires the DLC to be installed
    pub fn requires_dlc(&self) -> bool {
        self.tags
            .as_ref()
            .map(|tags| tags.contains(&REQUIRES_DLC_TAG.to_string()))
            .unwrap_or(false)
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
    /// The URL to download the prerelease from, always GitHub
    pub download_url: String,
    /// The version of the prerelease, usually in the format `major.minor.patch`
    pub version: String,
}

/// Contains URLs for a mod's README
#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModReadMe {
    /// The URL to the README in HTML format
    pub html_url: String,
    /// The URL to the README for download
    pub download_url: String,
}

/// Contains URL for a mod's thumbnail
///
/// Note these paths are relative to the database website:
///
/// `https://ow-mods.github.io/ow-mod-db/thumbails/`
///
/// This should be prepended to the URL to get the full URL.
///
/// Also note that open_graph is always `None` for mods with a static thumbnail,
/// so to always get a static thumbnail use `main` and `open_graph` together:
///
/// ```
/// # use owmods_core::mods::remote::ModThumbnail;
///
/// let thumb = ModThumbnail {
///     main: Some("main.gif".to_string()),
///     open_graph: Some("open_graph.webp".to_string())
/// };
///
/// let animated = thumb.main.unwrap();
/// let static_thumb = thumb.open_graph.unwrap_or(animated.clone());
///
/// assert_eq!(animated, "main.gif");
/// assert_eq!(static_thumb, "open_graph.webp");
/// ```
///
#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModThumbnail {
    /// Main thumbnail, this will be animated if the mod has an animated thumbnail
    pub main: Option<String>,
    /// Open-graph image, this will always be a static image but will always be `None` on mods with a static thumbnail
    pub open_graph: Option<String>,
}

impl ModThumbnail {
    /// The base URL for thumbnails
    pub const BASE_URL: &'static str = "https://ow-mods.github.io/ow-mod-db/thumbnails/";

    /// Get the URL for the main thumbnail
    pub fn get_main_url(&self) -> Option<String> {
        self.main
            .as_ref()
            .map(|main| format!("{}{}", Self::BASE_URL, main))
    }

    /// Get the URL for an always static thumbnail
    pub fn get_static_url(&self) -> Option<String> {
        self.open_graph
            .as_ref()
            .or(self.main.as_ref())
            .map(|open_graph| format!("{}{}", Self::BASE_URL, open_graph))
    }
}
