use http::StatusCode;
use std::fmt;

#[derive(Debug)]
pub struct InvalidContentType<'a>(&'a str);

impl<'a> fmt::Display for InvalidContentType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid URI: {}", self.0)
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

struct Headers {
    content_type: Option<ContentType>,
}

impl Headers {
    /// Create new ResponseHeader
    /// By default the content_type is None
    pub fn new() -> Self {
        Headers { content_type: None }
    }
}

pub struct Response {
    status: StatusCode,
    body: Vec<u8>,
    headers: Headers,
}
