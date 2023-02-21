use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use log::{debug, warn};
use reqwest::Client;
use serde::Serialize;

const MEASUREMENT_ID: &str = "G-2QQN7V5WE1";
const API_KEY: Option<&str> = option_env!("ANALYTICS_API_KEY");

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AnalyticsEventName {
    ModInstall,
    ModRequiredInstall,
    ModPrereleaseInstall,
    ModReinstall,
    ModUpdate,
}

#[derive(Serialize)]
struct AnalyticsEventParams {
    mod_unique_name: String,
    manager_version: String,
}

#[derive(Serialize)]
struct AnalyticsEvent {
    name: AnalyticsEventName,
    params: AnalyticsEventParams,
}

#[derive(Serialize)]
struct AnalyticsPayload {
    client_id: String,
    timestamp_micros: u128,
    non_personalized_ads: bool,
    events: Vec<AnalyticsEvent>,
}

impl AnalyticsPayload {
    pub fn new(event_name: &AnalyticsEventName, unique_name: &str) -> Self {
        let client_id = uuid::Uuid::new_v4();
        Self {
            // TODO: Make this global or smth ig? Does it rly matter?
            client_id: client_id.hyphenated().to_string(),
            timestamp_micros: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros(),
            non_personalized_ads: true,
            events: vec![AnalyticsEvent {
                name: event_name.to_owned(),
                params: AnalyticsEventParams {
                    mod_unique_name: unique_name.to_string(),
                    manager_version: env!("CARGO_PKG_VERSION").to_string(),
                },
            }],
        }
    }
}

pub async fn send_analytics_event(event_name: AnalyticsEventName, unique_name: &str) -> Result<()> {
    if let Some(api_key) = API_KEY {
        let url = format!("https://www.google-analytics.com/mp/collect?measurement_id=${MEASUREMENT_ID}&api_secret=${api_key}");
        let client = Client::new();
        let payload = AnalyticsPayload::new(&event_name, unique_name);
        let resp = client.post(url).json(&payload).send().await?;
        if resp.status().is_success() {
            debug!(
                "Successfully Sent Analytics Event {:?} for {}",
                event_name, unique_name
            );
        } else {
            warn!(
                "Couldn't Send Analytics Event For {}! {}",
                unique_name,
                resp.status().as_u16()
            )
        }
    } else {
        debug!(
            "Skipping Analytics As The ANALYTICS_API_KEY Is Null ({:?})",
            event_name
        );
    }
    Ok(())
}
