use anyhow::Result;
use log::{info, warn};
use serde::{Deserialize, Serialize};

/// Represents a value in a progress bar
pub type ProgressValue = u32;

/// Type of progress bar
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ProgressType {
    /// We know an amount that's incrementing (ex: 10/90, 11/90, etc).
    Definite,
    /// We don't know an amount and are just waiting.
    Indefinite,
}

/// The action this progress bar is reporting
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ProgressAction {
    /// We're downloading a file
    Download,
    /// We're extracting a ZIP archive
    Extract,
}

/// Payload sent when a progress bar is started
/// Contains all the information needed to create a progress bar
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressStartPayload {
    /// The ID of the progress bar
    pub id: String,
    /// The unique name of the mod this progress bar is for, sometimes this will be None if the progress bar doesn't know what mod it's for
    pub unique_name: Option<String>,
    /// The length of the progress bar
    pub len: ProgressValue,
    /// The message of the progress bar
    pub msg: String,
    /// The type of progress bar
    pub progress_type: ProgressType,
    /// The action this progress bar is reporting
    pub progress_action: ProgressAction,
}

/// Payload sent when a progress bar is incremented
/// Note progress bars internally throttle the amount of times they can be incremented and may not report every increment
/// This is done to prevent spamming small increments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressIncrementPayload {
    /// The ID of the progress bar
    pub id: String,
    /// The amount to increment the progress bar by
    pub progress: ProgressValue,
}

/// Payload sent when a progress bar's message is updated
/// This is usually used to show the current file being extracted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressMessagePayload {
    /// The ID of the progress bar
    pub id: String,
    /// The message of the progress bar
    pub msg: String,
}

/// Payload sent when a progress bar has finished its task
/// This is usually used to show the final message of the progress bar
/// If the progress bar failed, the message will be the failure message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressFinishPayload {
    /// The ID of the progress bar
    pub id: String,
    /// Whether the progress bar succeeded or failed
    pub success: bool,
    /// The message of the progress bar
    pub msg: String,
}

/// Payload sent when a progress bar is updated
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "payload")]
pub enum ProgressPayload {
    /// Payload sent when a progress bar is started
    Start(ProgressStartPayload),
    /// Payload sent when a progress bar is incremented
    Increment(ProgressIncrementPayload),
    /// Payload sent when a progress bar's message is updated
    Msg(ProgressMessagePayload),
    /// Payload sent when a progress bar has finished its task
    Finish(ProgressFinishPayload),
}

impl ProgressPayload {
    /// Parse a progress bar payload from a log line
    ///
    /// ## Returns
    ///
    /// The payload the log line contains.
    ///
    /// ## Panics
    ///
    /// If we cannot parse the line, this method should only be used when we know the line is valid
    ///
    pub fn parse(input: &str) -> Result<Self> {
        let payload = serde_json::from_str::<Self>(input)?;
        Ok(payload)
    }
}

/// Represents a progress bar
pub struct ProgressBar {
    id: String,
    len: ProgressValue,
    progress: ProgressValue,
    throttled_progress: ProgressValue,
    failure_message: String,
    complete: bool,
}

impl ProgressBar {
    /// Create a new progress bar
    /// This will begin reporting the progress bar to the log
    ///
    /// ## Arguments
    ///
    /// - `id` - The ID of the progress bar
    /// - `unique_name` - The unique name of the mod this progress bar is for, pass None if the progress bar doesn't know what mod it's for
    /// - `len` - The length of the progress bar
    /// - `msg` - The message of the progress bar
    /// - `failure_message` - The message to show if the progress bar fails
    /// - `progress_type` - The type of progress bar
    /// - `progress_action` - The action this progress bar is reporting
    ///
    /// ## Returns
    ///
    /// The new progress bar
    /// Note that if this is dropped without calling finish, it will be considered a failure, so make sure to call finish!
    pub fn new(
        id: &str,
        unique_name: Option<&str>,
        len: ProgressValue,
        msg: &str,
        failure_message: &str,
        progress_type: ProgressType,
        progress_action: ProgressAction,
    ) -> Self {
        let new = Self {
            id: id.to_string(),
            len,
            progress: 0,
            throttled_progress: 0,
            failure_message: failure_message.to_string(),
            complete: false,
        };
        let payload = ProgressPayload::Start(ProgressStartPayload {
            id: id.to_string(),
            unique_name: unique_name.map(|s| s.to_string()),
            len,
            msg: msg.to_string(),
            progress_type,
            progress_action,
        });
        new.emit_event(payload);
        new
    }

    fn emit_event(&self, payload: ProgressPayload) {
        let json = serde_json::to_string(&payload);
        match json {
            Ok(json) => {
                info!(target: "progress", "{}", json);
            }
            Err(why) => {
                warn!(target: "progress", "Failed to serialize progress bar event: {:?}", why);
            }
        }
    }

    /// Increment the progress bar
    /// This will throttle the amount of times the progress bar can be incremented, so an increment may not emit a log line
    /// This is done to prevent spamming small increments
    pub fn inc(&mut self, amount: ProgressValue) {
        const THROTTLING_AMOUNT: ProgressValue = 30;

        let new_progress = self.progress.saturating_add(amount);

        self.progress = new_progress.min(self.len);

        if self.progress - self.throttled_progress > self.len / THROTTLING_AMOUNT {
            self.throttled_progress = self.progress;
            let payload = ProgressPayload::Increment(ProgressIncrementPayload {
                id: self.id.clone(),
                progress: self.progress,
            });
            self.emit_event(payload);
        }
    }

    /// Set the message of the progress bar
    pub fn set_msg(&self, msg: &str) {
        let payload = ProgressPayload::Msg(ProgressMessagePayload {
            id: self.id.clone(),
            msg: msg.to_string(),
        });
        self.emit_event(payload);
    }

    /// Finish the progress bar
    ///
    /// This will emit a log line with the final message of the progress bar
    /// This function should always be called when the progress bar is done, as a drop will result in a failure
    ///
    /// ## Arguments
    ///
    /// - `success` - Whether the progress bar succeeded or failed
    /// - `msg` - The message of the progress bar, **this will be ignored if the progress bar failed,
    ///   and will instead use the failure message passed initially**
    pub fn finish(&mut self, success: bool, msg: &str) {
        self.complete = true;
        let msg = if success { msg } else { &self.failure_message };
        let payload = ProgressPayload::Finish(ProgressFinishPayload {
            id: self.id.clone(),
            success,
            msg: msg.to_string(),
        });
        self.emit_event(payload);
    }
}

impl Drop for ProgressBar {
    fn drop(&mut self) {
        if !self.complete {
            self.finish(false, "");
        }
    }
}

/// Generalized progress bar tools
pub mod bars {
    use std::collections::HashMap;

    use typeshare::typeshare;

    use super::*;

    /// Represents a progress bar
    #[typeshare]
    #[derive(Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ProgressBar {
        /// The ID of the progress bar
        pub id: String,
        /// The unique name of the mod this progress bar is for, sometimes this will be None if the progress bar doesn't know what mod it's for
        pub unique_name: Option<String>,
        /// The message of the progress bar
        pub message: String,
        /// The current progress of the progress bar
        pub progress: ProgressValue,
        /// The type of progress bar
        pub progress_type: ProgressType,
        /// The action this progress bar is reporting
        pub progress_action: ProgressAction,
        /// The length of the progress bar
        pub len: ProgressValue,
        /// Whether the progress bar succeeded or failed, and None if it hasn't finished
        pub success: Option<bool>,
        /// The position of the progress bar, the higher the position the higher it is in the list
        pub position: u32,
    }

    impl ProgressBar {
        /// Create a new progress bar from a [ProgressStartPayload]
        fn from_payload(value: ProgressStartPayload, position: u32) -> Self {
            Self {
                id: value.id,
                unique_name: value.unique_name,
                message: value.msg,
                progress_type: value.progress_type,
                progress_action: value.progress_action,
                progress: 0,
                len: value.len,
                success: None,
                position,
            }
        }
    }

    /// Represents a collection of progress bars  
    /// This is used as a generalized way to keep track of all the progress bars and their positions  
    /// This is also used to process progress payloads  
    ///
    /// Note that this still needs to be setup in your logger implementation
    ///
    /// ```no_run
    /// use owmods_core::progress::bars::ProgressBars;
    /// use owmods_core::progress::ProgressPayload;
    /// use std::sync::{Arc, Mutex};
    ///
    /// struct Logger {
    ///     progress_bars: Arc<Mutex<ProgressBars>>,
    /// };
    ///
    /// impl log::Log for Logger {
    /// #  fn enabled(&self, metadata: &log::Metadata) -> bool {
    /// #        true
    /// #  }
    /// #  fn flush(&self) {}
    ///
    ///    fn log(&self, record: &log::Record) {
    ///         if record.target() == "progress" {
    ///             // Get ProgressBars from your state somehow...
    ///             let mut progress_bars = self.progress_bars.lock().expect("Lock is tainted");
    ///             let payload = ProgressPayload::parse(&format!("{}", record.args())).unwrap();
    ///             let any_failed = progress_bars.process(payload);
    ///             // Then emit some sort of event to update your UI
    ///             // Also do stuff with any_failed, etc
    ///         }
    ///    }
    /// }
    /// ```
    #[typeshare]
    #[derive(Serialize, Clone)]
    pub struct ProgressBars {
        /// A map of progress bars by their ID
        pub bars: HashMap<String, ProgressBar>,
        counter: u32,
    }

    impl ProgressBars {
        /// Create a new collection of progress bars
        pub fn new() -> Self {
            Self {
                bars: HashMap::new(),
                counter: 0,
            }
        }

        /// Get a progress bar by its ID
        ///
        /// ## Examples
        ///
        /// ```no_run
        /// use owmods_core::progress::bars::ProgressBars;
        /// use owmods_core::progress::{ProgressPayload, ProgressStartPayload, ProgressType, ProgressAction};
        ///
        /// let mut bars = ProgressBars::new();
        /// let payload = ProgressPayload::Start(ProgressStartPayload {
        ///    id: "test".to_string(),
        ///    unique_name: Some("Test.test".to_string()),
        ///    len: 50,
        ///    msg: "Test Download".to_string(),
        ///    progress_type: ProgressType::Definite,
        ///    progress_action: ProgressAction::Download,
        /// });
        /// bars.process(payload);
        ///
        /// let bar = bars.by_id("test").unwrap();
        ///
        /// assert_eq!(bar.id, "test");
        /// assert_eq!(bar.len, 50);
        /// assert_eq!(bar.unique_name.as_ref().unwrap(), "Test.test");
        /// ```
        ///
        pub fn by_id(&self, id: &str) -> Option<&ProgressBar> {
            self.bars.get(id)
        }

        /// Get a progress bar by the mod associated with it
        ///
        /// ## Examples
        ///
        /// ```no_run
        /// use owmods_core::progress::bars::ProgressBars;
        /// use owmods_core::progress::{ProgressPayload, ProgressStartPayload, ProgressType, ProgressAction};
        ///
        /// let mut bars = ProgressBars::new();
        /// let payload = ProgressPayload::Start(ProgressStartPayload {
        ///    id: "test".to_string(),
        ///    unique_name: Some("Test.test".to_string()),
        ///    len: 50,
        ///    msg: "Test Download".to_string(),
        ///    progress_type: ProgressType::Definite,
        ///    progress_action: ProgressAction::Download,
        /// });
        /// bars.process(payload);
        ///
        /// let bar = bars.by_unique_name("Test.test").unwrap();
        ///
        /// assert_eq!(bar.id, "test");
        /// ```
        ///
        pub fn by_unique_name(&self, unique_name: &str) -> Option<&ProgressBar> {
            self.bars
                .values()
                .filter(|b| b.success.is_none())
                .find(|b| match &b.unique_name {
                    Some(bar_name) => bar_name == unique_name,
                    _ => false,
                })
        }

        /// Process a progress payload
        /// This will update the progress bar associated with the payload accordingly
        /// If the payload is a [ProgressPayload::Finish] payload and all progress bars are finished, this will return whether any of the progress bars failed
        ///
        /// ## Examples
        ///
        /// ```no_run
        /// use owmods_core::progress::bars::ProgressBars;
        /// use owmods_core::progress::*;
        ///
        /// let mut bars = ProgressBars::new();
        /// let payload = ProgressPayload::Start(ProgressStartPayload {
        ///    id: "test".to_string(),
        ///    unique_name: Some("Test.test".to_string()),
        ///    len: 50,
        ///    msg: "Test Download".to_string(),
        ///    progress_type: ProgressType::Definite,
        ///    progress_action: ProgressAction::Download,
        /// });
        /// bars.process(payload);
        /// let payload = ProgressPayload::Increment(ProgressIncrementPayload {
        ///    id: "test".to_string(),
        ///    progress: 30,
        /// });
        /// bars.process(payload);
        ///
        /// let bar = bars.by_id("test").unwrap();
        ///
        /// assert_eq!(bar.progress, 30);
        ///
        /// let payload = ProgressPayload::Finish(ProgressFinishPayload {
        ///   id: "test".to_string(),
        ///   success: true,
        ///   msg: "Finished".to_string(),
        /// });
        /// bars.process(payload);
        ///
        /// let bar = bars.by_id("test").unwrap();
        ///
        /// assert_eq!(bar.progress, 50);
        /// ```
        ///
        /// ```no_run
        /// use owmods_core::progress::bars::ProgressBars;
        /// use owmods_core::progress::*;
        ///
        /// let mut bars = ProgressBars::new();
        /// let payload = ProgressPayload::Start(ProgressStartPayload {
        ///    id: "test".to_string(),
        ///    unique_name: Some("Test.test".to_string()),
        ///    len: 50,
        ///    msg: "Test Download".to_string(),
        ///    progress_type: ProgressType::Definite,
        ///    progress_action: ProgressAction::Download,
        /// });
        /// bars.process(payload);
        /// let payload = ProgressPayload::Increment(ProgressIncrementPayload {
        ///    id: "test".to_string(),
        ///    progress: 30,
        /// });
        /// bars.process(payload);
        /// let payload = ProgressPayload::Finish(ProgressFinishPayload {
        ///   id: "test".to_string(),
        ///   success: true,
        ///   msg: "Finished".to_string(),
        /// });
        /// let any_failed = bars.process(payload);
        ///
        /// assert!(any_failed.is_none());
        ///
        /// let payload = ProgressPayload::Finish(ProgressFinishPayload {
        ///   id: "test2".to_string(),
        ///   success: false,
        ///   msg: "Failed".to_string(),
        /// });
        /// let any_failed = bars.process(payload);
        ///
        /// assert!(any_failed.unwrap());
        /// ```
        ///
        pub fn process(&mut self, payload: ProgressPayload) -> Option<bool> {
            match payload {
                ProgressPayload::Start(start_payload) => {
                    self.bars.insert(
                        start_payload.id.clone(),
                        ProgressBar::from_payload(start_payload.clone(), self.counter),
                    );
                    self.counter += 1;
                }
                ProgressPayload::Increment(payload) => {
                    if let Some(bar) = self.bars.get_mut(&payload.id) {
                        bar.progress = payload.progress
                    }
                }
                ProgressPayload::Msg(payload) => {
                    if let Some(bar) = self.bars.get_mut(&payload.id) {
                        bar.message = payload.msg
                    }
                }
                ProgressPayload::Finish(payload) => {
                    if let Some(bar) = self.bars.get_mut(&payload.id) {
                        bar.message = payload.msg;
                        bar.progress = bar.len;
                        bar.success = Some(payload.success);
                    }
                    if self.bars.values().all(|b| {
                        b.success.is_some() && matches!(b.progress_action, ProgressAction::Extract)
                    }) {
                        return Some(self.bars.iter().any(|b| !b.1.success.unwrap_or(true)));
                    }
                }
            }
            None
        }
    }

    impl Default for ProgressBars {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    mod bar_tests {

        use crate::progress::bars::ProgressBars;

        use super::*;

        fn get_start_payload() -> ProgressPayload {
            ProgressPayload::Start(ProgressStartPayload {
                id: "test".to_string(),
                unique_name: Some("Test.test".to_string()),
                len: 50,
                msg: "Test Download".to_string(),
                progress_type: ProgressType::Definite,
                progress_action: ProgressAction::Download,
            })
        }

        #[test]
        fn test_bar_start() {
            let mut bars = ProgressBars::new();
            let start_payload = get_start_payload();
            bars.process(start_payload);
            let bar = bars.by_id("test").unwrap();
            assert_eq!(bar.id, "test");
            assert_eq!(bar.len, 50);
            assert_eq!(bar.unique_name.as_ref().unwrap(), "Test.test");
            assert!(matches!(bar.progress_type, ProgressType::Definite));
            assert!(matches!(bar.progress_action, ProgressAction::Download));
            assert_eq!(bar.message, "Test Download");
        }

        #[test]
        fn test_bar_inc() {
            let mut bars = ProgressBars::new();
            let start_payload = get_start_payload();
            bars.process(start_payload);
            let inc_payload = ProgressPayload::Increment(ProgressIncrementPayload {
                id: "test".to_string(),
                progress: 30,
            });
            bars.process(inc_payload);
            let bar = bars.by_id("test").unwrap();
            assert_eq!(bar.progress, 30);
        }

        #[test]
        fn test_bar_msg() {
            let mut bars = ProgressBars::new();
            let start_payload = get_start_payload();
            bars.process(start_payload);
            let msg_payload = ProgressPayload::Msg(ProgressMessagePayload {
                id: "test".to_string(),
                msg: "Test Msg".to_string(),
            });
            bars.process(msg_payload);
            let bar = bars.by_id("test").unwrap();
            assert_eq!(bar.message, "Test Msg");
        }

        #[test]
        fn test_bar_finish() {
            let mut bars = ProgressBars::new();
            let start_payload = get_start_payload();
            bars.process(start_payload);
            let finish_payload = ProgressPayload::Finish(ProgressFinishPayload {
                id: "test".to_string(),
                success: true,
                msg: "Finished".to_string(),
            });
            bars.process(finish_payload);
            let bar = bars.by_id("test").unwrap();
            assert_eq!(bar.message, "Finished");
            assert!(bar.success.unwrap());
        }

        #[test]
        fn test_bar_finish_fail() {
            let mut bars = ProgressBars::new();
            let start_payload = get_start_payload();
            bars.process(start_payload);
            let finish_payload = ProgressPayload::Finish(ProgressFinishPayload {
                id: "test".to_string(),
                success: false,
                msg: "Failed".to_string(),
            });
            bars.process(finish_payload);
            let bar = bars.by_id("test").unwrap();
            assert_eq!(bar.message, "Failed");
            assert!(!bar.success.unwrap());
        }

        #[test]
        fn test_bar_finish_all() {
            let mut bars = ProgressBars::new();
            let start_payload = ProgressPayload::Start(ProgressStartPayload {
                id: "test".to_string(),
                unique_name: Some("Test.test".to_string()),
                len: 50,
                msg: "Test Download".to_string(),
                progress_type: ProgressType::Definite,
                progress_action: ProgressAction::Extract,
            });
            bars.process(start_payload);
            let start_payload = ProgressPayload::Start(ProgressStartPayload {
                id: "test2".to_string(),
                unique_name: Some("Test.test".to_string()),
                len: 50,
                msg: "Test Download".to_string(),
                progress_type: ProgressType::Definite,
                progress_action: ProgressAction::Extract,
            });
            bars.process(start_payload);
            let finish_payload = ProgressPayload::Finish(ProgressFinishPayload {
                id: "test".to_string(),
                success: true,
                msg: "Finished".to_string(),
            });
            let all_done = bars.process(finish_payload);
            assert!(all_done.is_none());
            let finish_payload = ProgressPayload::Finish(ProgressFinishPayload {
                id: "test2".to_string(),
                success: true,
                msg: "Finished".to_string(),
            });
            let failed = bars.process(finish_payload);
            assert!(!failed.unwrap());
        }

        #[test]
        fn test_bar_finish_all_fail() {
            let mut bars = ProgressBars::new();
            let start_payload = ProgressPayload::Start(ProgressStartPayload {
                id: "test".to_string(),
                unique_name: Some("Test.test".to_string()),
                len: 50,
                msg: "Test Download".to_string(),
                progress_type: ProgressType::Definite,
                progress_action: ProgressAction::Extract,
            });
            bars.process(start_payload);
            let start_payload = ProgressPayload::Start(ProgressStartPayload {
                id: "test2".to_string(),
                unique_name: Some("Test.test".to_string()),
                len: 50,
                msg: "Test Download".to_string(),
                progress_type: ProgressType::Definite,
                progress_action: ProgressAction::Extract,
            });
            bars.process(start_payload);
            let finish_payload = ProgressPayload::Finish(ProgressFinishPayload {
                id: "test".to_string(),
                success: true,
                msg: "Finished".to_string(),
            });
            let all_done = bars.process(finish_payload);
            assert!(all_done.is_none());
            let finish_payload = ProgressPayload::Finish(ProgressFinishPayload {
                id: "test2".to_string(),
                success: false,
                msg: "Failed".to_string(),
            });
            let failed = bars.process(finish_payload);
            assert!(failed.unwrap());
        }
    }
}
