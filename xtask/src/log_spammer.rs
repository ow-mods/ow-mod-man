use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;

use anyhow::Result;

pub fn spam_logs(port: u16) -> Result<()> {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{port}"))?;
    let mut i = 1;
    // Hard limit to save your computer
    while i < 20000 {
        println!("Message {i}, {}", i == usize::MAX);
        // I just want to easily change stuff when testing so im leaving the format here
        #[allow(clippy::useless_format)]
        let msg = format!("{{\"type\": 0, \"message\": \"Line {i}\", \"senderName\": \"xtask\", \"senderType\": \"log_spammer\"}}\n");
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
    println!("Hard Limit Reached");
    Ok(())
}
