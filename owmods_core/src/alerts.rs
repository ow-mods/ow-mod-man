use log::debug;
use serde::Deserialize;

use crate::{config::Config, db::LocalDatabase, mods::ModWarning};
use anyhow::Result;

#[derive(Deserialize)]
pub struct Alert {
    pub enabled: bool,
    pub severity: Option<String>,
    pub message: Option<String>,
}

pub async fn fetch_alert(config: &Config) -> Result<Alert> {
    debug!("Fetching {}", config.alert_url);
    let resp = reqwest::get(&config.alert_url).await?;
    let alert: Alert = serde_json::from_str(&resp.text().await?)?;
    Ok(alert)
}

pub fn get_warnings<'a>(
    local_db: &'a LocalDatabase,
    config: &'a Config,
) -> Result<Vec<(&'a String, &'a ModWarning)>> {
    Ok(local_db
        .active()
        .into_iter()
        .filter_map(|m| {
            if let Some(warning) = &m.manifest.warning {
                if config.viewed_alerts.contains(&m.manifest.unique_name) {
                    None
                } else {
                    Some((&m.manifest.unique_name, warning))
                }
            } else {
                None
            }
        })
        .collect())
}

pub fn save_warning_shown(unique_name: &str, config: &Config) -> Result<Config> {
    let mut new_config = config.clone();
    new_config.viewed_alerts.push(unique_name.to_string());
    new_config.save()?;
    Ok(new_config)
}
