use std::{
    fs::{create_dir_all, read_to_string, File},
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use directories::{BaseDirs, ProjectDirs};
use serde::{Deserialize, Serialize};

use crate::constants::OLD_MANAGER_FOLDER_NAME;

/// Utility function to deserialize an object from a JSON file
///
/// ## Returns
///
/// The json object deserialized to `T`.
///
/// ## Errors
///
/// If we can't read the file or parse the json or it doesn't conform to `T`.
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::file::deserialize_from_json;
/// use owmods_core::config::Config;
/// use std::path::PathBuf;
///
/// let config: Config = deserialize_from_json(&PathBuf::from("settings.json")).unwrap();
/// println!("OWML Path: {}", config.owml_path);
/// ```
///
/// ```no_run
/// use owmods_core::file::deserialize_from_json;
/// use serde::Deserialize;
/// use std::path::PathBuf;
///
/// #[derive(Deserialize)]
/// struct TestStruct {
///    prop: bool,
/// }
///
/// let test_struct: TestStruct = deserialize_from_json(&PathBuf::from("test.json")).unwrap();
/// println!("Prop: {}", test_struct.prop);
/// ```
///
pub fn deserialize_from_json<T>(file_path: &Path) -> Result<T>
where
    for<'a> T: Deserialize<'a>,
{
    let file = File::open(file_path)?;
    let buffer = BufReader::new(file);
    let result = serde_json::from_reader(buffer)?;
    Ok(result)
}

/// Utility function to serialize an object to a JSON file.
///
/// ## Errors
///
/// If we can't open the file for writing.
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::file::serialize_to_json;
/// use owmods_core::config::Config;
/// use std::path::PathBuf;
///
/// let config = Config::default(None).unwrap();
/// serialize_to_json(&config, &PathBuf::from("settings.json"), true).unwrap();
///
/// assert!(PathBuf::from("settings.json").is_file());
/// ```
///
/// ```no_run
/// use owmods_core::file::serialize_to_json;
/// use serde::Serialize;
/// use std::path::PathBuf;
///
/// #[derive(Serialize)]
/// struct TestStruct {
///     prop: bool,
/// }
///
/// let test_struct = TestStruct { prop: true };
/// serialize_to_json(&test_struct, &PathBuf::from("test.json"), true).unwrap();
///
/// assert!(PathBuf::from("test.json").is_file());
/// ```
///
pub fn serialize_to_json<T>(obj: &T, out_path: &Path, create_parents: bool) -> Result<()>
where
    T: Serialize,
{
    if create_parents {
        if let Some(parent_path) = out_path.parent() {
            create_dir_all(parent_path)?;
        }
    }
    let file = File::create(out_path)?;
    let buffer = BufWriter::new(file);
    serde_json::to_writer_pretty(buffer, obj)?;
    Ok(())
}

/// Utility function to get the application directory in the user's files
/// You should prefer to store settings and such here to keep everything centralized.
///
/// ## Returns
///
/// The path that resolves to about `~.local/share/ow-mod-man/`.
///
/// ## Errors
///
/// If we can't get the user's app data directory.
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::file::get_app_path;
/// use std::path::PathBuf;
///
/// let app_path = get_app_path().unwrap();
/// println!("App Path: {}", app_path.to_str().unwrap());
/// ```
///
pub fn get_app_path() -> Result<PathBuf> {
    let app_data_path = ProjectDirs::from("com", "ow-mods", "ow-mod-man");
    match app_data_path {
        Some(app_data_path) => Ok(app_data_path.data_dir().to_path_buf()),
        None => Err(anyhow!("Can't find user's app data dir")),
    }
}

/// Gets the default OWML path to install to / look for
/// This is a different path than our app path to keep compatibility with mods' build files
///
/// ## Returns
///
/// The path that resolves to %APPDATA%/OuterWildsModManager/OWML
///
/// ## Errors
///
/// If we can't get the user's app data dir (or equivalent on Linux.
///
pub fn get_default_owml_path() -> Result<PathBuf> {
    let base_dirs = BaseDirs::new().ok_or_else(|| anyhow!("Couldn't Get User App Data"))?;
    let appdata_dir = base_dirs.data_dir();
    Ok(appdata_dir.join(OLD_MANAGER_FOLDER_NAME).join("OWML"))
}

/// Fix a string of JSON by removing the BOM
pub fn fix_json(txt: &str) -> String {
    fix_bom(txt).to_string()
}

/// Removes the BOM on a JSON file
pub fn fix_json_file(path: &Path) -> Result<()> {
    let txt_old = read_to_string(path)?;
    let txt = fix_json(&txt_old);
    if txt != txt_old {
        let mut file = File::create(path)?;
        write!(file, "{}", txt)?;
    }
    Ok(())
}

/// Check if a path matches any path in a series of other paths
pub fn check_file_matches_paths(path: &Path, to_check: &[PathBuf]) -> bool {
    for check in to_check.iter() {
        if check.file_name().unwrap_or(check.as_os_str())
            == path.file_name().unwrap_or(path.as_os_str())
            || path.starts_with(check)
        {
            return true;
        }
    }
    false
}

/// Recursively creates the parent directories for a file if they don't exist
pub fn create_all_parents(file_path: &Path) -> Result<()> {
    if let Some(parent_path) = file_path.parent() {
        create_dir_all(parent_path)?;
    }
    Ok(())
}

fn fix_bom(str: &str) -> &str {
    str.strip_prefix('\u{FEFF}').unwrap_or(str)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Deserialize)]
    struct TestStruct {
        prop: bool,
    }

    // Simple test rn, if some mods ever use weird json we'll need to test for and fix that
    #[test]
    fn test_fix_json() {
        let json = include_str!("../test_files/whacky_json.json");
        let json = fix_json(json);
        let obj: TestStruct = serde_json::from_str(&json).unwrap();
        assert!(obj.prop)
    }

    #[test]
    fn test_file_matches_path() {
        let test_path = Path::new("folder/some_file.json");
        let test_parent = PathBuf::from("folder");
        let unrelated_parent = PathBuf::from("other_folder");
        assert!(check_file_matches_paths(test_path, &[test_parent]));
        assert!(!check_file_matches_paths(test_path, &[unrelated_parent]),);
    }
}
