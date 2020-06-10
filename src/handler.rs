use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::TcpStream;

#[derive(Serialize)]
struct Message {
    data: String,
    len: usize,
}

pub fn handle(stream: TcpStream) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = BufWriter::new(stream.try_clone().unwrap());

    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line);
        match len {
            Err(err) => {
                println!("reader failed with {}", err);
                break;
            }
            Ok(t) => {
                println!("Read {} bytes: {}", t, line);
                let msg = Message { data: line, len: t };

                serde_json::to_writer(&mut writer, &msg).expect("failed to write into client");
                writer.flush().expect("failed to flush buffer");

                // is EOF detected?
                if t == 0 {
                    break;
                }
            }
        }
    }
}
