use anyhow::{Context, Result};

use crate::{
    config::Config,
    constants::{DB_REPO_URL, OWML_DOCS_URL, WEBSITE_URL},
    db::{LocalDatabase, RemoteDatabase},
    mods::local::UnsafeLocalMod,
};

/// Open a shortcut
/// - "db", "website", "owml_docs", or "owml" are special
/// - If the identifier is a unique name, the folder for that mod is open
///
/// ## Errors
///
/// If the identifier isn't one of the special values and isn't a valid unique name.
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::open::open_shortcut;
/// use owmods_core::config::Config;
/// use owmods_core::db::LocalDatabase;
///
/// let config = Config::get(None).unwrap();
/// let local_db = LocalDatabase::fetch(&config.owml_path).unwrap();
///
/// open_shortcut("db", &config, &local_db).unwrap();
/// ```
///
/// ```no_run
/// use owmods_core::open::open_shortcut;
/// use owmods_core::config::Config;
/// use owmods_core::db::LocalDatabase;
///
/// let config = Config::get(None).unwrap();
/// let local_db = LocalDatabase::fetch(&config.owml_path).unwrap();
///
/// open_shortcut("Bwc9876.TimeSaver", &config, &local_db).unwrap();
/// ```
///
pub fn open_shortcut(identifier: &str, conf: &Config, local_db: &LocalDatabase) -> Result<()> {
    let target = match identifier {
        "db" => DB_REPO_URL,
        "website" => WEBSITE_URL,
        "owml_docs" => OWML_DOCS_URL,
        "owml" => &conf.owml_path,
        _ => "",
    };

    if target.is_empty() {
        let path: &str = local_db
            .get_mod_unsafe(identifier)
            .map(|m| match m {
                UnsafeLocalMod::Invalid(m) => &m.mod_path,
                UnsafeLocalMod::Valid(m) => &m.mod_path,
            })
            .with_context(|| format!("Mod {} not found", identifier))?;
        opener::open(path)?;
    } else {
        opener::open(target)?;
    }

    Ok(())
}

/// Open the readme (website page) for a mod in the user's browser
///
/// ## Errors
///
/// If the unique name provided is not an installed mod or we can't open the browser.
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::open::open_readme;
/// use owmods_core::config::Config;
/// use owmods_core::db::RemoteDatabase;
///
/// # tokio_test::block_on(async {
/// let config = Config::get(None).unwrap();
/// let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
///
/// open_readme("Bwc9876.TimeSaver", &remote_db).unwrap();
/// # });
/// ```
///
pub fn open_readme(unique_name: &str, db: &RemoteDatabase) -> Result<()> {
    let remote_mod = db
        .get_mod(unique_name)
        .with_context(|| format!("Mod {} not found", unique_name))?;
    let slug = &remote_mod.slug;
    opener::open(format!("{WEBSITE_URL}/mods/{slug}/"))?;
    Ok(())
}

/// Open the github repo for a mod in the user's browser
///
/// ## Errors
///
/// If the unique name provided is not an installed mod or we can't open the browser.
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::open::open_github;
/// use owmods_core::config::Config;
/// use owmods_core::db::RemoteDatabase;
///
/// # tokio_test::block_on(async {
/// let config = Config::get(None).unwrap();
/// let remote_db = RemoteDatabase::fetch(&config.database_url).await.unwrap();
///
/// open_github("Bwc9876.TimeSaver", &remote_db).unwrap();
/// # });
/// ```    
///
pub fn open_github(unique_name: &str, db: &RemoteDatabase) -> Result<()> {
    let remote_mod = db
        .get_mod(unique_name)
        .with_context(|| format!("Mod {} not found", unique_name))?;
    let repo = &remote_mod.repo; // this is the entire link to the repo
    opener::open(repo)?;
    Ok(())
}
