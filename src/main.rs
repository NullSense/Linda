#[macro_use]
extern crate log;
extern crate simple_logger;

use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{Read, Write};
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

fn parse_request_line(request: &str) -> Result<Request, Box<dyn Error>> {
    let mut parts = request.split_whitespace();

    let method = parts.next().ok_or("Method not specified")?;
    // We only accept GET requests
    if method != "GET" {
        Err("Unsupported method")?;
    }

    let uri = Path::new(parts.next().ok_or("URI not specified")?);
    let norm_uri = uri.to_str().expect("Invalid unicode!");

    const ROOT: &str = "/home/ongo/Programming/linda";

    if !Path::new(&format!("{}{}", ROOT, norm_uri)).exists() {
        Err("Requested resource does not exist")?;
    }

    let http_version = parts.next().ok_or("HTTP version not specified")?;
    if http_version != "HTTP/1.1" {
        Err("Unsupported HTTP version, use HTTP/1.1")?;
    }

    Ok(Request {
        method,
        uri,
        http_version,
    })
}

fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    // 512 bytes is enough for a toy HTTP server
    let mut buffer = [0; 512];

    // writes stream into buffer
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap();

    match parse_request_line(&request_line) {
        Ok(request) => {
            info!("Request: {}", &request);

            let contents = fs::read_to_string("index.html").unwrap();
            let response = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", contents);

            info!("Response: {}", &response);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => error!("Bad request: {}", e),
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
