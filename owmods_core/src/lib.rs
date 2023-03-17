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

#[cfg(test)]
mod test_utils {
    use std::path::{Path, PathBuf};

    use tempdir::TempDir;

    pub fn make_test_dir() -> TempDir {
        TempDir::new("owmods_test").unwrap()
    }

    pub fn get_test_file(path: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("test_files/{path}"))
    }
}
