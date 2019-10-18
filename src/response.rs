//! HTTP request type

use http::StatusCode;
use std::error::Error;
use std::io::ErrorKind;
use std::{env, error, fmt, fs, str};

use crate::Method;
use crate::Request;

/// Whenever an unsupported/invalid content type gets requested
#[derive(Debug)]
pub struct InvalidContentType(String);
impl error::Error for InvalidContentType {}

impl fmt::Display for InvalidContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Content Type: {}", self.0)
    }
}

impl From<&str> for InvalidContentType {
    fn from(content_type: &str) -> Self {
        InvalidContentType(content_type.to_string())
    }
}

enum ContentType {
    CSS,
    HTML,
    GIF,
    PNG,
    JPEG,
    TEXT,
    SVG,
    XML,
    PDF,
    ICO,
}

impl ContentType {
    fn from_ext_str(ext: &str) -> Result<ContentType, InvalidContentType> {
        match ext {
            "css" => Ok(ContentType::CSS),
            "gif" => Ok(ContentType::GIF),
            "htm" => Ok(ContentType::HTML),
            "html" => Ok(ContentType::HTML),
            "jpeg" => Ok(ContentType::JPEG),
            "jpg" => Ok(ContentType::JPEG),
            "png" => Ok(ContentType::PNG),
            "svg" => Ok(ContentType::SVG),
            "txt" => Ok(ContentType::TEXT),
            "xml" => Ok(ContentType::XML),
            "pdf" => Ok(ContentType::PDF),
            "ico" => Ok(ContentType::ICO),
            ext => Err(InvalidContentType(ext.to_string())),
        }
    }

    fn as_str(&self) -> &str {
        match *self {
            ContentType::CSS => "text/css",
            ContentType::GIF => "image/gif",
            ContentType::HTML => "text/html",
            ContentType::JPEG => "image/jpeg",
            ContentType::PNG => "image/png",
            ContentType::SVG => "image/svg+xml",
            ContentType::TEXT => "text/plain",
            ContentType::XML => "application/xml",
            ContentType::PDF => "application/pdf",
            ContentType::ICO => "image/x-icon",
        }
    }
}

#[derive(Default)]
pub struct Headers {
    content_type: Option<ContentType>,
}

impl Headers {
    /// Create new ResponseHeader
    /// By default the content_type is None
    pub fn new() -> Self {
        Headers { content_type: None }
    }
}

fn add_file(path: &str, head: bool) -> Result<Response, Box<dyn Error>> {
    let mut root = String::from("/var/www");

    if env::var("LINDA_ROOT").is_ok() {
        root = env::var("LINDA_ROOT").unwrap();
    };

    let mut path = path.to_string();
    if path == "/" {
        path.push_str("index.html");
    }
    path = format!("{}{}", root, path);

    let contents = fs::read(&path);

    let mut response = Response::new();

    match contents {
        Ok(contents) => {
            // check if method type is not HEAD
            if !head {
                response.body = Some(contents);
            }

            // Get file extension
            let ext = path.split('.').last().unwrap_or("");
            response.headers.content_type = Some(ContentType::from_ext_str(ext)?);

            Ok(response)
        }
        Err(e) => {
            response.status = match e.kind() {
                ErrorKind::NotFound => {
                    // Set response body to 404.html if file not found
                    // check if method type is not HEAD
                    if !head {
                        response.body =
                            Some(fs::read(format!("{}/404.html", root)).unwrap_or_else(|_| vec![]));
                        response.headers.content_type = Some(ContentType::HTML);
                    }
                    StatusCode::NOT_FOUND
                }
                ErrorKind::PermissionDenied => StatusCode::FORBIDDEN,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            Ok(response)
        }
    }
}

/// Process Request, returning a Response
///
/// # Error
///
/// Should not error, except for rare cases when the URI string is not valid UTF-8
pub fn response(request: &Request) -> Result<Response, Box<dyn Error>> {
    match *request.method() {
        Method::GET => add_file(
            request.uri().to_str().expect("Invalid file URI UTF8"),
            false,
        ),
        Method::HEAD => add_file(request.uri().to_str().expect("Invalid file URI UTF8"), true),
        _ => {
            let mut response = Response::new();
            response.status = StatusCode::NOT_IMPLEMENTED;
            Ok(response)
        }
    }
}

/// HTTP Response
///
/// Response = Status-Line
///           *(( general-header
///           | response-header
///           | entity-header ) CRLF)
///             CRLF
///           [ message-body ]
#[derive(Default)]
pub struct Response {
    status: StatusCode,
    body: Option<Vec<u8>>,
    headers: Headers,
}

impl Response {
    /// Creates a new Response object with defaults:
    /// StatusCode::OK
    /// body: None
    /// empty Headers
    pub fn new() -> Self {
        Response {
            status: StatusCode::OK,
            body: None,
            headers: Headers::new(),
        }
    }

    /// Format Response object and return it as a Vec of bytes to write to a buffer
    pub fn format_response(&mut self) -> Vec<u8> {
        // Append Status-Line
        // Status-Line = HTTP-Version SP Status-Code SP Reason-Phrase CRLF
        let mut result = format!("HTTP/1.1 {}\r\n", self.status);

        // Append Content-Type entity-header
        if let Some(content_type) = &self.headers.content_type {
            result = format!("{}Content-type: {}\r\n\r\n", result, content_type.as_str());
        }

        // Append body (if file)
        let mut bytes = result.as_bytes().to_vec();
        if self.body.is_some() {
            let body = self.body.as_mut().unwrap();

            bytes.append(body);
        }

        bytes
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Status-Line = HTTP-Version SP Status-Code SP Reason-Phrase CRLF
        writeln!(f, "HTTP/1.1 {}", self.status)
    }
}
