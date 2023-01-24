pub mod file {

    use std::{
        fs::{create_dir_all, File},
        io::{BufReader, BufWriter, Read, Write},
        path::{Path, PathBuf},
    };

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
            None => Err(anyhow::Error::msg("Can't find user's app data dir")),
        }
    }

    pub fn fix_json(path: &Path) -> Result<(), anyhow::Error> {
        let mut file = File::open(path)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)?;

        // BOMs are really really annoying
        buffer = buffer
            .strip_prefix('\u{FEFF}')
            .unwrap_or(&buffer)
            .to_string();
        // Some mods' default-config.json do "true" instead of true for some reason, fix that.
        buffer = buffer.replace("\"true\"", "true");
        buffer = buffer.replace("\"false\"", "false");

        let mut file = File::create(path)?;
        write!(file, "{}", buffer)?;

        Ok(())
    }
}
