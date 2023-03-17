use std::{
    fs::{create_dir_all, read_to_string, File},
    io::{BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

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
/// The path that resolves to about `%APPDATA%/owmods/`.
///
/// ## Errors
///
/// If we can't get the user's app data directory.
///
pub fn get_app_path() -> Result<PathBuf> {
    let app_data_path = ProjectDirs::from("com", "ow-mods", "ow-mod-man");
    match app_data_path {
        Some(app_data_path) => Ok(app_data_path.data_dir().to_path_buf()),
        None => Err(anyhow!("Can't find user's app data dir")),
    }
}

/// Fix a string of JSON by removing the BOM
pub fn fix_json(txt: &str) -> String {
    fix_bom(txt).to_string()
}

/// Removes the BOM on a JSON file
pub fn fix_json_file(path: &Path) -> Result<()> {
    let txt = read_to_string(path)?;
    let txt = fix_json(&txt);
    let mut file = File::create(path)?;
    write!(file, "{}", txt)?;
    Ok(())
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
}
