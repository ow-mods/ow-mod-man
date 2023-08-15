use std::{io::Write, net::TcpStream};

use anyhow::Result;

pub fn log_client() -> Result<()> {
    let port = std::env::args().nth(2).expect("Missing Port");
    let mut stream = TcpStream::connect(format!("127.0.0.1:{port}"))?;
    loop {
        let mut input = String::new();
        let user_entered = std::io::stdin().read_line(&mut input)?;
        if user_entered == 0 {
            break;
        }
        let message = format!(
            "{{\"type\": 0, \"message\": \"{}\", \"senderName\": \"xtask\", \"senderType\": \"log_client\"}}\n",
            input.trim()
        );
        stream.write_all(message.as_bytes())?;
    }
    Ok(())
}
