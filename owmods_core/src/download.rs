use std::{
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use anyhow::Result;
use futures::{stream::FuturesUnordered, StreamExt};
use log::{debug, info};
use tempfile::TempDir;
use zip::ZipArchive;

use crate::{
    analytics::{send_analytics_event, AnalyticsEventName},
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    file::{create_all_parents, fix_json, get_app_path},
    mods::{get_paths_to_preserve, LocalMod, ModManifest, RemoteMod},
    progress::{ProgressAction, ProgressBar, ProgressType},
    toggle::generate_config,
};

fn get_end_of_url(url: &str) -> &str {
    url.split('/').last().unwrap_or(url)
}

fn check_file_matches_paths(path: &Path, to_check: &[PathBuf]) -> bool {
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

fn get_unique_name_from_zip(zip_path: &PathBuf) -> Result<String> {
    let (manifest_name, _) = get_manifest_path_from_zip(zip_path)?;
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut manifest = archive.by_name(&manifest_name)?;
    let mut buf = String::new();
    manifest.read_to_string(&mut buf)?;
    let txt = fix_json(&buf);
    let manifest: ModManifest = serde_json::from_str(&txt)?;
    Ok(manifest.unique_name)
}

fn extract_zip(zip_path: &PathBuf, target_path: &PathBuf, display_name: &str) -> Result<()> {
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
    progress.finish(&format!("Extracted {display_name}!"));
    Ok(())
}

fn extract_mod_zip(
    zip_path: &PathBuf,
    target_path: &Path,
    exclude_paths: Vec<PathBuf>,
) -> Result<LocalMod> {
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

    let new_mod = LocalDatabase::read_local_mod(&target_path.join("manifest.json"))?;
    progress.finish(&format!("Installed {}", new_mod.manifest.name));
    Ok(new_mod)
}

/// Downloads and install OWML to the path specified in config.owml_path
///
/// ## Errors
///
/// If we can't download or extract the OWML zip for any reason.
///
pub async fn download_and_install_owml(config: &Config, owml: &RemoteMod) -> Result<()> {
    let url = &owml.download_url;
    let target_path = if config.owml_path.is_empty() {
        let app_path = get_app_path()?;
        Ok::<PathBuf, anyhow::Error>(app_path.join("OWML"))
    } else {
        Ok(PathBuf::from(&config.owml_path))
    }?;

    let temp_dir = TempDir::new()?;
    let download_path = temp_dir.path().join("OWML.zip");
    download_zip(url, &download_path).await?;
    extract_zip(&download_path, &target_path, "OWML")?;

    if config.owml_path.is_empty() {
        let mut new_config = config.clone();
        new_config.owml_path = String::from(target_path.to_str().unwrap());
        new_config.save()?;
    }

    temp_dir.close()?;

    Ok(())
}

/// Install a mod from a local ZIP file
///
/// ## Returns
///
/// The newly installed LocalMod
///
/// ## Errors
///
/// - If we can't find a `manifest.json` file within the archive
/// - If we can't extract the zip file
///
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

    let new_mod = extract_mod_zip(zip_path, &target_path, paths_to_preserve)?;
    if local_mod.is_none() {
        // First install, generate config
        generate_config(&target_path.join("config.json"))?;
    }
    Ok(new_mod)
}

/// Download and install a mod from a URL
///
/// ## Returns
///
/// The newly installed local mod
///
/// ## Errors
///
/// - We can't download the ZIP file
/// - We can't extract the ZIP file
/// - There is no `manifest.json` present in the archive / it's not readable
///
pub async fn install_mod_from_url(
    url: &str,
    config: &Config,
    local_db: &LocalDatabase,
) -> Result<LocalMod> {
    let zip_name = get_end_of_url(url).replace(".zip", "");

    let temp_dir = TempDir::new()?;
    let download_path = temp_dir.path().join(format!("{}.zip", zip_name));

    download_zip(url, &download_path).await?;
    let new_mod = install_mod_from_zip(&download_path, config, local_db)?;

    temp_dir.close()?;

    Ok(new_mod)
}

/// Install a list of mods concurrently.
/// This should be your preferred method when installing many mods.
/// **Note that this does no send an analytics event**
///
/// ## Returns
///
/// The newly installed mods
///
/// ## Errors
///
/// If __any__ mod fails to install from the list
///
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

/// Install mod from the databse with the given unique name.
/// This should be the preffered method when installing a specific mod.
/// It can also install prereleases and auto-install dependencies (recursively) as well.
/// This will also send analytics events given you set `ANALYTICS_API_KEY`.
///
/// ## Errors
///
/// - If you requested a prerelease and the mod doesn't have one.
/// - If we can't install the target mod for any reason.
/// - If we can't install __any__ dependencies for any reason.
///
pub async fn install_mod_from_db(
    unique_name: &String,
    config: &Config,
    remote_db: &RemoteDatabase,
    local_db: &LocalDatabase,
    recursive: bool,
    prerelease: bool,
) -> Result<()> {
    let already_installed = local_db.get_mod(unique_name).is_some();

    let remote_mod = remote_db
        .get_mod(unique_name)
        .ok_or_else(|| anyhow!("Mod {} not found", unique_name))?;
    let target_url = if prerelease {
        let prerelease = remote_mod
            .prerelease
            .as_ref()
            .ok_or_else(|| anyhow!("No prerelease for {} found", unique_name))?;
        let url = &prerelease.download_url;
        info!(
            "Using Prerelease {} for {}",
            prerelease.version, remote_mod.name
        );
        url.clone()
    } else {
        remote_mod.download_url.clone()
    };
    let new_mod = install_mod_from_url(&target_url, config, local_db).await?;

    if recursive {
        let mut to_install: Vec<String> = new_mod.manifest.dependencies.unwrap_or_default();
        let mut installed: Vec<String> = local_db
            .valid()
            .filter_map(|m| {
                if m.manifest.unique_name == *unique_name {
                    None
                } else {
                    Some(m.manifest.unique_name.clone())
                }
            })
            .collect();

        installed.push(new_mod.manifest.unique_name);

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
    }

    let mod_event = if prerelease {
        AnalyticsEventName::ModPrereleaseInstall
    } else if already_installed {
        AnalyticsEventName::ModReinstall
    } else {
        AnalyticsEventName::ModInstall
    };

    send_analytics_event(mod_event, unique_name).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file::serialize_to_json;
    use crate::mods::UnsafeLocalMod;
    use crate::test_utils::{get_test_file, make_test_dir};
    use std::fs::read_to_string;
    const TEST_URL: &str =
        "https://github.com/Bwc9876/OW-TimeSaver/releases/download/1.1.1/Bwc9876.TimeSaver.zip";

    #[test]
    fn test_file_matches_path() {
        let test_path = Path::new("folder/some_file.json");
        let test_parent = PathBuf::from("folder");
        let unrelated_parent = PathBuf::from("other_folder");
        assert!(check_file_matches_paths(test_path, &[test_parent]));
        assert!(!check_file_matches_paths(test_path, &[unrelated_parent]),);
    }

    #[test]
    fn test_download_zip() {
        tokio_test::block_on(async {
            let dir = make_test_dir();
            let path = dir.path().join("test.zip");
            download_zip(TEST_URL, &path).await.unwrap();
            assert!(path.is_file());
            dir.close().unwrap();
        });
    }

    #[test]
    fn test_get_manifest_path() {
        let path = get_test_file("Bwc9876.NestedManifest.zip");
        let (_, manifest_path) = get_manifest_path_from_zip(&path).unwrap();
        assert_eq!(
            manifest_path,
            PathBuf::from("Bwc9876.NestedManifest/Folder1/Folder2/manifest.json")
        );
    }

    #[test]
    fn test_get_unique_name() {
        let path = get_test_file("Bwc9876.TimeSaver.zip");
        let name = get_unique_name_from_zip(&path).unwrap();
        assert_eq!(name, "Bwc9876.TimeSaver");
    }

    #[test]
    fn test_extract_zip() {
        let zip_path = get_test_file("Bwc9876.TimeSaver.zip");
        let dir = make_test_dir();
        let target_path = dir.path().join("Bwc9876.TimeSaver");
        extract_zip(&zip_path, &target_path, "Test").unwrap();
        assert!(target_path.is_dir());
        assert!(target_path.join("manifest.json").is_file());
        dir.close().unwrap();
    }

    #[test]
    fn test_extract_mod_zip_nested() {
        let zip_path = get_test_file("Bwc9876.NestedManifest.zip");
        let dir = make_test_dir();
        let target_path = dir.path().join("Bwc9876.TimeSaver");
        let new_mod = extract_mod_zip(&zip_path, &target_path, vec![]).unwrap();
        assert!(target_path.join("manifest.json").is_file());
        assert_eq!(new_mod.mod_path, target_path.to_str().unwrap());
        assert!(!target_path.join("Folder1").is_dir());
        dir.close().unwrap();
    }

    #[test]
    fn test_extract_mod_zip_preserve() {
        let zip_path = get_test_file("Bwc9876.TimeSaver.zip");
        let dir = make_test_dir();
        let target_path = dir.path().join("Bwc9876.TimeSaver");
        extract_mod_zip(&zip_path, &target_path, vec![]).unwrap();
        let preserve_path = target_path.join("preserve_me.json");
        assert!(preserve_path.is_file());
        let mut file = File::create(&preserve_path).unwrap();
        write!(file, "yippee!").unwrap();
        drop(file);
        extract_mod_zip(
            &zip_path,
            &target_path,
            vec![PathBuf::from("preserve_me.json")],
        )
        .unwrap();
        assert!(preserve_path.is_file());
        let contents = read_to_string(&preserve_path).unwrap();
        assert_eq!(contents, "yippee!");
        dir.close().unwrap();
    }

    #[test]
    fn test_install_mod_from_zip() {
        let zip_path = get_test_file("Bwc9876.TimeSaver.zip");
        let dir = make_test_dir();
        let target_path = dir.path().join("Mods").join("Bwc9876.TimeSaver");
        let mut config = Config::default(None).unwrap();
        config.owml_path = dir.path().to_str().unwrap().to_string();
        let db = LocalDatabase::default();
        let new_mod = install_mod_from_zip(&zip_path, &config, &db).unwrap();
        assert!(target_path.is_dir());
        assert!(target_path.join("config.json").is_file());
        assert!(target_path.join("manifest.json").is_file());
        assert_eq!(new_mod.manifest.name, "TimeSaver");
        assert_eq!(new_mod.mod_path, target_path.to_str().unwrap());
        dir.close().unwrap();
    }

    #[test]
    fn test_install_mod_from_url() {
        tokio_test::block_on(async {
            let dir = make_test_dir();
            let target_path = dir.path().join("Mods").join("Bwc9876.TimeSaver");
            let mut config = Config::default(None).unwrap();
            config.owml_path = dir.path().to_str().unwrap().to_string();
            let db = LocalDatabase::default();
            let new_mod = install_mod_from_url(TEST_URL, &config, &db).await.unwrap();
            assert!(target_path.is_dir());
            assert_eq!(new_mod.mod_path, target_path.to_str().unwrap());
            dir.close().unwrap();
        });
    }

    #[test]
    fn test_install_mods_parallel() {
        tokio_test::block_on(async {
            let dir = make_test_dir();
            let mut config = Config::default(None).unwrap();
            let target_path = dir.path().join("Mods");
            config.owml_path = dir.path().to_str().unwrap().to_string();
            let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
            let local_db = LocalDatabase::default();
            let mods: Vec<String> = vec![
                "Bwc9876.TimeSaver".to_string(),
                "Bwc9876.SaveEditor".to_string(),
            ];
            let mods = install_mods_parallel(mods, &config, &remote_db, &local_db)
                .await
                .unwrap();
            assert_eq!(mods.len(), 2);
            assert!(target_path.join("Bwc9876.TimeSaver").is_dir());
            assert!(target_path.join("Bwc9876.SaveEditor").is_dir());
            dir.close().unwrap();
        });
    }

    #[test]
    fn test_install_mod_from_db() {
        tokio_test::block_on(async {
            let dir = make_test_dir();
            let mut config = Config::default(None).unwrap();
            let target_path = dir.path().join("Mods").join("Bwc9876.TimeSaver");
            config.owml_path = dir.path().to_str().unwrap().to_string();
            let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
            let local_db = LocalDatabase::default();
            install_mod_from_db(
                &"Bwc9876.TimeSaver".to_string(),
                &config,
                &remote_db,
                &local_db,
                false,
                false,
            )
            .await
            .unwrap();
            assert!(target_path.is_dir());
            dir.close().unwrap();
        });
    }

    #[test]
    fn test_install_mod_from_db_recursive() {
        tokio_test::block_on(async {
            let dir = make_test_dir();
            let zip_path = get_test_file("Bwc9876.TimeSaver.zip");
            let mut config = Config::default(None).unwrap();
            let target_path = dir.path().join("Mods").join("Bwc9876.TimeSaver");
            config.owml_path = dir.path().to_str().unwrap().to_string();
            let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
            let mut local_db = LocalDatabase::default();
            let mut new_mod = install_mod_from_zip(&zip_path, &config, &local_db).unwrap();
            new_mod.manifest.dependencies = Some(vec!["Bwc9876.SaveEditor".to_string()]);
            new_mod.manifest.paths_to_preserve = Some(vec!["manifest.json".to_string()]);
            serialize_to_json(&new_mod.manifest, &target_path.join("manifest.json"), true).unwrap();
            local_db.mods.insert(
                "Bwc9876.TimeSaver".to_string(),
                UnsafeLocalMod::Valid(new_mod),
            );
            install_mod_from_db(
                &"Bwc9876.TimeSaver".to_string(),
                &config,
                &remote_db,
                &local_db,
                true,
                false,
            )
            .await
            .unwrap();
            assert!(dir.path().join("Mods").join("Bwc9876.TimeSaver").is_dir());
            dir.close().unwrap();
        });
    }

    #[test]
    fn test_install_mod_from_db_cyclical_deps() {
        tokio_test::block_on(async {
            let dir = make_test_dir();
            let zip_path = get_test_file("Bwc9876.TimeSaver.zip");
            let zip_path_2 = get_test_file("Bwc9876.SaveEditor.zip");
            let mut config = Config::default(None).unwrap();
            let target_path = dir.path().join("Mods").join("Bwc9876.TimeSaver");
            let target_path_2 = dir.path().join("Mods").join("Bwc9876.SaveEditor");
            config.owml_path = dir.path().to_str().unwrap().to_string();
            let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
            let mut local_db = LocalDatabase::default();
            let mut new_mod = install_mod_from_zip(&zip_path, &config, &local_db).unwrap();
            let mut new_mod_2 = install_mod_from_zip(&zip_path_2, &config, &local_db).unwrap();
            new_mod.manifest.dependencies = Some(vec!["Bwc9876.SaveEditor".to_string()]);
            new_mod.manifest.paths_to_preserve = Some(vec!["manifest.json".to_string()]);
            new_mod_2.manifest.dependencies = Some(vec!["Bwc9876.TimeSaver".to_string()]);
            new_mod_2.manifest.paths_to_preserve = Some(vec!["manifest.json".to_string()]);
            serialize_to_json(&new_mod.manifest, &target_path.join("manifest.json"), true).unwrap();
            serialize_to_json(
                &new_mod_2.manifest,
                &target_path_2.join("manifest.json"),
                true,
            )
            .unwrap();
            local_db.mods.insert(
                "Bwc9876.TimeSaver".to_string(),
                UnsafeLocalMod::Valid(new_mod),
            );
            local_db.mods.insert(
                "Bwc9876.SaveEditor".to_string(),
                UnsafeLocalMod::Valid(new_mod_2),
            );
            install_mod_from_db(
                &"Bwc9876.TimeSaver".to_string(),
                &config,
                &remote_db,
                &local_db,
                true,
                false,
            )
            .await
            .unwrap();
            assert!(dir.path().join("Mods").join("Bwc9876.TimeSaver").is_dir());
            assert!(dir.path().join("Mods").join("Bwc9876.SaveEditor").is_dir());
            dir.close().unwrap();
        });
    }
}
