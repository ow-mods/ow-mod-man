use crate::config::Config;
use anyhow::Result;
use log::{debug, info};
use std::{path::PathBuf, process::Command};

#[cfg(windows)]
pub fn launch_game(config: &Config) -> Result<()> {
    let owml_path = PathBuf::from(&config.owml_path);
    let mut child = Command::new(owml_path.join("OWML.Launcher.exe").to_str().unwrap())
        .current_dir(PathBuf::from(&owml_path))
        .spawn()?;
    child.wait()?;
    info!("Quit Game");
    Ok(())
}

#[cfg(windows)]
pub fn setup_wine_prefix(log: &Logger, config: &Config) -> Result<Config> {
    use log::error;
    error!("How in the ever-loving FUCK did you get here");
    error!("...report this please");
    Ok(config.clone()) // Never reached so idc
}

#[cfg(not(windows))]
pub fn launch_game(config: &Config) -> Result<()> {
    use anyhow::anyhow;
    use std::{process::Stdio, thread, time::Duration};

    use crate::{
        file::{deserialize_from_json, serialize_to_json},
        mods::OWMLConfig,
    };

    if let Some(wine_prefix) = &config.wine_prefix {
        let owml_path = PathBuf::from(&config.owml_path);
        let mut owml_config: OWMLConfig =
            deserialize_from_json(&owml_path.join("OWML.Config.json")).unwrap_or(
                deserialize_from_json(&owml_path.join("OWML.DefaultConfig.json"))?,
            );
        owml_config.force_exe = true;
        owml_config.game_path =
            "C:/Program Files (x86)/Steam/steamapps/common/Outer Wilds".to_string();
        serialize_to_json(&owml_config, &owml_path.join("OWML.Config.json"), false)?;

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
                debug!("Actually Starting Game Now...");
                opener::open("steam://rungameid/753640")?;
                child.wait()?;
                info!("Quit Game");
            }
        }
    } else {
        return Err(anyhow!("wine_prefix not set in settings"));
    }

    Ok(())
}

#[cfg(not(windows))]
pub fn setup_wine_prefix(config: &Config) -> Result<Config> {
    use anyhow::anyhow;
    use directories::BaseDirs;
    use std::{os::unix::fs::symlink, path::Path, process::Stdio};

    use crate::{
        file::{create_all_parents, get_app_path},
        progress::{ProgressAction, ProgressBar, ProgressType},
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
    let progress = ProgressBar::new(
        "PREFIX",
        0,
        "Setting Up Wine Prefix...",
        ProgressType::Indefinite,
        ProgressAction::Wine,
    );

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
    let progress = ProgressBar::new(
        "SYMLINK",
        0,
        "Creating Symlink To OW...",
        ProgressType::Indefinite,
        ProgressAction::Wine,
    );

    create_all_parents(&link_path)?;
    // Only link Outer Wilds to minimize destruction should the user accidentally delete while following symlinks
    symlink(ow_dir, &link_path)?;

    progress.finish("Symlink Created!");

    // .NET 4.8
    let progress = ProgressBar::new(
        ".NET",
        0,
        "Installing .NET 4.8...",
        ProgressType::Indefinite,
        ProgressAction::Wine,
    );

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
    new_config.save()?;
    Ok(new_config)
}
