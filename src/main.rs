extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod handler;
mod settings;

use std::net::TcpListener;
use std::thread;

fn main() {
    let s = settings::Settings::new().unwrap();

    let listener =
        TcpListener::bind(format!("{}:{}", s.host(), s.port())).expect("listening failed");

    println!("Listening on {:?}", listener);

    // accept connections and process them serially
    for stream in listener.incoming() {
        thread::spawn(move || handler::handle(stream.expect("connection failed")));
    }
}
