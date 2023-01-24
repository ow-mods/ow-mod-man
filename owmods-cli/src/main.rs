use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};

use colored::Colorize;
use owmods_core as core;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct BaseCli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short = 'r', long = "recursive")]
    recursive: bool,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Print Version")]
    Version,
    #[command(about = "Install/Update OWML (default installs to %APPDATA%/ow-mod-man/OWML)")]
    Setup,
    #[command(about = "View the current database alert (if there is one)")]
    Alert,
    #[command(about = "Updates all mods")]
    Update,
    #[command(about = "List local (installed) or remote (in the database) mods")]
    List {
        #[command(subcommand)]
        mod_type: Option<ModListTypes>,
    },
    Info { unique_name: String },
    #[command(about = "Enable a mod (use -r to enable dependencies too)")]
    Enable { unique_name: String },
    #[command(about = "Disable a mod (use -r to disable dependencies too)")]
    Disable { unique_name: String },
    #[command(about = "Install a mod (use -r to auto-install dependencies)")]
    Install { unique_name: String },
    #[command(
        about = "Install a mod from a .zip file, useful for workflow results (-r not supported)"
    )]
    InstallZip { zip_path: PathBuf },
    #[command(about = "Export enabled mods to stdout as JSON")]
    Export,
    #[command(
        about = "Import mods from a .json file (installs if not there, enables if already installed)"
    )]
    Import {
        file_path: PathBuf,
        #[arg(short = 'd', long = "disable-missing")]
        disable_missing: bool,
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

    let config = core::config::get_config()?;

    match &cli.command {
        Commands::Version => {
            println!(env!("CARGO_PKG_VERSION"));
        }
        Commands::Setup => {
            let db = core::db::fetch_remote_db(&config).await?;
            let owml = db.get_mod("Alek.OWML").expect("OWML Not found");
            core::download::download_owml(&config, owml).await?;
        }
        Commands::Alert => {
            let alert = core::alerts::fetch_alert(&config).await?;
            if alert.enabled {
                println!(
                    "[{}] {}",
                    alert.severity.to_ascii_uppercase(),
                    alert.message
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
                        "{} {} by {} ({})\n",
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
            let local_mod = local_db.get_mod(&unique_name);
            let remote_mod = remote_db.get_mod(&unique_name);
            let installed = local_mod.is_some();
            let has_remote = local_mod.is_some();
            if !installed && !has_remote {
                println!("Mod not found in local or remote db: {}", unique_name);
            } else {
                let name = if installed { &local_mod.unwrap().manifest.name } else { &remote_mod.unwrap().name };
                let author = if installed { &local_mod.unwrap().manifest.author } else { &remote_mod.unwrap().author_display.unwrap_or("Can't Fetch".to_string()) };
                println!("=== {} ===", unique_name);
                println!("Name: {}", name);
                println!("Author(s): {}", author);
                println!("Installed: {}", if installed { "Yes" } else { "No" });
                if installed {
                    let local_mod = local_mod.unwrap();
                    println!("Installed At: {}", local_mod.mod_path);
                    println!("Enabled: {}", if local_mod.enabled { "Yes" } else { "No" });
                    println!("Installed Version: {}", local_mod.manifest.version);
                    // TODO: Deps and Conflicts.
                }
                println!("In Database: {}", if has_remote { "Yes" } else { "No" });
                if has_remote {
                    let remote_mod = remote_mod.unwrap();
                    println!("Remote Version: {}", remote_mod.version);
                }
            }
        }
        Commands::Install { unique_name } => {
            let remote_db = core::db::fetch_remote_db(&config).await?;
            let local_db = core::db::fetch_local_db(&config)?;
            if let Some(remote_mod) = remote_db.get_mod(unique_name) {
                core::download::download_mod(&config, &local_db, &remote_db, remote_mod, r).await?;
            } else {
                println!("Mod {unique_name} Not Found, Enter The Unique Name Of The Mod You Wish To Install (run `list remote` for a list)");
            }
        }
        Commands::InstallZip { zip_path } => {
            println!("Extracting...");
            core::download::install_from_zip(&config, zip_path)?;
            println!("Installed!");
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
    }
    Ok(())
}
