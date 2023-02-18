use anyhow::Result;
use log::{error, info};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpListener,
};

#[derive(Debug, Deserialize_repr)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum SocketMessageType {
    Message = 0,
    Error = 1,
    Warning = 2,
    Info = 3,
    Success = 4,
    Quit = 5,
    Fatal = 6,
    Debug = 7,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocketMessage {
    pub sender_name: Option<String>,
    pub sender_type: Option<String>,
    pub message: String,
    #[serde(alias = "type")]
    pub message_type: SocketMessageType,
}

pub struct LogServer {
    pub port: u16,
    listener: TcpListener,
}

impl LogServer {
    pub async fn new(port: u16) -> Result<Self> {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&address).await?;
        // Get the actual port we bound too in case the user passed port 0
        let port = listener.local_addr()?.port();
        Ok(Self { port, listener })
    }

    pub async fn listen(
        self,
        f: &dyn Fn(&SocketMessage, &str),
        disconnect_on_quit: bool,
    ) -> Result<()> {
        let target = format!("game:{}", self.port);
        info!(
            target: &target,
            "Ready To Receive Game Logs On Port {}!", self.port
        );
        let mut keep_going = true;
        while keep_going {
            let stream = self.listener.accept().await;
            info!(target: &target, "===Client Attached To Console===");
            if let Ok((mut stream, _)) = stream {
                let mut reader = BufReader::new(&mut stream);
                let mut body = String::new();
                while let Ok(bytes_read) = reader.read_line(&mut body).await {
                    if bytes_read == 0 {
                        break;
                    }
                    if body.trim() == "" {
                        continue;
                    }
                    let message: Result<SocketMessage, _> = serde_json::from_str(body.trim());
                    match message {
                        Ok(message) => {
                            match message.message_type {
                                SocketMessageType::Quit => {
                                    info!(target: &target, "Quit Message Received");
                                    keep_going = !disconnect_on_quit;
                                    break;
                                }
                                _ => {
                                    f(&message, &target);
                                }
                            };
                        }
                        Err(why) => {
                            error!(target: &target, "Invalid Log From Game Sent: {:?}", why);
                        }
                    }
                    body = String::default();
                }
            } else {
                error!(target: &target, "Invalid Log From Game Sent!");
            }
            info!(target: &target, "===Client De-Attached From Console===");
        }
        Ok(())
    }
}
