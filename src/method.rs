//! HTTP request method
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
#[derive(Debug, Eq, PartialEq)]
pub struct Method(Inner);

/// Get and Head have to be implemented under HTTP/1.1
#[derive(Debug, Eq, PartialEq)]
enum Inner {
    Options,
    Get,
    Head,
    Post,
    Put,
    Delete,
    Trace,
    Connect,
}

impl Method {
    pub const GET: Method = Method(Get);
    pub const HEAD: Method = Method(Head);

    /// Return HTTP method as Method object
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(src: &str) -> Result<Method, InvalidMethod> {
        match src {
            "OPTIONS" => Ok(Method(Options)),
            "GET" => Ok(Method(Get)),
            "HEAD" => Ok(Method(Head)),
            "POST" => Ok(Method(Post)),
            "PUT" => Ok(Method(Put)),
            "DELETE" => Ok(Method(Delete)),
            "TRACE" => Ok(Method(Trace)),
            "CONNECT" => Ok(Method(Connect)),
            e => Err(InvalidMethod(e)),
        }
    }

    /// Return HTTP method as &str
    pub fn as_str(&self) -> &str {
        match self.0 {
            Options => "OPTIONS",
            Get => "GET",
            Head => "HEAD",
            Post => "POST",
            Put => "PUT",
            Delete => "DELETE",
            Trace => "TRACE",
            Connect => "CONNECT",
        }
    }
}

impl Default for Method {
    /// Set default method to GET
    fn default() -> Method {
        Method::GET
    }
}
