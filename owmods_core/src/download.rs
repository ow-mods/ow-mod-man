use std::{
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::Result;
use anyhow::{anyhow, Context};
use futures::{stream::FuturesUnordered, StreamExt};
use log::{debug, info};
use tempfile::TempDir;
use zip::ZipArchive;

use crate::{
    analytics::{send_analytics_event, AnalyticsEventName},
    config::Config,
    constants::OWML_UNIQUE_NAME,
    db::{LocalDatabase, RemoteDatabase},
    file::{check_file_matches_paths, create_all_parents, fix_bom},
    mods::{
        local::{get_paths_to_preserve, LocalMod, ModManifest},
        remote::RemoteMod,
    },
    progress::{ProgressAction, ProgressBar, ProgressType},
    remove::remove_old_mod_files,
    toggle::generate_config,
};

fn get_end_of_url(url: &str) -> &str {
    url.split('/').last().unwrap_or(url)
}

async fn download_zip(url: &str, unique_name: Option<&str>, target_path: &Path) -> Result<()> {
    debug!(
        "Begin download of {} to {}",
        url,
        target_path.to_str().unwrap()
    );
    let client = reqwest::Client::new();
    let zip_name = get_end_of_url(url);
    let request = client.get(url);

    let mut stream = File::create(target_path)?;
    let mut download = request.send().await?.error_for_status()?;

    let file_size = download.content_length().unwrap_or(0);

    let progress_type = if file_size > 0 {
        ProgressType::Definite
    } else {
        ProgressType::Indefinite
    };

    let mut progress = ProgressBar::new(
        target_path.to_str().unwrap(),
        unique_name,
        file_size.try_into().unwrap_or(u32::MAX), // Fallback for HUGE files, means files >4GB will get progress reported incorrectly
        &format!("Downloading {}", zip_name),
        &format!("Failed to download {}", zip_name),
        progress_type,
        ProgressAction::Download,
    );

    while let Some(chunk) = download.chunk().await? {
        progress.inc(chunk.len().try_into().unwrap());
        stream.write_all(&chunk)?;
    }

    progress.finish(true, &format!("Downloaded {}", zip_name));

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
                        .context("Error reading zip file")?
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
    let manifest: ModManifest = serde_json::from_str(fix_bom(&buf))?;
    Ok(manifest.unique_name)
}

fn extract_zip(zip_path: &PathBuf, target_path: &PathBuf, display_name: &str) -> Result<()> {
    debug!(
        "Begin extraction of {} to {}",
        zip_path.to_str().unwrap(),
        target_path.to_str().unwrap()
    );
    let mut progress = ProgressBar::new(
        zip_path.to_str().unwrap(),
        None,
        0,
        &format!("Extracting {display_name}"),
        &format!("Failed To Extract {display_name}"),
        ProgressType::Indefinite,
        ProgressAction::Extract,
    );
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    archive.extract(target_path)?;
    progress.finish(true, &format!("Extracted {display_name}!"));
    Ok(())
}

fn extract_mod_zip(
    zip_path: &PathBuf,
    unique_name: Option<&str>,
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
    let mut archive = ZipArchive::new(file);

    let mut progress = ProgressBar::new(
        zip_path.to_str().unwrap(),
        unique_name,
        archive
            .as_ref()
            .map(|a| a.len().try_into().unwrap_or(0))
            .unwrap_or(0),
        &format!("Extracting {}", zip_name),
        &format!("Failed To Extract {}", zip_name),
        ProgressType::Definite,
        ProgressAction::Extract,
    );

    match &mut archive {
        Ok(archive) => {
            for idx in 0..archive.len() {
                progress.inc(1);
                let zip_file = archive.by_index(idx)?;
                if zip_file.is_file() {
                    let file_path = zip_file.enclosed_name().context("Can't Read Zip File")?;
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
            progress.finish(true, &format!("Installed {}", new_mod.manifest.name));
            Ok(new_mod)
        }
        Err(why) => {
            progress.finish(false, "");
            Err(anyhow!("Failed to extract {zip_name}: {why:?}"))
        }
    }
}

/// Downloads and installs OWML to the path specified in `config.owml_path`
///
/// ## Errors
///
/// If we can't download or extract the OWML zip for any reason.
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::config::Config;
/// use owmods_core::download::download_and_install_owml;
/// use owmods_core::db::RemoteDatabase;
///
/// # tokio_test::block_on(async {
/// let config = Config::get(None).unwrap();
/// let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
/// let owml = remote_db.get_owml().unwrap();
///
/// download_and_install_owml(&config, &owml, false).await.unwrap();
///
/// println!("Installed OWML!");
/// # });
/// ```
///
/// ```no_run
/// use owmods_core::config::Config;
/// use owmods_core::download::download_and_install_owml;
/// use owmods_core::db::RemoteDatabase;
///
/// # tokio_test::block_on(async {
/// let config = Config::get(None).unwrap();
/// let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
/// let owml = remote_db.get_owml().unwrap();
///
/// download_and_install_owml(&config, &owml, true).await.unwrap();
///
/// println!("Installed OWML Prerelease!");
/// # });
/// ```
///
pub async fn download_and_install_owml(
    config: &Config,
    owml: &RemoteMod,
    prerelease: bool,
) -> Result<()> {
    let url = if prerelease {
        owml.prerelease
            .as_ref()
            .map(|p| &p.download_url)
            .context("No prerelease for OWML found")
    } else {
        Ok(&owml.download_url)
    }?;
    let target_path = PathBuf::from(&config.owml_path);
    let temp_dir = TempDir::new()?;
    let download_path = temp_dir.path().join("OWML.zip");
    download_zip(url, Some(OWML_UNIQUE_NAME), &download_path).await?;
    extract_zip(&download_path, &target_path, "OWML")?;

    if config.owml_path.is_empty() {
        let mut new_config = config.clone();
        new_config.owml_path = String::from(target_path.to_str().unwrap());
        new_config.save()?;
    }

    temp_dir.close()?;

    send_analytics_event(
        AnalyticsEventName::ModRequiredInstall,
        OWML_UNIQUE_NAME,
        config,
    )
    .await;

    Ok(())
}

/// Install a mod from a local ZIP file
///
/// ## Returns
///
/// The newly installed [LocalMod]
///
/// ## Errors
///
/// - If we can't find a `manifest.json` file within the archive
/// - If we can't extract the zip file
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::db::LocalDatabase;
/// use owmods_core::config::Config;
/// use owmods_core::download::install_mod_from_zip;
///
/// let config = Config::get(None).unwrap();
/// let local_db = LocalDatabase::fetch(&config.owml_path).unwrap();
///
/// let new_mod = install_mod_from_zip(&"/home/user/Downloads/Mod.zip".into(), &config, &local_db).unwrap();
///
/// println!("Installed {}", new_mod.manifest.name);
/// ```
///
pub fn install_mod_from_zip(
    zip_path: &PathBuf,
    config: &Config,
    local_db: &LocalDatabase,
) -> Result<LocalMod> {
    let unique_name = get_unique_name_from_zip(zip_path);

    match unique_name {
        Ok(unique_name) => {
            let target_path = local_db
                .get_mod_unsafe(&unique_name)
                .map(|m| PathBuf::from(m.get_path().to_string()))
                .unwrap_or_else(|| {
                    PathBuf::from(&config.owml_path)
                        .join("Mods")
                        .join(&unique_name)
                });
            let local_mod = local_db.get_mod(&unique_name);

            if let Some(local_mod) = local_mod {
                remove_old_mod_files(local_mod)?;
            }

            let paths_to_preserve = get_paths_to_preserve(local_mod);

            let new_mod = extract_mod_zip(
                zip_path,
                Some(&unique_name),
                &target_path,
                paths_to_preserve,
            )?;
            let config_path = target_path.join("config.json");
            if local_mod.is_none() || !config_path.is_file() {
                // First install, generate config
                generate_config(&config_path)?;
            }
            Ok(new_mod)
        }
        Err(why) => {
            // Make a stub progress bar
            let mut progress = ProgressBar::new(
                zip_path.to_str().unwrap(),
                None,
                0,
                "",
                &format!("Failed To Extract {}", zip_path.to_str().unwrap()),
                ProgressType::Indefinite,
                ProgressAction::Extract,
            );
            // Need to wait a sec for the progress to be reported, otherwise the log messages overlap and create an unknown bar
            std::thread::sleep(Duration::from_secs(1));
            progress.finish(false, "");
            Err(anyhow!(
                "Failed To Extract {}: {why:?}",
                zip_path.to_str().unwrap()
            ))
        }
    }
}

/// Download and install a mod from a URL
///
/// ## Returns
///
/// The newly installed [LocalMod]
///
/// ## Errors
///
/// - We can't download the ZIP file
/// - We can't extract the ZIP file
/// - There is no `manifest.json` present in the archive / it's not readable
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::db::LocalDatabase;
/// use owmods_core::config::Config;
/// use owmods_core::download::install_mod_from_url;
///
/// # tokio_test::block_on(async {
/// let config = Config::get(None).unwrap();
/// let local_db = LocalDatabase::fetch(&config.owml_path).unwrap();
///
/// let new_mod = install_mod_from_url("https://example.com/Mod.zip", None, &config, &local_db).await.unwrap();
///
/// println!("Installed {}", new_mod.manifest.name);
/// # });
/// ```
///
pub async fn install_mod_from_url(
    url: &str,
    unique_name: Option<&str>,
    config: &Config,
    local_db: &LocalDatabase,
) -> Result<LocalMod> {
    let zip_name = get_end_of_url(url).replace(".zip", "");

    let temp_dir = TempDir::new()?;
    let download_path = temp_dir.path().join(format!("{}.zip", zip_name));

    download_zip(url, unique_name, &download_path).await?;
    let new_mod = install_mod_from_zip(&download_path, config, local_db)?;

    temp_dir.close()?;

    Ok(new_mod)
}

/// Install a list of mods concurrently.
/// This should be your preferred method when installing many mods.
/// **Note that this does not send an analytics event**
///
/// ## Returns
///
/// The newly installed mods
///
/// ## Errors
///
/// If **any** mod fails to install from the list
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::db::{LocalDatabase, RemoteDatabase};
/// use owmods_core::config::Config;
/// use owmods_core::download::install_mods_parallel;
/// use owmods_core::analytics::{send_analytics_event, AnalyticsEventName};
///
/// # tokio_test::block_on(async {
/// let config = Config::get(None).unwrap();
/// let local_db = LocalDatabase::fetch(&config.owml_path).unwrap();
/// let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
///
/// let installed = install_mods_parallel(vec!["Bwc9876.TimeSaver".into(), "Raicuparta.NomaiVR".into()], &config, &remote_db, &local_db).await.unwrap();
///
/// for installed_mod in installed {
///     println!("Installed {}", installed_mod.manifest.name);
///     send_analytics_event(AnalyticsEventName::ModInstall, &installed_mod.manifest.unique_name, &config).await;
/// }
/// # });
/// ```
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
            .with_context(|| format!("Mod {} not found in database.", name))?;

        let task = install_mod_from_url(
            &remote_mod.download_url,
            Some(&remote_mod.unique_name),
            config,
            local_db,
        );
        set.push(task);
    }
    while let Some(res) = set.next().await {
        let m = res?;
        installed.push(m);
    }
    Ok(installed)
}

/// Install mod from the database with the given unique name.
/// This should be the preferred method when installing a specific mod.
/// It can also install prereleases and auto-install dependencies (recursively) as well.
/// This will also send analytics events given you set `ANALYTICS_API_KEY`.
///
/// ## Errors
///
/// - If you requested a prerelease and the mod doesn't have one.
/// - If we can't install the target mod for any reason.
/// - If we can't install **any** dependencies for any reason.
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::db::{LocalDatabase, RemoteDatabase};
/// use owmods_core::config::Config;
/// use owmods_core::download::install_mod_from_db;
///
/// # tokio_test::block_on(async {
/// let config = Config::get(None).unwrap();
/// let local_db = LocalDatabase::fetch(&config.owml_path).unwrap();
/// let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
///
/// install_mod_from_db(&"Bwc9876.TimeSaver".to_string(), &config, &remote_db, &local_db, false, false).await.unwrap();
///
/// println!("Installed Bwc9876.TimeSaver!");
/// # });
/// ```
///
/// ```no_run
/// use owmods_core::db::{LocalDatabase, RemoteDatabase};
/// use owmods_core::config::Config;
/// use owmods_core::download::install_mod_from_db;
///
/// # tokio_test::block_on(async {
/// let config = Config::get(None).unwrap();
/// let local_db = LocalDatabase::fetch(&config.owml_path).unwrap();
/// let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
///
/// install_mod_from_db(&"Bwc9876.TimeSaver".to_string(), &config, &remote_db, &local_db, false, true).await.unwrap();
///
/// println!("Installed Bwc9876.TimeSaver Prerelease!");
/// # });
/// ```
///
/// ```no_run
/// use owmods_core::db::{LocalDatabase, RemoteDatabase};
/// use owmods_core::config::Config;
/// use owmods_core::download::install_mod_from_db;
///
/// # tokio_test::block_on(async {
/// let config = Config::get(None).unwrap();
/// let local_db = LocalDatabase::fetch(&config.owml_path).unwrap();
/// let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
///
/// install_mod_from_db(&"xen.NewHorizons".to_string(), &config, &remote_db, &local_db, true, false).await.unwrap();
///
/// println!("Installed xen.NewHorizons and all dependencies!");
/// # });
/// ```
///
pub async fn install_mod_from_db(
    unique_name: &String,
    config: &Config,
    remote_db: &RemoteDatabase,
    local_db: &LocalDatabase,
    recursive: bool,
    prerelease: bool,
) -> Result<LocalMod> {
    let existing_mod = local_db.get_mod(unique_name);

    let already_installed = existing_mod.is_some();
    let existing_version = existing_mod
        .as_ref()
        .map(|m| m.manifest.version.clone())
        .unwrap_or_default();

    let remote_mod = remote_db
        .get_mod(unique_name)
        .with_context(|| format!("Mod {} not found", unique_name))?;
    let target_url = if prerelease {
        let prerelease = remote_mod
            .prerelease
            .as_ref()
            .with_context(|| format!("No prerelease for {} found", unique_name))?;
        let url = &prerelease.download_url;
        info!(
            "Using Prerelease {} for {}",
            prerelease.version, remote_mod.name
        );
        url.clone()
    } else {
        remote_mod.download_url.clone()
    };
    let new_mod =
        install_mod_from_url(&target_url, Some(&remote_mod.unique_name), config, local_db).await?;

    if recursive && new_mod.manifest.dependencies.is_some() {
        let mut to_install = new_mod.manifest.dependencies.as_ref().unwrap().clone();
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

        installed.push(new_mod.manifest.unique_name.clone());

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
                    config,
                )
                .await;
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
        if existing_version == new_mod.manifest.version {
            AnalyticsEventName::ModReinstall
        } else {
            AnalyticsEventName::ModUpdate
        }
    } else {
        AnalyticsEventName::ModInstall
    };

    send_analytics_event(mod_event, unique_name, config).await;
    Ok(new_mod)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        file::serialize_to_json,
        test_utils::{get_test_file, make_test_dir, TestContext},
    };
    use std::fs::read_to_string;

    const TEST_URL: &str =
        "https://github.com/Bwc9876/OW-TimeSaver/releases/download/1.1.1/Bwc9876.TimeSaver.zip";

    #[test]
    fn test_download_zip() {
        tokio_test::block_on(async {
            let dir = make_test_dir();
            let path = dir.path().join("test.zip");
            download_zip(TEST_URL, None, &path).await.unwrap();
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
        let new_mod = extract_mod_zip(&zip_path, None, &target_path, vec![]).unwrap();
        assert!(target_path.join("manifest.json").is_file());
        assert_eq!(new_mod.mod_path, target_path.to_str().unwrap());
        assert!(!target_path.join("Folder1").is_dir());
        dir.close().unwrap();
    }

    #[test]
    fn test_extract_mod_zip_preserve() {
        let zip_path = get_test_file("Bwc9876.NestedManifest.zip");
        let mut ctx = TestContext::new();
        let target_path = ctx.get_test_path("Bwc9876.TimeSaver");
        ctx.install_test_zip("Bwc9876.TimeSaver.zip", true);
        let preserve_path = target_path.join("preserve_me.json");
        assert!(preserve_path.is_file());
        let mut file = File::create(&preserve_path).unwrap();
        write!(file, "yippee!").unwrap();
        drop(file);
        extract_mod_zip(
            &zip_path,
            None,
            &target_path,
            vec![PathBuf::from("preserve_me.json")],
        )
        .unwrap();
        assert!(preserve_path.is_file());
        let contents = read_to_string(&preserve_path).unwrap();
        assert_eq!(contents, "yippee!");
    }

    #[test]
    fn test_install_mod_from_zip() {
        let zip_path = get_test_file("Bwc9876.TimeSaver.zip");
        let ctx = TestContext::new();
        let target_path = ctx.get_test_path("Bwc9876.TimeSaver");
        let new_mod = install_mod_from_zip(&zip_path, &ctx.config, &ctx.local_db).unwrap();
        assert!(target_path.is_dir());
        assert!(target_path.join("config.json").is_file());
        assert!(target_path.join("manifest.json").is_file());
        assert_eq!(new_mod.manifest.name, "TimeSaver");
        assert_eq!(new_mod.mod_path, target_path.to_str().unwrap());
    }

    #[test]
    fn test_install_from_zip_diff_path() {
        let zip_path = get_test_file("Bwc9876.TimeSaver.zip");
        let mut ctx = TestContext::new();
        let target_path = ctx.get_test_path("Other.Path");
        extract_mod_zip(&zip_path, None, &target_path, vec![]).unwrap();
        ctx.fetch_local_db();
        let new_mod = install_mod_from_zip(&zip_path, &ctx.config, &ctx.local_db).unwrap();
        ctx.fetch_local_db();
        assert!(target_path.is_dir());
        assert!(target_path.join("manifest.json").is_file());
        assert_eq!(new_mod.manifest.name, "TimeSaver");
        assert_eq!(new_mod.mod_path, target_path.to_str().unwrap());
        assert!(!ctx.join_mods_folder("Bwc9876.TimeSaver").is_dir());
    }

    #[test]
    fn test_install_mod_from_url() {
        tokio_test::block_on(async {
            let ctx = TestContext::new();
            let new_mod = install_mod_from_url(TEST_URL, None, &ctx.config, &ctx.local_db)
                .await
                .unwrap();
            let target_path = ctx.get_test_path("Bwc9876.TimeSaver");
            assert!(target_path.is_dir());
            assert_eq!(new_mod.mod_path, target_path.to_str().unwrap());
        });
    }

    #[test]
    fn test_install_mods_parallel() {
        tokio_test::block_on(async {
            let mut ctx = TestContext::new();
            ctx.fetch_remote_db().await;
            let mods: Vec<String> = vec![
                "Bwc9876.TimeSaver".to_string(),
                "Bwc9876.SaveEditor".to_string(),
            ];
            let mods = install_mods_parallel(mods, &ctx.config, &ctx.remote_db, &ctx.local_db)
                .await
                .unwrap();
            assert_eq!(mods.len(), 2);
            assert!(ctx.get_test_path("Bwc9876.TimeSaver").is_dir());
            assert!(ctx.get_test_path("Bwc9876.SaveEditor").is_dir());
        });
    }

    #[test]
    fn test_install_mod_from_db() {
        tokio_test::block_on(async {
            let mut ctx = TestContext::new();
            ctx.fetch_remote_db().await;
            let target_path = ctx.get_test_path("Bwc9876.TimeSaver");
            install_mod_from_db(
                &"Bwc9876.TimeSaver".to_string(),
                &ctx.config,
                &ctx.remote_db,
                &ctx.local_db,
                false,
                false,
            )
            .await
            .unwrap();
            assert!(target_path.is_dir());
        });
    }

    async fn setup_recursive() -> TestContext {
        let mut ctx = TestContext::new();
        let mut new_mod = ctx.install_test_zip("Bwc9876.TimeSaver.zip", true);
        ctx.fetch_remote_db().await;
        new_mod.manifest.dependencies = Some(vec!["Bwc9876.SaveEditor".to_string()]);
        new_mod.manifest.paths_to_preserve = Some(vec!["manifest.json".to_string()]);
        let target_path = ctx.get_test_path("Bwc9876.TimeSaver");
        serialize_to_json(&new_mod.manifest, &target_path.join("manifest.json"), true).unwrap();
        ctx
    }

    #[test]
    fn test_install_mod_from_db_recursive() {
        tokio_test::block_on(async {
            let mut ctx = setup_recursive().await;
            let target_path = ctx.get_test_path("Bwc9876.TimeSaver");
            ctx.fetch_local_db();
            install_mod_from_db(
                &"Bwc9876.TimeSaver".to_string(),
                &ctx.config,
                &ctx.remote_db,
                &ctx.local_db,
                true,
                false,
            )
            .await
            .unwrap();
            assert!(target_path.is_dir());
        });
    }

    #[test]
    fn test_install_mod_from_db_cyclical_deps() {
        tokio_test::block_on(async {
            let mut ctx = setup_recursive().await;

            let mut new_mod_2 = ctx.install_test_zip("Bwc9876.SaveEditor.zip", true);
            let target_path_2 = ctx.get_test_path("Bwc9876.SaveEditor");
            new_mod_2.manifest.dependencies = Some(vec!["Bwc9876.TimeSaver".to_string()]);
            new_mod_2.manifest.paths_to_preserve = Some(vec!["manifest.json".to_string()]);
            serialize_to_json(
                &new_mod_2.manifest,
                &target_path_2.join("manifest.json"),
                true,
            )
            .unwrap();
            ctx.fetch_local_db();
            install_mod_from_db(
                &"Bwc9876.TimeSaver".to_string(),
                &ctx.config,
                &ctx.remote_db,
                &ctx.local_db,
                true,
                false,
            )
            .await
            .unwrap();
            assert!(target_path_2.is_dir());
        });
    }
}
