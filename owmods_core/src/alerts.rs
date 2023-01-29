use serde::Deserialize;

use crate::{config::Config, log, logging::Logger};

#[derive(Deserialize)]
pub struct Alert {
    pub enabled: bool,
    pub severity: Option<String>,
    pub message: Option<String>,
}

pub async fn fetch_alert(log: &Logger, config: &Config) -> Result<Alert, anyhow::Error> {
    log!(log, debug, "Fetching {}", config.alert_url);
    let resp = reqwest::get(&config.alert_url).await?;
    let alert: Alert = serde_json::from_str(&resp.text().await?)?;
    Ok(alert)
}
