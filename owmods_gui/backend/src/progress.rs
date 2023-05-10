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
    message: String,
    progress: ProgressValue,
    progress_type: ProgressType,
    progress_action: ProgressAction,
    len: ProgressValue,
    total: ProgressValue,
    success: Option<bool>,
}

impl From<ProgressStartPayload> for ProgressBar {
    fn from(value: ProgressStartPayload) -> Self {
        Self {
            id: value.id,
            message: value.msg,
            progress_type: value.progress_type,
            progress_action: value.progress_action,
            progress: 0,
            len: value.len,
            success: None,
            total: value.len,
        }
    }
}

#[typeshare]
#[derive(Serialize, Clone)]
pub struct ProgressBars(pub HashMap<String, ProgressBar>);

impl ProgressBars {
    pub fn process(&mut self, payload: &str) {
        let payload = ProgressPayload::parse(payload);
        match payload {
            ProgressPayload::Start(start_payload) => {
                self.0
                    .insert(start_payload.id.clone(), start_payload.into());
            }
            ProgressPayload::Increment(payload) => {
                if let Some(bar) = self.0.get_mut(&payload.id) {
                    bar.progress = payload.progress
                }
            }
            ProgressPayload::Msg(payload) => {
                if let Some(bar) = self.0.get_mut(&payload.id) {
                    bar.message = payload.msg
                }
            }
            ProgressPayload::Finish(payload) => {
                if let Some(bar) = self.0.get_mut(&payload.id) {
                    bar.message = payload.msg;
                    bar.progress = bar.len;
                    bar.success = Some(payload.success);
                }
            }
            ProgressPayload::Unknown => {}
        }
    }
}
