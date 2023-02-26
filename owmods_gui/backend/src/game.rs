use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use anyhow::anyhow;
use anyhow::Result;
use owmods_core::socket::SocketMessage;
use tauri::{AppHandle, Window, WindowBuilder};
use tempdir::TempDir;

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

pub fn write_log(log_dir: &TempDir, msg: SocketMessage) -> Result<()> {
    let log_file = File::options()
        .create(true)
        .append(true)
        .open(log_dir.path().join("log.txt"))?;
    let mut buffer = BufWriter::new(log_file);
    writeln!(buffer, "{}", serde_json::to_string(&msg)?)?;
    Ok(())
}

pub fn get_log_len(log_dir: &TempDir) -> Result<usize> {
    let log_file = File::open(log_dir.path().join("log.txt"))?;
    let buffer = BufReader::new(log_file);
    Ok(buffer.lines().count() - 1)
}

pub fn get_log_from_line(log_dir: &TempDir, line: usize) -> Result<SocketMessage> {
    let log_file = File::open(log_dir.path().join("log.txt"))?;
    let buffer = BufReader::new(log_file);
    let line = buffer
        .lines()
        .nth(line)
        .ok_or_else(|| anyhow!("Line Not In File"))??;
    let msg = serde_json::from_str::<SocketMessage>(&line)?;
    Ok(msg)
}
