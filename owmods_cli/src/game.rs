use anyhow::Result;
use log::{debug, error, info, warn};
use owmods_core::{
    config::Config,
    game::launch_game,
    socket::{LogServer, SocketMessage, SocketMessageType},
};
use tokio::try_join;

fn handle_game_log(message: &SocketMessage, target: &str) {
    let unknown = &"Unknown".to_string();
    let out_message = format!(
        "[{}::{}][{:?}] {}",
        message.sender_name.as_ref().unwrap_or(unknown),
        message.sender_type.as_ref().unwrap_or(unknown),
        message.message_type,
        message.message
    );
    match message.message_type {
        SocketMessageType::Message
        | SocketMessageType::Info
        | SocketMessageType::Success
        | SocketMessageType::Quit => {
            info!(target: target, "{}", out_message)
        }
        SocketMessageType::Error | SocketMessageType::Fatal => {
            error!(target: target, "{}", out_message)
        }
        SocketMessageType::Warning => warn!(target: target, "{}", out_message),
        SocketMessageType::Debug => debug!(target: target, "{}", out_message),
    }
}

pub async fn start_just_logs(port: &u16) -> Result<()> {
    let server = LogServer::new(*port).await?;
    server.listen(&handle_game_log, false).await?;
    Ok(())
}

pub async fn start_game(config: &Config, port: &u16) -> Result<()> {
    let server = LogServer::new(*port).await?;
    let port = server.port;
    try_join!(
        server.listen(&handle_game_log, true),
        launch_game(config, &port)
    )?;
    Ok(())
}
