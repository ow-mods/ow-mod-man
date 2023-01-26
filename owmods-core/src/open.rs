use crate::{
    config::Config,
    db::{LocalDatabase, RemoteDatabase},
};

pub fn open_shortcut(
    identifier: &str,
    conf: &Config,
    local_db: &LocalDatabase,
) -> Result<(), anyhow::Error> {
    let target = match identifier {
        "db" => "https://github.com/ow-mods/ow-mod-db",
        "website" => "https://outerwildsmods.com",
        "owml_docs" => "https://owml.outerwildsmods.com",
        "owml" => &conf.owml_path,
        _ => "",
    };

    if target.is_empty() {
        let local_mod = local_db
            .get_mod_path(identifier)
            .ok_or(anyhow::Error::msg(format!("Mod {} not found", identifier)))?;
        opener::open(local_mod)?;
    } else {
        opener::open(target)?;
    }

    Ok(())
}

pub fn open_readme(unique_name: &str, db: &RemoteDatabase) -> Result<(), anyhow::Error> {
    let remote_mod = db
        .get_mod(unique_name)
        .ok_or(anyhow::Error::msg(format!("Mod {} not found", unique_name)))?;
    let mod_readme = remote_mod
        .readme
        .as_ref()
        .ok_or(anyhow::Error::msg("Mod doesn't have README"))?;
    opener::open_browser(&mod_readme.html_url)?;
    Ok(())
}
