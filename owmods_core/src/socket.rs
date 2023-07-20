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
    /// To get it from a number use [serde_json::from_str] instead.
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    pub fn make_internal(message: &str, message_type: SocketMessageType) -> Self {
        Self {
            message: message.to_string(),
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
    const CLIENT_CONNECTED: &'static str = "====== Client Connected To Console ======";
    const CLIENT_DISCONNECTED: &'static str = "====== Client Disconnected From Console ======";

    /// Create and bind a log server to the given port, pass port 0 to auto-assign.
    /// **IMPORTANT:** If you pass port 0 make sure to get the port after binding. Otherwise the port you have and the port the server is bound to won't match.
    ///
    /// ## Returns
    ///
    /// A new log server that's bound to the given port, **but not ready to listen to** logs.
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

    // Give a log to the tx channel
    async fn yield_log(tx: &LogServerSender, message: SocketMessage) {
        let res = tx.send(message).await;
        if let Err(why) = res {
            error!("Couldn't Yield Log: {why:?}")
        }
    }

    // Loop that runs when a client connects to the server
    // Handles all messages from the client
    // Returns true if the client sent a quit message
    async fn client_loop(mut stream: TcpStream, tx: &LogServerSender) -> bool {
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
                            &format!("Invalid Log From Game Received: {:?}", why),
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

    // Loop that runs on start, listens for clients to connect
    // Makes a new client_loop for each client
    async fn server_loop(
        &self,
        tx: &LogServerSender,
        shutdown_sender: mpsc::Sender<()>,
        disconnect_on_quit: bool,
    ) {
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
                                Self::CLIENT_CONNECTED,
                                SocketMessageType::Info,
                            ),
                        )
                        .await;

                        let quit_received = Self::client_loop(stream, &tx2).await;

                        if quit_received && disconnect_on_quit {
                            shutdown_sender2.send(()).await.ok();
                        }

                        Self::yield_log(
                            &tx2,
                            SocketMessage::make_internal(
                                Self::CLIENT_DISCONNECTED,
                                SocketMessageType::Info,
                            ),
                        )
                        .await;
                    });
                }
                Err(why) => {
                    Self::yield_log(
                        tx,
                        SocketMessage::make_internal(
                            &format!("Client Connection Failure! {why:?}"),
                            SocketMessageType::Error,
                        ),
                    )
                    .await;
                }
            }
        }
    }

    /// Listen to this server for any logs from the game.
    ///
    /// - tx will send [SocketMessage]s from the game
    /// - disconnect_on_quit will make the server stop listening if the game sends a [SocketMessageType::Quit] message
    ///
    pub async fn listen(&self, tx: LogServerSender, disconnect_on_quit: bool) -> Result<()> {
        Self::yield_log(
            &tx,
            SocketMessage::make_internal(
                &format!("Ready to receive game logs on port {}!", self.port),
                SocketMessageType::Info,
            ),
        )
        .await;

        // Make a channel to listen for a shutdown message
        let (shutdown_sender, mut shutdown_receiver) = mpsc::channel::<()>(2);

        // Run the server loop, but also listen for a shutdown message
        tokio::select! {
            _ = async {
                let shutdown_sender = &shutdown_sender.clone();
                self.server_loop(&tx, shutdown_sender.clone(), disconnect_on_quit).await;
            } => {},
            _ = shutdown_receiver.recv() => info!("Quit Message Received")
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use tokio::{
        io::{AsyncWriteExt, BufWriter},
        net::TcpStream,
        sync::mpsc,
    };

    use super::*;

    struct TestLogServer {
        pub server: LogServer,
        pub port: u16,
        pub logs: Vec<SocketMessage>,
    }

    impl TestLogServer {
        pub async fn new() -> Self {
            let server = LogServer::new(0).await.unwrap();
            let port = server.port;
            Self {
                server,
                port,
                logs: vec![],
            }
        }

        pub async fn listen(&mut self, disconnect_on_quit: bool) {
            let (tx, mut rx) = mpsc::channel(32);
            tokio::join!(self.server.listen(tx, disconnect_on_quit), async {
                while let Some(msg) = rx.recv().await {
                    self.logs.push(msg);
                }
            })
            .0
            .unwrap();
        }

        pub fn assert_logs(&self, expected: Vec<SocketMessage>) {
            assert_eq!(self.logs.len(), expected.len());
            for (i, log) in self.logs.iter().enumerate() {
                if log != &expected[i] {
                    panic!(
                        "Log {} doesn't match expected!\nExpected: {:?}\nActual: {:?}",
                        i, expected[i], log
                    );
                }
            }
        }
    }

    struct MockGame {
        pub stream: TcpStream,
    }

    impl MockGame {
        pub async fn new(port: u16) -> Self {
            let stream = TcpStream::connect(format!("127.0.0.1:{}", port))
                .await
                .unwrap();
            Self { stream }
        }

        pub async fn send(&mut self, msg: SocketMessage) {
            let mut writer = BufWriter::new(&mut self.stream);
            let str = format!("{}\n", serde_json::to_string(&msg).unwrap());
            writer.write_all(str.as_bytes()).await.unwrap();
            writer.flush().await.unwrap();
            // Wait for the server to finish appending logs
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        pub fn make_test_msg(message: &str, message_type: SocketMessageType) -> SocketMessage {
            let mut msg = SocketMessage::make_internal(message, message_type);
            msg.sender_type = Some("TestClient".to_string());
            msg
        }

        pub async fn send_test_msg(&mut self, message: &str, message_type: SocketMessageType) {
            let msg = Self::make_test_msg(message, message_type);
            self.send(msg).await;
        }
    }

    #[test]
    fn test_log_server() {
        tokio_test::block_on(async {
            let mut server = TestLogServer::new().await;

            let port = server.port;

            tokio::join!(server.listen(true), async move {
                let mut game = MockGame::new(port).await;
                game.send_test_msg("Test Message", SocketMessageType::Info)
                    .await;
                game.send_test_msg("Success!", SocketMessageType::Success)
                    .await;
                game.send_test_msg("", SocketMessageType::Quit).await;
                // Wait for the server to finish appending logs
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            });

            let expected = vec![
                SocketMessage::make_internal(
                    &format!("Ready to receive game logs on port {}!", server.port),
                    SocketMessageType::Info,
                ),
                SocketMessage::make_internal(LogServer::CLIENT_CONNECTED, SocketMessageType::Info),
                MockGame::make_test_msg("Test Message", SocketMessageType::Info),
                MockGame::make_test_msg("Success!", SocketMessageType::Success),
                SocketMessage::make_internal(
                    LogServer::CLIENT_DISCONNECTED,
                    SocketMessageType::Info,
                ),
            ];

            server.assert_logs(expected);
        });
    }

    #[test]
    fn test_log_server_no_disconnect_on_quit() {
        tokio_test::block_on(async {
            let mut server = TestLogServer::new().await;

            let port = server.port;

            tokio::select!(_ = server.listen(false) => panic!("Other should've completed first!"), _ = async move {
                let mut game = MockGame::new(port).await;
                game.send_test_msg("Test Message", SocketMessageType::Info)
                    .await;
                game.send_test_msg("Success!", SocketMessageType::Success)
                    .await;
                game.send_test_msg("", SocketMessageType::Quit).await;
                let mut game2 = MockGame::new(port).await;
                game2
                    .send_test_msg("Test Message", SocketMessageType::Info)
                    .await;
                game2
                    .send_test_msg("Warning!", SocketMessageType::Warning)
                    .await;
                game2.send_test_msg("", SocketMessageType::Quit).await;
            } => {});

            let expected = vec![
                SocketMessage::make_internal(
                    &format!("Ready to receive game logs on port {}!", server.port),
                    SocketMessageType::Info,
                ),
                SocketMessage::make_internal(LogServer::CLIENT_CONNECTED, SocketMessageType::Info),
                MockGame::make_test_msg("Test Message", SocketMessageType::Info),
                MockGame::make_test_msg("Success!", SocketMessageType::Success),
                SocketMessage::make_internal(
                    LogServer::CLIENT_DISCONNECTED,
                    SocketMessageType::Info,
                ),
                SocketMessage::make_internal(LogServer::CLIENT_CONNECTED, SocketMessageType::Info),
                MockGame::make_test_msg("Test Message", SocketMessageType::Info),
                MockGame::make_test_msg("Warning!", SocketMessageType::Warning),
                SocketMessage::make_internal(
                    LogServer::CLIENT_DISCONNECTED,
                    SocketMessageType::Info,
                ),
            ];

            server.assert_logs(expected);
        });
    }

    #[test]
    fn test_log_server_multi_client() {
        tokio_test::block_on(async {
            let mut server = TestLogServer::new().await;

            let port = server.port;

            tokio::join!(server.listen(true), async move {
                let mut game = MockGame::new(port).await;
                game.send_test_msg("Test Message", SocketMessageType::Info)
                    .await;
                game.send_test_msg("Success!", SocketMessageType::Success)
                    .await;
                let mut game2 = MockGame::new(port).await;
                game2
                    .send_test_msg("Test Message", SocketMessageType::Info)
                    .await;
                game2
                    .send_test_msg("Warning!", SocketMessageType::Warning)
                    .await;
                game.send_test_msg("Other Info", SocketMessageType::Info)
                    .await;
                game2
                    .send_test_msg("Other Warning", SocketMessageType::Warning)
                    .await;
                game2.send_test_msg("", SocketMessageType::Quit).await;
                game.send_test_msg("", SocketMessageType::Quit).await;
            });

            let expected = vec![
                SocketMessage::make_internal(
                    &format!("Ready to receive game logs on port {}!", server.port),
                    SocketMessageType::Info,
                ),
                SocketMessage::make_internal(LogServer::CLIENT_CONNECTED, SocketMessageType::Info),
                MockGame::make_test_msg("Test Message", SocketMessageType::Info),
                MockGame::make_test_msg("Success!", SocketMessageType::Success),
                SocketMessage::make_internal(LogServer::CLIENT_CONNECTED, SocketMessageType::Info),
                MockGame::make_test_msg("Test Message", SocketMessageType::Info),
                MockGame::make_test_msg("Warning!", SocketMessageType::Warning),
                MockGame::make_test_msg("Other Info", SocketMessageType::Info),
                MockGame::make_test_msg("Other Warning", SocketMessageType::Warning),
                SocketMessage::make_internal(
                    LogServer::CLIENT_DISCONNECTED,
                    SocketMessageType::Info,
                ),
                SocketMessage::make_internal(
                    LogServer::CLIENT_DISCONNECTED,
                    SocketMessageType::Info,
                ),
            ];

            server.assert_logs(expected);
        });
    }
}
