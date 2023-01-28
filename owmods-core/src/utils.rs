use std::path::PathBuf;

pub mod file {

    use super::fix_bom;

    use std::{
        fs::{create_dir_all, File},
        io::{BufReader, BufWriter, Read, Write},
        path::{Path, PathBuf},
    };

    use anyhow::anyhow;
    use directories::ProjectDirs;
    use serde::{Deserialize, Serialize};

    pub fn deserialize_from_json<T>(file_path: &Path) -> Result<T, anyhow::Error>
    where
        for<'a> T: Deserialize<'a>,
    {
        let file = File::open(file_path)?;
        let buffer = BufReader::new(file);
        let result = serde_json::from_reader(buffer)?;
        Ok(result)
    }

    pub fn serialize_to_json<T>(
        obj: &T,
        out_path: &Path,
        create_parents: bool,
    ) -> Result<(), anyhow::Error>
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

    pub fn get_app_path() -> Result<PathBuf, anyhow::Error> {
        let app_data_path = ProjectDirs::from("com", "ow-mods", "ow-mod-man");
        match app_data_path {
            Some(app_data_path) => Ok(app_data_path.data_dir().to_path_buf()),
            None => Err(anyhow!("Can't find user's app data dir")),
        }
    }

    pub fn fix_json(path: &Path) -> Result<(), anyhow::Error> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)?;

        // BOMs are really really annoying
        buffer = fix_bom(&mut buffer);

        let mut file = File::create(path)?;
        write!(file, "{}", buffer)?;

        Ok(())
    }

    pub fn create_all_parents(file_path: &Path) -> Result<(), anyhow::Error> {
        if let Some(parent_path) = file_path.parent() {
            create_dir_all(&parent_path)?;
        }
        Ok(())
    }
}

pub fn fix_bom(str: &mut String) -> String {
    str.strip_prefix('\u{FEFF}').unwrap_or(&str).to_string()
}

pub fn fix_version(version: &str) -> String {
    let mut str = version.to_owned();
    while str.starts_with('v') {
        str = str.strip_prefix('v').unwrap_or(&str).to_string();
    }
    str
}

pub fn get_end_of_url(url: &str) -> &str {
    url.split('/').last().unwrap_or(url)
}

pub fn check_file_matches_paths(path: &PathBuf, to_check: &Vec<PathBuf>) -> bool {
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
