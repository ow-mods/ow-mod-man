use std::{
    fs::File,
    io::{BufWriter, Write},
};

use anyhow::Result;
use owmods_core::{
    alerts::get_warnings,
    config::Config,
    db::LocalDatabase,
    socket::{SocketMessage, SocketMessageType},
};
use serde::Serialize;
use tauri::{api::dialog, AppHandle, Window, WindowBuilder};
use typeshare::typeshare;

use crate::LogPort;

#[typeshare]
#[derive(Serialize, Clone, Debug)]
pub struct GameMessage {
    port: LogPort,
    message: SocketMessage,
}

impl GameMessage {
    pub fn new(port: LogPort, message: SocketMessage) -> Self {
        Self { port, message }
    }
}

pub async fn make_log_window(handle: &AppHandle) -> Result<Window> {
    let log_window = WindowBuilder::new(
        handle,
        "game",
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
        local_db.mods.values().collect(),
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
    lines: &Vec<GameMessage>,
    filter_port: Option<LogPort>,
    filter_type: Option<SocketMessageType>,
) -> Result<Vec<usize>> {
    let mut indices: Vec<usize> = vec![];
    if filter_port.is_some() || filter_type.is_some() {
        for (line_number, line) in lines.iter().enumerate() {
            let matches_port = filter_port.is_none() || line.port == *filter_port.as_ref().unwrap();
            let matches_type = filter_type.is_none()
                || line.message.message_type == *filter_type.as_ref().unwrap();
            if matches_port && matches_type {
                indices.push(line_number);
            }
        }
    } else {
        indices = (0..lines.len()).collect();
    }
    Ok(indices)
}
