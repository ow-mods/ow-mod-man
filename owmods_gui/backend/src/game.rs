use std::{
    fs::File,
    io::{BufWriter, Write},
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::{anyhow, Result};
use log::error;
use owmods_core::{
    alerts::get_warnings,
    config::Config,
    db::LocalDatabase,
    file::{create_all_parents, get_app_path},
    search::matches_query,
    socket::{SocketMessage, SocketMessageType},
};
use serde::{Deserialize, Serialize};
use tauri::{api::dialog, AppHandle, Window, WindowBuilder};
use time::{macros::format_description, OffsetDateTime};
use typeshare::typeshare;

use crate::{
    events::{CustomEventEmitterAll, Event, LogLineCountUpdatePayload, LogsBehindPayload},
    LogPort,
};

#[typeshare]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GameMessage {
    pub port: LogPort,
    pub message: SocketMessage,
    pub amount: u32,
    pub timestamp: String,
}

impl GameMessage {
    fn get_timestamp() -> String {
        let now = OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc());
        now.format(format_description!(
            "[hour repr:12]:[minute]:[second] [period] (UTC[offset_hour sign:mandatory])"
        ))
        .unwrap_or("Unknown".to_string())
    }

    pub fn new(port: LogPort, message: SocketMessage) -> Self {
        Self {
            port,
            message,
            amount: 1,
            timestamp: Self::get_timestamp(),
        }
    }
}

pub struct LogData {
    port: LogPort,
    messages: Vec<GameMessage>,
    indices: Vec<usize>,
    writer: BufWriter<File>,
    active_filter: Option<SocketMessageType>,
    active_search: String,
    app_handle: AppHandle,
    message_tracker: (u32, Instant),
    // If the usize is None, it's a regular update
    // If it's Some, it's a count update
    // Fatal emits always happen instantly
    queued_emits: Vec<Option<u32>>,
}

impl LogData {
    const LOG_LIMIT_PER_SECOND: u32 = 25;
    const LOG_TIME_UNTIL_FORCED_EMIT_SEC: u64 = 1;

    pub fn new(port: LogPort, handle: &AppHandle) -> Result<Self> {
        let now = OffsetDateTime::now_utc();
        let logs_path = get_app_path()?
            .join("game_logs")
            .join(
                now.format(format_description!("[year]-[month]-[day]"))
                    .unwrap(),
            )
            .join(format!(
                "{}_{port}.log",
                now.format(format_description!("[hour]-[minute]-[second]"))
                    .unwrap()
            ));
        create_all_parents(&logs_path)?;
        let file = File::options()
            .read(true)
            .append(true)
            .create(true)
            .open(&logs_path)
            .map_err(|e| anyhow!("Couldn't create log file: {:?}", e))?;
        let writer = BufWriter::new(file);

        Ok(Self {
            port,
            messages: vec![],
            indices: vec![],
            writer,
            active_filter: None,
            active_search: String::new(),
            app_handle: handle.clone(),
            message_tracker: (0, Instant::now()),
            queued_emits: vec![],
        })
    }

    fn emit_update(&self) {
        let res = self.app_handle.typed_emit_all(&Event::LogUpdate(self.port));
        if let Err(why) = res {
            error!("Couldn't Emit Game Log: {}", why)
        }
    }

    fn emit_count_update(&self, line: u32) {
        let res =
            self.app_handle
                .typed_emit_all(&Event::LogLineCountUpdate(LogLineCountUpdatePayload {
                    port: self.port,
                    line,
                }));
        if let Err(why) = res {
            error!("Couldn't Emit Game Log Count: {}", why)
        }
    }

    fn emit_fatal_alert(&self, msg: &GameMessage) {
        let res = self
            .app_handle
            .typed_emit_all(&Event::LogFatal(msg.clone()));
        if let Err(why) = res {
            error!("Couldn't Emit Fatal Alert: {}", why)
        }
    }

    fn emit_behind(&self, behind: bool) {
        let res = self
            .app_handle
            .typed_emit_all(&Event::LogsBehind(LogsBehindPayload {
                port: self.port,
                behind,
            }));
        if let Err(why) = res {
            error!("Couldn't Emit Logs Behind: {}", why)
        }
    }

    pub fn process_emit_queue(&mut self) {
        if self.queued_emits.is_empty() {
            return;
        }
        let mut queue = self.queued_emits.clone();
        queue.sort_unstable();
        queue.dedup();
        for emit in queue.drain(..) {
            if let Some(line) = emit {
                self.emit_count_update(line);
            } else {
                self.emit_update();
            }
        }
        self.queued_emits.clear();
        self.eval_indices();
        self.emit_behind(false);
    }

    pub fn get_lines(
        &mut self,
        active_filter: Option<SocketMessageType>,
        active_search: &str,
    ) -> Vec<usize> {
        if self.active_filter != active_filter || self.active_search != *active_search {
            self.active_filter = active_filter;
            self.active_search = active_search.to_string();
            self.eval_indices();
        } else if self.indices.is_empty() {
            self.eval_indices();
        }
        self.indices.clone()
    }

    pub fn get_message(&self, index: usize) -> Option<&GameMessage> {
        self.messages.get(index)
    }

    pub fn get_count(&self) -> u32 {
        self.messages.iter().map(|msg| msg.amount).sum()
    }

    fn eval_indices(&mut self) {
        self.indices = self
            .messages
            .iter()
            .enumerate()
            .filter(|(_, msg)| {
                if let Some(filter) = self.active_filter.as_ref() {
                    msg.message.message_type == *filter
                } else {
                    true
                }
            })
            .filter(|(_, msg)| {
                if !self.active_search.is_empty() {
                    matches_query(&msg.message, &self.active_search)
                } else {
                    true
                }
            })
            .map(|(i, _)| i)
            .collect();
    }

    pub fn take_message(&mut self, msg: SocketMessage) {
        let msg = GameMessage::new(self.port, msg);
        if let Err(why) = write_log(&mut self.writer, &msg.message) {
            error!("Couldn't write to log file: {}", why);
        }
        // Reset the message tracker every second
        if self.message_tracker.1.elapsed().as_secs_f32() > 1.0 {
            self.message_tracker = (0, Instant::now());
        }
        self.message_tracker.0 = self.message_tracker.0.saturating_add(1);
        if let Some(last) = self.messages.last_mut() {
            if last.message == msg.message {
                last.amount = last.amount.saturating_add(1);
                // If we're getting logs too fast, queue up an emit so the UI isn't sent a bazillion updates
                if self.message_tracker.0 >= Self::LOG_LIMIT_PER_SECOND {
                    if self.queued_emits.is_empty() {
                        self.emit_behind(true);
                    }
                    self.queued_emits.push(Some(self.get_count()));
                    self.queued_emits.push(None);
                } else {
                    self.emit_count_update(self.get_count());
                    self.emit_update();
                }
                return;
            }
        }
        if msg.message.message_type == SocketMessageType::Fatal {
            self.emit_fatal_alert(&msg);
        }
        let msg_type = msg.message.message_type.clone();
        if (self.active_filter.is_none() && self.active_search.is_empty())
            || (self.active_filter == Some(msg_type)
                && matches_query(&msg.message, &self.active_search))
        {
            self.indices.push(self.messages.len());
        }
        self.messages.push(msg);
        // If we're getting logs too fast, queue up an emit so the UI isn't sent a bazillion updates
        if self.message_tracker.0 >= Self::LOG_LIMIT_PER_SECOND {
            if self.queued_emits.is_empty() {
                self.emit_behind(true);
            }
            self.queued_emits.push(None);
        } else {
            self.eval_indices();
            self.emit_update();
        }
        // Process the emit queue semi-regularly
        if self.message_tracker.1.elapsed().as_secs() >= Self::LOG_TIME_UNTIL_FORCED_EMIT_SEC {
            self.process_emit_queue();
        }
    }

    pub fn clear(&mut self) {
        // First make the UI not render any rows to avoid errors
        self.indices.clear();
        self.emit_update();
        // Then actually clear our internal list
        self.messages.clear();
        self.emit_update();
    }
}

impl Drop for LogData {
    fn drop(&mut self) {
        if let Err(why) = self.writer.flush() {
            error!("Couldn't flush log file: {}", why);
        }
    }
}

pub async fn make_log_window(handle: &AppHandle) -> Result<Window> {
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let log_window = WindowBuilder::new(
        handle,
        format!("game-{epoch}"),
        tauri::WindowUrl::App("/logs/index.html".parse()?),
    );
    let window = log_window
        .center()
        .title("Game Logs")
        .min_inner_size(450.0, 450.0)
        .enable_clipboard_access()
        .build()?;
    Ok(window)
}

pub fn show_warnings(window: &Window, local_db: &LocalDatabase, config: &Config) -> Result<Config> {
    let warnings = get_warnings(
        local_db.active().collect(),
        config.viewed_alerts.iter().map(|s| s.as_str()).collect(),
    );
    let mut config = config.clone();
    for (unique_name, warning) in warnings {
        dialog::blocking::message(Some(window), &warning.title, &warning.body);
        config.set_warning_shown(unique_name);
    }
    Ok(config)
}

pub fn write_log(writer: &mut BufWriter<File>, msg: &SocketMessage) -> Result<()> {
    writeln!(
        writer,
        "[{}][{}][{:?}] {}",
        msg.sender_name.as_ref().unwrap_or(&"Unknown".to_string()),
        msg.sender_type.as_ref().unwrap_or(&"Unknown".to_string()),
        msg.message_type,
        msg.message
    )?;
    writer.flush()?;
    Ok(())
}
