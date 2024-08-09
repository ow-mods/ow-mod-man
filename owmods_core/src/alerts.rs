use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::mods::local::{LocalMod, ModWarning};

/// Represents an alert gotten from the database.
#[typeshare]
#[derive(Debug, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    /// Whether the alert should be shown
    pub enabled: bool,
    /// The severity for the alert, should be `info`, `warning`, or `error`
    pub severity: Option<String>,
    /// The message for the alert
    pub message: Option<String>,
    /// Displays a link or button in the cli and gui respectively. **Note this is limited to GitHub, Discord, and the Mods Website**
    pub url: Option<String>,
    /// Optional label to display for the link instead of "More Info"
    pub url_label: Option<String>,
}

impl Alert {
    /// Compute the hash of the alert. Used to determine if the alert has changed.
    /// You'll want to compare the output of this against [Config::last_viewed_db_alert]
    pub fn compute_hash(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let num = hasher.finish();
        format!("{:x}", num)
    }
}

/// Fetch an alert from the given url.
///
/// ## Returns
///
/// The alert from the url given.
///
/// ## Errors
///
/// Any errors that can happen when fetching json (Networking errors, Deserialization errors).  
///
/// It should be noted this will **NOT** error if we get a 404 or other HTTP error code,
/// and instead will return a disabled alert.
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::alerts::fetch_alert;
/// use owmods_core::config::Config;
///
/// # tokio_test::block_on(async {
/// let config = Config::get(None).unwrap();
/// let alert = fetch_alert(&config.alert_url).await.unwrap();
///
/// if alert.enabled {
///    println!("Alert: {}", alert.message.unwrap());
/// }
/// # });
/// ```
///
pub async fn fetch_alert(url: &str) -> Result<Alert> {
    debug!("Fetching Alert At: {}", url);
    let req = reqwest::get(url).await?.error_for_status();
    // If we get a 404 or anything that's not an actual networking issue simply return a disabled result
    if let Ok(alert) = req {
        let alert = alert.json().await?;
        Ok(alert)
    } else {
        Ok(Alert {
            enabled: false,
            severity: None,
            message: None,
            url: None,
            url_label: None,
        })
    }
}

/// Get the warnings for a list of mods, ignoring the ones in `ignore`
///
/// ## Returns
///
/// A vector of tuples
/// - The first item in the tuple is the unique name of the mod that has the warning
/// - The second item is the warning itself.
///
/// ## Examples
///
/// ```no_run
/// use owmods_core::alerts::get_warnings;
/// use owmods_core::db::LocalDatabase;
/// use owmods_core::config::Config;
///
/// let config = Config::get(None).unwrap();
/// let local_db = LocalDatabase::default();
///
/// let warnings = get_warnings(local_db.valid().collect(), vec![]);
///
/// for (unique_name, warning) in warnings {
///    println!("Warning for {}: {}", unique_name, warning.title);
/// }
/// ```
///
/// ```no_run
/// use owmods_core::alerts::get_warnings;
/// use owmods_core::db::LocalDatabase;
/// use owmods_core::config::Config;
///
/// let config = Config::get(None).unwrap();
/// let local_db = LocalDatabase::default();
///
/// let warnings = get_warnings(local_db.valid().collect(), vec!["Bwc9876.TimeSaver"]);
///
/// assert!(!warnings.iter().any(|(unique_name, _)| unique_name == &"Bwc9876.TimeSaver"));
/// ```  
///
pub fn get_warnings<'a>(
    mods: Vec<&'a LocalMod>,
    ignore: Vec<&'a str>,
) -> Vec<(&'a str, &'a ModWarning)> {
    mods.into_iter()
        .filter_map(|m| {
            if let Some(warning) = &m.manifest.warning {
                let name = m.manifest.unique_name.to_string();
                if ignore.contains(&name.as_str()) {
                    None
                } else {
                    Some((m.manifest.unique_name.as_ref(), warning))
                }
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::DEFAULT_ALERT_URL;

    #[test]
    pub fn test_get_alert() {
        tokio_test::block_on(async {
            let alert = fetch_alert(DEFAULT_ALERT_URL).await;
            assert!(alert.is_ok());
        });
    }

    #[test]
    pub fn test_get_warnings() {
        let mut mod1 = LocalMod::get_test(1);
        mod1.manifest.warning = Some(ModWarning {
            title: "Test".to_string(),
            body: "Test".to_string(),
        });
        let mod2 = LocalMod::get_test(2);
        let warnings = get_warnings(vec![&mod1, &mod2], vec![]);
        assert_eq!(warnings.len(), 1);
        let warnings = get_warnings(vec![&mod1, &mod2], vec![&mod1.manifest.unique_name]);
        assert_eq!(warnings.len(), 0);
    }
}
