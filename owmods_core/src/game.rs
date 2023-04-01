use crate::config::Config;
use anyhow::Result;
use std::path::PathBuf;
use tokio::process::Command;

#[cfg(not(windows))]
#[cfg(not(windows))]

/// Launch the game using the given port for logs
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

#[cfg(not(windows))]
fn fix_dlls(config: &Config) -> Result<()> {
    use std::{fs::File, io::Write};

    // Replaces the DLLs that break OWML.Launcher.exe on Linux, any questions spam JohnCorby
    const SYSTEM_DLL: &[u8] = include_bytes!("../linux_replacement_dlls/System.dll");
    const SYSTEM_CORE_DLL: &[u8] = include_bytes!("../linux_replacement_dlls/System.Core.dll");

    let owml_dir = PathBuf::from(&config.owml_path);
    let mut file = File::create(owml_dir.join("System.dll"))?;
    file.write_all(SYSTEM_DLL)?;
    let mut file = File::create(owml_dir.join("System.Core.dll"))?;
    file.write_all(SYSTEM_CORE_DLL)?;

    Ok(())
}

/// Launch the game using the given port for logs
#[cfg(not(windows))]
pub async fn launch_game(config: &Config, port: &u16) -> Result<()> {
    use crate::mods::OWMLConfig;
    use anyhow::anyhow;
    use directories::UserDirs;
    use log::{error, info};
    use std::process::Stdio;

    const LINUX_GAME_PATH: &str = ".steam/steam/steamapps/common/Outer Wilds/";

    fix_dlls(config)?;

    let mut owml_config = OWMLConfig::get(config)?;
    let dirs = UserDirs::new().ok_or_else(|| anyhow!("Can't get user data dir"))?;
    owml_config.game_path = dirs
        .home_dir()
        .join(LINUX_GAME_PATH)
        .to_str()
        .unwrap()
        .to_string();
    owml_config.socket_port = *port;
    owml_config.save(config)?;

    let child = Command::new("mono")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("OWML.Launcher.exe")
        .arg("-consolePort")
        .arg(port.to_string())
        .current_dir(PathBuf::from(&config.owml_path))
        .spawn()?;

    let res = child.wait_with_output().await;

    match res {
        Ok(res) => {
            if !res.status.success() {
                info!(
                    "{:?}\n{:?}",
                    String::from_utf8(res.stdout),
                    String::from_utf8(res.stderr)
                );
            }
        }
        Err(why) => {
            error!("Failed to launch game: {:?}", why);
        }
    }

    Ok(())
}
