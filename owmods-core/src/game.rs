use crate::config::Config;

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
pub fn launch_game(config: &Config) -> Result<(), anyhow::Error> {
    use anyhow::anyhow;
    use std::{io, process::Stdio, thread, time::Duration};

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
                opener::open("steam://rungameid/753640")?;
                child.wait()?;
            }
        }
    } else {
        println!(
            "Hey there! Before you can run the game, you'll need to setup or create a wine prefix."
        );
        println!("If you'd like to use an existing prefix enter one in the wine_prefix field in ~/.local/share/ow-mod-man/settings.json");
        println!(
            "Otherwise we can set one up for you. You'll need both wine and winetricks installed."
        );
        println!("Setup a wine prefix now? (y/n)");
        let mut answer = String::new();

        io::stdin().read_line(&mut answer)?;

        if answer.trim().to_ascii_lowercase() == "y" {
            setup_wine_prefix(config)?;
        } else {
            println!("Alright then! Run `owmods run` if you want to get back to this dialog");
        }
    }

    Ok(())
}

#[cfg(not(windows))]
fn setup_wine_prefix(config: &Config) -> Result<(), anyhow::Error> {
    use anyhow::anyhow;
    use directories::BaseDirs;
    use indicatif::{ProgressBar, ProgressStyle};
    use std::{os::unix::fs::symlink, path::Path, process::Stdio, time::Duration};

    use crate::{
        config::write_config,
        utils::file::{create_all_parents, get_app_path},
    };

    let prefix_path = get_app_path()?.join("wine_pfx");
    let prefix_str = prefix_path.to_str().unwrap();

    let app_data = BaseDirs::new().ok_or_else(|| anyhow!("Couldn't Get Local App Data dir"))?;
    let app_data = app_data.data_local_dir();

    let ow_rel_dir = Path::new("Steam/steamapps/common/Outer Wilds");

    let ow_dir = app_data.join(ow_rel_dir);
    let link_path = prefix_path
        .join("drive_c/Program Files (x86)")
        .join(ow_rel_dir);

    let pb = ProgressBar::new(0);
    pb.set_style(ProgressStyle::default_spinner());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Creating Wine Prefix...");

    let status = Command::new("wine")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .env("WINEPREFIX", prefix_str)
        .arg("wineboot")
        .status()?;

    if !status.success() {
        return Err(anyhow!(
            "Failed to create wine prefix with exit code: {}",
            status.code().unwrap()
        ));
    }

    pb.finish_with_message("Wine Prefix Created!");

    println!("Next we need to create a symlink for the game.");
    println!(
        "This is going to assume you've installed the game to {}.",
        ow_dir.as_os_str().to_str().unwrap()
    );
    println!("If you want to use another path, just delete the symlink and create one yourself.");

    create_all_parents(&link_path)?;
    symlink(&ow_dir, &link_path)?;

    println!("Done!\n");

    // .NET 4.8
    println!("Next we'll install .NET 4.8, this will take a while and you'll get GUI windows to go through, so be ready");
    println!("If you see a prompt to restart, select \"Restart Later\"");

    let pb = ProgressBar::new(0);
    pb.set_style(ProgressStyle::default_spinner());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Installing .NET 4.8");

    let status = Command::new("winetricks")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .env("WINEPREFIX", prefix_str)
        .arg("dotnet48")
        .status()?;

    if !status.success() {
        pb.abandon();
        return Err(anyhow!(
            "Installing .NET failed with exit code {}",
            status.code().unwrap()
        ));
    }
    pb.finish_with_message(".NET 4.8 Installed To Wine Prefix!");

    println!("All done! Run `owmods run` one more time to run the game");
    let mut new_config = config.clone();
    new_config.wine_prefix = Some(prefix_str.to_string());
    write_config(&new_config)?;
    Ok(())
}
