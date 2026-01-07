use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint};
use clap_complete::Shell;

#[derive(Parser)]
#[command(name="owmods", author, version, about, long_about = None)]
pub struct BaseCli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(
        global = true,
        short = 'r',
        long = "recursive",
        help = "Apply the action recursively (to all dependencies)"
    )]
    pub recursive: bool,
    #[arg(
        global = true,
        long = "assert-setup",
        help = "Assert that OWML is setup, if not simply exit with code 2 (2 is used to differentiate between a normal exit and an exit due to OWML not being setup)"
    )]
    pub assert_setup: bool,
    #[arg(
        global = true,
        long = "analytics",
        help = "Enable or disable analytics (this will be saved to the config so you won't need it in subsequent runs)"
    )]
    pub analytics: Option<bool>,
    #[arg(global = true, long = "debug", help = "Enable debug output")]
    pub debug: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Print Version")]
    Version,
    #[command(about = "Install/Update OWML")]
    Setup {
        owml_path: Option<PathBuf>,
        #[arg(
            help = "Use the prerelease version of OWML to setup (if there is one)",
            short = 'p',
            long = "prerelease"
        )]
        prerelease: bool,
    },
    #[command(
        about = "View the current database alert (if there is one)",
        alias = "alerts"
    )]
    Alert,
    #[command(about = "Updates all mods", alias = "up")]
    Update {
        #[arg(
            help = "Don't update anything, just list mods that would update",
            short = 'd',
            long = "dry-run"
        )]
        dry: bool,
    },
    #[command(
        about = "List local (installed) or remote (in the database) mods",
        alias = "ls"
    )]
    List {
        #[command(subcommand)]
        mod_type: Option<ModListTypes>,
        #[arg(global = true, help = "Filter by a specific tags", long="tag", short='t', num_args(0..), value_hint = ValueHint::Other)]
        tag: Option<Vec<String>>,
    },
    #[command(about = "Get the tags you can use in `owmods search` or `owmods ls`")]
    Tags,
    #[command(about = "Search the remote database for mods")]
    Search {
        #[arg(help = "The search query to use in the search", value_hint = ValueHint::Other)]
        query: String,
        #[arg(help = "Filter by a specific tags", long="tag", short='t', num_args(0..), value_hint = ValueHint::Other)]
        tag: Option<Vec<String>>,
    },
    #[command(about = "View info about a specific mod")]
    Info {
        #[arg(help = "The unique name of the mod to view the info of", value_hint = ValueHint::Other)]
        unique_name: String,
    },
    #[command(
        about = "Enable a mod (use -r to enable dependencies too)",
        alias = "e"
    )]
    Enable {
        #[arg(help = "The unique name of the mod to enable", value_hint = ValueHint::Other)]
        unique_name: String,
    },
    #[command(
        about = "Disable a mod (use -r to disable dependencies too)",
        alias = "d"
    )]
    Disable {
        #[arg(help = "The unique name of the mod to disable", value_hint = ValueHint::Other)]
        unique_name: String,
    },
    #[command(
        about = "Install a mod (use -r to auto-install dependencies)",
        alias = "i"
    )]
    Install {
        #[arg(help = "The unique name of the mod to install", value_hint = ValueHint::Other)]
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
    InstallZip {
        #[arg(help = "The path to the zip file containing the mod to install", value_hint = ValueHint::FilePath)]
        zip_path: PathBuf,
    },
    #[command(about = "Install a mod from a URL (-r not supported)", alias = "iu")]
    InstallUrl {
        #[arg(help = "The URL to install the mod from", value_hint = ValueHint::Url)]
        url: String,
    },
    #[command(
        about = "Uninstall a mod (use -r to uninstall dependencies too)",
        alias = "rm"
    )]
    Uninstall {
        #[arg(help = "The unique name of the mod to uninstall", value_hint = ValueHint::Other)]
        unique_name: String,
    },
    #[command(about = "Export enabled mods to stdout as JSON")]
    Export,
    #[command(
        about = "Import mods from a .json file (installs if not there, enables if already installed)"
    )]
    Import {
        #[arg(help = "The path to the JSON file to import mods from", value_hint = ValueHint::FilePath)]
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
            short = 'n',
            long = "no-server",
            help = "Don't start a log server, send OWML's logs to stdout directly (this will be set to true if you pass --new-window)"
        )]
        no_server: bool,
        #[arg(
            short = 'w',
            long = "new-window",
            help = "Start the logs in a new cmd window (*WINDOWS ONLY*)"
        )]
        new_window: bool,
        #[arg(
            short = 'p',
            long = "port",
            help = "Port to use for logging",
            default_value = "0",
            value_hint = ValueHint::Other
        )]
        port: u16,
    },
    #[command(about = "Run a server to listen for log messages on")]
    LogServer {
        #[arg(
            short = 'p',
            long = "port",
            help = "Port to use for logging",
            default_value = "0",
            value_hint = ValueHint::Other
        )]
        port: u16,
    },
    #[command(about = "Quickly open something")]
    Open {
        #[arg(help = "db, owml, owml_docs, website, or a mod's unique name", value_hint = ValueHint::Other)]
        identifier: String,
    },
    #[command(about = "Open the readme for a mod", alias = "man")]
    Readme {
        #[arg(help = "The unique name of the mod to open the README of", value_hint = ValueHint::Other)]
        unique_name: String,
    },
    #[command(about = "Open the github repo for a mod", alias = "repo")]
    Github {
        #[arg(help = "The unique name of the mod to open the GitHub repo of", value_hint = ValueHint::Other)]
        unique_name: String,
    },
    #[command(
        about = "Validate local mods for missing dependencies and conflicts",
        alias = "check"
    )]
    Validate {
        #[arg(short = 'f', long = "fix-deps", help = "Try to fix dependency issues")]
        fix: bool,
    },
    #[command(about = "Clear which mod warnings were already shown")]
    ClearWarnings,
    #[command(about = "Generate auto completions for the given shell")]
    GenerateCompletions {
        #[arg(
            help = "Set this to not run the command and instead generate completions for it",
            value_enum
        )]
        shell: Shell,
    },
    #[command(about = "Use the owmods:// protocol to install a mod", alias = "proto")]
    Protocol {
        #[arg(help = "The owmods:// protocol link to use (include the owmods:// part)", value_hint = ValueHint::Other)]
        uri: String,
    },
    #[command(about = "Get the raw JSON of the remote or local database or a specific mod")]
    Raw {
        #[arg(
            short = 'm',
            long = "minify",
            help = "Remove all whitespace the JSON output"
        )]
        minify: bool,
        #[arg(
            help = "`local`, `remote`, or the unique name of a mod",
            value_hint = ValueHint::Other
        )]
        unique_name: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ModListTypes {
    #[command(about = "Show the mods that are currently installed")]
    Local,
    #[command(about = "Show all mods in the database (may want to use grep/find with this!)")]
    Remote,
}
