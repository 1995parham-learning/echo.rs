mod handler;

use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:1378").expect("listening failed");

    // accept connections and process them serially
    for stream in listener.incoming() {
        thread::spawn(move || handler::handle(stream.expect("connection failed")));
    }
}
