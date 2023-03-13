use anyhow::anyhow;
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
        // Get the actual port we bound to in case the user passed port 0
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

#[cfg(test)]
mod tests {

    use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};
    use std::sync::Mutex;

    use futures::try_join;
    use tokio::io::{BufWriter, AsyncWriteExt};
    use tokio::net::TcpStream;

    use super::*;

    fn make_test_msg(message: &str, message_type: SocketMessageType) -> SocketMessage {
        let mut msg = SocketMessage::make_internal(message.to_string(), message_type);
        msg.sender_type = Some("TestClient".to_string());
        msg
    }

    async fn write_msg(writer: &mut BufWriter<TcpStream>, msg: SocketMessage) {
        let str = format!("{}\n", serde_json::to_string(&msg).unwrap());
        writer.write(str.as_bytes()).await.unwrap();
        writer.flush().await.unwrap();
    }

    #[test]
    fn test_log_server() {
        tokio_test::block_on(async {
            let server = LogServer::new(0).await.unwrap();
            let port = server.port.clone();
            let count_ref = Mutex::new(0);
            let handle_log = |msg: &SocketMessage, _: &str| {
                let mut counter = count_ref.lock().unwrap();
                match *counter {
                    0 => {
                        assert!(matches!(msg.message_type, SocketMessageType::Info));
                        assert_eq!(msg.message, format!("Ready to receive game logs on port {}!", port));
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
                        assert_eq!(msg.message, "====== Client Disconnected From Console ======");
                    }
                    _ => { panic!("Too many calls!"); }
                }
                *counter += 1;
            };
            let test_fn = async {
                let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port));
                let client = TcpStream::connect(addr).await.unwrap();
                let mut writer = BufWriter::new(client);
                write_msg(&mut writer, make_test_msg("Test Message", SocketMessageType::Info)).await;
                write_msg(&mut writer, make_test_msg("Success!", SocketMessageType::Success)).await;
                write_msg(&mut writer, make_test_msg("", SocketMessageType::Quit)).await;
                writer.shutdown().await.unwrap();
                Ok(())
            };
            try_join!(
                server.listen(handle_log, true),
                test_fn
            ).unwrap();
            assert_eq!(*count_ref.lock().unwrap(), 5);
        });
    }

}
