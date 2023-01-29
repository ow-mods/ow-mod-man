pub enum ProgressType {
    Definite,
    Indefinite,
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
        Logger { backend: backend }
    }

    pub fn start_progress<'a>(
        &'a self,
        progress_type: ProgressType,
        msg: &str,
        len: u64,
    ) -> Box<dyn ProgressHandler> {
        self.backend.create_progress(msg, progress_type, len)
    }

    log_msg!(Log::Debug, debug);
    log_msg!(Log::Info, info);
    log_msg!(Log::Success, success);
    log_msg!(Log::Warning, warning);
    log_msg!(Log::Error, error);
}

// impl ProgressLogger<'_> {
//     pub fn new<'a>(id: &'a str, backend: &'a Box<dyn LoggerBackend>) -> ProgressLogger<'a> {
//         ProgressLogger {
//             id: id.to_string(),
//             backend: backend,
//         }
//     }

//     pub fn increment(&self, amount: u64) {
//         let payload = ProgressAction::Increment {
//             id: self.id.clone(),
//             amount: amount,
//         };
//         self.backend.handle_progress(payload);
//     }

//     pub fn change_message(&self, new_message: &str) {
//         let payload = ProgressAction::Message {
//             id: self.id.clone(),
//             message: new_message.to_string(),
//         };
//         self.backend.handle_progress(payload);
//     }

//     // Takes ownership and drops bc it shouldn't be used anymore
//     pub fn finish(&self, msg: &str) {
//         let payload = ProgressAction::Finish {
//             id: self.id.clone(),
//             message: msg.to_string(),
//         };
//         self.backend.handle_progress(payload);
//         drop(self);
//     }
// }
