use anyhow::{anyhow, Result};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use typeshare::typeshare;

pub type LogServerSender = mpsc::Sender<SocketMessage>;

/// Represents the type of message sent from the game
#[typeshare]
#[derive(Eq, PartialEq, Clone, Debug, Serialize_repr, Deserialize_repr)]
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

impl SocketMessageType {
    /// Parse a socket message type **from a string**.
    pub fn parse(str: &str) -> Result<Self> {
        match str {
            "Message" => Ok(Self::Message),
            "Error" => Ok(Self::Error),
            "Warning" => Ok(Self::Warning),
            "Info" => Ok(Self::Info),
            "Success" => Ok(Self::Success),
            "Quit" => Ok(Self::Quit),
            "Fatal" => Ok(Self::Fatal),
            "Debug" => Ok(Self::Debug),
            _ => Err(anyhow!("Invalid Variant!")),
        }
    }
}

/// Represents a message sent from the game
#[typeshare]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SocketMessage {
    pub sender_name: Option<String>,
    pub sender_type: Option<String>,
    pub message: String,
    #[serde(alias = "type")]
    pub message_type: SocketMessageType,
}

impl SocketMessage {
    /// Make an internal SocketMessage to send to the server for debugging/information.
    ///
    /// ## Returns
    ///
    /// A SocketMessage with sender_name set to "Manager" and sender_type set to "Log Server".
    ///
    pub fn make_internal(message: String, message_type: SocketMessageType) -> Self {
        Self {
            message,
            message_type,
            sender_name: Some("Manager".to_string()),
            sender_type: Some("LogServer".to_string()),
        }
    }
}

/// A server used to listen to logs from the game
pub struct LogServer {
    pub port: u16,
    listener: TcpListener,
}

impl LogServer {
    /// Create and bind a log server to the given port, pass port 0 to auto-assign.
    /// **IMPORTANT:** If you pass port 0 make sure to get the port after binding. Otherwise the port you have and the port the server is bound to won't match.
    ///
    /// ## Returns
    ///
    /// A new log server that's bound to the given port, **but not ready to listen to logs**.
    ///
    /// ## Errors
    ///
    /// If we can't bind to the given port
    ///
    pub async fn new(port: u16) -> Result<Self> {
        let address = format!("127.0.0.1:{}", port);
        let listener = TcpListener::bind(&address).await?;
        // Get the actual port we bound to in case the user passed port 0
        let port = listener.local_addr()?.port();
        Ok(Self { port, listener })
    }

    async fn accept(mut stream: TcpStream, tx: &LogServerSender) -> bool {
        let mut reader = BufReader::new(&mut stream);
        let mut body = String::new();
        let mut flag = false;
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
                            flag = true;
                            break;
                        }
                        _ => {
                            Self::yield_log(tx, message).await;
                        }
                    };
                }
                Err(why) => {
                    Self::yield_log(
                        tx,
                        SocketMessage::make_internal(
                            format!("Invalid Log From Game Received: {:?}", why),
                            SocketMessageType::Error,
                        ),
                    )
                    .await;
                }
            }
            body.clear();
        }
        flag
    }

    async fn yield_log(tx: &LogServerSender, message: SocketMessage) {
        let res = tx.send(message).await;
        if let Err(why) = res {
            error!("Couldn't Yield Log: {why:?}")
        }
    }

    /// Listen to this server for any logs from the game.
    ///
    /// - tx will send [SocketMessage]s from the game
    /// - disconnect_on_quit will make the server stop listening if the game sends a [SocketMessageType::Quit] message
    ///
    pub async fn listen(self, tx: LogServerSender, disconnect_on_quit: bool) -> Result<()> {
        Self::yield_log(
            &tx,
            SocketMessage::make_internal(
                format!("Ready to receive game logs on port {}!", self.port),
                SocketMessageType::Info,
            ),
        )
        .await;

        let (shutdown_sender, mut shutdown_receiver) = mpsc::channel::<()>(2);

        tokio::select! {
            _ = async {
                let shutdown_sender = &shutdown_sender.clone();
                loop {
                    let stream = self.listener.accept().await;
                    match stream {
                        Ok((stream, _)) => {
                            let tx2 = tx.clone();
                            let shutdown_sender2 = shutdown_sender.clone();
                            tokio::spawn(async move {
                                Self::yield_log(
                                    &tx2,
                                    SocketMessage::make_internal(
                                        "====== Client Connected To Console ======".to_string(),
                                        SocketMessageType::Info,
                                    ),
                                )
                                .await;

                                let quit_received = Self::accept(stream, &tx2).await;

                                if quit_received && disconnect_on_quit {
                                    shutdown_sender2.send(()).await.ok();
                                }

                                Self::yield_log(
                                    &tx2,
                                    SocketMessage::make_internal(
                                        "====== Client Disconnected From Console ======".to_string(),
                                        SocketMessageType::Info,
                                    ),
                                )
                                .await;
                            });
                        }
                        Err(why) => {
                            Self::yield_log(
                                &tx,
                                SocketMessage::make_internal(
                                    format!("Client Connection Failure! {why:?}"),
                                    SocketMessageType::Error,
                                ),
                            )
                            .await;
                        }
                    }
                }
            } => {},
            _ = shutdown_receiver.recv() => info!("Quit Message Received")
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::sync::Mutex;

    use tokio::{
        io::{AsyncWriteExt, BufWriter},
        net::TcpStream,
        sync::mpsc,
    };

    use super::*;

    fn make_test_msg(message: &str, message_type: SocketMessageType) -> SocketMessage {
        let mut msg = SocketMessage::make_internal(message.to_string(), message_type);
        msg.sender_type = Some("TestClient".to_string());
        msg
    }

    async fn write_msg(writer: &mut BufWriter<TcpStream>, msg: SocketMessage) {
        let str = format!("{}\n", serde_json::to_string(&msg).unwrap());
        writer.write_all(str.as_bytes()).await.unwrap();
        writer.flush().await.unwrap();
    }

    #[test]
    fn test_log_server() {
        tokio_test::block_on(async {
            let server = LogServer::new(0).await.unwrap();
            let port = server.port;
            let count_ref = Mutex::new(0);
            let test_fn = async move {
                let client = TcpStream::connect(format!("127.0.0.1:{}", port))
                    .await
                    .unwrap();
                let mut writer = BufWriter::new(client);
                write_msg(
                    &mut writer,
                    make_test_msg("Test Message", SocketMessageType::Info),
                )
                .await;
                write_msg(
                    &mut writer,
                    make_test_msg("Success!", SocketMessageType::Success),
                )
                .await;
                write_msg(&mut writer, make_test_msg("", SocketMessageType::Quit)).await;
                writer.shutdown().await.unwrap();
            };
            let (tx, mut rx) = mpsc::channel(32);

            tokio::spawn(async {
                server.listen(tx, true).await.unwrap();
            });

            tokio::spawn(test_fn);

            while let Some(msg) = rx.recv().await {
                let mut counter = count_ref.lock().unwrap();
                match *counter {
                    0 => {
                        assert!(matches!(msg.message_type, SocketMessageType::Info));
                        assert_eq!(
                            msg.message,
                            format!("Ready to receive game logs on port {}!", port)
                        );
                    }
                    1 => {
                        assert_eq!(msg.message, "====== Client Connected To Console ======");
                    }
                    2 => {
                        assert_eq!(msg.message, "Test Message");
                        assert_eq!(msg.sender_name.as_ref().unwrap(), "Manager");
                        assert_eq!(msg.sender_type.as_ref().unwrap(), "TestClient");
                    }
                    3 => {
                        assert_eq!(msg.message, "Success!");
                        assert!(matches!(msg.message_type, SocketMessageType::Success));
                    }
                    4 => {
                        assert_eq!(
                            msg.message,
                            "====== Client Disconnected From Console ======"
                        );
                    }
                    _ => {
                        panic!("Too many calls!");
                    }
                }
                *counter += 1;
            }

            assert_eq!(*count_ref.lock().unwrap(), 5);
        });
    }
}
