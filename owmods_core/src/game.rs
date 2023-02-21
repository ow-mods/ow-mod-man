use crate::config::Config;
use anyhow::Result;
use std::path::PathBuf;
use tokio::process::Command;

#[cfg(not(windows))]
const LINUX_GAME_PATH: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Outer Wilds";

#[cfg(windows)]
pub async fn launch_game(config: &Config, port: &u16) -> Result<()> {
    let owml_path = PathBuf::from(&config.owml_path);
    let mut child = Command::new(owml_path.join("OWML.Launcher.exe").to_str().unwrap())
        .arg("-consolePort")
        .arg(port.to_string())
        .current_dir(PathBuf::from(&owml_path))
        .spawn()?;
    child.wait().await?;
    Ok(())
}

#[cfg(windows)]
pub async fn setup_wine_prefix(config: &Config) -> Result<Config> {
    use log::error;
    error!("How in the ever-loving FUCK did you get here");
    error!("...report this please");
    Ok(config.clone()) // Never reached so idc
}

#[cfg(not(windows))]
pub async fn launch_game(config: &Config, port: &u16) -> Result<()> {
    use anyhow::anyhow;
    use std::process::Stdio;

    if let Some(wine_prefix) = &config.wine_prefix {
        let mut owml_config = OWMLConfig::get(config)?;
        owml_config.force_exe = true;
        owml_config.game_path = LINUX_GAME_PATH.to_string();
        owml_config.socket_port = *port;
        owml_config.save(config)?;

        let mut child = Command::new("wine")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("WINEPREFIX", wine_prefix)
            .arg("OWML.Launcher.exe")
            .arg("-consolePort")
            .arg(port.to_string())
            .current_dir(PathBuf::from(&config.owml_path))
            .spawn()?;

        child.wait().await?;
        // OWML really likes forward slashes
        fix_slashes_on_game(config)?;
        opener::open("steam://rungameid/753640")?;
    } else {
        return Err(anyhow!("wine_prefix not set in settings"));
    }

    Ok(())
}

#[cfg(not(windows))]
pub fn fix_slashes_on_game(config: &Config) -> Result<()> {
    let conf_path= PathBuf::from(config.wine_prefix.as_ref().unwrap()).join("drive_c/Program Files (x86)/Steam/steamapps/common/Outer Wilds/OuterWilds_Data/Managed/OWML.Config.json");
    let mut owml_config = OWMLConfig::get_from_path(&conf_path)?;
    owml_config.game_path = LINUX_GAME_PATH.to_string();
    OWMLConfig::save_to_path(&owml_config, &conf_path)
}

#[cfg(not(windows))]
pub async fn setup_wine_prefix(config: &Config) -> Result<Config> {
    use anyhow::anyhow;
    use directories::BaseDirs;
    use std::{os::unix::fs::symlink, path::Path, process::Stdio};

    use crate::{
        file::{create_all_parents, get_app_path},
        mods::OWMLConfig,
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
        .output()
        .await?;

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
        .output()
        .await?;

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
