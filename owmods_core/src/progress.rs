use log::info;
use serde::Serialize;

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
    pub fn parse(input: &str) -> Self {
        match input {
            "Download" => ProgressAction::Download,
            "Extract" => ProgressAction::Extract,
            _ => ProgressAction::Download,
        }
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressStartPayload {
    pub id: String,
    pub unique_name: Option<String>,
    pub len: ProgressValue,
    pub msg: String,
    pub progress_type: ProgressType,
    pub progress_action: ProgressAction,
}

#[derive(Clone, Serialize)]
pub struct ProgressIncrementPayload {
    pub id: String,
    pub progress: ProgressValue,
}

#[derive(Clone, Serialize)]
pub struct ProgressMessagePayload {
    pub id: String,
    pub msg: String,
}

#[derive(Clone, Serialize)]
pub struct ProgressFinishPayload {
    pub id: String,
    pub success: bool,
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

    pub fn inc(&mut self, amount: ProgressValue) {
        const THROTTLING_AMOUNT: ProgressValue = 30;

        self.progress = if self.progress + amount >= self.len {
            self.len
        } else {
            self.progress + amount
        };

        if self.progress - self.throttled_progress > self.len / THROTTLING_AMOUNT {
            self.throttled_progress = self.progress;
            info!(target: "progress", "Increment|{}|{}", self.id, self.progress);
        }
    }

    pub fn set_msg(&self, msg: &str) {
        info!(target: "progress", "Msg|{}|{}", self.id, msg);
    }

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
}
