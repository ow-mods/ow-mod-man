use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use game::start_just_logs;
use log::{debug, error, info, warn, LevelFilter};
use owmods_core::{
    alerts::fetch_alert,
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
    download::{
        download_and_install_owml, install_mod_from_db, install_mod_from_url, install_mod_from_zip,
    },
    game::setup_wine_prefix,
    io::{export_mods, import_mods},
    mods::LocalMod,
    open::{open_readme, open_shortcut},
    remove::remove_mod,
    toggle::toggle_mod,
    updates::update_all,
    validate::{self, has_errors},
};

mod game;
mod logging;
use logging::Logger;

use crate::game::start_game;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct BaseCli {
    #[command(subcommand)]
    command: Commands,
    #[arg(
        global = true,
        short = 'r',
        long = "recursive",
        help = "Apply the action recursively (to all dependencies)"
    )]
    recursive: bool,
    #[arg(global = true, long = "debug", help = "Enable debug output")]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Print Version")]
    Version,
    #[command(about = "Install/Update OWML (default installs to %APPDATA%/ow-mod-man/OWML)")]
    Setup { owml_path: Option<PathBuf> },
    #[command(
        about = "View the current database alert (if there is one)",
        alias = "alerts"
    )]
    Alert,
    #[command(about = "Updates all mods", alias = "up")]
    Update,
    #[command(
        about = "List local (installed) or remote (in the database) mods",
        alias = "ls"
    )]
    List {
        #[command(subcommand)]
        mod_type: Option<ModListTypes>,
    },
    #[command(about = "View info about a specific mod")]
    Info { unique_name: String },
    #[command(
        about = "Enable a mod (use -r to enable dependencies too)",
        alias = "e"
    )]
    Enable { unique_name: String },
    #[command(
        about = "Disable a mod (use -r to disable dependencies too)",
        alias = "d"
    )]
    Disable { unique_name: String },
    #[command(
        about = "Install a mod (use -r to auto-install dependencies)",
        alias = "i"
    )]
    Install {
        unique_name: String,
        #[arg(
            short = 'o',
            long = "overwrite",
            help = "Overwrite existing installation"
        )]
        overwrite: bool,
        #[arg(
            short = 'p',
            long = "prerelease",
            help = "Install the prerelease of this mod"
        )]
        prerelease: bool,
    },
    #[command(
        about = "Install a mod from a .zip file (-r not supported)",
        alias = "iz"
    )]
    InstallZip { zip_path: PathBuf },
    #[command(about = "Install a mod from a URL (-r not supported)", alias = "iu")]
    InstallUrl { url: String },
    #[command(
        about = "Uninstall a mod (use -r to uninstall dependencies too)",
        alias = "rm"
    )]
    Uninstall { unique_name: String },
    #[command(about = "Export enabled mods to stdout as JSON")]
    Export,
    #[command(
        about = "Import mods from a .json file (installs if not there, enables if already installed)"
    )]
    Import {
        file_path: PathBuf,
        #[arg(
            short = 'd',
            long = "disable-missing",
            help = "Disable mods that aren't present in the file"
        )]
        disable_missing: bool,
    },
    #[command(about = "Run the game")]
    Run {
        #[arg(
            short = 'f',
            long = "force",
            help = "Force the game to run even if there's conflicting mods or missing dependencies"
        )]
        force: bool,
        #[arg(
            short = 'p',
            long = "port",
            help = "Port to use for logging",
            default_value = "0"
        )]
        port: u16,
    },
    LogServer {
        #[arg(
            short = 'p',
            long = "port",
            help = "Port to use for logging",
            default_value = "0"
        )]
        port: u16,
    },
    #[command(about = "Quickly open something")]
    Open {
        #[arg(help = "db, owml, owml_docs, website, or a mod's unique name")]
        identifier: String,
    },
    #[command(about = "Open the readme for a mod", alias = "man")]
    Readme { unique_name: String },
    #[command(
        about = "Validate local mods for missing dependencies and conflicts",
        alias = "check"
    )]
    Validate {
        #[arg(short = 'f', long = "fix-deps", help = "Try to fix dependency issues")]
        fix_deps: bool,
    },
    #[command(about = "Clear which mod warnings were already shown")]
    ClearWarnings,
}

#[derive(Subcommand)]
enum ModListTypes {
    #[command(about = "Show the mods that are currently installed")]
    Local,
    #[command(about = "Show all mods in the database (may want to use grep/find with this!)")]
    Remote,
}

async fn run_from_cli(cli: BaseCli) -> Result<()> {
    let r = cli.recursive;

    let config = Config::get(None)?;

    let ran_setup = matches!(&cli.command, Commands::Setup { owml_path: _ });

    if config.owml_path.is_empty() && !ran_setup {
        info!(
            "Welcome to the Outer Wild Mods CLI! In order to continue you'll need to setup OWML.",
        );
        info!("To do this, run `owmods setup {{PATH_TO_OWML}}`. Or, run with no path to auto-install it.");
        info!("This message will display so long as owml_path is empty in %APPDATA%/ow-mod-man/settings.json.");
        return Ok(());
    }

    match &cli.command {
        Commands::Version => {
            info!(env!("CARGO_PKG_VERSION"));
        }
        Commands::Setup { owml_path } => {
            if let Some(owml_path) = owml_path {
                if owml_path.is_dir() && owml_path.join("OWML.Manifest.json").is_file() {
                    info!("Path to OWML is valid! Updating config...");
                    let mut new_config = config.clone();
                    new_config.owml_path = owml_path.to_str().unwrap().to_string();
                    new_config.save()?;
                    info!("Done! Happy Modding!");
                } else {
                    error!(
                        "Error: OWML.Manifest.json Not Found In {}",
                        owml_path.to_str().unwrap()
                    );
                }
            } else {
                let mut config = config.clone();
                config.owml_path = "".to_string();
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
                let mut mods: Vec<&LocalMod> = db.mods.values().collect();
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
        Commands::Update => {
            let remote_db = RemoteDatabase::fetch(&config.database_url).await?;
            let local_db = LocalDatabase::fetch(&config.owml_path)?;
            let updated = update_all(&config, &local_db, &remote_db).await?;
            if updated {
                info!("Update Complete!");
            } else {
                info!("No Updates Available!");
            }
        }
        Commands::Enable { unique_name } | Commands::Disable { unique_name } => {
            let db = LocalDatabase::fetch(&config.owml_path)?;
            let enable = matches!(cli.command, Commands::Enable { unique_name: _ });
            if unique_name == "*" || unique_name == "all" {
                for local_mod in db.mods.values() {
                    toggle_mod(&PathBuf::from(&local_mod.mod_path), &db, enable, false)?;
                }
            } else {
                let mod_path = db.get_mod_path(unique_name);
                if let Some(mod_path) = mod_path {
                    toggle_mod(&mod_path, &db, enable, r)?;
                } else {
                    info!("Mod {} is not installed", unique_name);
                }
            }
        }
        Commands::LogServer { port } => {
            start_just_logs(port).await?;
        }
        Commands::Run { force, port } => {
            info!("Attempting to launch game...");
            let local_db = LocalDatabase::fetch(&config.owml_path)?;
            if !*force && has_errors(&local_db) {
                error!("Errors found, refusing to launch");
                info!("Run `owmods validate` to see issues");
                info!("...or run with -f to launch anyway");
                return Ok(());
            }
            if cfg!(windows) || config.wine_prefix.is_some() {
                start_game(&local_db, &config, port).await?;
            } else {
                info!("Hey there! Before you can run the game we'll need to setup a wine prefix.",);
                info!("You can either set wine_prefix in ~/.local/share/ow-mod-man/settings.json.",);
                info!("Or we can set one up for you, you'll need wine and winetricks installed.",);
                println!("Set up a wine prefix now? (y/n)");
                let mut answer = String::new();
                std::io::stdin().read_line(&mut answer)?;
                if answer.trim().to_ascii_lowercase() == "y" {
                    info!("Alright! We'll need about 10 minutes to set up, during setup dialog boxes will appear so make sure to go through them.");
                    info!("When prompted to restart, select \"Restart Later\"");
                    debug!("Begin creating wine prefix");
                    let new_conf = setup_wine_prefix(&config).await?;
                    info!("Success! Launching the game now...");
                    start_game(&local_db, &new_conf, port).await?;
                }
            }
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
        Commands::Validate { fix_deps } => {
            let local_db = LocalDatabase::fetch(&config.owml_path)?;
            info!("Checking For Issues...");
            let mut flag = false;
            if *fix_deps {
                let remote_db = RemoteDatabase::fetch(&config.database_url).await?;
                validate::fix_deps(&config, &local_db, &remote_db).await?;
            }
            for local_mod in local_db.active().iter() {
                let name = &local_mod.manifest.name;
                if !*fix_deps {
                    let (missing, disabled) = validate::check_deps(local_mod, &local_db);
                    for missing in missing.iter() {
                        warn!("{}: Missing Dependency {}", name, missing);
                        flag = true;
                    }
                    for disabled in disabled.iter() {
                        warn!("{}: Disabled Dependency {}", name, disabled.manifest.name);
                        flag = true;
                    }
                }
                for conflicting in validate::check_conflicts(local_mod, &local_db).iter() {
                    warn!("{}: Conflicts With {}", name, conflicting);
                    flag = true;
                }
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
    let err = log::set_boxed_logger(Box::new(logger)).map(|_| {
        log::set_max_level(if cli.debug {
            LevelFilter::Trace
        } else {
            LevelFilter::Info
        })
    });
    if err.is_err() {
        println!("Error setting up logger");
    } else {
        let res = run_from_cli(cli).await;
        match res {
            Ok(_) => {}
            Err(e) => {
                error!("{:?}", e);
            }
        };
    }
}
