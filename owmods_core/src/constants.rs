/// OWML is considered a mod in the remote database, this is its unique name
pub const OWML_UNIQUE_NAME: &str = "Alek.OWML";

/// The default URL to the database
pub const DEFAULT_DB_URL: &str = "https://ow-mods.github.io/ow-mod-db/database.json";

/// The URL used by the old manager to fetch alerts, this is here so we can migrate users to the new alert system
pub const OLD_ALERT_URL: &str =
    "https://raw.githubusercontent.com/ow-mods/ow-mod-db/source/alert.json";

/// The default URL to fetch alerts from
pub const DEFAULT_ALERT_URL: &str =
    "https://raw.githubusercontent.com/ow-mods/ow-mod-db/source/alert-v2.json";

/// The name of the config file
pub const CONFIG_FILE_NAME: &str = "settings.json";

/// The URL to the repository for the mod database
pub const DB_REPO_URL: &str = "https://github.com/ow-mods/ow-mod-db";

/// The URL to the documentation for OWML
pub const OWML_DOCS_URL: &str = "https://owml.outerwildsmods.com";

/// The name of OWML's manifest file
pub const OWML_MANIFEST_NAME: &str = "OWML.Manifest.json";

/// The name of OWML's default config file
pub const OWML_DEFAULT_CONFIG_NAME: &str = "OWML.DefaultConfig.json";

/// The name of OWML's launcher exe
pub const OWML_EXE_NAME: &str = "OWML.Launcher.exe";

/// The URL to the website
pub const WEBSITE_URL: &str = "https://outerwildsmods.com";

/// The name of the old manager folder, the new manager uses the OWML installation here to make migration easier
pub const OLD_MANAGER_FOLDER_NAME: &str = "OuterWildsModManager";
