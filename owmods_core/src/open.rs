use crate::{
    config::Config,
    constants::{DB_REPO_URL, OWML_DOCS_URL, WEBSITE_URL},
    db::{LocalDatabase, RemoteDatabase},
};
use anyhow::anyhow;
use anyhow::Result;

pub fn open_shortcut(identifier: &str, conf: &Config, local_db: &LocalDatabase) -> Result<()> {
    let target = match identifier {
        "db" => DB_REPO_URL,
        "website" => WEBSITE_URL,
        "owml_docs" => OWML_DOCS_URL,
        "owml" => &conf.owml_path,
        _ => "",
    };

    if target.is_empty() {
        let local_mod = local_db
            .get_mod_path(identifier)
            .ok_or_else(|| anyhow!("Mod {} not found", identifier))?;
        opener::open(local_mod)?;
    } else {
        opener::open(target)?;
    }

    Ok(())
}

pub fn open_readme(unique_name: &str, db: &RemoteDatabase) -> Result<()> {
    let remote_mod = db
        .get_mod(unique_name)
        .ok_or_else(|| anyhow!("Mod {} not found", unique_name))?;
    let slug = &remote_mod.slug;
    opener::open_browser(format!("{WEBSITE_URL}/mods/{slug}/"))?;
    Ok(())
}
