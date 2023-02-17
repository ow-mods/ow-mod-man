use log::{Level, STATIC_MAX_LEVEL};
use owmods_core::progress::ProgressPayload;
use serde::Serialize;
use tauri::{AppHandle, Manager};

pub struct Logger {
    app: AppHandle,
}

#[derive(Serialize, Clone)]
struct LogPayload {
    log_type: Level,
    message: String,
}

impl Logger {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= STATIC_MAX_LEVEL
    }

    fn log(&self, record: &log::Record) {
        let result = if record.target() == "progress" {
            let raw = format!("{}", record.args());
            let payload = ProgressPayload::parse(&raw);
            match payload {
                ProgressPayload::Start(payload) => self.app.emit_all("PROGRESS-START", payload),
                ProgressPayload::Increment(payload) => {
                    self.app.emit_all("PROGRESS-INCREMENT", payload)
                }
                ProgressPayload::Msg(payload) => self.app.emit_all("PROGRESS-MESSAGE", payload),
                ProgressPayload::Finish(payload) => self.app.emit_all("PROGRESS-FINISH", payload),
                ProgressPayload::Unknown => Ok(()),
            }
        } else if self.enabled(record.metadata()) {
            self.app.emit_all(
                "LOG",
                LogPayload {
                    log_type: record.level(),
                    message: format!("{}", record.args()),
                },
            )
        } else {
            Ok(())
        };

        if result.is_err() {
            println!(
                "Error Logging: {:?}\nORIGINAL LOG: {}",
                result.unwrap_err(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
