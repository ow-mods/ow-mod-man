use std::time::{SystemTime, UNIX_EPOCH};

use lazy_static::lazy_static;
use log::{debug, warn};
use reqwest::Client;
use serde::Serialize;

const MEASUREMENT_ID: &str = "G-2QQN7V5WE1";
const API_KEY: Option<&str> = option_env!("ANALYTICS_API_KEY");

lazy_static! {
    static ref ANALYTICS_ID: String = uuid::Uuid::new_v4().hyphenated().to_string();
}

/// Represents an event sent to GAnalytics when an action is performed on a mod
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AnalyticsEventName {
    /// A mod was installed
    ModInstall,
    /// A dependency of a mod was installed
    ModRequiredInstall,
    /// A prerelease of a mod was installed
    ModPrereleaseInstall,
    /// A mod was installed when it was already installed
    ModReinstall,
    /// A mod was updated
    ModUpdate,
}

#[derive(Debug, Serialize)]
struct AnalyticsEventParams {
    mod_unique_name: String,
    manager_version: String,
}

#[derive(Debug, Serialize)]
struct AnalyticsEvent {
    name: AnalyticsEventName,
    params: AnalyticsEventParams,
}

#[derive(Debug, Serialize)]
struct AnalyticsPayload {
    client_id: String,
    timestamp_micros: u128,
    non_personalized_ads: bool,
    events: Vec<AnalyticsEvent>,
}

impl AnalyticsPayload {
    pub fn new(event_name: &AnalyticsEventName, unique_name: &str) -> Self {
        Self {
            client_id: ANALYTICS_ID.to_string(),
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

// Note how this function doesn't return a result, it shouldn't. We want to simply move on if we can't
// send an event because it's not the end of the world.

/// Send an analytics event with the given [AnalyticsEventName] for the given mod's `unique_name`
///
/// **Please note that unless an `ANALYTICS_API_KEY` env variable is specified at build time this function does nothing.**
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::analytics::{send_analytics_event, AnalyticsEventName};
///
/// # tokio_test::block_on(async {
/// // Time saver is the best mod!
/// loop {
///     send_analytics_event(AnalyticsEventName::ModInstall, "Bwc9876.TimeSaver").await;
/// }
/// # });
///
pub async fn send_analytics_event(event_name: AnalyticsEventName, unique_name: &str) {
    if let Some(api_key) = API_KEY {
        let url = format!("https://www.google-analytics.com/mp/collect?measurement_id={MEASUREMENT_ID}&api_secret={api_key}");
        let client = Client::new();
        let payload = AnalyticsPayload::new(&event_name, unique_name);
        debug!("Sending {:?}", payload);
        let resp = client.post(url).json(&payload).send().await;
        match resp {
            Ok(resp) => {
                if resp.status().is_success() {
                    debug!(
                        "Successfully Sent Analytics Event {:?} for {}",
                        event_name, unique_name
                    );
                } else {
                    warn!(
                        "Couldn't Send Analytics Event For {}! {}",
                        unique_name,
                        resp.status()
                    )
                }
            }
            Err(why) => {
                let err_text = format!(
                    "Couldn't Send Analytics Event For {}! {:?}",
                    unique_name, why
                )
                .replace(api_key, "***");
                warn!("{}", err_text);
            }
        }
    } else {
        debug!(
            "Skipping Analytics As The ANALYTICS_API_KEY Is Null ({:?})",
            event_name
        );
    }
}
