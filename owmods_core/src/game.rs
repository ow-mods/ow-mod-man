use std::{path::PathBuf, process::Stdio};

use anyhow::{anyhow, Result};
use log::warn;
use tokio::process::Command;

use crate::{config::Config, constants::OWML_EXE_NAME, owml::OWMLConfig};

// Converted to lowercase to make it easier to compare
const OUTER_WORLDS_FOLDER_NAMES: [&str; 3] =
    ["theouterworlds", "theouterworldsost", "towspacerschoice"];

const OUTER_WORLDS_TEXT: &str = "You appear to be trying to mod The Outer Worlds. This mod manager is for Outer *Wilds*.
You probably want to uninstall this manager now,
but while you're here Outer Wilds is a great game and you should get it ::)
Then you can play all of our cool mods!
To stop this dialog from displaying please edit your game path in settings to not point to an Outer Worlds install. 
You can leave it blank to have OWML auto-detect it";

/// Launch the game using the given port for logs.  
/// If no port is given, the output of OWML.Launcher.exe will be written to stdout.  
/// You can set `open_in_new_window` to `true` to make the command open in a new cmd window (**Windows Only**).  
/// On Linux there's no reliable way to open a new terminal window, so it's recommended you disallow that arg to be false on linux.  
///
/// ## Errors
///
/// If we can't launch the game/OWML, if we can't start a log server, or if we can't read the config.
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::game::launch_game;
///
/// # tokio_test::block_on(async {
/// let config = owmods_core::config::Config::get(None).unwrap();
/// launch_game(&config, true, None).await.unwrap();
/// # });
/// ```
///
/// ```no_run
/// use owmods_core::game::launch_game;
///
/// # tokio_test::block_on(async {
/// let config = owmods_core::config::Config::get(None).unwrap();
/// launch_game(&config, false, Some(&12345)).await.unwrap();
/// # });
/// ```
///
/// See LogServer for an example of how use with the log server and the game.
///
pub async fn launch_game(
    config: &Config,
    open_in_new_window: bool,
    port: Option<&u16>,
) -> Result<()> {
    if option_env!("NO_GAME").unwrap_or("FALSE") == "TRUE" {
        return Ok(());
    }

    let mut cmd = get_cmd(config, open_in_new_window)?;

    cmd.current_dir(PathBuf::from(&config.owml_path));

    if let Some(port) = port {
        cmd.arg("-consolePort")
            .arg(port.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        // Sometimes OWML.Launcher.exe doesn't like setting the socket port, just do it ourselves.
        let mut owml_config = OWMLConfig::get(config)?;
        let lower_path = owml_config.game_path.to_lowercase();
        if OUTER_WORLDS_FOLDER_NAMES
            .iter()
            .any(|name| lower_path.contains(name))
        {
            return Err(anyhow!(OUTER_WORLDS_TEXT));
        }
        owml_config.socket_port = *port;
        owml_config.save(config)?;
    }

    let child = cmd.spawn().map_err(|why| {
        if cfg!(windows) {
            anyhow!("Failed to launch game: {why:?}")
        } else {
            anyhow!("Failed to launch game: {why:?}. Is Mono Installed?")
        }
    })?;

    let res = child
        .wait_with_output()
        .await
        .map_err(|why| anyhow!("Failed to launch game: {why:?}"))?;

    if !res.status.success() {
        warn!(
            "Potentially failed to start game (exit code): {}",
            res.status
                .code()
                .map(|c| c.to_string())
                .unwrap_or("Unknown".to_string())
        );
        if let Ok(stdout) = String::from_utf8(res.stdout) {
            warn!("Potentially Failed to Start Game (stdout): {stdout}");
        }
        if let Ok(stderr) = String::from_utf8(res.stderr) {
            warn!("Potentially Failed to Start Game (stderr): {stderr}");
        }
    }

    Ok(())
}

#[cfg(windows)]
fn get_cmd(config: &Config, open_in_new_window: bool) -> Result<Command> {
    let owml_path = PathBuf::from(&config.owml_path).join(OWML_EXE_NAME);
    let exe_path = owml_path.to_str().unwrap();
    if open_in_new_window {
        let mut cmd = Command::new("cmd");
        cmd.arg("/c")
            .arg("start")
            .arg("cmd")
            .arg("/c")
            .arg(exe_path);
        Ok(cmd)
    } else {
        let cmd = Command::new(exe_path);
        Ok(cmd)
    }
}

#[cfg(unix)]
fn get_cmd(config: &Config, _: bool) -> Result<Command> {
    let owml_path = PathBuf::from(&config.owml_path).join(OWML_EXE_NAME);
    let exe_path = owml_path.to_str().unwrap();
    fix_dlls(config)?;
    let mut cmd = Command::new("mono");
    cmd.arg(exe_path);
    Ok(cmd)
}

#[cfg(unix)]
fn fix_dlls(config: &Config) -> Result<()> {
    use std::{fs::File, io::Write};

    // Replaces the DLLs that break OWML.Launcher.exe on Linux, any questions spam JohnCorby
    const SYSTEM_DLL: &[u8] = include_bytes!("../linux_replacement_dlls/System.dll");
    const SYSTEM_CORE_DLL: &[u8] = include_bytes!("../linux_replacement_dlls/System.Core.dll");
    // const OWML_MOD_LOADER_DLL: &[u8] =
    //     include_bytes!("../linux_replacement_dlls/OWML.ModLoader.dll");

    let owml_dir = PathBuf::from(&config.owml_path);
    let mut file = File::create(owml_dir.join("System.dll"))?;
    file.write_all(SYSTEM_DLL)?;
    let mut file = File::create(owml_dir.join("System.Core.dll"))?;
    file.write_all(SYSTEM_CORE_DLL)?;
    // let mut file = File::create(owml_dir.join("OWML.ModLoader.dll"))?;
    // file.write_all(OWML_MOD_LOADER_DLL)?;

    Ok(())
}
