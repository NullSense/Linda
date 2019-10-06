pub mod method;
pub mod request;
pub mod response;
use method::Method;
use request::{parse_request_line, Request};
use response::*;

use log::{error, info};
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

#[allow(clippy::unused_io_amount)]
pub fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![0u8; 1024];

    // writes stream into buffer
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let request_line = request.lines().next().expect("Request line doesn't exist");
    info!("Request-Line: {}", &request_line);

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

            stream
                .write(&response.format_response())
                .expect("Couldn't write response");
            stream.flush().expect("Error flushing stream");
        }
        Err(e) => error!("{}", e),
    }

    Ok(())
}
