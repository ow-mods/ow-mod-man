use std::{fmt::Write, path::PathBuf, process};

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use colored::Colorize;
use log::{error, info, warn, LevelFilter};
use owmods_core::{
    alerts::fetch_alert,
    config::Config,
    constants::OWML_UNIQUE_NAME,
    db::{LocalDatabase, RemoteDatabase},
    download::{
        download_and_install_owml, install_mod_from_db, install_mod_from_url, install_mod_from_zip,
    },
    file::get_default_owml_path,
    io::{export_mods, import_mods},
    mods::{
        local::{LocalMod, UnsafeLocalMod},
        remote::RemoteMod,
    },
    open::{open_github, open_readme, open_shortcut},
    protocol::{ProtocolPayload, ProtocolVerb},
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
use logging::{log_mod_validation_errors, show_pre_patcher_warning, Logger};

async fn run_from_cli(cli: BaseCli) -> Result<()> {
    let r = cli.recursive;
    let assert_setup = cli.assert_setup;

    let mut config = Config::get(None)?;

    if let Some(analytics) = cli.analytics {
        if analytics != config.send_analytics {
            info!("Setting send_analytics to {analytics}");
            config.send_analytics = analytics;
            config.save()?;
        }
    }

    let ran_setup = matches!(
        &cli.command,
        Commands::Setup {
            owml_path: _,
            prerelease: _
        } | Commands::Version
    );

    if !config.check_owml() && !ran_setup {
        if assert_setup {
            process::exit(2);
        }

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
        Commands::Setup {
            owml_path,
            prerelease,
        } => {
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
                    .context("OWML not found, is the database URL correct?")?;
                download_and_install_owml(&config, owml, *prerelease).await?;
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
                if let Some(url) = alert.url {
                    info!(
                        "{}: {}",
                        alert.url_label.unwrap_or(String::from("More Info")),
                        url
                    );
                }
            } else {
                info!("No new alerts!");
            };
        }
        Commands::List { mod_type, tag } => match mod_type {
            Some(ModListTypes::Local) | None => {
                let db = LocalDatabase::fetch(&config.owml_path)?;
                let mut output = String::new();
                let mut mods: Vec<&LocalMod> = db.valid().collect();
                if let Some(tags) = tag {
                    match RemoteDatabase::fetch(&config.database_url).await {
                        Ok(remote_db) => {
                            let remote_mods_matching: Vec<&str> = remote_db
                                .matches_tags(tags.clone())
                                .map(|m| m.unique_name.as_str())
                                .collect();
                            mods.retain(|m| {
                                remote_mods_matching.contains(&m.manifest.unique_name.as_str())
                            });
                        }
                        Err(_) => error!("Couldn't Fetch Remote Database, Can't Filter By Tag"),
                    }
                }
                output += &format!(
                    "Found {} Installed Mods at {}:\n(+): Enabled\n(-): Disabled\n\n",
                    mods.len(),
                    config.owml_path
                );
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
                let mods: Vec<&RemoteMod> = if let Some(tags) = tag {
                    db.matches_tags(tags.clone()).collect()
                } else {
                    db.mods.values().collect()
                };
                let mut output = String::new();
                output += &format!("Found {} Remote Mods:\n", mods.len());
                for remote_mod in mods {
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
        Commands::Tags => {
            let db = RemoteDatabase::fetch(&config.database_url).await?;
            for tag in db.get_tags() {
                info!("- {tag}");
            }
        }
        Commands::Search { query, tag } => {
            let db = RemoteDatabase::fetch(&config.database_url).await?;
            let mut mods = db.search(query);
            if let Some(tags) = tag {
                let db_tags = db.get_tags();
                for tag in tags {
                    if !db_tags.contains(tag) {
                        warn!("Tag {tag} was not found in the database");
                    }
                }
                mods = RemoteDatabase::filter_by_tags(mods.into_iter(), tags.clone()).collect();
            }
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
                info!("Mod not found in local or remote db: {unique_name}");
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
                info!("========== {unique_name} ==========");
                info!("Name: {name}");
                info!("Author(s): {author}");
                info!("Installed: {}", yes_no(installed));
                if installed {
                    let local_mod = local_mod.unwrap();
                    info!("Installed At: {}", local_mod.mod_path);
                    info!("Enabled: {}", yes_no(local_mod.enabled));
                    info!("Installed Version: {}", local_mod.manifest.version);
                    if let Some(owml_version) = &local_mod.manifest.owml_version {
                        info!("Expected OWML Version: {owml_version}");
                    }
                    if let Some(deps) = &local_mod.manifest.dependencies {
                        info!("Dependencies:{}", format_list(deps));
                    }
                    if let Some(conflicts) = &local_mod.manifest.conflicts {
                        info!("Conflicts:{}", format_list(conflicts));
                    }
                    if let Some(donate_links) = &local_mod.manifest.donate_links {
                        info!("Donate Links:{}", format_list(donate_links));
                    }
                }
                info!("In Database: {}", yes_no(has_remote));
                if has_remote {
                    let remote_mod = remote_mod.unwrap();
                    info!("Description: {}", remote_mod.description);
                    info!("GitHub Repo URL: {}", remote_mod.repo);
                    info!("Downloads: {}", remote_mod.download_count);
                    if let Some(parent) = &remote_mod.parent {
                        info!("Parent Mod: {parent}");
                    }
                    if let Some(tags) = &remote_mod.tags {
                        info!("Tags: {}", format_list(tags));
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

            if unique_name == OWML_UNIQUE_NAME {
                warn!("OWML is already installed, use `owmods update` to update it");
                warn!("Or use `owmods setup` to reinstall it");
                flag = false;
            } else if *overwrite && local_mod.is_some() {
                warn!("Overriding {unique_name}");
            } else if let Some(local_mod) = local_db.get_mod(unique_name) {
                error!(
                    "{} is already installed at {}, use -o to overwrite",
                    unique_name, local_mod.mod_path
                );
                flag = false;
            }

            if flag {
                install_mod_from_db(unique_name, &config, &remote_db, &local_db, r, *prerelease)
                    .await
                    .map(|_| ())?
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
            info!("Installing From {url}");
            let new_mod = install_mod_from_url(url, None, &config, &local_db).await?;
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
                    let show_warnings_for = remove_mod(local_mod, &db, r)?;
                    for mod_name in show_warnings_for {
                        show_pre_patcher_warning(&mod_name);
                    }
                    info!("Done");
                } else {
                    error!("Mod {unique_name} Is Not Installed");
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
            let mut show_warnings_for: Vec<String> = vec![];
            if unique_name == "*" || unique_name == "all" {
                for local_mod in db.valid() {
                    show_warnings_for.extend(toggle_mod(
                        &local_mod.manifest.unique_name,
                        &db,
                        enable,
                        false,
                    )?);
                }
            } else {
                show_warnings_for = toggle_mod(unique_name, &db, enable, r)?;
            }
            for mod_name in show_warnings_for {
                show_pre_patcher_warning(&mod_name);
            }
        }
        Commands::LogServer { port } => {
            start_just_logs(port).await?;
        }
        Commands::Run {
            force,
            port,
            no_server,
            new_window,
        } => {
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
            let no_server = *no_server || *new_window;
            if *new_window && cfg!(unix) {
                warn!("Skipping option --new-window as this is a Windows only flag");
            }
            let port = if no_server { None } else { Some(port) };
            start_game(&local_db, &config, port, *new_window).await?;
        }
        Commands::Open { identifier } => {
            info!("Opening {identifier}");
            let local_db = LocalDatabase::fetch(&config.owml_path)?;
            open_shortcut(identifier, &config, &local_db)?;
        }
        Commands::Readme { unique_name } => {
            info!("Opening README for {unique_name}");
            let remote_db = RemoteDatabase::fetch(&config.database_url).await?;
            open_readme(unique_name, &remote_db)?;
        }
        Commands::Github { unique_name } => {
            info!("Opening GitHub repo for {unique_name}");
            let remote_db = RemoteDatabase::fetch(&config.database_url).await?;
            open_github(unique_name, &remote_db)?;
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
        Commands::Protocol { uri } => {
            let remote_db = RemoteDatabase::fetch(&config.database_url).await?;
            let local_db = LocalDatabase::fetch(&config.owml_path)?;
            let payload = ProtocolPayload::parse(uri);
            match payload.verb {
                ProtocolVerb::InstallMod | ProtocolVerb::InstallPreRelease => {
                    info!("Installing from {}", payload.payload);
                    install_mod_from_db(
                        &payload.payload,
                        &config,
                        &remote_db,
                        &local_db,
                        r,
                        matches!(payload.verb, ProtocolVerb::InstallPreRelease),
                    )
                    .await?;
                }
                ProtocolVerb::InstallURL | ProtocolVerb::InstallZip => {
                    warn!("WARNING: This will install a mod from a potentially untrusted source, continue? (yes/no)");
                    let mut answer = String::new();
                    std::io::stdin().read_line(&mut answer)?;
                    answer = answer.trim().to_ascii_lowercase();
                    if answer == "yes" || answer == "y" {
                        info!("Installing from {}", payload.payload);
                        match payload.verb {
                            ProtocolVerb::InstallURL => {
                                install_mod_from_url(&payload.payload, None, &config, &local_db)
                                    .await?;
                            }
                            ProtocolVerb::InstallZip => {
                                install_mod_from_zip(
                                    &PathBuf::from(&payload.payload),
                                    &config,
                                    &local_db,
                                )?;
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        warn!("Aborting");
                    }
                }
                ProtocolVerb::RunGame => {
                    let local_db = LocalDatabase::fetch(&config.owml_path)?;
                    let target_mod = local_db.get_mod(&payload.payload);
                    if let Some(target_mod) = target_mod {
                        info!("Launching game with {}", target_mod.manifest.name);
                        toggle_mod(&target_mod.manifest.unique_name, &local_db, true, true)?;
                    } else {
                        warn!("Mod {} not found, ignoring", payload.payload);
                    }
                    start_game(&local_db, &config, None, false).await?;
                }
                ProtocolVerb::Unknown => {
                    error!("Unknown install type, ignoring");
                }
            }
        }
        Commands::Raw {
            minify,
            unique_name,
        } => match unique_name.as_ref().map(|s| s.as_str()).unwrap_or("remote") {
            "local" => {
                let db = LocalDatabase::fetch(&config.owml_path)?;
                let mods = db.all().collect::<Vec<_>>();
                let serialized = if *minify {
                    serde_json::to_string(&mods)?
                } else {
                    serde_json::to_string_pretty(&mods)?
                };
                println!("{serialized}");
            }
            "remote" => {
                let db = RemoteDatabase::fetch(&config.database_url).await?;
                let mods = db.mods.values().collect::<Vec<_>>();
                let serialized = if *minify {
                    serde_json::to_string(&mods)?
                } else {
                    serde_json::to_string_pretty(&mods)?
                };
                println!("{serialized}");
            }
            _ => {
                let remote_db = RemoteDatabase::fetch(&config.database_url).await?;
                let remote_mod = remote_db.get_mod(unique_name.as_ref().unwrap());
                let serialized = if *minify {
                    serde_json::to_string(&remote_mod)?
                } else {
                    serde_json::to_string_pretty(&remote_mod)?
                };
                println!("{serialized}");
            }
        },
    }
    Ok(())
}

fn format_list(l: &[String]) -> String {
    l.iter().fold(String::new(), |mut out, line| {
        let _ = write!(out, "\n  - {line}");
        out
    })
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
                error!("{e:?}");
                process::exit(1);
            }
        };
    }
}
