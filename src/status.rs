use std::collections::HashMap;
use std::fmt;

#[derive(Hash, Eq, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub(super) enum StatusCode {
    CONTINUE,
    SWITCHING_PROTOCOLS,
    OK,
    CREATED,
    ACCEPTED,
    NON_AUTHORITATIVE_INFORMATION,
    NO_CONTENT,
    RESET_CONTENT,
    PARTIAL_CONTENT,
    MULTIPLE_CHOICES,
    MOVED_PERMANENTLY,
    FOUND,
    SEE_OTHER,
    NOT_MODIFIED,
    USE_PROXY,
    TEMPORARY_REDIRECT,
    BAD_REQUEST,
    UNAUTHORIZED,
    PAYMENT_REQUIRED,
    FORBIDDEN,
    NOT_FOUND,
    METHOD_NOT_ALLOWED,
    NOT_ACCEPTABLE,
    PROXY_AUTHENTICATION_REQUIRED,
    REQUEST_TIME_OUT,
    CONFLICT,
    GONE,
    LENGTH_REQUIRED,
    PRECONDITION_FAILED,
    REQUEST_ENTITY_TOO_LARGE,
    REQUEST_URI_TOO_LARGE,
    UNSUPPORTED_MEDIA_TYPE,
    REQUEST_RANGE_NOT_SATISFIABLE,
    EXPECTATION_FAILED,
    INTERNAL_SERVER_ERROR,
    NOT_IMPLEMENTED,
    BAD_GATEWAY,
    SERVICE_UNAVAILABLE,
    GATEWAY_TIME_OUT,
    HTTP_VERSION_NOT_SUPPORTED,
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode::OK
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let codes: HashMap<StatusCode, i32> = [
            (StatusCode::CONTINUE, 100),
            (StatusCode::SWITCHING_PROTOCOLS, 101),
            (StatusCode::OK, 200),
            (StatusCode::CREATED, 201),
            (StatusCode::ACCEPTED, 202),
            (StatusCode::NON_AUTHORITATIVE_INFORMATION, 203),
            (StatusCode::NO_CONTENT, 204),
            (StatusCode::RESET_CONTENT, 205),
            (StatusCode::PARTIAL_CONTENT, 206),
            (StatusCode::MULTIPLE_CHOICES, 300),
            (StatusCode::MOVED_PERMANENTLY, 301),
            (StatusCode::FOUND, 302),
            (StatusCode::SEE_OTHER, 303),
            (StatusCode::NOT_MODIFIED, 304),
            (StatusCode::USE_PROXY, 305),
            (StatusCode::TEMPORARY_REDIRECT, 307),
            (StatusCode::BAD_REQUEST, 400),
            (StatusCode::UNAUTHORIZED, 401),
            (StatusCode::PAYMENT_REQUIRED, 402),
            (StatusCode::FORBIDDEN, 403),
            (StatusCode::NOT_FOUND, 404),
            (StatusCode::METHOD_NOT_ALLOWED, 405),
            (StatusCode::NOT_ACCEPTABLE, 406),
            (StatusCode::PROXY_AUTHENTICATION_REQUIRED, 407),
            (StatusCode::REQUEST_TIME_OUT, 408),
            (StatusCode::CONFLICT, 409),
            (StatusCode::GONE, 410),
            (StatusCode::LENGTH_REQUIRED, 411),
            (StatusCode::PRECONDITION_FAILED, 412),
            (StatusCode::REQUEST_ENTITY_TOO_LARGE, 413),
            (StatusCode::REQUEST_URI_TOO_LARGE, 414),
            (StatusCode::UNSUPPORTED_MEDIA_TYPE, 415),
            (StatusCode::REQUEST_RANGE_NOT_SATISFIABLE, 416),
            (StatusCode::EXPECTATION_FAILED, 417),
            (StatusCode::INTERNAL_SERVER_ERROR, 500),
            (StatusCode::NOT_IMPLEMENTED, 501),
            (StatusCode::BAD_GATEWAY, 502),
            (StatusCode::SERVICE_UNAVAILABLE, 503),
            (StatusCode::GATEWAY_TIME_OUT, 504),
            (StatusCode::HTTP_VERSION_NOT_SUPPORTED, 505),
        ]
        .iter()
        .cloned()
        .collect();
        write!(f, "{}", codes.get(self).unwrap())
    }
}
