//! HTTP request type

use std::error::Error;
use std::path::Path;
use std::{error, fmt, str};

use crate::Method;

#[derive(Debug)]
pub struct InvalidUri<'a>(&'a str);

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

pub fn parse_request_line<'a>(request: &'a str) -> Result<Request, Box<dyn Error + 'a>> {
    let mut parts = request.split_whitespace();

    let method = parts.next().ok_or("Method not specified")?;
    let uri = parts.next().ok_or("URI not specified")?;
    let http_version = parts.next().ok_or("HTTP version not specified")?;

    let mut request = Request::new();
    request.method(method)?.uri(uri)?.version(http_version)?;

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
    /// Set Request HTTP method
    pub fn method(&mut self, method: &'a str) -> Result<&mut Self, Box<dyn Error + 'a>> {
        self.method = Method::from_str(method)?;
        Ok(self)
    }

    /// Set Request HTTP uri
    pub fn uri(&mut self, uri: &'a str) -> Result<&mut Self, Box<dyn Error + 'a>> {
        self.uri = Request::validate_uri(&uri)?;
        Ok(self)
    }

    /// Set Request HTTP version
    pub fn version(&mut self, version: &'a str) -> Result<&mut Self, Box<dyn Error>> {
        if version != "HTTP/1.1" {
            return Err(format!("HTTP version {} is not supported.", version).into());
        }
        self.version = version;
        Ok(self)
    }

    fn validate_uri(uri: &str) -> Result<&Path, InvalidUri> {
        const ROOT: &str = "/home/ongo/Programming/linda";

        if Path::new(&format!("{}{}", ROOT, uri)).exists() {
            Ok(Path::new(uri))
        } else {
            Err(InvalidUri(uri))
        }
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
