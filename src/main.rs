mod handler;

use clap::{App, Arg};
use std::net::TcpListener;
use std::thread;

fn main() {
    let matches = App::new("echo.rs says what you said")
        .author("Parham Alvani <parham.alvani@gmail.com>")
        .arg(
            Arg::with_name("listen")
                .short("l")
                .takes_value(true)
                .value_name("address")
                .help("listen address e.g. 127.0.0.1:1378"),
        )
        .get_matches();

    let listener = TcpListener::bind(matches.value_of("listen").unwrap_or("127.0.0.1:1378"))
        .expect("listening failed");

    println!("Listening on {:?}", listener);

    // accept connections and process them serially
    for stream in listener.incoming() {
        thread::spawn(move || handler::handle(stream.expect("connection failed")));
    }
}
