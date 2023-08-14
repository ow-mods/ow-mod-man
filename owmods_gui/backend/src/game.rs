use std::{
    fs::File,
    io::{BufWriter, Write},
    time::{SystemTime, UNIX_EPOCH},
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
    events::{CustomEventEmitterAll, Event, LogLineCountUpdatePayload},
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
    pub fn new(port: LogPort, message: SocketMessage) -> Self {
        let now = OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc());
        Self {
            port,
            message,
            amount: 1,
            timestamp: now
                .format(format_description!(
                    "[hour repr:12]:[minute]:[second] [period] (UTC[offset_hour sign:mandatory])"
                ))
                .unwrap_or("Unknown".to_string()),
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
}

impl LogData {
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
        })
    }

    fn emit_update(&self) {
        let res = self.app_handle.typed_emit_all(&Event::LogUpdate(self.port));
        if let Err(why) = res {
            error!("Couldn't Emit Game Log: {}", why)
        }
    }

    fn emit_count_update(&self) {
        let res =
            self.app_handle
                .typed_emit_all(&Event::LogLineCountUpdate(LogLineCountUpdatePayload {
                    port: self.port,
                    line: (self.messages.len() - 1).try_into().unwrap_or(u32::MAX),
                }));
        if let Err(why) = res {
            error!("Couldn't Emit Game Log: {}", why)
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
        if let Some(last) = self.messages.last_mut() {
            if last.message == msg.message {
                last.amount = last.amount.saturating_add(1);
                self.emit_count_update();
                self.emit_update();
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
        self.emit_update();
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.indices.clear();
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
