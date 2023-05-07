use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::{Arc, Mutex},
};

use anyhow::Result;
use log::{Level, STATIC_MAX_LEVEL};
use owmods_core::{file::get_app_path, progress::ProgressPayload};
use serde::Serialize;
use std::fs::create_dir_all;
use tauri::{AppHandle, Manager};
use time::macros::format_description;
use time::OffsetDateTime;
use typeshare::typeshare;

pub struct Logger {
    app: AppHandle,
    writer: Arc<Mutex<BufWriter<File>>>,
}

#[typeshare]
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct LogPayload {
    log_type: Level,
    target: String,
    message: String,
}

impl Logger {
    pub fn new(app: AppHandle) -> Self {
        let now = OffsetDateTime::now_utc();
        let logs_path = get_app_path()
            .expect("Couldn't Make Log File")
            .join("logs")
            .join(
                now.format(format_description!("[year]-[month]-[day]"))
                    .unwrap(),
            )
            .join(format!(
                "{}.log",
                now.format(format_description!("[hour]-[minute]-[second]"))
                    .unwrap()
            ));
        create_dir_all(logs_path.parent().unwrap()).unwrap();
        let file = File::create(logs_path).expect("Couldn't Make Log File");
        let writer = BufWriter::new(file);
        Self {
            app,
            writer: Arc::new(Mutex::new(writer)),
        }
    }

    pub fn write_log_to_file(&self, log_type: Level, message: &str) -> Result<()> {
        let mut writer = self.writer.lock().unwrap();
        let now = OffsetDateTime::now_utc()
            .format(format_description!("[hour]:[minute]:[second]"))
            .unwrap();
        let message = format!("[{}][{}] {}", now, log_type, message);
        println!("{}", message);
        writeln!(writer, "{}", message)?;
        writer.flush()?;
        Ok(())
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
        } else if self.enabled(record.metadata()) && record.target().starts_with("owmods") {
            let message = format!("{}", record.args());
            self.write_log_to_file(record.level(), &message)
                .unwrap_or_else(|e| {
                    println!("FAILED TO WRITE LOG: {:?}", e);
                });
            self.app.emit_all(
                "LOG",
                LogPayload {
                    log_type: record.level(),
                    target: record.target().to_string(),
                    message,
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
