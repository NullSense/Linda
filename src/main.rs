#[macro_use]
extern crate log;
extern crate simple_logger;

use std::fmt;
use std::io::{Error, Read};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

/// Request-Line = Method SP Request-URI SP HTTP-Version CRLF
struct Request<'a> {
    method: &'a str,
    uri: &'a str,
    http_version: &'a str,
}

impl<'a> fmt::Display for Request<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}\r\n", self.method, self.uri, self.http_version)
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    // 512 bytes is enough for a toy HTTP server
    let mut buffer = [0; 512];

    // writes stream into buffer
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap();

    match parse_request_line(&request_line) {
        Ok(request) => {
            info!("\n{}", request);
        }
        Err(()) => error!("Badly formatted request: {}", &request_line),
    }

    Ok(())
}

fn parse_request_line(request: &str) -> Result<Request, ()> {
    let mut parts = request.split_whitespace();

    let method = parts.next().ok_or(())?;
    // We only accept GET requests
    if !method.contains("GET") {
        return Err(());
    }

    let uri = parts.next().ok_or(())?;
    if !Path::new(uri).exists() {
        return Err(());
    }

    let http_version = parts.next().ok_or(())?;

    Ok(Request {
        method,
        uri,
        http_version,
    })
}

fn main() {
    simple_logger::init().unwrap();
    info!("Starting server...");

    let ip = "127.0.0.1:8594";

    let listener = TcpListener::bind(ip).expect("Unable to create listener.");
    info!("Server started on: {}{}", "http://", ip);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match handle_connection(stream) {
                Ok(_) => (),
                Err(e) => error!("Error handling connection: {}", e),
            },
            Err(e) => error!("Connection failed: {}", e),
        }
    }
}
