use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::Level;
use owmods_core::progress::{
    ProgressAction, ProgressIncrementPayload, ProgressMessagePayload, ProgressPayload,
    ProgressStartPayload, ProgressType,
};

const PROGRESS_TEMPLATE: &str = "{spinner} {wide_msg} [{bar:100.green/cyan}]";
const PROGRESS_CHARS: &str = "=>-";
const SPINNER_TEMPLATE: &str = "{spinner} {msg} {elapsed}";

pub struct Logger {
    multi: MultiProgress,
    bars: Arc<Mutex<HashMap<String, ProgressBar>>>,
}

impl Logger {
    fn start_progress(&self, payload: ProgressStartPayload) {
        let pb = ProgressBar::hidden();
        pb.set_length(payload.len);
        let template = if matches!(payload.progress_type, ProgressType::Definite) {
            PROGRESS_TEMPLATE
        } else {
            SPINNER_TEMPLATE
        };
        let style = ProgressStyle::with_template(template)
            .unwrap()
            .progress_chars(PROGRESS_CHARS);
        pb.set_style(style);
        pb.set_message(payload.msg.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        // Try to make downloads and extracts a bit organized
        let pb = match payload.progress_action {
            ProgressAction::Download => self.multi.insert(0, pb),
            ProgressAction::Extract => self.multi.insert_from_back(0, pb),
            _ => self.multi.add(pb),
        };
        self.bars.lock().unwrap().insert(payload.id, pb);
    }

    fn increment_progress(&self, payload: ProgressIncrementPayload) {
        self.bars
            .lock()
            .unwrap()
            .get::<String>(&payload.id)
            .unwrap()
            .set_position(payload.progress);
    }

    fn set_message(&self, payload: ProgressMessagePayload) {
        self.bars
            .lock()
            .unwrap()
            .get::<String>(&payload.id)
            .unwrap()
            .set_message(payload.msg.to_string());
    }

    fn finish(&self, payload: ProgressMessagePayload) {
        let bars = self.bars.lock().unwrap();
        let pb = bars.get::<String>(&payload.id).unwrap();
        pb.set_message(payload.msg.to_string());
        pb.finish();
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            multi: MultiProgress::default(),
            bars: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::STATIC_MAX_LEVEL
    }

    fn log(&self, record: &log::Record) {
        if record.target() == "progress" {
            let raw = format!("{}", record.args());
            let payload = ProgressPayload::parse(&raw);
            match payload {
                ProgressPayload::Start(payload) => self.start_progress(payload),
                ProgressPayload::Increment(payload) => self.increment_progress(payload),
                ProgressPayload::Msg(payload) => self.set_message(payload),
                ProgressPayload::Finish(payload) => self.finish(payload),
                ProgressPayload::Unknown => {}
            };
        } else if self.enabled(record.metadata()) {
            let args = format!("{}", record.args());
            let msg = match record.level() {
                Level::Error => args.red(),
                Level::Warn => args.yellow(),
                Level::Info => args.cyan(),
                Level::Debug => args.bright_black(),
                Level::Trace => args.bright_black(),
            };
            println!("{}", msg);
        }
    }

    fn flush(&self) {
        todo!()
    }
}
