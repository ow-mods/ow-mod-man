use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum ProgressType {
    Definite,
    Indefinite,
}

#[derive(Clone, Serialize)]
pub enum ProgressAction {
    Download,
    Extract,
    Wine,
}

#[derive(Clone, Serialize)]
pub struct LogMessage {
    pub message: String,
}

impl LogMessage {
    pub fn new(msg: &str) -> LogMessage {
        LogMessage {
            message: msg.to_string(),
        }
    }
}

#[derive(Clone, Serialize)]
pub enum Log {
    Debug(LogMessage),
    Info(LogMessage),
    Success(LogMessage),
    Warning(LogMessage),
    Error(LogMessage),
}

pub trait LoggerBackend: Send + Sync {
    fn handle_log(&self, log: Log);
    fn create_progress(
        &self,
        id: &str,
        msg: &str,
        progress_type: ProgressType,
        action_type: ProgressAction,
        len: u64,
    ) -> Box<dyn ProgressHandler>;
}

pub trait ProgressHandler: Send + Sync {
    fn increment(&self, amount: u64);
    fn change_message(&self, new_message: &str);
    fn finish(&self, msg: &str);
}

pub struct Logger {
    pub backend: Box<dyn LoggerBackend>,
}

macro_rules! log_msg {
    { $ty:path, $n:ident } => {
        pub fn $n (&self, msg: &str) {
            self.backend.handle_log($ty(LogMessage::new(msg)));
        }
    }
}

#[macro_export]
macro_rules! log {
    ($l:expr, $ty:ident, $($arg:tt)*) => {
        let res = format!("{}", format_args!($($arg)*));
        $l.$ty(&res);
    };
}

impl Logger {
    pub fn new(backend: Box<dyn LoggerBackend>) -> Logger {
        Logger { backend }
    }

    pub fn start_progress(
        &self,
        id: &str,
        progress_type: ProgressType,
        action_type: ProgressAction,
        msg: &str,
        len: u64,
    ) -> Box<dyn ProgressHandler> {
        self.backend
            .create_progress(id, msg, progress_type, action_type, len)
    }

    log_msg!(Log::Debug, debug);
    log_msg!(Log::Info, info);
    log_msg!(Log::Success, success);
    log_msg!(Log::Warning, warning);
    log_msg!(Log::Error, error);
}

pub struct BasicConsoleBackend;
struct IgnoreProgressHandler;

impl LoggerBackend for BasicConsoleBackend {
    fn handle_log(&self, log: Log) {
        match log {
            Log::Debug(l) => {
                println!("[DEBUG]: {}", l.message);
            }
            Log::Info(l) => {
                println!("[INFO]: {}", l.message);
            }
            Log::Success(l) => {
                println!("[SUCCESS]: {}", l.message);
            }
            Log::Warning(l) => {
                println!("[WARNING]: {}", l.message);
            }
            Log::Error(l) => {
                println!("[ERROR]: {}", l.message);
            }
        };
    }

    fn create_progress(
        &self,
        _id: &str,
        _msg: &str,
        _progress_type: ProgressType,
        _action_type: ProgressAction,
        _len: u64,
    ) -> Box<dyn ProgressHandler> {
        Box::new(IgnoreProgressHandler)
    }
}

impl ProgressHandler for IgnoreProgressHandler {
    fn increment(&self, _amount: u64) {}
    fn change_message(&self, _new_message: &str) {}
    fn finish(&self, _msg: &str) {}
}
