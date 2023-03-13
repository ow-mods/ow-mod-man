use std::{
    fs::File,
    io::{BufWriter, Write},
};

use anyhow::Result;
use owmods_core::{
    alerts::{get_warnings, save_warning_shown},
    config::Config,
    db::LocalDatabase,
    socket::{SocketMessage, SocketMessageType},
};
use tauri::{api::dialog, AppHandle, Window, WindowBuilder};

pub async fn make_log_window(handle: &AppHandle, port: u16) -> Result<Window> {
    let label = format!("game-{port}");
    let log_window = WindowBuilder::new(
        handle,
        &label,
        tauri::WindowUrl::App("/logs/index.html".parse()?),
    );
    let window = log_window
        .center()
        .title(format!("Game Logs (Port {port})"))
        .min_inner_size(450.0, 450.0)
        .enable_clipboard_access()
        .build()?;
    Ok(window)
}

pub fn show_warnings(window: &Window, local_db: &LocalDatabase, config: &Config) -> Result<Config> {
    let warnings = get_warnings(local_db, config)?;
    let mut config = config.clone();
    for (unique_name, warning) in warnings {
        dialog::blocking::message(Some(window), &warning.title, &warning.body);
        config = save_warning_shown(unique_name, &config)?;
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
    lines: &Vec<SocketMessage>,
    filter_type: Option<SocketMessageType>,
) -> Result<Vec<usize>> {
    let mut indices: Vec<usize> = vec![];
    if let Some(filter_type) = filter_type {
        let mut line_number = 0;
        for line in lines {
            if line.message_type == filter_type {
                indices.push(line_number);
                line_number += 1;
            }
        }
    } else {
        indices = (0..lines.len()).collect();
    }
    Ok(indices)
}
