mod handler;

use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind(
        std::env::args()
            .nth(1)
            .unwrap_or_else(|| "127.0.0.1:1378".to_string()),
    )
    .expect("listening failed");

    println!("Listening on {:?}", listener);

    // accept connections and process them serially
    for stream in listener.incoming() {
        thread::spawn(move || handler::handle(stream.expect("connection failed")));
    }
}
