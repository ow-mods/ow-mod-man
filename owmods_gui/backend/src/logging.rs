use owmods_core::logging::{
    Log, Logger, LoggerBackend, ProgressAction, ProgressHandler, ProgressType,
};
use serde::Serialize;
use tauri::{AppHandle, Manager};

struct TauriLogBackend {
    app: AppHandle,
}

struct TauriProgressBackend {
    id: String,
    app: AppHandle,
}

#[derive(Clone, Serialize)]
struct ProgressStartPayload {
    id: String,
    message: String,
    progress_type: ProgressType,
    progress_action: ProgressAction,
    len: u64,
}

#[derive(Clone, Serialize)]
enum ProgressUpdatePayload {
    Increment { id: String, amount: u64 },
    ChangeMsg { id: String, new_msg: String },
    Finish { id: String, msg: String },
}

impl TauriProgressBackend {
    pub fn new(id: &str, app: AppHandle) -> TauriProgressBackend {
        TauriProgressBackend {
            id: id.to_string(),
            app: app,
        }
    }
}

impl LoggerBackend for TauriLogBackend {
    fn handle_log(&self, log: Log) {
        self.app.emit_all("LOG", log).ok();
    }

    fn create_progress(
        &self,
        id: &str,
        msg: &str,
        progress_type: ProgressType,
        action_type: ProgressAction,
        len: u64,
    ) -> Box<dyn ProgressHandler> {
        self.app
            .emit_all(
                "PROGRESS-START",
                ProgressStartPayload {
                    id: id.to_string(),
                    message: msg.to_string(),
                    progress_type: progress_type,
                    progress_action: action_type,
                    len: len,
                },
            )
            .ok();
        Box::new(TauriProgressBackend::new(id, self.app.clone()))
    }
}

impl ProgressHandler for TauriProgressBackend {
    fn increment(&self, amount: u64) {
        self.app
            .emit_all(
                "PROGRESS-INCREMENT",
                ProgressUpdatePayload::Increment {
                    id: self.id.clone(),
                    amount: amount,
                },
            )
            .ok();
    }

    fn change_message(&self, new_message: &str) {
        self.app
            .emit_all(
                "PROGRESS-MSG",
                ProgressUpdatePayload::ChangeMsg {
                    id: self.id.clone(),
                    new_msg: new_message.to_string(),
                },
            )
            .ok();
    }

    fn finish(&self, msg: &str) {
        self.app
            .emit_all(
                "PROGRESS-FINISH",
                ProgressUpdatePayload::Finish {
                    id: self.id.clone(),
                    msg: msg.to_string(),
                },
            )
            .ok();
    }
}

pub fn get_logger(handle: AppHandle) -> Logger {
    let backend = TauriLogBackend { app: handle };
    Logger::new(Box::new(backend))
}
