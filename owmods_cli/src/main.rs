use std::path::PathBuf;

use anyhow::anyhow;
use clap::{Parser, Subcommand};

use colored::Colorize;
use logging::ConsoleLogBackend;
use owmods_core as core;

use owmods_core::log;

mod logging;

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
    #[arg(global = true, long = "settings", help = "Override the settings file")]
    settings_path: Option<PathBuf>,
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
    #[command(about = "Updates all mods")]
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
}

#[derive(Subcommand)]
enum ModListTypes {
    #[command(about = "Show the mods that are currently installed")]
    Local,
    #[command(about = "Show all mods in the database (may want to use grep with this!)")]
    Remote,
}

async fn run_from_cli(cli: BaseCli, logger: &core::logging::Logger) -> Result<(), anyhow::Error> {
    let r = cli.recursive;

    let config = if let Some(override_config) = cli.settings_path {
        core::config::read_config(logger, &override_config)?
    } else {
        core::config::get_config(logger)?
    };

    let ran_setup = matches!(&cli.command, Commands::Setup { owml_path: _ });

    if config.owml_path.is_empty() && !ran_setup {
        logger.info(
            "Welcome to the Outer Wild Mods CLI! In order to continue you'll need to setup OWML.",
        );
        logger.info("To do this, run `owmods setup {{PATH_TO_OWML}}`. Or, run with no path to auto-install it.");
        logger.info("This message will display so long as owml_path is empty in %APPDATA%/ow-mod-man/settings.json.");
        return Ok(());
    }

    match &cli.command {
        Commands::Version => {
            println!(env!("CARGO_PKG_VERSION"));
        }
        Commands::Setup { owml_path } => {
            if let Some(owml_path) = owml_path {
                if owml_path.is_dir() && owml_path.join("OWML.Manifest.json").is_file() {
                    logger.success("Path to OWML is valid! Updating config...");
                    let mut new_config = config.clone();
                    new_config.owml_path = owml_path.to_str().unwrap().to_string();
                    core::config::write_config(logger, &new_config)?;
                    logger.success("Done! Happy Modding!");
                } else {
                    log!(
                        logger,
                        error,
                        "Error: OWML.Manifest.json Not Found In {}",
                        owml_path.to_str().unwrap()
                    );
                }
            } else {
                let mut config = config.clone();
                config.owml_path = "".to_string();
                let db = core::db::fetch_remote_db(&config).await?;
                let owml = db
                    .get_owml()
                    .ok_or_else(|| anyhow!("OWML not found, is the database URL correct?"))?;
                core::download::download_and_install_owml(logger, &config, owml).await?;
                logger.success("Done! Happy Modding!");
            }
        }
        Commands::Alert => {
            let alert = core::alerts::fetch_alert(logger, &config).await?;
            if alert.enabled {
                log!(
                    logger,
                    info,
                    "[{}] {}",
                    alert
                        .severity
                        .unwrap_or_else(|| "info".to_string())
                        .to_ascii_uppercase(),
                    alert.message.unwrap_or_else(|| "No message".to_string())
                );
            } else {
                logger.info("No alert");
            };
        }
        Commands::List { mod_type } => match mod_type {
            Some(ModListTypes::Local) | None => {
                let db = core::db::fetch_local_db(logger, &config)?;
                let mut output = String::new();
                output += &format!(
                    "Found {} Installed Mods at {}:\n(+): Enabled\n(-): Disabled\n\n",
                    db.mods.len(),
                    config.owml_path
                );
                for local_mod in db.mods.iter() {
                    output += &format!(
                        "({}) {} by {} ({})\n",
                        if local_mod.enabled { "+" } else { "-" },
                        local_mod.manifest.name,
                        local_mod.manifest.author,
                        &local_mod.manifest.unique_name.to_string().bold()
                    );
                }
                logger.info(&output);
            }
            Some(ModListTypes::Remote) => {
                let db = core::db::fetch_remote_db(&config).await?;
                let mut output = String::new();
                output += &format!("Found {} Remote Mods:\n", db.releases.len());
                for remote_mod in db.releases.iter() {
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
                logger.info(&output);
            }
        },
        Commands::Info { unique_name } => {
            let remote_db = core::db::fetch_remote_db(&config).await?;
            let local_db = core::db::fetch_local_db(logger, &config)?;
            let local_mod = local_db.get_mod(unique_name);
            let remote_mod = remote_db.get_mod(unique_name);
            let installed = local_mod.is_some();
            let has_remote = remote_mod.is_some();
            if (!installed) && (!has_remote) {
                log!(
                    logger,
                    error,
                    "Mod not found in local or remote db: {}",
                    unique_name
                );
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
                println!("========== {} ==========", unique_name);
                println!("Name: {}", name);
                println!("Author(s): {}", author);
                println!("Installed: {}", if installed { "Yes" } else { "No" });
                if installed {
                    let local_mod = local_mod.unwrap();
                    println!("Installed At: {}", local_mod.mod_path);
                    println!("Enabled: {}", if local_mod.enabled { "Yes" } else { "No" });
                    println!("Installed Version: {}", local_mod.get_version());
                    if let Some(owml_version) = &local_mod.manifest.owml_version {
                        println!("Expected OWML Version: {}", owml_version);
                    }
                    if let Some(deps) = &local_mod.manifest.dependencies {
                        println!("Dependencies: {}", deps.join(", "));
                    }
                    if let Some(conflicts) = &local_mod.manifest.conflicts {
                        println!("Conflicts: {}", conflicts.join(", "));
                    }
                }
                println!("In Database: {}", if has_remote { "Yes" } else { "No" });
                if has_remote {
                    let remote_mod = remote_mod.unwrap();
                    println!("GitHub Repo URL: {}", remote_mod.repo);
                    println!("Downloads: {}", remote_mod.download_count);
                    if let Some(parent) = &remote_mod.parent {
                        println!("Parent Mod: {}", parent);
                    }
                    if let Some(tags) = &remote_mod.tags {
                        println!("Tags: {}", tags.join(", "));
                    }
                    println!("Remote Version: {}", remote_mod.get_version());
                    if let Some(prerelease) = &remote_mod.prerelease {
                        println!(
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
        } => {
            let remote_db = core::db::fetch_remote_db(&config).await?;
            let local_db = core::db::fetch_local_db(logger, &config)?;
            let local_mod = local_db.get_mod(unique_name);
            let mut flag = true;

            if *overwrite && local_mod.is_some() {
                log!(logger, warning, "Overriding {}", unique_name);
            } else if let Some(local_mod) = local_db.get_mod(unique_name) {
                log!(
                    logger,
                    error,
                    "{} is already installed at {}, use -o to overwrite",
                    unique_name,
                    local_mod.mod_path
                );
                flag = false;
            }

            if flag {
                core::download::install_mod_from_db(
                    logger,
                    unique_name,
                    &config,
                    &remote_db,
                    &local_db,
                    r,
                )
                .await?
            }
        }
        Commands::InstallZip { zip_path } => {
            let local_db = core::db::fetch_local_db(logger, &config)?;
            core::download::install_mod_from_zip(logger, zip_path, &config, &local_db)?;
        }
        Commands::InstallUrl { url } => {
            let local_db = core::db::fetch_local_db(logger, &config)?;
            println!("Installing {}", url);
            core::download::install_mod_from_url(logger, url, &config, &local_db).await?;
        }
        Commands::Uninstall { unique_name } => {
            let db = core::db::fetch_local_db(logger, &config)?;
            let local_mod = db.get_mod(unique_name);
            if let Some(local_mod) = local_mod {
                log!(
                    logger,
                    info,
                    "Uninstalling {}{}...",
                    unique_name,
                    if r { " and dependencies" } else { "" }
                );
                core::remove::remove_mod(local_mod, &db, r)?;
                logger.success("Done");
            } else {
                log!(logger, error, "Mod {} Is Not Installed", unique_name);
            }
        }
        Commands::Export => {
            let local_db = core::db::fetch_local_db(logger, &config)?;
            println!("{}", core::io::export_mods(&local_db)?);
        }
        Commands::Import {
            file_path,
            disable_missing,
        } => {
            let remote_db = core::db::fetch_remote_db(&config).await?;
            let local_db = core::db::fetch_local_db(logger, &config)?;
            core::io::import_mods(
                logger,
                &config,
                &local_db,
                &remote_db,
                file_path,
                *disable_missing,
            )
            .await?;
        }
        Commands::Update => {
            let remote_db = core::db::fetch_remote_db(&config).await?;
            let local_db = core::db::fetch_local_db(logger, &config)?;
            core::updates::check_for_updates(logger, &config, &local_db, &remote_db).await?;
        }
        Commands::Enable { unique_name } | Commands::Disable { unique_name } => {
            let db = core::db::fetch_local_db(logger, &config)?;
            let enable = matches!(cli.command, Commands::Enable { unique_name: _ });
            if unique_name == "*" || unique_name == "all" {
                for local_mod in db.mods.iter() {
                    core::toggle::toggle_mod(
                        logger,
                        &PathBuf::from(&local_mod.mod_path),
                        &db,
                        enable,
                        false,
                    )?;
                }
            } else {
                let mod_path = db.get_mod_path(unique_name);
                if let Some(mod_path) = mod_path {
                    core::toggle::toggle_mod(logger, &mod_path, &db, enable, r)?;
                } else {
                    log!(logger, error, "Mod {} is not installed", unique_name);
                }
            }
        }
        Commands::Run { force } => {
            logger.info("Attempting to launch game...");
            if !*force {
                let local_db = core::db::fetch_local_db(logger, &config)?;
                if core::validate::has_errors(&local_db) {
                    logger.error("Errors found, refusing to launch");
                    logger.info("Run `owmods validate` to see issues");
                    logger.info("...or run with -f to launch anyway");
                    return Ok(());
                }
            }
            core::game::launch_game(logger, &config)?;
        }
        Commands::Open { identifier } => {
            log!(logger, info, "Opening {}", identifier);
            let local_db = core::db::fetch_local_db(logger, &config)?;
            core::open::open_shortcut(identifier, &config, &local_db)?;
        }
        Commands::Readme { unique_name } => {
            log!(logger, info, "Opening README for {}", unique_name);
            let remote_db = core::db::fetch_remote_db(&config).await?;
            core::open::open_readme(unique_name, &remote_db)?;
        }
        Commands::Validate { fix_deps } => {
            let local_db = core::db::fetch_local_db(logger, &config)?;
            logger.info("Checking For Issues...");
            let mut flag = false;
            if *fix_deps {
                let remote_db = core::db::fetch_remote_db(&config).await?;
                core::validate::fix_deps(logger, &config, &local_db, &remote_db).await?;
            }
            for local_mod in local_db.active().iter() {
                let name = &local_mod.manifest.name;
                if !*fix_deps {
                    let (missing, disabled) = core::validate::check_deps(local_mod, &local_db);
                    for missing in missing.iter() {
                        log!(logger, error, "{}: Missing Dependency {}", name, missing);
                        flag = true;
                    }
                    for disabled in disabled.iter() {
                        log!(
                            logger,
                            error,
                            "{}: Disabled Dependency {}",
                            name,
                            disabled.manifest.name
                        );
                        flag = true;
                    }
                }
                for conflicting in core::validate::check_conflicts(local_mod, &local_db).iter() {
                    log!(logger, error, "{}: Conflicts With {}", name, conflicting);
                    flag = true;
                }
            }
            if flag {
                logger.info("Issues found, run with -f to fix dependency issues, or disable conflicting mods");
            } else {
                logger.success("No issues found!");
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = BaseCli::parse();
    let log_backend: Box<dyn core::logging::LoggerBackend> =
        Box::new(ConsoleLogBackend::new(cli.debug));
    let logger = &core::logging::Logger::new(log_backend);
    let res = run_from_cli(cli, logger).await;
    match res {
        Ok(_) => {}
        Err(e) => {
            log!(logger, error, "{:?}", e);
        }
    };
}
