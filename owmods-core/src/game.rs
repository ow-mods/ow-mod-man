use crate::config::Config;


// For now Windows only
#[cfg(windows)]
use std::{process::Command, path::PathBuf};

#[cfg(windows)]
pub fn launch_game(config: &Config, log_port: Option<u32>) -> Result<(), anyhow::Error> {
    let mut cmd = Command::new("./OWML.Launcher.exe");
    
    if let Some(log_port) = log_port {
        cmd.arg("-consolePort").arg(log_port.to_string());
    }

    cmd.current_dir(PathBuf::from(&config.owml_path));
    
    if log_port.is_none() {
        cmd.spawn()?;
    } else {
        cmd.status()?;
    }

    Ok(())
}

#[cfg(not(windows))]
pub fn launch_game(config: &Config, _log_port: Option<u32>) -> Result<(), anyhow::Error> {
    println!("Running the game on Linux (or whatever platform you're on) is currently not supported.");
    println!("The CLI is currently pointed to {}, you could try running OWML.Launcher.exe through a wine prefix with dotnet48 installed.", config.owml_path);
    Ok(())
}
