use std::{
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use anyhow::Result;
use futures::{stream::FuturesUnordered, StreamExt};
use log::debug;
use tempdir::TempDir;
use zip::ZipArchive;

use crate::{
    analytics::{send_analytics_event, AnalyticsEventName},
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    file::{create_all_parents, fix_bom, get_app_path},
    mods::{get_paths_to_preserve, LocalMod, ModManifest, RemoteMod},
    progress::{ProgressAction, ProgressBar, ProgressType},
    toggle::generate_config,
    utils::{check_file_matches_paths, get_end_of_url},
};

async fn download_zip(url: &str, target_path: &Path) -> Result<()> {
    debug!(
        "Begin download of {} to {}",
        url,
        target_path.to_str().unwrap()
    );
    let client = reqwest::Client::new();
    let zip_name = get_end_of_url(url);
    let request = client.get(url);

    let mut stream = File::create(target_path)?;
    let mut download = request.send().await?;

    let file_size = download.content_length().unwrap_or(0);

    let progress_type = if file_size > 0 {
        ProgressType::Definite
    } else {
        ProgressType::Indefinite
    };

    let mut progress = ProgressBar::new(
        target_path.to_str().unwrap(),
        file_size,
        &format!("Downloading {}", zip_name),
        progress_type,
        ProgressAction::Download,
    );

    while let Some(chunk) = download.chunk().await? {
        progress.inc(chunk.len().try_into().unwrap());
        stream.write_all(&chunk)?;
    }

    progress.finish(&format!("Downloaded {}", zip_name));

    Ok(())
}

// Does this mean that i'll have to re-open the archive to do anything with it? Yes.
// Do I really care? No.
// You want a better one make it pls thx.
fn get_manifest_path_from_zip(zip_path: &PathBuf) -> Result<(String, PathBuf)> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    for index in 0..archive.len() {
        let zip_file = archive.by_index(index)?;
        let path = zip_file.enclosed_name();

        if let Some(path) = path {
            let name = path.file_name();
            if name == Some(OsStr::new("manifest.json")) {
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

fn extract_zip(zip_path: &PathBuf, target_path: &PathBuf) -> Result<()> {
    debug!(
        "Begin extraction of {} to {}",
        zip_path.to_str().unwrap(),
        target_path.to_str().unwrap()
    );
    let progress = ProgressBar::new(
        zip_path.to_str().unwrap(),
        0,
        "Extracting...",
        ProgressType::Indefinite,
        ProgressAction::Extract,
    );
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    archive.extract(target_path)?;
    progress.finish("Extracted!");
    Ok(())
}

fn get_unique_name_from_zip(zip_path: &PathBuf) -> Result<String> {
    let (manifest_name, _) = get_manifest_path_from_zip(zip_path)?;
    let file = File::open(zip_path)?;
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
    target_path: &Path,
    exclude_paths: Vec<PathBuf>,
) -> Result<()> {
    debug!(
        "Begin extraction of {} to {}",
        zip_path.to_str().unwrap(),
        target_path.to_str().unwrap()
    );
    let (_, manifest_path) = get_manifest_path_from_zip(zip_path)?;
    debug!(
        "Found manifest at {} in zip, extracting siblings",
        manifest_path.to_str().unwrap()
    );
    let parent_path = manifest_path.parent().unwrap_or_else(|| Path::new(""));
    let zip_name = zip_path.file_name().unwrap().to_str().unwrap();

    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    let mut progress = ProgressBar::new(
        zip_path.to_str().unwrap(),
        archive.len().try_into().unwrap(),
        &format!("Extracting {}", zip_name),
        ProgressType::Definite,
        ProgressAction::Extract,
    );

    for idx in 0..archive.len() {
        progress.inc(1);
        let zip_file = archive.by_index(idx)?;
        if zip_file.is_file() {
            let file_path = zip_file
                .enclosed_name()
                .ok_or_else(|| anyhow!("Can't Read Zip File"))?;
            if file_path.starts_with(parent_path) {
                // Unwrap is safe bc we know it's a file and OsStr.to_str shouldn't fail
                let file_name = file_path.file_name().unwrap().to_str().unwrap();
                progress.set_msg(&format!("Extracting {}", file_name));
                // Unwrap is safe bc we just checked if it starts with the parent path
                let rel_path = file_path.strip_prefix(parent_path).unwrap();
                if !check_file_matches_paths(rel_path, &exclude_paths) {
                    let output_path = target_path.join(rel_path);
                    create_all_parents(&output_path)?;
                    let out_file = File::create(&output_path)?;
                    let reader = BufReader::new(zip_file);
                    let mut writer = BufWriter::new(out_file);
                    for byte in reader.bytes() {
                        writer.write_all(&[byte?])?;
                    }
                }
            }
        }
    }

    let mod_name = LocalDatabase::read_local_mod(&target_path.join("manifest.json"))?
        .manifest
        .name;
    progress.finish(&format!("Installed {}", mod_name));

    Ok(())
}

pub async fn download_and_install_owml(config: &Config, owml: &RemoteMod) -> Result<()> {
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
        new_config.save()?;
    }

    temp_dir.close()?;

    Ok(())
}

pub fn install_mod_from_zip(
    zip_path: &PathBuf,
    config: &Config,
    local_db: &LocalDatabase,
) -> Result<LocalMod> {
    let unique_name = get_unique_name_from_zip(zip_path)?;
    let target_path = PathBuf::from(&config.owml_path)
        .join("Mods")
        .join(&unique_name);
    let local_mod = local_db.get_mod(&unique_name);

    let paths_to_preserve = get_paths_to_preserve(local_mod);

    extract_mod_zip(zip_path, &target_path, paths_to_preserve)?;
    if local_mod.is_none() {
        // First install, generate config
        generate_config(&target_path)?;
    }

    let new_local_mod = LocalDatabase::read_local_mod(&target_path.join("manifest.json"))?;
    Ok(new_local_mod)
}

pub async fn install_mod_from_url(
    url: &str,
    config: &Config,
    local_db: &LocalDatabase,
) -> Result<LocalMod> {
    let zip_name = get_end_of_url(url).replace(".zip", "");

    let temp_dir = TempDir::new("owmods")?;
    let download_path = temp_dir.path().join(format!("{}.zip", zip_name));

    download_zip(url, &download_path).await?;
    let new_mod = install_mod_from_zip(&download_path, config, local_db)?;

    temp_dir.close()?;

    Ok(new_mod)
}

pub async fn install_mods_parallel(
    unique_names: Vec<String>,
    config: &Config,
    remote_db: &RemoteDatabase,
    local_db: &LocalDatabase,
) -> Result<Vec<LocalMod>> {
    let mut set = FuturesUnordered::new();
    let mut installed: Vec<LocalMod> = vec![];
    for name in unique_names.iter() {
        let remote_mod = remote_db
            .get_mod(name)
            .ok_or_else(|| anyhow!("Mod {} not found in database.", name))?;

        let task = install_mod_from_url(&remote_mod.download_url, config, local_db);
        set.push(task);
    }
    while let Some(res) = set.next().await {
        let m = res?;
        installed.push(m);
    }
    Ok(installed)
}

pub async fn install_mod_from_db(
    unique_name: &String,
    config: &Config,
    remote_db: &RemoteDatabase,
    local_db: &LocalDatabase,
    recursive: bool,
) -> Result<()> {
    let already_installed = local_db.get_mod(unique_name).is_some();
    if recursive {
        let mut to_install: Vec<String> = vec![unique_name.clone()];
        let mut installed: Vec<String> = local_db
            .active()
            .iter()
            .filter_map(|m| {
                if m.manifest.unique_name == *unique_name {
                    None
                } else {
                    Some(m.manifest.unique_name.clone())
                }
            })
            .collect();

        let mut count = 1;

        while !to_install.is_empty() {
            debug!(
                "Begin round {} of install with {} dependencies",
                count,
                installed.len()
            );
            let newly_installed = install_mods_parallel(
                to_install
                    .drain(..)
                    .filter(|m| !installed.contains(m))
                    .collect(),
                config,
                remote_db,
                local_db,
            )
            .await?;
            for installed_mod in newly_installed
                .iter()
                .filter(|m| &m.manifest.unique_name != unique_name)
            {
                send_analytics_event(
                    AnalyticsEventName::ModRequiredInstall,
                    &installed_mod.manifest.unique_name,
                )
                .await?;
            }
            installed.append(
                &mut newly_installed
                    .iter()
                    .map(|m| m.manifest.unique_name.to_owned())
                    .collect(),
            );
            for new_mod in newly_installed.into_iter() {
                if let Some(mut deps) = new_mod.manifest.dependencies {
                    to_install.append(&mut deps);
                }
            }
            count += 1;
        }
    } else {
        let remote_mod = remote_db.get_mod(unique_name).ok_or_else(|| {
            anyhow!(
                "Mod {} not found, run `owmods list remote` to view a list.",
                unique_name
            )
        })?;
        install_mod_from_url(&remote_mod.download_url, config, local_db).await?;
    }
    if already_installed {
        send_analytics_event(AnalyticsEventName::ModReinstall, unique_name).await
    } else {
        send_analytics_event(AnalyticsEventName::ModInstall, unique_name).await
    }
}
