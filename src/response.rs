use http::StatusCode;
use std::error::Error;
use std::io::ErrorKind;
use std::{error, fmt, fs, str};

use crate::Method;
use crate::Request;

#[derive(Debug)]
pub struct InvalidContentType<'a>(&'a str);
impl<'a> error::Error for InvalidContentType<'a> {}

impl<'a> fmt::Display for InvalidContentType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Content Type: {}", self.0)
    }
}

impl<'a> From<&'a str> for InvalidContentType<'a> {
    fn from(content_type: &'a str) -> Self {
        InvalidContentType(content_type)
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
            ext => Err(InvalidContentType(ext)),
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
        }
    }
}

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

fn add_file(path: &str) -> Result<Response, Box<dyn Error>> {
    const ROOT: &str = "/home/ongo/Programming/linda";

    let path = format!("{}{}", ROOT, path);
    let contents = fs::read(&path);

    let mut response = Response::new();

    match contents {
        Ok(contents) => {
            response.body = Some(contents);

            let ext = path.split(".").last().unwrap_or("");
            response.headers.content_type =
                Some(ContentType::from_ext_str(ext).expect("========="));

            Ok(response)
        }
        Err(e) => {
            response.status = match e.kind() {
                ErrorKind::NotFound => StatusCode::NOT_FOUND,
                ErrorKind::PermissionDenied => StatusCode::FORBIDDEN,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            Ok(response)
        }
    }
}

pub fn response(request: &Request) -> Result<Response, Box<dyn Error>> {
    match *request.method() {
        Method::GET => add_file(request.uri().to_str().expect("Invalid file URI UTF8")),
        _ => {
            let mut response = Response::new();
            response.status = StatusCode::NOT_IMPLEMENTED;
            Ok(response)
        }
    }
}

/// HTTP Response representation
pub struct Response {
    pub status: StatusCode,
    pub body: Option<Vec<u8>>,
    pub headers: Headers,
}

impl Response {
    pub fn new() -> Self {
        Response {
            status: StatusCode::OK,
            body: None,
            headers: Headers::new(),
        }
    }

    pub fn format_response(&mut self) -> Vec<u8> {
        let mut result;
        let status_reason = match self.status.canonical_reason() {
            Some(reason) => reason,
            None => "",
        };

        result = format!("HTTP/1.1 {} {}\n", self.status.as_str(), status_reason,);
        result = format!("{}Allow: GET, HEAD\n", result);

        match &self.headers.content_type {
            Some(content_type) => {
                result = format!("{}Content-type: {}\n", result, content_type.as_str());
            }
            _ => (),
        }

        let mut bytes = result.as_bytes().to_vec();
        if self.body.is_some() {
            let body = self.body.as_mut().unwrap();

            bytes.append(&mut "\n".as_bytes().to_vec());
            bytes.append(body);
        }

        bytes
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = format!("HTTP/1.1 {}\r\n", self.status);

        // Add header
        result = format!("{}Allow: GET, HEAD\n", result);

        match &self.headers.content_type {
            Some(content_type) => {
                result = format!("{}Content-type: {}\n", result, content_type.as_str());
            }
            _ => (),
        }

        writeln!(f, "{}", result)
    }
}
