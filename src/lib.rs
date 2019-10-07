pub mod method;
pub mod request;
pub mod response;
use method::Method;
use request::{parse_request_line, Request};
use response::*;

use log::{error, info};
use std::error::Error;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::{error, fmt};

#[derive(Debug)]
struct RequestLineNotFound;

impl fmt::Display for RequestLineNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request-Line not found")
    }
}

impl From<io::Error> for RequestLineNotFound {
    fn from(error: io::Error) -> Self {
        RequestLineNotFound
    }
}

impl error::Error for RequestLineNotFound {}

#[allow(clippy::unused_io_amount)]
pub fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![0u8; 1024];

    // writes stream into buffer
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().ok_or(RequestLineNotFound)?;
    info!("Request-Line: {}", &request_line);

    // Get response from request_line
    let response = match parse_request_line(&request_line) {
        Ok(request) => response(&request),
        Err(e) => {
            error!("Bad request: {}", e);
            Err(e)
        }
    };

    match response {
        Ok(mut response) => {
            info!("Response: {}", response);
            stream.write(&response.format_response())?;
            stream.flush()?;
        }
        Err(e) => error!("Response error: {}", e),
    }

    Ok(())
}
