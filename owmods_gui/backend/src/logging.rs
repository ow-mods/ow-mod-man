use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::{Arc, Mutex},
};

use crate::format_description;
use anyhow::Result;
use log::{warn, Level, STATIC_MAX_LEVEL};
use owmods_core::file::get_app_path;
use owmods_core::progress::ProgressPayload;
use serde::Serialize;
use std::fs::create_dir_all;
use tauri::{async_runtime, AppHandle, Manager};
use time::OffsetDateTime;
use typeshare::typeshare;

use crate::{
    events::{CustomEventEmitterAll, Event},
    State,
};

pub struct Logger {
    app: AppHandle,
    writer: Option<Arc<Mutex<BufWriter<File>>>>,
}

#[typeshare]
#[derive(Serialize, Clone)]
#[allow(dead_code)] // Clippy marks this as dead??
#[serde(rename_all = "camelCase")]
struct LogPayload {
    log_type: Level,
    target: String,
    message: String,
}

impl Logger {
    pub fn new(app: AppHandle, enable_file: bool) -> Self {
        let writer = if enable_file {
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
            Some(Arc::new(Mutex::new(writer)))
        } else {
            None
        };
        Self { app, writer }
    }

    pub fn write_log_to_file(&self, log_type: Level, message: &str) -> Result<()> {
        if let Some(writer) = &self.writer {
            let mut writer = writer.lock().unwrap();
            let now = OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc());
            let now = now
                .format(format_description!("[hour]:[minute]:[second]"))
                .unwrap_or("Failed to get time".to_string());
            let message = format!("[{now}][{log_type}] {message}");
            println!("{message}");
            writeln!(writer, "{message}")?;
            writer.flush()?;
        }
        Ok(())
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= STATIC_MAX_LEVEL
    }

    fn log(&self, record: &log::Record) {
        let result: Result<(), anyhow::Error> = if record.target() == "progress" {
            let raw = format!("{}", record.args());
            let handle = self.app.clone();
            async_runtime::spawn(async move {
                let state = handle.state::<State>();
                let mut bars = state.progress_bars.write().await;
                let payload = ProgressPayload::parse(&raw);
                match payload {
                    Ok(payload) => {
                        let batch_finished = bars.process(payload);
                        handle.typed_emit_all(&Event::ProgressUpdate(())).ok();
                        if let Some(has_error) = batch_finished {
                            handle
                                .typed_emit_all(&Event::ProgressBatchFinish(has_error))
                                .ok();
                        }
                    }
                    Err(why) => {
                        warn!("Failed to parse progress payload: {why}");
                    }
                }
            });
            Ok(())
        } else if self.enabled(record.metadata()) && record.target().starts_with("owmods")
            || record.target().starts_with("outer-wilds-mod-manager")
        {
            let message = format!("{}", record.args());
            self.write_log_to_file(record.level(), &message)
                .unwrap_or_else(|e| {
                    println!("FAILED TO WRITE LOG: {e:?}");
                });
            Ok(())
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
