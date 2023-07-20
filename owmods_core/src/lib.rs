#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://github.com/Bwc9876/ow-mod-man/blob/main/.github/assets/logo-core.png?raw=true"
)]

/// Fetch database alerts and get mod warnings.
pub mod alerts;

/// Send analytics events.
pub mod analytics;

/// Work with the configuration of the app.
pub mod config;

/// Useful constants
pub mod constants;

/// Work with both remote and local databases.
pub mod db;

/// Download and install mods and OWML.
pub mod download;

/// Utilities when working with files.
pub mod file;

/// Run the game and setup prerequisites on Linux.
pub mod game;

/// Import and export mods from JSON arrays.
pub mod io;

/// Work with local and remote mods.
pub mod mods;

/// Work with the OWML config.
pub mod owml;

/// Open shortcuts and mod readmes.
pub mod open;

/// Types for consuming progress payloads.
pub mod progress;

/// Uninstall mods
pub mod remove;

/// Listen to logs from the game.
pub mod socket;

/// Enable/Disable mods.
pub mod toggle;

/// Check for and update mods.
pub mod updates;

/// Validate the local database for common issues
pub mod validate;

mod search;

#[cfg(test)]
mod test_utils {
    use std::path::{Path, PathBuf};

    use tempfile::TempDir;

    use crate::{
        config::Config,
        db::{LocalDatabase, RemoteDatabase},
        download::install_mod_from_zip,
        mods::local::{LocalMod, UnsafeLocalMod},
    };

    pub struct TestContext {
        pub temp_dir: TempDir,
        pub owml_dir: PathBuf,
        pub config: Config,
        pub local_db: LocalDatabase,
        pub remote_db: RemoteDatabase,
    }

    pub fn make_test_dir() -> TempDir {
        TempDir::new().unwrap()
    }

    pub fn get_test_file(path: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_files")
            .join(path)
    }

    impl TestContext {
        pub fn new() -> Self {
            let temp_dir = make_test_dir();
            let owml_dir = temp_dir.path().join("OWML");
            let mut config = Config::default(Some(temp_dir.path().join("settings.json"))).unwrap();
            config.owml_path = owml_dir.to_str().unwrap().to_string();
            let local_db = LocalDatabase::default();
            let remote_db = RemoteDatabase::default();
            Self {
                temp_dir,
                owml_dir,
                config,
                local_db,
                remote_db,
            }
        }

        pub fn join_mods_folder(&self, path: &str) -> PathBuf {
            self.owml_dir.join("Mods").join(path)
        }

        pub fn get_test_path(&self, unique_name: &str) -> PathBuf {
            if let Some(local_mod) = self.local_db.get_mod(unique_name) {
                PathBuf::from(&local_mod.mod_path)
            } else {
                self.owml_dir.join("Mods").join(unique_name)
            }
        }

        pub fn fetch_local_db(&mut self) {
            self.local_db = LocalDatabase::fetch(&self.config.owml_path).unwrap();
        }

        pub async fn fetch_remote_db(&mut self) {
            self.remote_db = RemoteDatabase::fetch(&self.config.database_url)
                .await
                .unwrap();
        }

        pub fn insert_test_mod(&mut self, local_mod: &LocalMod) {
            self.local_db.mods.insert(
                local_mod.manifest.unique_name.clone(),
                UnsafeLocalMod::Valid(local_mod.clone()),
            );
        }

        pub fn install_test_zip(&mut self, zip_name: &str, refresh: bool) -> LocalMod {
            let zip_path = get_test_file(zip_name);
            let local_mod = install_mod_from_zip(&zip_path, &self.config, &self.local_db).unwrap();
            if refresh {
                self.fetch_local_db();
            }
            local_mod
        }
    }
}
