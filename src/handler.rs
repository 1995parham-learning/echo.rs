use bufstream::BufStream;
use serde_derive::Serialize;
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Serialize)]
struct Message {
    data: String,
    len: usize,
}

pub fn handle(stream: TcpStream) {
    let mut bstream = BufStream::new(stream);

    loop {
        let mut line = String::new();
        match bstream.read_line(&mut line) {
            Err(err) => {
                println!("reader failed with {}", err);
                break;
            }
            Ok(len) => {
                println!("Read {} bytes: {}", len, line);
                let msg = Message { data: line, len };

                serde_json::to_writer(&mut bstream, &msg).expect("failed to write into client");
                bstream.flush().expect("failed to flush buffer");

                // is EOF detected?
                if len == 0 {
                    break;
                }
            }
        }
    }
}
