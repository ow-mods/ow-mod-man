use lazy_static::lazy_static;
use log::{debug, warn};
use reqwest::Client;
use serde::Serialize;

use crate::config::Config;

/// If the analytics server is broken, pls ask Rai about it.
const COLLECT_URL: &str = "https://events.raicuparta.com/ow-mod-man/collect";

lazy_static! {
    static ref ANALYTICS_ID: String = uuid::Uuid::new_v4().hyphenated().to_string();
}

/// Represents an event sent to the analytics server when an action is performed on a mod
#[derive(Serialize, Debug, Clone)]
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
#[serde(rename_all = "camelCase")]
struct AnalyticsEvent {
    id: AnalyticsEventName,
    data: AnalyticsEventParams,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AnalyticsPayload {
    client_id: Option<String>,
    session_id: Option<String>,
    events: Vec<AnalyticsEvent>,
}

impl AnalyticsPayload {
    pub fn new(event_name: &AnalyticsEventName, unique_name: &str) -> Self {
        Self {
            client_id: None,
            session_id: Some(ANALYTICS_ID.to_string()),
            events: vec![AnalyticsEvent {
                id: event_name.to_owned(),
                data: AnalyticsEventParams {
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
/// ## Examples
///
/// ```no_run
/// use owmods_core::{config::Config, analytics::{send_analytics_event, AnalyticsEventName}};
///
/// # tokio_test::block_on(async {
/// // Time saver is the best mod!
/// let config = Config::get(None).unwrap();
/// loop {
///     send_analytics_event(AnalyticsEventName::ModInstall, "Bwc9876.TimeSaver",
///     !config.send_analytics).await;
/// }
/// # });
///
pub async fn send_analytics_event(
    event_name: AnalyticsEventName,
    unique_name: &str,
    is_disabled: bool,
) {
    if is_disabled {
        debug!("Skipping Analytics As It's Disabled");
        return;
    }

    let payload = AnalyticsPayload::new(&event_name, unique_name);

    debug!("Sending {payload:?}");
    let client = Client::new();
    let resp = client.post(COLLECT_URL).json(&payload).send().await;

    match resp {
        Ok(resp) => {
            if resp.status().is_success() {
                debug!("Successfully Sent Analytics Event {event_name:?} for {unique_name}");
            } else {
                warn!(
                    "Couldn't Send Analytics Event For {}! {}",
                    unique_name,
                    resp.status()
                )
            }
        }
        Err(err) => {
            warn!("Couldn't Send Analytics Event For {unique_name}! {err:?}");
        }
    }
}

/// Send an analytics event, but don't wait for it to complete.
pub async fn send_analytics_deferred(
    event: AnalyticsEventName,
    unique_name: impl Into<String>,
    config: &Config,
) {
    let unique_name = unique_name.into();
    let should_skip = !config.send_analytics;

    tokio::spawn(async move {
        send_analytics_event(event, &unique_name, should_skip).await;
    });
}
