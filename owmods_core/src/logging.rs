pub enum ProgressType {
    Definite,
    Indefinite,
}

pub enum ProgressAction {
    Download,
    Extract,
    Wine,
}

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
    backend: Box<dyn LoggerBackend>,
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
        progress_type: ProgressType,
        action_type: ProgressAction,
        msg: &str,
        len: u64,
    ) -> Box<dyn ProgressHandler> {
        self.backend
            .create_progress(msg, progress_type, action_type, len)
    }

    log_msg!(Log::Debug, debug);
    log_msg!(Log::Info, info);
    log_msg!(Log::Success, success);
    log_msg!(Log::Warning, warning);
    log_msg!(Log::Error, error);
}
