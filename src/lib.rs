use log::{error, info};
use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;

pub mod method;
pub mod request;
pub use method::Method;
pub use request::Request;

fn parse_request_line<'a>(request: &'a str) -> Result<Request, Box<dyn Error + 'a>> {
    let mut parts = request.split_whitespace();

    let method = parts.next().ok_or("Method not specified")?;
    let uri = parts.next().ok_or("URI not specified")?;
    let http_version = parts.next().ok_or("HTTP version not specified")?;

    let mut request = Request::new();
    request.method(method)?.uri(uri)?.version(http_version)?;

    Ok(request)
}

#[allow(clippy::unused_io_amount)]
pub fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    // 512 bytes is enough for a toy HTTP server
    let mut buffer = [0u8; 1024];

    // writes stream into buffer
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap();

    match parse_request_line(&request_line) {
        Ok(request) => {
            info!("Request: {}", &request);

            let contents = fs::read_to_string("index.html").unwrap();
            let response = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", contents);

            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => error!("Bad request: {}", e),
    }

    Ok(())
}
