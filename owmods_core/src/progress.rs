use log::info;
use serde::Serialize;

/// Represents a value in a progress bar
pub type ProgressValue = u32;

/// Type of progress bar
#[derive(Clone, Serialize, Debug)]
pub enum ProgressType {
    /// We know an amount that's incrementing (ex: 10/90, 11/90, etc).
    Definite,
    /// We don't know an amount and are just waiting.
    Indefinite,
}

impl ProgressType {
    fn parse(input: &str) -> Self {
        match input {
            "Definite" => ProgressType::Definite,
            _ => ProgressType::Indefinite,
        }
    }
}

/// The action this progress bar is reporting
#[derive(Clone, Serialize, Debug)]
pub enum ProgressAction {
    /// We're downloading a file
    Download,
    /// We're extracting a ZIP archive
    Extract,
}

impl ProgressAction {
    /// Parse a progress action from a string
    pub fn parse(input: &str) -> Self {
        match input {
            "Download" => ProgressAction::Download,
            "Extract" => ProgressAction::Extract,
            _ => ProgressAction::Download,
        }
    }
}

/// Payload sent when a progress bar is started
/// Contains all the information needed to create a progress bar
#[derive(Clone, Serialize)]
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
#[derive(Clone, Serialize)]
pub struct ProgressIncrementPayload {
    /// The ID of the progress bar
    pub id: String,
    /// The amount to increment the progress bar by
    pub progress: ProgressValue,
}

/// Payload sent when a progress bar's message is updated
/// This is usually used to show the current file being extracted
#[derive(Clone, Serialize)]
pub struct ProgressMessagePayload {
    /// The ID of the progress bar
    pub id: String,
    /// The message of the progress bar
    pub msg: String,
}

/// Payload sent when a progress bar has finished its task
/// This is usually used to show the final message of the progress bar
/// If the progress bar failed, the message will be the failure message
#[derive(Clone, Serialize)]
pub struct ProgressFinishPayload {
    /// The ID of the progress bar
    pub id: String,
    /// Whether the progress bar succeeded or failed
    pub success: bool,
    /// The message of the progress bar
    pub msg: String,
}

/// Payload sent when a progress bar is updated
#[derive(Clone, Serialize)]
pub enum ProgressPayload {
    /// Payload sent when a progress bar is started
    Start(ProgressStartPayload),
    /// Payload sent when a progress bar is incremented
    Increment(ProgressIncrementPayload),
    /// Payload sent when a progress bar's message is updated
    Msg(ProgressMessagePayload),
    /// Payload sent when a progress bar has finished its task
    Finish(ProgressFinishPayload),
    /// An invalid payload
    Unknown,
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
    pub fn parse(input: &str) -> Self {
        let (action, rest) = input.split_once('|').unwrap();
        let (id, args) = rest.split_once('|').unwrap();
        match action {
            "Start" => {
                let (unique_name, r) = args.split_once('|').unwrap();
                let unique_name = if unique_name == "None" {
                    None
                } else {
                    Some(unique_name.to_string())
                };
                let (len, r) = r.split_once('|').unwrap();
                let (progress_type, r) = r.split_once('|').unwrap();
                let (progress_action, msg) = r.split_once('|').unwrap();
                ProgressPayload::Start(ProgressStartPayload {
                    id: id.to_string(),
                    unique_name,
                    msg: msg.to_string(),
                    progress_action: ProgressAction::parse(progress_action),
                    progress_type: ProgressType::parse(progress_type),
                    len: len.parse::<ProgressValue>().unwrap(),
                })
            }
            "Increment" => ProgressPayload::Increment(ProgressIncrementPayload {
                id: id.to_string(),
                progress: args.parse::<ProgressValue>().unwrap(),
            }),
            "Msg" => ProgressPayload::Msg(ProgressMessagePayload {
                id: id.to_string(),
                msg: args.to_string(),
            }),
            "Finish" => {
                let (success, r) = args.split_once('|').unwrap();
                ProgressPayload::Finish(ProgressFinishPayload {
                    id: id.to_string(),
                    success: success == "true",
                    msg: r.to_string(),
                })
            }
            _ => ProgressPayload::Unknown,
        }
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
        info!(target: "progress", "Start|{}|{}|{}|{:?}|{:?}|{}", id, unique_name.unwrap_or("None"), len, progress_type, progress_action, msg);
        new
    }

    /// Increment the progress bar
    /// This will throttle the amount of times the progress bar can be incremented, so an increment may not emit a log line
    /// This is done to prevent spamming small increments
    pub fn inc(&mut self, amount: ProgressValue) {
        const THROTTLING_AMOUNT: ProgressValue = 30;

        let new_progress = self.progress.saturating_add(amount);

        self.progress = if new_progress >= self.len {
            self.len
        } else {
            new_progress
        };

        if self.progress - self.throttled_progress > self.len / THROTTLING_AMOUNT {
            self.throttled_progress = self.progress;
            info!(target: "progress", "Increment|{}|{}", self.id, self.progress);
        }
    }

    /// Set the message of the progress bar
    pub fn set_msg(&self, msg: &str) {
        info!(target: "progress", "Msg|{}|{}", self.id, msg);
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
    /// and will instead use the failure message passed initially**
    pub fn finish(&mut self, success: bool, msg: &str) {
        self.complete = true;
        let msg = if success { msg } else { &self.failure_message };
        info!(target: "progress", "Finish|{}|{}|{}", self.id, success, msg);
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
    ///             let any_failed = progress_bars.process(&format!("{}", record.args()));
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
        ///
        /// let mut bars = ProgressBars::new();
        /// bars.process("Start|test|Test.test|50|Definite|Download|Test Download");
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
        ///
        /// let mut bars = ProgressBars::new();
        /// bars.process("Start|test|Test.test|50|Definite|Download|Test Download");
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
        ///
        /// let mut bars = ProgressBars::new();
        /// bars.process("Start|test|Test.test|50|Definite|Download|Test Download");
        /// bars.process("Increment|test|30");
        ///
        /// let bar = bars.by_id("test").unwrap();
        ///
        /// assert_eq!(bar.progress, 30);
        ///
        /// bars.process("Finish|test|true|Finished");
        ///
        /// let bar = bars.by_id("test").unwrap();
        ///
        /// assert_eq!(bar.progress, 50);
        /// ```
        ///
        /// ```no_run
        /// use owmods_core::progress::bars::ProgressBars;
        ///
        /// let mut bars = ProgressBars::new();
        /// bars.process("Start|test|Test.test|50|Definite|Download|Test Download");
        /// bars.process("Increment|test|30");
        /// let any_failed = bars.process("Finish|test|true|Finished");
        ///
        /// assert!(any_failed.is_none());
        ///
        /// let any_failed = bars.process("Finish|test2|false|Failed");
        ///
        /// assert!(any_failed.unwrap());
        /// ```
        ///
        pub fn process(&mut self, payload: &str) -> Option<bool> {
            let payload = ProgressPayload::parse(payload);
            match payload {
                ProgressPayload::Start(start_payload) => {
                    self.bars.insert(
                        start_payload.id.clone(),
                        ProgressBar::from_payload(start_payload, self.counter),
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
                ProgressPayload::Unknown => {}
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

    #[test]
    fn test_progress_start() {
        let start =
            ProgressPayload::parse("Start|test|Test.test|50|Definite|Download|Test Download");
        match start {
            ProgressPayload::Start(ProgressStartPayload {
                id,
                len,
                unique_name,
                msg,
                progress_type,
                progress_action,
            }) => {
                assert_eq!(id, "test");
                assert_eq!(len, 50);
                assert_eq!(unique_name.unwrap(), "Test.test");
                assert!(matches!(progress_type, ProgressType::Definite));
                assert!(matches!(progress_action, ProgressAction::Download));
                assert_eq!(msg, "Test Download");
            }
            _ => {
                panic!("Start Payload Not Start!")
            }
        }
    }

    #[test]
    fn test_progress_inc() {
        let inc = ProgressPayload::parse("Increment|test|30");
        match inc {
            ProgressPayload::Increment(ProgressIncrementPayload { id, progress }) => {
                assert_eq!(id, "test");
                assert_eq!(progress, 30);
            }
            _ => {
                panic!("Inc Payload Not Inc!");
            }
        }
    }

    #[test]
    fn test_progress_msg() {
        let msg = ProgressPayload::parse("Msg|test|Test Msg");
        match msg {
            ProgressPayload::Msg(ProgressMessagePayload { id, msg }) => {
                assert_eq!(id, "test");
                assert_eq!(msg, "Test Msg");
            }
            _ => {
                panic!("Msg Payload Not Msg!");
            }
        }
    }

    #[test]
    fn test_progress_finish() {
        let finish = ProgressPayload::parse("Finish|test|true|Finished");
        match finish {
            ProgressPayload::Finish(ProgressFinishPayload { id, success, msg }) => {
                assert_eq!(id, "test");
                assert!(success);
                assert_eq!(msg, "Finished");
            }
            _ => {
                panic!("Finish Payload Not Finish!");
            }
        }
    }

    #[test]
    fn test_progress_finish_fail() {
        let finish = ProgressPayload::parse("Finish|test|false|Failed");
        match finish {
            ProgressPayload::Finish(ProgressFinishPayload { id, success, msg }) => {
                assert_eq!(id, "test");
                assert!(!success);
                assert_eq!(msg, "Failed");
            }
            _ => {
                panic!("Finish Payload Not Finish!");
            }
        }
    }

    mod bar_tests {

        use crate::progress::bars::ProgressBars;

        use super::*;

        #[test]
        fn test_bar_start() {
            let mut bars = ProgressBars::new();
            bars.process("Start|test|Test.test|50|Definite|Download|Test Download");
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
            bars.process("Start|test|Test.test|50|Definite|Download|Test Download");
            bars.process("Increment|test|30");
            let bar = bars.by_id("test").unwrap();
            assert_eq!(bar.progress, 30);
        }

        #[test]
        fn test_bar_msg() {
            let mut bars = ProgressBars::new();
            bars.process("Start|test|Test.test|50|Definite|Download|Test Download");
            bars.process("Msg|test|Test Msg");
            let bar = bars.by_id("test").unwrap();
            assert_eq!(bar.message, "Test Msg");
        }

        #[test]
        fn test_bar_finish() {
            let mut bars = ProgressBars::new();
            bars.process("Start|test|Test.test|50|Definite|Download|Test Download");
            bars.process("Finish|test|true|Finished");
            let bar = bars.by_id("test").unwrap();
            assert_eq!(bar.message, "Finished");
            assert!(bar.success.unwrap());
        }

        #[test]
        fn test_bar_finish_fail() {
            let mut bars = ProgressBars::new();
            bars.process("Start|test|Test.test|50|Definite|Download|Test Download");
            bars.process("Finish|test|false|Failed");
            let bar = bars.by_id("test").unwrap();
            assert_eq!(bar.message, "Failed");
            assert!(!bar.success.unwrap());
        }

        #[test]
        fn test_bar_finish_all() {
            let mut bars = ProgressBars::new();
            bars.process("Start|test|Test.test|50|Definite|Extract|Test Download");
            bars.process("Start|test2|Test.test|50|Definite|Extract|Test Download");
            let all_done = bars.process("Finish|test|true|Finished");
            assert!(all_done.is_none());
            let failed = bars.process("Finish|test2|true|Finished");
            assert!(!failed.unwrap());
        }

        #[test]
        fn test_bar_finish_all_fail() {
            let mut bars = ProgressBars::new();
            bars.process("Start|test|Test.test|50|Definite|Extract|Test Download");
            bars.process("Start|test2|Test.test|50|Definite|Extract|Test Download");
            let all_done = bars.process("Finish|test|true|Finished");
            assert!(all_done.is_none());
            let failed = bars.process("Finish|test2|false|Finished");
            assert!(failed.unwrap());
        }
    }
}
