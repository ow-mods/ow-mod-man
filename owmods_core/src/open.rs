use crate::{
    config::Config,
    constants::{DB_REPO_URL, OWML_DOCS_URL, WEBSITE_URL},
    db::{LocalDatabase, RemoteDatabase},
};
use anyhow::anyhow;
use anyhow::Result;

/// Open a shortcut
/// - "db", "website", "owml_docs", or "owml" are special
/// - If the identifier is a unique name, the folder for that mod is open
///
/// ## Errors
///
/// If the identifier isn't one of the special values and isn't a valid unique name.
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
        let local_mod: &str = local_db
            .get_mod(identifier)
            .map(|m| m.manifest.unique_name.as_ref())
            .ok_or_else(|| anyhow!("Mod {} not found", identifier))?;
        opener::open(local_mod)?;
    } else {
        opener::open(target)?;
    }

    Ok(())
}

/// Open the readme (website page) for a mod in the user's browser
///
/// ## Errors
///
/// If the unique name provided is not an installed mod
///
pub fn open_readme(unique_name: &str, db: &RemoteDatabase) -> Result<()> {
    let remote_mod = db
        .get_mod(unique_name)
        .ok_or_else(|| anyhow!("Mod {} not found", unique_name))?;
    let slug = &remote_mod.slug;
    opener::open_browser(format!("{WEBSITE_URL}/mods/{slug}/"))?;
    Ok(())
}
