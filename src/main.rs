#[macro_use]
extern crate log;
extern crate simple_logger;

use std::fmt;
use std::fs;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

/// Request-Line = Method SP Request-URI SP HTTP-Version CRLF
struct Request<'a> {
    method: &'a str,
    uri: &'a Path,
    http_version: &'a str,
}

impl<'a> fmt::Display for Request<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}\r\n",
            self.method,
            self.uri.display(),
            self.http_version
        )
    }
}

fn parse_request_line(request: &str) -> Result<Request, ()> {
    let mut parts = request.split_whitespace();

    let method = parts.next().ok_or(())?;
    // We only accept GET requests
    if method != "GET" {
        return Err(());
    }

    let uri = parts.next().ok_or(())?;
    let uri_path = Path::new(uri);
    if !uri_path.exists() {
        return Err(());
    }

    let http_version = parts.next().ok_or(())?;
    if http_version != "HTTP/1.1" {
        return Err(());
    }

    Ok(Request {
        method,
        uri: uri_path,
        http_version,
    })
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
            let contents = fs::read_to_string("index.html").unwrap();
            let response = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", contents);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(()) => error!("Badly formatted request: {}", &request_line),
    }

    Ok(())
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
