//! HTTP request type

use std::error::Error;
use std::path::Path;
use std::{error, fmt, str};

use crate::Method;

#[derive(Debug)]
struct InvalidUri<'a>(&'a str);

impl<'a> fmt::Display for InvalidUri<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid URI: {}", self.0)
    }
}

impl<'a> From<&'a str> for InvalidUri<'a> {
    fn from(error: &'a str) -> Self {
        InvalidUri(error)
    }
}

impl<'a> error::Error for InvalidUri<'a> {}

#[derive(Debug)]
struct RequestLineNotFound;

impl fmt::Display for RequestLineNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Request-Line not found")
    }
}

impl error::Error for RequestLineNotFound {}

/// Return a Request-Line given a buffer
pub fn get_request_line(buffer: &[u8]) -> Result<&str, Box<dyn Error>> {
    let request = str::from_utf8(&buffer[..])?;
    Ok(request.lines().next().ok_or(RequestLineNotFound)?)
}

/// Takes in a request line and returns a properly formatted Request
/// wrapped in a Result<>
///
/// # Errors
///
/// Propagates errors up if:
/// * Not specified:
///     - Request Method
///     - Request URI
///     - Request HTTP Version
/// * If Method invalid (InvalidMethod Error type), see Method
/// * If the HTTP version is not supported
pub fn parse_request_line<'a>(request: &'a str) -> Result<Request, Box<dyn Error + 'a>> {
    let mut parts = request.split_whitespace();

    let method = parts.next().ok_or("Method not specified")?;
    let uri = parts.next().ok_or("URI not specified")?;
    let http_version = parts.next().ok_or("HTTP version not specified")?;

    let mut request = Request::new();
    request
        .method_mut(method)?
        .uri_mut(uri)
        .version_mut(http_version)?;

    Ok(request)
}

/// HTTP Request representation
///
/// Request-Line = Method SP Request-URI SP HTTP-Version CRLF
pub struct Request<'a> {
    method: Method,
    uri: &'a Path,
    version: &'a str,
}

/// Implement builder for Request
impl<'a> Request<'a> {
    /// Get Request method
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Get Request uri
    pub fn uri(&self) -> &Path {
        &self.uri
    }

    /// Get Request uri
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Set Request HTTP method
    pub fn method_mut(&mut self, method: &'a str) -> Result<&mut Self, Box<dyn Error + 'a>> {
        self.method = Method::from_str(method)?;
        Ok(self)
    }

    /// Set Request HTTP uri
    pub fn uri_mut(&mut self, uri: &'a str) -> &mut Self {
        self.uri = Path::new(uri);
        self
    }

    /// Set Request HTTP version
    pub fn version_mut(&mut self, version: &'a str) -> Result<&mut Self, Box<dyn Error>> {
        if version != "HTTP/1.1" {
            return Err(format!("HTTP version {} is not supported.", version).into());
        }
        self.version = version;
        Ok(self)
    }

    /// Create a new Request object with defaults:
    ///
    /// method: Method::Get
    /// uri: Path::new("/"),
    /// version: "HTTP/1.1",
    pub fn new() -> Self {
        Request::default()
    }
}

impl<'a> fmt::Display for Request<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} {} {}\r",
            self.method.as_str(),
            self.uri.display(),
            self.version
        )
    }
}

impl<'a> Default for Request<'a> {
    fn default() -> Request<'a> {
        Request {
            method: Method::default(),
            uri: Path::new("/"),
            version: "HTTP/1.1",
        }
    }
}
