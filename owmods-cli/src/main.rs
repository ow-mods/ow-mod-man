use std::{error::Error, path::PathBuf};

use anyhow::anyhow;
use clap::{Parser, Subcommand};

use colored::Colorize;
use owmods_core as core;

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
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Print Version")]
    Version,
    #[command(about = "Install/Update OWML (default installs to %APPDATA%/ow-mod-man/OWML)")]
    Setup { owml_path: Option<PathBuf> },
    #[command(about = "View the current database alert (if there is one)")]
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
    #[command(about = "Install a mod from a .zip file (-r not supported)")]
    InstallZip { zip_path: PathBuf },
    #[command(about = "Install a mod from a URL (-r not supported)")]
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = BaseCli::parse();

    let r = cli.recursive;

    let config = if let Some(override_config) = cli.settings_path {
        core::config::read_config(&override_config)?
    } else {
        core::config::get_config()?
    };

    let ran_setup = matches!(&cli.command, Commands::Setup { owml_path: _ });

    if config.owml_path.is_empty() && !ran_setup {
        println!(
            "Welcome to the Outer Wild Mods CLI! In order to continue you'll need to setup OWML."
        );
        println!("To do this, run `owmods setup {{PATH_TO_OWML}}`. Or, run with no path to auto-install it.");
        println!("This message will display so long as owml_path is empty in %APPDATA%/ow-mod-man/settings.json.");
        return Ok(());
    }

    match &cli.command {
        Commands::Version => {
            println!(env!("CARGO_PKG_VERSION"));
        }
        Commands::Setup { owml_path } => {
            if let Some(owml_path) = owml_path {
                if owml_path.is_dir() && owml_path.join("OWML.Manifest.json").is_file() {
                    println!("Path to OWML is valid! Updating config...");
                    let mut new_config = config.clone();
                    new_config.owml_path = owml_path.to_str().unwrap().to_string();
                    core::config::write_config(&new_config)?;
                    println!("Done! Happy Modding!");
                } else {
                    println!(
                        "Error: OWML.Manifest.json Not Found In {}",
                        owml_path.to_str().unwrap()
                    );
                }
            } else {
                let db = core::db::fetch_remote_db(&config).await?;
                let owml = db
                    .get_owml()
                    .ok_or_else(|| anyhow!("OWML not found, is the database URL correct?"))?;
                core::download::download_and_install_owml(&config, owml).await?;
                println!("Done! Happy Modding!");
            }
        }
        Commands::Alert => {
            let alert = core::alerts::fetch_alert(&config).await?;
            if alert.enabled {
                println!(
                    "[{}] {}",
                    alert
                        .severity
                        .unwrap_or_else(|| "info".to_string())
                        .to_ascii_uppercase(),
                    alert.message.unwrap_or_else(|| "No message".to_string())
                )
            } else {
                println!("No alert");
            }
        }
        Commands::List { mod_type } => match mod_type {
            Some(ModListTypes::Local) | None => {
                let db = core::db::fetch_local_db(&config)?;
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
                println!("{}", output);
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
                println!("{}", output);
            }
        },
        Commands::Info { unique_name } => {
            let remote_db = core::db::fetch_remote_db(&config).await?;
            let local_db = core::db::fetch_local_db(&config)?;
            let local_mod = local_db.get_mod(unique_name);
            let remote_mod = remote_db.get_mod(unique_name);
            let installed = local_mod.is_some();
            let has_remote = remote_mod.is_some();
            if (!installed) && (!has_remote) {
                println!("Mod not found in local or remote db: {}", unique_name);
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
            let local_db = core::db::fetch_local_db(&config)?;
            let local_mod = local_db.get_mod(unique_name);
            let mut flag = true;

            if *overwrite && local_mod.is_some() {
                println!("Overriding {}", unique_name);
            } else if let Some(local_mod) = local_db.get_mod(unique_name) {
                println!(
                    "{} is already installed at {}, use -o to overwrite",
                    unique_name, local_mod.mod_path
                );
                flag = false;
            }

            if flag {
                core::download::install_mod_from_db(unique_name, &config, &remote_db, &local_db, r)
                    .await?
            }
        }
        Commands::InstallZip { zip_path } => {
            let local_db = core::db::fetch_local_db(&config)?;
            core::download::install_mod_from_zip(zip_path, &config, &local_db)?;
        }
        Commands::InstallUrl { url } => {
            let local_db = core::db::fetch_local_db(&config)?;
            println!("Installing {}", url);
            core::download::install_mod_from_url(url, &config, &local_db).await?;
        }
        Commands::Uninstall { unique_name } => {
            let db = core::db::fetch_local_db(&config)?;
            let local_mod = db.get_mod(unique_name);
            if let Some(local_mod) = local_mod {
                println!(
                    "Uninstalling {}{}...",
                    unique_name,
                    if r { " and dependencies" } else { "" }
                );
                core::remove::remove_mod(local_mod, &db, r)?;
                println!("Success");
            } else {
                println!("Mod {} Is Not Installed", unique_name);
            }
        }
        Commands::Export => {
            let local_db = core::db::fetch_local_db(&config)?;
            println!("{}", core::io::export_mods(&local_db)?);
        }
        Commands::Import {
            file_path,
            disable_missing,
        } => {
            let remote_db = core::db::fetch_remote_db(&config).await?;
            let local_db = core::db::fetch_local_db(&config)?;
            core::io::import_mods(&config, &local_db, &remote_db, file_path, *disable_missing)
                .await?;
        }
        Commands::Update => {
            let remote_db = core::db::fetch_remote_db(&config).await?;
            let local_db = core::db::fetch_local_db(&config)?;
            core::updates::check_for_updates(&config, &local_db, &remote_db).await?;
        }
        Commands::Enable { unique_name } | Commands::Disable { unique_name } => {
            let db = core::db::fetch_local_db(&config)?;
            let enable = matches!(cli.command, Commands::Enable { unique_name: _ });
            if unique_name == "*" || unique_name == "all" {
                for local_mod in db.mods.iter() {
                    core::toggle::toggle_mod(
                        &PathBuf::from(&local_mod.mod_path),
                        &db,
                        enable,
                        false,
                    )?;
                }
            } else {
                let mod_path = db.get_mod_path(unique_name);
                if let Some(mod_path) = mod_path {
                    core::toggle::toggle_mod(&mod_path, &db, enable, r)?;
                } else {
                    println!("Mod {} is not installed", unique_name);
                }
            }
        }
        Commands::Run { force } => {
            println!("Attempting to launch game...");
            if !*force {
                let local_db = core::db::fetch_local_db(&config)?;
                if core::validate::has_errors(&local_db) {
                    println!("Errors found, refusing to launch");
                    println!("Run `owmods validate` to see issues");
                    println!("...or run with -f to launch anyway");
                    return Ok(());
                }
            }
            core::game::launch_game(&config, None)?;
        }
        Commands::Open { identifier } => {
            println!("Opening {}", identifier);
            let local_db = core::db::fetch_local_db(&config)?;
            core::open::open_shortcut(identifier, &config, &local_db)?;
        }
        Commands::Readme { unique_name } => {
            println!("Opening README for {}", unique_name);
            let remote_db = core::db::fetch_remote_db(&config).await?;
            core::open::open_readme(unique_name, &remote_db)?;
        }
        Commands::Validate { fix_deps } => {
            let local_db = core::db::fetch_local_db(&config)?;
            println!("Checking For Issues...");
            let mut flag = false;
            if *fix_deps {
                let remote_db = core::db::fetch_remote_db(&config).await?;
                core::validate::fix_deps(&config, &local_db, &remote_db).await?;
            }
            for local_mod in local_db.active().iter() {
                let name = &local_mod.manifest.name;
                if !*fix_deps {
                    let (missing, disabled) = core::validate::check_deps(local_mod, &local_db);
                    for missing in missing.iter() {
                        println!("{}: Missing Dependency {}", name, missing);
                        flag = true;
                    }
                    for disabled in disabled.iter() {
                        println!("{}: Disabled Dependency {}", name, disabled.manifest.name);
                        flag = true;
                    }
                }
                for conflicting in core::validate::check_conflicts(local_mod, &local_db).iter() {
                    println!("{}: Conflicts With {}", name, conflicting);
                    flag = true;
                }
            }
            if flag {
                println!("Issues found, run with -f to fix dependency issues, or disable conflicting mods");
            } else {
                println!("No issues found!");
            }
        }
    }
    Ok(())
}
