pub mod method;
pub mod request;
pub mod response;
pub use method::Method;
pub use request::{parse_request_line, Request};
pub use response::*;

use log::{error, info};
use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;

#[allow(clippy::unused_io_amount)]
pub fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0u8; 1024];

    // writes stream into buffer
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().unwrap();

    info!("Request-Line: {}", &request_line);
    let response = match parse_request_line(&request_line) {
        Ok(request) => {
            let contents = fs::read_to_string("index.html").unwrap();
            let response = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", contents);

            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(e) => error!("Bad request: {}", e),
    };

    Ok(())
}
