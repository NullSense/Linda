//! HTTP request method
//!
use self::Inner::*;
use std::{error, fmt, str};

#[derive(Debug)]
pub struct InvalidMethod<'a>(&'a str);

impl<'a> fmt::Display for InvalidMethod<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid HTTP method: {}", self.0)
    }
}

impl<'a> From<&'a str> for InvalidMethod<'a> {
    fn from(error: &'a str) -> Self {
        InvalidMethod(error)
    }
}

impl<'a> error::Error for InvalidMethod<'a> {}

/// Request Method
///
/// Contains constants for multiple HTTP headers:
/// e.g. GET, HEAD
#[derive(Debug)]
pub struct Method(Inner);

/// Get and Head have to be implemented under HTTP/1.1
#[derive(Debug)]
enum Inner {
    Get,
    Head,
}

impl Method {
    pub const GET: Method = Method(Get);
    pub const HEAD: Method = Method(Head);

    /// Return HTTP method as Method object
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(src: &str) -> Result<Method, InvalidMethod> {
        match src {
            "GET" => Ok(Method(Get)),
            "HEAD" => Ok(Method(Head)),
            e => Err(InvalidMethod(e)),
        }
    }

    /// Return HTTP method as &str
    pub fn as_str(&self) -> &str {
        match self.0 {
            Get => "GET",
            Head => "HEAD",
        }
    }
}

impl Default for Method {
    /// Set default method to GET
    fn default() -> Method {
        Method::GET
    }
}
