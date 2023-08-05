use std::{
    fs::File,
    io::{BufWriter, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use owmods_core::{
    alerts::get_warnings,
    config::Config,
    db::LocalDatabase,
    search::matches_query,
    socket::{SocketMessage, SocketMessageType},
};
use serde::{Deserialize, Serialize};
use tauri::{api::dialog, AppHandle, Window, WindowBuilder};
use time::{macros::format_description, OffsetDateTime};
use typeshare::typeshare;

use crate::LogPort;

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

pub fn get_logs_indices(
    lines: &[GameMessage],
    filter_type: Option<SocketMessageType>,
    search: &str,
) -> Result<Vec<usize>> {
    let mut indices = Vec::with_capacity(lines.len());
    let search = search.to_ascii_lowercase();
    for (line_number, line) in lines.iter().enumerate() {
        let mut include = true;
        if filter_type.is_some() || !search.trim().is_empty() {
            let matches_type = filter_type.is_none()
                || line.message.message_type == *filter_type.as_ref().unwrap();
            let matches_search = matches_query(&line.message, &search);
            include = matches_type && matches_search;
        }
        if include {
            indices.push(line_number);
        }
    }
    Ok(indices)
}
