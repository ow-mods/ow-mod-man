use crate::{config::Config, logging::Logger};

use std::{path::PathBuf, process::Command};

#[cfg(windows)]
pub fn launch_game(config: &Config) -> Result<(), anyhow::Error> {
    let mut child = Command::new("./OWML.Launcher.exe")
        .current_dir(PathBuf::from(&config.owml_path))
        .spawn()?;
    child.wait()?;
    Ok(())
}

#[cfg(not(windows))]
pub fn launch_game(log: &Logger, config: &Config) -> Result<(), anyhow::Error> {
    use anyhow::anyhow;
    use std::{process::Stdio, thread, time::Duration};

    if let Some(wine_prefix) = &config.wine_prefix {
        let mut child = Command::new("wine")
            .stderr(Stdio::null())
            .env("WINEPREFIX", wine_prefix)
            .arg("OWML.Launcher.exe")
            .current_dir(PathBuf::from(&config.owml_path))
            .spawn()?;

        thread::sleep(Duration::from_secs(3));

        if let Ok(res) = child.try_wait() {
            if let Some(res) = res {
                return Err(anyhow!(
                    "Couldn't start game with exit code: {}",
                    res.code().unwrap()
                ));
            } else {
                log.debug("Actually Starting Game Now...");
                opener::open("steam://rungameid/753640")?;
                child.wait()?;
            }
        }
    } else {
        return Err(anyhow!("wine_prefix not set in settings"));
    }

    Ok(())
}

#[cfg(not(windows))]
pub fn setup_wine_prefix(log: &Logger, config: &Config) -> Result<(), anyhow::Error> {
    use anyhow::anyhow;
    use directories::BaseDirs;
    use std::{os::unix::fs::symlink, path::Path, process::Stdio};

    use crate::{
        config::write_config,
        logging::ProgressType,
        utils::file::{create_all_parents, get_app_path},
    };

    // SETUP
    let prefix_path = get_app_path()?.join("wine_pfx");
    let prefix_str = prefix_path.to_str().unwrap();

    let app_data = BaseDirs::new().ok_or_else(|| anyhow!("Couldn't Get Local App Data dir"))?;
    let app_data = app_data.data_local_dir();

    let ow_rel_dir = Path::new("Steam/steamapps/common/Outer Wilds");

    let ow_dir = app_data.join(ow_rel_dir);
    let link_path = prefix_path
        .join("drive_c/Program Files (x86)")
        .join(ow_rel_dir);

    // WINE PREFIX
    let progress = log.start_progress(ProgressType::Indefinite, "Setting Up Wine Prefix...", 0);

    let out = Command::new("wine")
        .stdout(Stdio::null()) // Wine uses stderr, so stdout not needed
        .stderr(Stdio::piped())
        .env("WINEPREFIX", prefix_str)
        .arg("wineboot")
        .output()?;

    if !out.status.success() {
        return Err(anyhow!(
            "Failed to create wine prefix with exit code {}:\n{}",
            out.status.code().unwrap(),
            String::from_utf8(out.stderr)?
        ));
    }

    progress.finish("Wine Prefix Created!");

    // SYMLINK

    let progress = log.start_progress(ProgressType::Indefinite, "Creating Symlink To OW...", 0);

    create_all_parents(&link_path)?;
    symlink(&ow_dir, &link_path)?;

    progress.finish("Symlink Created!");

    // .NET 4.8

    let progress = log.start_progress(ProgressType::Indefinite, "Installing .NET 4.8", 0);

    let out = Command::new("winetricks")
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .env("WINEPREFIX", prefix_str)
        .arg("dotnet48")
        .output()?;

    if !out.status.success() {
        return Err(anyhow!(
            "Installing .NET failed with exit code {}: {}",
            out.status.code().unwrap(),
            String::from_utf8(out.stderr)?
        ));
    }

    progress.finish(".NET 4.8 Installed!");

    let mut new_config = config.clone();
    new_config.wine_prefix = Some(prefix_str.to_string());
    write_config(log, &new_config)?;
    Ok(())
}
