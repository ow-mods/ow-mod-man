use std::process;

use anyhow::{anyhow, Result};
use clap::{CommandFactory, Parser};
use colored::Colorize;
use log::{error, info, warn, LevelFilter};
use owmods_core::{
    alerts::fetch_alert,
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    download::{
        download_and_install_owml, install_mod_from_db, install_mod_from_url, install_mod_from_zip,
    },
    file::get_default_owml_path,
    io::{export_mods, import_mods},
    mods::local::{LocalMod, UnsafeLocalMod},
    open::{open_readme, open_shortcut},
    remove::{remove_failed_mod, remove_mod},
    toggle::toggle_mod,
    updates::update_all,
    validate::fix_deps,
};

mod cli;
mod game;
mod logging;

use cli::{BaseCli, Commands, ModListTypes};
use game::{start_game, start_just_logs};
use logging::{log_mod_validation_errors, Logger};

async fn run_from_cli(cli: BaseCli) -> Result<()> {
    let r = cli.recursive;

    let config = Config::get(None)?;

    let ran_setup = matches!(&cli.command, Commands::Setup { owml_path: _ });

    if !config.check_owml() && !ran_setup {
        info!(
            "Welcome to the Outer Wild Mods CLI! In order to continue you'll need to setup OWML.",
        );
        info!("To do this, run `owmods setup /path/to/owml`. Or, run with no path to auto-install it to {}.", config.owml_path);
        info!("This message will display until a valid OWML path is set or OWML is installed");
        return Ok(());
    }

    match &cli.command {
        Commands::Version => {
            info!(env!("CARGO_PKG_VERSION"));
        }
        Commands::Setup { owml_path } => {
            if let Some(owml_path) = owml_path {
                let mut new_config = config.clone();
                new_config.owml_path = owml_path.to_str().unwrap().to_string();
                if new_config.check_owml() {
                    info!("Path to OWML is valid! Updating config...");
                    new_config.save()?;
                    info!("Done! Happy Modding!");
                } else {
                    error!(
                        "Error: OWML.Manifest.json, OWML.Launcher.exe, or OWML.DefaultConfig.json Not Found In {}",
                        owml_path.to_str().unwrap()
                    );
                }
            } else {
                let mut config = config.clone();
                config.owml_path = get_default_owml_path()?.to_str().unwrap().to_string();
                let db = RemoteDatabase::fetch(&config.database_url).await?;
                let owml = db
                    .get_owml()
                    .ok_or_else(|| anyhow!("OWML not found, is the database URL correct?"))?;
                download_and_install_owml(&config, owml).await?;
                info!("Done! Happy Modding!");
            }
        }
        Commands::Alert => {
            let alert = fetch_alert(&config.alert_url).await?;
            if alert.enabled {
                info!(
                    "[{}] {}",
                    alert
                        .severity
                        .unwrap_or_else(|| "info".to_string())
                        .to_ascii_uppercase(),
                    alert.message.unwrap_or_else(|| "No message".to_string())
                );
            } else {
                info!("No alert");
            };
        }
        Commands::List { mod_type } => match mod_type {
            Some(ModListTypes::Local) | None => {
                let db = LocalDatabase::fetch(&config.owml_path)?;
                let mut output = String::new();
                output += &format!(
                    "Found {} Installed Mods at {}:\n(+): Enabled\n(-): Disabled\n\n",
                    db.mods.len(),
                    config.owml_path
                );
                let mut mods: Vec<&LocalMod> = db.valid().collect();
                mods.sort_by(|a, b| b.enabled.cmp(&a.enabled));
                for local_mod in mods.iter() {
                    output += &format!(
                        "({}) {} v{} by {} ({})\n",
                        if local_mod.enabled { "+" } else { "-" },
                        local_mod.manifest.name,
                        local_mod.manifest.version,
                        local_mod.manifest.author,
                        &local_mod.manifest.unique_name.to_string().bold()
                    );
                }
                info!("{}", &output);
            }
            Some(ModListTypes::Remote) => {
                let db = RemoteDatabase::fetch(&config.database_url).await?;
                let mut output = String::new();
                output += &format!("Found {} Remote Mods:\n", db.mods.values().len());
                for remote_mod in db.mods.values() {
                    output += &format!(
                        "- {} by {} ({})\n",
                        remote_mod.name,
                        remote_mod
                            .author_display
                            .as_ref()
                            .unwrap_or(&remote_mod.author),
                        &remote_mod.unique_name.to_string().bold()
                    )
                }
                info!("{}", &output);
            }
        },
        Commands::Search { query } => {
            let db = RemoteDatabase::fetch(&config.database_url).await?;
            let mods = db.search(query);
            for remote_mod in mods {
                info!(
                    "{} v{} by {} ({})",
                    remote_mod.name,
                    remote_mod.version,
                    remote_mod.get_author(),
                    remote_mod.unique_name
                );
            }
        }
        Commands::Info { unique_name } => {
            let remote_db = RemoteDatabase::fetch(&config.database_url).await?;
            let local_db = LocalDatabase::fetch(&config.owml_path)?;
            let local_mod = local_db.get_mod(unique_name);
            let remote_mod = remote_db.get_mod(unique_name);
            let installed = local_mod.is_some();
            let has_remote = remote_mod.is_some();
            if (!installed) && (!has_remote) {
                info!("Mod not found in local or remote db: {}", unique_name);
            } else {
                let name = if installed {
                    &local_mod.unwrap().manifest.name
                } else {
                    &remote_mod.unwrap().name
                };
                let author = if installed {
                    &local_mod.unwrap().manifest.author
                } else {
                    remote_mod.unwrap().get_author()
                };
                info!("========== {} ==========", unique_name);
                info!("Name: {}", name);
                info!("Author(s): {}", author);
                info!("Installed: {}", yes_no(installed));
                if installed {
                    let local_mod = local_mod.unwrap();
                    info!("Installed At: {}", local_mod.mod_path);
                    info!("Enabled: {}", yes_no(local_mod.enabled));
                    info!("Installed Version: {}", local_mod.manifest.version);
                    if let Some(owml_version) = &local_mod.manifest.owml_version {
                        info!("Expected OWML Version: {}", owml_version);
                    }
                    if let Some(deps) = &local_mod.manifest.dependencies {
                        info!("Dependencies: {}", deps.join(", "));
                    }
                    if let Some(conflicts) = &local_mod.manifest.conflicts {
                        info!("Conflicts: {}", conflicts.join(", "));
                    }
                }
                info!("In Database: {}", yes_no(has_remote));
                if has_remote {
                    let remote_mod = remote_mod.unwrap();
                    info!("Description: {}", remote_mod.description);
                    info!("GitHub Repo URL: {}", remote_mod.repo);
                    info!("Downloads: {}", remote_mod.download_count);
                    if let Some(parent) = &remote_mod.parent {
                        info!("Parent Mod: {}", parent);
                    }
                    if let Some(tags) = &remote_mod.tags {
                        info!("Tags: {}", tags.join(", "));
                    }
                    info!("Remote Version: {}", remote_mod.version);
                    if let Some(prerelease) = &remote_mod.prerelease {
                        info!(
                            "Prerelease Version: {} ({})",
                            prerelease.version, prerelease.download_url
                        );
                    }
                }
            }
        }
        Commands::Install {
            unique_name,
            overwrite,
            prerelease,
        } => {
            let remote_db = RemoteDatabase::fetch(&config.database_url).await?;
            let local_db = LocalDatabase::fetch(&config.owml_path)?;
            let local_mod = local_db.get_mod(unique_name);
            let mut flag = true;

            if *overwrite && local_mod.is_some() {
                warn!("Overriding {}", unique_name);
            } else if let Some(local_mod) = local_db.get_mod(unique_name) {
                error!(
                    "{} is already installed at {}, use -o to overwrite",
                    unique_name, local_mod.mod_path
                );
                flag = false;
            }

            if flag {
                install_mod_from_db(unique_name, &config, &remote_db, &local_db, r, *prerelease)
                    .await?
            }
        }
        Commands::InstallZip { zip_path } => {
            info!("Installing From {}", zip_path.to_str().unwrap());
            let local_db = LocalDatabase::fetch(&config.owml_path)?;
            let new_mod = install_mod_from_zip(zip_path, &config, &local_db)?;
            info!("Installed {}!", new_mod.manifest.name);
        }
        Commands::InstallUrl { url } => {
            let local_db = LocalDatabase::fetch(&config.owml_path)?;
            info!("Installing From {}", url);
            let new_mod = install_mod_from_url(url, &config, &local_db).await?;
            info!("Installed {}!", new_mod.manifest.name);
        }
        Commands::Uninstall { unique_name } => {
            let db = LocalDatabase::fetch(&config.owml_path)?;
            if unique_name == "all" {
                let mut answer = String::new();
                warn!("WARNING: This will uninstall ALL MODS. Continue? (yes/no)");
                std::io::stdin().read_line(&mut answer)?;
                if answer.trim() == "yes" {
                    info!("Uninstalling all mods...");
                    for local_mod in db.all() {
                        info!("Uninstalling {}...", local_mod.get_name());
                        match local_mod {
                            UnsafeLocalMod::Invalid(local_mod) => {
                                remove_failed_mod(local_mod)?;
                            }
                            UnsafeLocalMod::Valid(local_mod) => {
                                remove_mod(local_mod, &db, false)?;
                            }
                        }
                    }
                    info!("Complete");
                } else {
                    warn!("Aborting");
                }
            } else {
                let local_mod = db.get_mod(unique_name);
                if let Some(local_mod) = local_mod {
                    info!(
                        "Uninstalling {}{}...",
                        unique_name,
                        if r { " and dependencies" } else { "" }
                    );
                    remove_mod(local_mod, &db, r)?;
                    info!("Done");
                } else {
                    error!("Mod {} Is Not Installed", unique_name);
                }
            }
        }
        Commands::Export => {
            let local_db = LocalDatabase::fetch(&config.owml_path)?;
            println!("{}", export_mods(&local_db)?);
        }
        Commands::Import {
            file_path,
            disable_missing,
        } => {
            let remote_db = RemoteDatabase::fetch(&config.database_url).await?;
            let local_db = LocalDatabase::fetch(&config.owml_path)?;
            import_mods(&config, &local_db, &remote_db, file_path, *disable_missing).await?;
        }
        Commands::Update { dry } => {
            let remote_db = RemoteDatabase::fetch(&config.database_url).await?;
            let local_db = LocalDatabase::fetch(&config.owml_path)?;
            let updated = update_all(&config, &local_db, &remote_db, *dry).await?;
            if updated {
                if !*dry {
                    info!("Update Complete!");
                }
            } else {
                info!("No Updates Available!");
            }
        }
        Commands::Enable { unique_name } | Commands::Disable { unique_name } => {
            let db = LocalDatabase::fetch(&config.owml_path)?;
            let enable = matches!(cli.command, Commands::Enable { unique_name: _ });
            if unique_name == "*" || unique_name == "all" {
                for local_mod in db.valid() {
                    toggle_mod(&local_mod.manifest.unique_name, &db, enable, false)?;
                }
            } else {
                toggle_mod(unique_name, &db, enable, r)?;
            }
        }
        Commands::LogServer { port } => {
            start_just_logs(port).await?;
        }
        Commands::Run { force, port } => {
            info!("Attempting to launch game...");
            let mut local_db = LocalDatabase::fetch(&config.owml_path)?;
            let remote_db = RemoteDatabase::fetch(&config.database_url).await;
            if let Ok(remote_db) = remote_db {
                local_db.validate_updates(&remote_db);
            }
            let mut flag = false;
            for local_mod in local_db.invalid() {
                flag = true;
                log_mod_validation_errors(local_mod, &local_db);
            }
            if !*force && flag {
                error!("Errors found, refusing to launch");
                info!("Run `owmods validate` to see issues");
                info!("...or run with -f to launch anyway");
                return Ok(());
            }
            start_game(&local_db, &config, port).await?;
        }
        Commands::Open { identifier } => {
            info!("Opening {}", identifier);
            let local_db = LocalDatabase::fetch(&config.owml_path)?;
            open_shortcut(identifier, &config, &local_db)?;
        }
        Commands::Readme { unique_name } => {
            info!("Opening README for {}", unique_name);
            let remote_db = RemoteDatabase::fetch(&config.database_url).await?;
            open_readme(unique_name, &remote_db)?;
        }
        Commands::Validate { fix } => {
            let mut local_db = LocalDatabase::fetch(&config.owml_path)?;
            let remote_db = RemoteDatabase::fetch(&config.database_url).await?;
            local_db.validate_updates(&remote_db);
            if *fix {
                info!("Trying to fix dependency issues...");
                for local_mod in local_db.active() {
                    fix_deps(local_mod, &config, &local_db, &remote_db).await?;
                }
                local_db = LocalDatabase::fetch(&config.owml_path)?;
                info!("Done! Checking for other issues...")
            } else {
                info!("Checking for issues...");
            }
            let mut flag = false;
            for local_mod in local_db.invalid() {
                flag = true;
                log_mod_validation_errors(local_mod, &local_db);
            }
            if flag {
                error!("Issues found, run with -f to fix dependency issues, or disable conflicting mods");
            } else {
                info!("No issues found!");
            }
        }
        Commands::ClearWarnings => {
            let mut new_config = config.clone();
            new_config.viewed_alerts = vec![];
            new_config.save()?;
            info!("Warnings Cleared");
        }
        Commands::GenerateCompletions { shell } => {
            let mut cmd = BaseCli::command();
            let name = cmd.get_name().to_string();
            clap_complete::generate(*shell, &mut cmd, name, &mut std::io::stdout());
        }
    }
    Ok(())
}

fn yes_no(v: bool) -> String {
    if v {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[tokio::main]
async fn main() {
    let cli = BaseCli::parse();
    let logger = Logger::default();
    if let Err(why) = log::set_boxed_logger(Box::new(logger)) {
        println!("Error setting up logger: {why:?}");
    } else {
        log::set_max_level(if cli.debug {
            LevelFilter::Trace
        } else {
            LevelFilter::Info
        });
        let res = run_from_cli(cli).await;
        match res {
            Ok(_) => {}
            Err(e) => {
                error!("{:?}", e);
                process::exit(1);
            }
        };
    }
}
