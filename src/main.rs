use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod core;

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
    #[command(about = "Updates all mods")]
    Update,
    #[command(about = "List local (installed) or remote (in the database) mods")]
    List {
        #[command(subcommand)]
        mod_type: Option<ModListTypes>,
    },
    #[command(about = "Access the CLI's config")]
    Config {
        #[command(subcommand)]
        command: ConfigSubcommands,
    },
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
enum ConfigSubcommands {
    #[command(about = "Show the config")]
    Show,
    #[command(about = "Reset the config")]
    Reset,
    #[command(about = "Open the config in the OS's default program")]
    Open,
}

#[derive(Subcommand)]
enum ModListTypes {
    #[command(about = "Show the mods that are currently installed")]
    Local,
    #[command(about = "Show all mods in the database (may want to use grep with this!)")]
    Remote,
}

#[tokio::main]
async fn main() {
    let cli = BaseCli::parse();

    let r = cli.recursive;

    let config = core::config::get_config();

    match &cli.command {
        Commands::Version => {
            println!(env!("CARGO_PKG_VERSION"));
        }
        Commands::Setup => {
            let db = core::db::fetch_remote_db(&config).await;
            let owml = db.get_mod("Alek.OWML").expect("OWML Not found");
            core::download::download_owml(&config, owml).await;
        }
        Commands::List { mod_type } => match mod_type {
            Some(ModListTypes::Local) | None => {
                println!("{}", core::db::local_mod_list_str(&config))
            }
            Some(ModListTypes::Remote) => {
                println!("{}", core::db::remote_mod_list_str(&config).await)
            }
        },
        Commands::Install { unique_name } => {
            let remote_db = core::db::fetch_remote_db(&config).await;
            let local_db = core::db::fetch_local_db(&config);
            if let Some(remote_mod) = remote_db.get_mod(unique_name) {
                core::download::download_mod(&config, &local_db, &remote_db, remote_mod, r).await;
            } else {
                println!("Mod {unique_name} Not Found, Enter The Unique Name Of The Mod You Wish To Install (run `list remote` for a list)");
            }
        }
        Commands::InstallZip { zip_path } => {
            println!("Extracting...");
            core::download::install_from_zip(&config, zip_path);
            println!("Installed!");
        }
        Commands::Export => {
            let local_db = core::db::fetch_local_db(&config);
            println!("{}", core::io::export_mods(&local_db));
        }
        Commands::Import {
            file_path,
            disable_missing,
        } => {
            let remote_db = core::db::fetch_remote_db(&config).await;
            let local_db = core::db::fetch_local_db(&config);
            core::io::import_mods(&config, &local_db, &remote_db, file_path, *disable_missing)
                .await;
        }
        Commands::Update => {
            let remote_db = core::db::fetch_remote_db(&config).await;
            let local_db = core::db::fetch_local_db(&config);
            core::updates::check_for_updates(&config, &local_db, &remote_db).await;
        }
        Commands::Config { command } => match command {
            ConfigSubcommands::Show => {
                println!("{}", serde_json::to_string_pretty(&config).unwrap());
            }
            ConfigSubcommands::Reset => {
                core::config::generate_default_config();
            }
            ConfigSubcommands::Open => {
                opener::open(core::config::config_path()).expect("Unable To Open Config");
            }
        },
        Commands::Enable { unique_name } | Commands::Disable { unique_name } => {
            let db = core::db::fetch_local_db(&config);
            let enable = matches!(cli.command, Commands::Enable { unique_name: _ });
            if unique_name == "*" || unique_name == "all" {
                for local_mod in db.mods.iter() {
                    core::toggle::toggle_mod(
                        &PathBuf::from(&local_mod.mod_path),
                        &db,
                        enable,
                        false,
                    );
                }
            } else {
                let mod_path = db.get_mod_path(unique_name);
                core::toggle::toggle_mod(&mod_path, &db, enable, r);
            }
        }
    }
}
