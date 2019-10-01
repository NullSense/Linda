#[macro_use]
extern crate log;
extern crate simple_logger;

use std::net::TcpListener;

fn main() {
    simple_logger::init().unwrap();
    info!("Starting server...");

    let ip = "127.0.0.1:8594";

    let listener = TcpListener::bind(ip).expect("Unable to create listener.");
    info!("Server started on: {}{}", "http://", ip);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => info!("New client!"),
            Err(e) => error!("Connection failed: {}", e),
        }
    }
}
