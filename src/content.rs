//! HTTP Content (MIME) Types

use std::collections::HashMap;
use std::{error, fmt};

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

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub(super) enum ContentType {
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
    pub(super) fn from_ext_str(ext: &str) -> Result<ContentType, InvalidContentType> {
        let content_types: HashMap<&str, ContentType> = [
            ("css", ContentType::CSS),
            ("gif", ContentType::GIF),
            ("htm", ContentType::HTML),
            ("html", ContentType::HTML),
            ("jpeg", ContentType::JPEG),
            ("jpg", ContentType::JPEG),
            ("png", ContentType::PNG),
            ("svg", ContentType::SVG),
            ("txt", ContentType::TEXT),
            ("xml", ContentType::XML),
            ("pdf", ContentType::PDF),
            ("ico", ContentType::ICO),
        ]
        .iter()
        .cloned()
        .collect();
        if let Some(content_type) = content_types.get(ext) {
            Ok(*content_type)
        } else {
            Err(InvalidContentType(ext.to_string()))
        }
    }

    pub(super) fn as_str(&self) -> &str {
        let content_types: HashMap<ContentType, &str> = [
            (ContentType::CSS, "text/css"),
            (ContentType::GIF, "image/gif"),
            (ContentType::HTML, "text/html"),
            (ContentType::HTML, "text/html"),
            (ContentType::JPEG, "image/jpeg"),
            (ContentType::JPEG, "image/jpeg"),
            (ContentType::PNG, "image/png"),
            (ContentType::SVG, "image/svg+xml"),
            (ContentType::TEXT, "text/plain"),
            (ContentType::XML, "application/xml"),
            (ContentType::PDF, "application/pdf"),
            (ContentType::ICO, "image/x-icon"),
        ]
        .iter()
        .cloned()
        .collect();
        content_types.get(self).unwrap()
    }
}
