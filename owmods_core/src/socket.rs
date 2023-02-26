use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpListener,
};
use typeshare::typeshare;

#[typeshare]
#[derive(Clone, Debug, Serialize_repr, Deserialize_repr)]
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

#[typeshare]
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SocketMessage {
    pub sender_name: Option<String>,
    pub sender_type: Option<String>,
    pub message: String,
    #[serde(alias = "type")]
    pub message_type: SocketMessageType,
}

impl SocketMessage {
    pub fn make_internal(message: String, message_type: SocketMessageType) -> Self {
        Self {
            message,
            message_type,
            sender_name: Some("Manager".to_string()),
            sender_type: Some("Log Server".to_string()),
        }
    }
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
        f: impl Fn(&SocketMessage, &str),
        disconnect_on_quit: bool,
    ) -> Result<()> {
        let target = format!("game:{}", self.port);
        f(
            &SocketMessage::make_internal(
                format!("Ready to receive game logs on port {}!", self.port),
                SocketMessageType::Info,
            ),
            &target,
        );
        let mut keep_going = true;
        while keep_going {
            let stream = self.listener.accept().await;
            f(
                &SocketMessage::make_internal(
                    "====== Client Connected To Console ======".to_string(),
                    SocketMessageType::Info,
                ),
                &target,
            );
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
                            f(
                                &SocketMessage::make_internal(
                                    format!("Invalid Log From Game Received: {:?}", why),
                                    SocketMessageType::Error,
                                ),
                                &target,
                            );
                        }
                    }
                    body.clear();
                }
            } else {
                f(
                    &SocketMessage::make_internal(
                        "Invalid Log Received!".to_string(),
                        SocketMessageType::Error,
                    ),
                    &target,
                );
            }
            f(
                &SocketMessage::make_internal(
                    "====== Client Disconnected From Console ======".to_string(),
                    SocketMessageType::Info,
                ),
                &target,
            );
        }
        Ok(())
    }
}
