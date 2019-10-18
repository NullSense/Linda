pub mod method;
pub mod request;
pub mod response;
pub mod threadpool;

use method::Method;
use request::{get_request_line, parse_request_line, Request};
use response::response;

use log::{error, info};
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

/// Handles TcpStream connection
///
/// Writes requests coming from the TcpStream to a buffer
/// Writes out the response for the corresponding request
///
/// # Errors
///
/// Propagates errors up if:
/// * Was not able to read the stream into the buffer
/// * The request was invlid
/// * Was not able to write out response
/// * Was not able to flush the stream
pub fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0u8; 1024];

    stream.read(&mut buffer)?;

    let request_line = get_request_line(&buffer)?;
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
            stream.write_all(&response.format_response())?;
            stream.flush()?;
        }
        Err(e) => error!("Response error: {}", e),
    }

    Ok(())
}
