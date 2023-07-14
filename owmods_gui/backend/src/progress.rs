use std::collections::HashMap;

use owmods_core::progress::{
    ProgressAction, ProgressPayload, ProgressStartPayload, ProgressType, ProgressValue,
};
use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProgressBar {
    id: String,
    unique_name: Option<String>,
    message: String,
    progress: ProgressValue,
    progress_type: ProgressType,
    progress_action: ProgressAction,
    len: ProgressValue,
    success: Option<bool>,
    position: u32,
}

impl ProgressBar {
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

#[typeshare]
#[derive(Serialize, Clone)]
pub struct ProgressBars {
    pub bars: HashMap<String, ProgressBar>,
    counter: u32,
}

impl ProgressBars {
    pub fn new() -> Self {
        Self {
            bars: HashMap::new(),
            counter: 0,
        }
    }

    pub fn by_unique_name(&self, unique_name: &str) -> Option<&ProgressBar> {
        self.bars
            .values()
            .filter(|b| matches!(b.success, None))
            .find(|b| match &b.unique_name {
                Some(bar_name) => bar_name == unique_name,
                _ => false,
            })
    }

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
                    matches!(b.success, Some(_))
                        && matches!(b.progress_action, ProgressAction::Extract)
                }) {
                    return Some(self.bars.iter().any(|b| !b.1.success.unwrap_or(true)));
                }
            }
            ProgressPayload::Unknown => {}
        }
        None
    }
}
