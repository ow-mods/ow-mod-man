use std::time::Duration;

use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use owmods_core::logging::{Log, LoggerBackend, ProgressAction, ProgressHandler, ProgressType};

const PROGRESS_TEMPLATE: &str = "{spinner} {wide_msg} [{bar:100.green/cyan}]";
const PROGRESS_CHARS: &str = "=>-";
const SPINNER_TEMPLATE: &str = "{spinner} {msg} {elapsed}";

macro_rules! print_log {
    ($l: expr, $col:ident) => {
        println!("{}", $l.message.$col());
    };
}

pub struct ConsoleLogBackend {
    debug: bool,
    multi_progress: MultiProgress,
}

pub struct ConsoleProgressHandler {
    pb: ProgressBar,
}

impl ConsoleLogBackend {
    pub fn new(debug: bool) -> ConsoleLogBackend {
        ConsoleLogBackend {
            debug,
            multi_progress: MultiProgress::new(),
        }
    }
}

impl ConsoleProgressHandler {
    pub fn new(pb: ProgressBar) -> ConsoleProgressHandler {
        ConsoleProgressHandler { pb }
    }
}

impl LoggerBackend for ConsoleLogBackend {
    fn handle_log(&self, log: Log) {
        match log {
            Log::Debug(l) => {
                if self.debug {
                    print_log!(l, bright_black);
                }
            }
            Log::Info(l) => {
                print_log!(l, cyan);
            }
            Log::Success(l) => {
                print_log!(l, bright_green);
            }
            Log::Warning(l) => {
                print_log!(l, bright_yellow);
            }
            Log::Error(l) => {
                print_log!(l, bright_red);
            }
        };
    }

    fn create_progress(
        &self,
        msg: &str,
        progress_type: ProgressType,
        action_type: ProgressAction,
        len: u64,
    ) -> Box<dyn ProgressHandler> {
        let pb = ProgressBar::hidden();
        pb.set_length(len);
        let template = if matches!(progress_type, ProgressType::Definite) {
            PROGRESS_TEMPLATE
        } else {
            SPINNER_TEMPLATE
        };
        let style = ProgressStyle::with_template(template)
            .unwrap()
            .progress_chars(PROGRESS_CHARS);
        pb.set_style(style);
        pb.set_message(msg.to_string());
        pb.enable_steady_tick(Duration::from_millis(100));
        // Try to make downloads and extracts a bit organized
        let pb = match action_type {
            ProgressAction::Download => self.multi_progress.insert(0, pb),
            ProgressAction::Extract => self.multi_progress.insert_from_back(0, pb),
            _ => self.multi_progress.add(pb),
        };
        Box::new(ConsoleProgressHandler::new(pb))
    }
}

impl ProgressHandler for ConsoleProgressHandler {
    fn increment(&self, amount: u64) {
        self.pb.inc(amount);
    }

    fn change_message(&self, new_message: &str) {
        self.pb.set_message(new_message.to_string())
    }

    fn finish(&self, msg: &str) {
        self.pb.finish_with_message(format!("✓ {}", msg));
    }
}
