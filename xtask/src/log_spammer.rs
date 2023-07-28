use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;

use anyhow::Result;

pub fn spam_logs(port: u16) -> Result<()> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{port}"))?;
    let mut i = 1;
    loop {
        println!("Message {i}, {}", i == usize::MAX);
        let msg = "{\"type\": 0, \"message\": \"Line 1\\nLine 2\", \"senderName\": \"xtask\", \"senderType\": \"log_spammer\"}\n";
        stream.write_all(msg.as_bytes())?;
        i += 1;
        std::thread::sleep(Duration::from_secs_f32(
            std::env::args()
                .nth(3)
                .unwrap_or("0.01666666666666667".to_string())
                .parse()
                .unwrap(),
        ));
    }
}
