use std::{
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use async_recursion::async_recursion;
use indicatif::{ProgressBar, ProgressStyle};
use tempdir::TempDir;
use zip::ZipArchive;

use crate::{
    db::{fetch_local_db, RemoteDatabase},
    mods::{get_paths_to_preserve, LocalMod, ModManifest},
    utils::{check_file_matches_paths, file::create_all_parents, fix_bom, get_end_of_url},
};

use super::{
    config::{write_config, Config},
    db::{read_local_mod, LocalDatabase},
    mods::RemoteMod,
    toggle::generate_config,
    utils::file::get_app_path,
};

const PROGRESS_TEMPLATE: &str = "{percent}% {wide_msg} [{bar:100.green/cyan}]";
const PROGRESS_CHARS: &str = "=>-";

async fn download_zip(url: &str, target_path: &Path) -> Result<(), anyhow::Error> {
    let client = reqwest::Client::new();

    let zip_name = get_end_of_url(&url);

    let request = client.get(url);

    let mut stream = File::create(&target_path)?;
    let mut download = request.send().await?;

    let file_size = download.content_length().unwrap_or(0);

    let pb = ProgressBar::new(file_size);
    pb.set_style(if file_size != 0 {
        ProgressStyle::default_bar()
            .template(PROGRESS_TEMPLATE)?
            .progress_chars(PROGRESS_CHARS)
    } else {
        // In the event we can't get content size, just use a spinner
        ProgressStyle::default_bar().template("{spinner} {msg} (Unknown Size)")?
    });
    pb.set_message(format!("Downloading {}", zip_name));

    while let Some(chunk) = download.chunk().await? {
        pb.inc(chunk.len() as u64);
        stream.write_all(&chunk)?;
    }

    pb.finish_with_message(format!("Downloaded {}", zip_name));
    pb.finish();

    Ok(())
}

// Does this mean that i'll have to re-open the archive to do anything with it? Yes.
// Do I really care? No.
// You want a better one make it pls thx.
fn get_manifest_path_from_zip(zip_path: &PathBuf) -> Result<(String, PathBuf), anyhow::Error> {
    let file = File::open(&zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for index in 0..archive.len() {
        let zip_file = archive.by_index(index)?;
        let path = zip_file.enclosed_name();

        if let Some(path) = path {
            let name = path.file_name();
            if name == Some(&OsStr::new("manifest.json")) {
                return Ok((
                    zip_file.name().to_string(),
                    zip_file
                        .enclosed_name()
                        .ok_or_else(|| anyhow!("Error reading zip file"))?
                        .to_path_buf(),
                ));
            }
        }
    }
    Err(anyhow!("Manifest not found in zip archive"))
}

fn extract_zip(zip_path: &PathBuf, target_path: &PathBuf) -> Result<(), anyhow::Error> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    archive.extract(&target_path)?;
    Ok(())
}

fn get_unique_name_from_zip(zip_path: &PathBuf) -> Result<String, anyhow::Error> {
    let (manifest_name, _) = get_manifest_path_from_zip(&zip_path)?;
    let file = File::open(&zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut manifest = archive.by_name(&manifest_name)?;
    let mut buf = String::new();
    manifest.read_to_string(&mut buf)?;
    buf = fix_bom(&mut buf);
    let manifest: ModManifest = serde_json::from_str(&buf)?;
    Ok(manifest.unique_name)
}

fn extract_mod_zip(
    zip_path: &PathBuf,
    target_path: &PathBuf,
    exclude_paths: Vec<PathBuf>,
) -> Result<(), anyhow::Error> {
    let (_, manifest_path) = get_manifest_path_from_zip(&zip_path)?;
    let parent_path = manifest_path.parent().unwrap_or(&Path::new(""));
    let zip_name = zip_path.file_name().unwrap().to_str().unwrap();

    let file = File::open(&zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    let pb = ProgressBar::new(archive.len().try_into().unwrap());
    pb.set_style(
        ProgressStyle::default_bar()
            .template(PROGRESS_TEMPLATE)?
            .progress_chars(PROGRESS_CHARS),
    );
    pb.set_message(format!("Extracting {}", zip_name));

    for idx in 0..archive.len() {
        pb.inc(1);
        let zip_file = archive.by_index(idx)?;
        if zip_file.is_file() {
            let file_path = zip_file
                .enclosed_name()
                .ok_or_else(|| anyhow!("Can't Read Zip File"))?;
            if file_path.starts_with(&parent_path) {
                // Unwrap is safe bc we know it's a file and OsStr.to_str shouldn't fail
                let file_name = file_path.file_name().unwrap().to_str().unwrap();
                pb.set_message(format!("Extracting {}", file_name));
                // Unwrap is safe bc we just checked if it starts with the parent path
                let rel_path = file_path.strip_prefix(&parent_path).unwrap();
                if !check_file_matches_paths(&rel_path.to_path_buf(), &exclude_paths) {
                    let output_path = target_path.join(rel_path);
                    create_all_parents(&output_path)?;
                    let out_file = File::create(&output_path)?;
                    let reader = BufReader::new(zip_file);
                    let mut writer = BufWriter::new(out_file);
                    for byte in reader.bytes() {
                        writer.write(&[byte?])?;
                    }
                }
            }
        }
    }

    pb.finish_with_message(format!("Extracted {}", zip_name));

    Ok(())
}

pub async fn download_and_install_owml(
    config: &Config,
    owml: &RemoteMod,
) -> Result<(), anyhow::Error> {
    let url = &owml.download_url;
    let target_path = if config.owml_path.is_empty() {
        let app_path = get_app_path()?;
        Ok::<PathBuf, anyhow::Error>(app_path.join("OWML"))
    } else {
        Ok(PathBuf::from(&config.owml_path))
    }?;

    let temp_dir = TempDir::new("owmods")?;
    let download_path = temp_dir.path().join("OWML.zip");
    download_zip(url, &download_path).await?;
    extract_zip(&download_path, &target_path)?;

    if config.owml_path.is_empty() {
        let mut new_config = config.clone();
        new_config.owml_path = String::from(target_path.to_str().unwrap());
        write_config(&new_config)?;
    }

    temp_dir.close()?;

    Ok(())
}

pub fn install_mod_from_zip(
    zip_path: &PathBuf,
    config: &Config,
    local_db: &LocalDatabase,
) -> Result<LocalMod, anyhow::Error> {
    let unique_name = get_unique_name_from_zip(zip_path)?;
    let target_path = PathBuf::from(&config.owml_path)
        .join("Mods")
        .join(&unique_name);
    let local_mod = local_db.get_mod(&unique_name);

    let paths_to_preserve = get_paths_to_preserve(local_mod);

    extract_mod_zip(&zip_path, &target_path, paths_to_preserve)?;
    if local_mod.is_none() {
        // First install, generate config
        generate_config(&target_path)?;
    }

    let new_local_mod = read_local_mod(&target_path.join("manifest.json"))?;
    Ok(new_local_mod)
}

pub async fn install_mod_from_url(
    url: &str,
    config: &Config,
    local_db: &LocalDatabase,
) -> Result<LocalMod, anyhow::Error> {
    let zip_name = get_end_of_url(url).replace(".zip", "");

    let temp_dir = TempDir::new("owmods")?;
    let download_path = temp_dir.path().join(format!("{}.zip", zip_name));

    download_zip(&url, &download_path).await?;
    let new_mod = install_mod_from_zip(&download_path, &config, &local_db)?;

    Ok(new_mod)
}

#[async_recursion]
pub async fn install_mod_from_db(
    unique_name: &String,
    config: &Config,
    remote_db: &RemoteDatabase,
    local_db: &LocalDatabase,
    recursive: bool,
) -> Result<(), anyhow::Error> {
    let remote_mod = remote_db.get_mod(unique_name).ok_or_else(|| {
        anyhow!(
            "Mod {} not found, run `owmods list remote` to view a list.",
            unique_name
        )
    })?;

    let new_mod = install_mod_from_url(&remote_mod.download_url, &config, &local_db).await?;

    // Not the **best** way to do recursive installs.
    // This should only re-build the local database when the mod has deps though,
    // and nested deps aren't really done too much fo performance overhead should be negligible
    if recursive {
        if let Some(deps) = &new_mod.manifest.dependencies {
            let local_db = fetch_local_db(&config)?;
            let local_mods: Vec<String> = local_db
                .mods
                .iter()
                .map(|m| m.manifest.unique_name.clone())
                .collect();
            for dep in deps.iter() {
                if !local_mods.contains(dep) {
                    install_mod_from_db(&dep, &config, &remote_db, &local_db, true).await?;
                }
            }
        }
    }

    Ok(())
}
