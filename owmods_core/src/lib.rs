pub mod alerts;
pub mod analytics;
pub mod config;
pub mod constants;
pub mod db;
pub mod download;
pub mod file;
pub mod game;
pub mod io;
pub mod mods;
pub mod open;
pub mod progress;
pub mod remove;
pub mod socket;
pub mod toggle;
pub mod updates;
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
