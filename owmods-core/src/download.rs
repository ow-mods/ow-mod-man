use std::{
    fs::{create_dir, remove_dir_all, remove_file, File},
    io::{Cursor, Write},
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use async_recursion::async_recursion;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header;

use super::{
    config::{write_config, Config},
    db::{fetch_local_db, read_local_mod, LocalDatabase, RemoteDatabase},
    mods::RemoteMod,
    toggle::generate_config,
    utils::file::get_app_path,
};

async fn download_zip(
    url: &str,
    target_path: &Path,
    item_name: &String,
) -> Result<(), anyhow::Error> {
    let client = reqwest::Client::new();

    // Getting Content Length
    let file_size = {
        let resp = client.head(url).send().await?;
        if resp.status().is_success() {
            Ok(resp
                .headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|ct_len| ct_len.to_str().ok())
                .and_then(|ct_len| ct_len.parse().ok())
                .unwrap_or(0))
        } else {
            Err(anyhow!("Couldn't Fetch File, Error {}", resp.status()))
        }
    }?;

    let request = client.get(url);

    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\t [{wide_bar:.green/cyan}] {bytes}/{total_bytes}")?
            .progress_chars("#>-"),
    );
    pb.set_message(format!("Downloading {}", item_name));

    let mut stream = Cursor::new(vec![]);
    let mut download = request.send().await?;

    while let Some(chunk) = download.chunk().await? {
        pb.inc(chunk.len() as u64);
        stream.write_all(&chunk)?;
    }

    pb.finish();

    println!("Extracting...");
    zip_extract::extract(stream, target_path, true)?;
    println!("{} Installed!", item_name);

    Ok(())
}

pub fn install_from_zip(config: &Config, zip_path: &PathBuf) -> Result<(), anyhow::Error> {
    let target_name = &zip_path
        .file_name()
        .ok_or(anyhow!("Invalid zip Path"))?
        .to_str()
        .unwrap()
        .replace(".zip", "");
    let target_path = PathBuf::from(&config.owml_path)
        .join("Mods")
        .join(target_name);
    let file = File::open(zip_path)?;
    zip_extract::extract(file, &target_path, true)?;
    Ok(())
}

fn create_preserve(base_path: &Path, paths: &[String]) -> Result<(), anyhow::Error> {
    let preserve_path = base_path.join("~~preserve~~");
    if preserve_path.is_dir() {
        remove_dir_all(&preserve_path)?;
    }
    create_dir(&preserve_path)?;
    for path in paths.iter() {
        let source_path = base_path.join(path);
        if source_path.is_dir() || source_path.is_file() {
            let target_path = preserve_path.join(path);
            copy_dir::copy_dir(&source_path, target_path)?;
        }
    }
    Ok(())
}

fn restore_preserve(base_path: &Path, paths: &[String]) -> Result<(), anyhow::Error> {
    let preserve_path = base_path.join("~~preserve~~");
    if preserve_path.is_dir() {
        for path in paths.iter() {
            let source_path = preserve_path.join(path);
            if source_path.is_dir() || source_path.is_file() {
                let target_path = base_path.join(path);
                if target_path.is_dir() {
                    remove_dir_all(&target_path)?
                } else if target_path.is_file() {
                    remove_file(&target_path)?
                }
                copy_dir::copy_dir(&source_path, &target_path)?;
            }
        }
        remove_dir_all(preserve_path)?;
        Ok(())
    } else {
        Err(anyhow!(
            "Preserve Path No Longer Exists, Unable To Restore pathsToPreserve",
        ))
    }
}

pub async fn download_owml(config: &Config, owml: &RemoteMod) -> Result<(), anyhow::Error> {
    let url = &owml.download_url;
    let target_path = if config.owml_path.is_empty() {
        let app_path = get_app_path()?;
        Ok::<PathBuf, anyhow::Error>(app_path.join("OWML"))
    } else {
        Ok(PathBuf::from(&config.owml_path))
    }?;

    if target_path.exists() {
        let owml_preserve: Vec<String> = vec![String::from("OWML.Config.json")];
        create_preserve(&target_path, &owml_preserve)?;
        download_zip(url, &target_path, &String::from("OWML")).await?;
        restore_preserve(&target_path, &owml_preserve)?;
    } else {
        download_zip(url, &target_path, &String::from("OWML")).await?;
    }

    if config.owml_path.is_empty() {
        let mut new_config = config.clone();
        new_config.owml_path = String::from(target_path.to_str().unwrap());
        write_config(&new_config)?;
    }

    Ok(())
}

#[async_recursion]
pub async fn download_deps(
    config: &Config,
    local_db: &LocalDatabase,
    remote_db: &RemoteDatabase,
    remote_mod: &RemoteMod,
) -> Result<(), anyhow::Error> {
    let local_mod_path = PathBuf::from(&config.owml_path)
        .join("Mods")
        .join(&remote_mod.unique_name)
        .join("manifest.json");
    let local_mod = read_local_mod(&local_mod_path)?;
    if let Some(deps) = local_mod.manifest.dependencies {
        if !deps.is_empty() {
            println!(
                "{} Has {} Dependencies, Installing...",
                remote_mod.name,
                deps.len()
            );
            for dep in deps.iter() {
                if local_db.get_mod(dep).is_none() {
                    let dep_mod = remote_db.get_mod(dep);
                    if let Some(dep_mod) = dep_mod {
                        // Rebuild the local db so a circular dep doesn't bypass the base case
                        let local_db = fetch_local_db(config)?;
                        download_mod(config, &local_db, remote_db, dep_mod, true).await?;
                    } else {
                        println!("{} Is A Dep But It Isn't In The Database... Guh??", dep);
                    }
                } else {
                    println!("{} Is Already Installed, Skipping", dep);
                }
            }
        }
    }
    Ok(())
}

#[async_recursion]
pub async fn download_mod(
    config: &Config,
    local_db: &LocalDatabase,
    remote_db: &RemoteDatabase,
    remote_mod: &RemoteMod,
    recursive: bool,
) -> Result<(), anyhow::Error> {
    let url = &remote_mod.download_url;
    let target_path = PathBuf::from(&config.owml_path)
        .join("Mods")
        .join(&remote_mod.unique_name);
    let local_mod = local_db.get_mod(&remote_mod.unique_name);
    if let Some(local_mod) = local_mod {
        let empty: &Vec<String> = &vec![];
        let base: &Vec<String> = &vec![String::from("config.json"), String::from("save.json")];
        let paths_to_preserve = local_mod
            .manifest
            .paths_to_preserve
            .as_ref()
            .unwrap_or(empty);
        let paths_to_preserve: Vec<String> =
            [paths_to_preserve.as_slice(), base.as_slice()].concat();
        let target_path = &PathBuf::from(&local_mod.mod_path);
        create_preserve(target_path, &paths_to_preserve)?;
        download_zip(url, target_path, &remote_mod.name).await?;
        restore_preserve(target_path, &paths_to_preserve)?
    } else {
        download_zip(url, &target_path, &remote_mod.name).await?;
        generate_config(&target_path)?;
    }
    if recursive {
        download_deps(config, local_db, remote_db, remote_mod).await?;
    }
    Ok(())
}
