//! 对齐: `cn.hutool.http.Header`
//! 来源: hutool-http/src/main/java/cn/hutool/http/Header.java
//! 中文说明: HTTP元数据模块，定义常用Header枚举、内容类型和HTTP版本常量

use std::{
    collections::{HashMap, hash_map::Entry},
    fmt,
};

macro_rules! headers {
    ($($variant:ident => $value:literal),+ $(,)?) => {
        /// Common HTTP header names exposed by Hutool's `Header` enum.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Header {
            $(
                #[doc = concat!("HTTP header `", $value, "`.")]
                $variant,
            )+
        }

        impl Header {
            /// All Hutool header variants in declaration order.
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];

            /// Returns the canonical wire name.
            #[must_use]
            pub const fn value(self) -> &'static str {
                match self {
                    $(Self::$variant => $value),+
                }
            }
        }
    };
}

headers! {
    Authorization => "Authorization",
    ProxyAuthorization => "Proxy-Authorization",
    Date => "Date",
    Connection => "Connection",
    MimeVersion => "MIME-Version",
    Trailer => "Trailer",
    TransferEncoding => "Transfer-Encoding",
    Upgrade => "Upgrade",
    Via => "Via",
    CacheControl => "Cache-Control",
    Pragma => "Pragma",
    ContentType => "Content-Type",
    Host => "Host",
    Referer => "Referer",
    Origin => "Origin",
    UserAgent => "User-Agent",
    Accept => "Accept",
    AcceptLanguage => "Accept-Language",
    AcceptEncoding => "Accept-Encoding",
    AcceptCharset => "Accept-Charset",
    Cookie => "Cookie",
    ContentLength => "Content-Length",
    WwwAuthenticate => "WWW-Authenticate",
    SetCookie => "Set-Cookie",
    ContentEncoding => "Content-Encoding",
    ContentDisposition => "Content-Disposition",
    Etag => "ETag",
    Location => "Location",
}

impl fmt::Display for Header {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.value())
    }
}

/// Common media types exposed by Hutool's `ContentType` enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentType {
    /// `application/x-www-form-urlencoded`.
    FormUrlEncoded,
    /// `multipart/form-data`.
    Multipart,
    /// `application/json`.
    Json,
    /// `application/xml`.
    Xml,
    /// `text/plain`.
    TextPlain,
    /// `text/xml`.
    TextXml,
    /// `text/html`.
    TextHtml,
    /// `application/octet-stream`.
    OctetStream,
    /// `text/event-stream`.
    EventStream,
}

impl ContentType {
    /// All Hutool content types in declaration order.
    pub const ALL: &'static [Self] = &[
        Self::FormUrlEncoded,
        Self::Multipart,
        Self::Json,
        Self::Xml,
        Self::TextPlain,
        Self::TextXml,
        Self::TextHtml,
        Self::OctetStream,
        Self::EventStream,
    ];

    /// Returns the media-type value without parameters.
    #[must_use]
    pub const fn value(self) -> &'static str {
        match self {
            Self::FormUrlEncoded => "application/x-www-form-urlencoded",
            Self::Multipart => "multipart/form-data",
            Self::Json => "application/json",
            Self::Xml => "application/xml",
            Self::TextPlain => "text/plain",
            Self::TextXml => "text/xml",
            Self::TextHtml => "text/html",
            Self::OctetStream => "application/octet-stream",
            Self::EventStream => "text/event-stream",
        }
    }

    /// Adds a charset parameter using Hutool's compact formatting.
    #[must_use]
    pub fn with_charset(self, charset: &str) -> String {
        Self::build(self.value(), charset)
    }

    /// Reports whether a missing or form-urlencoded value is Hutool's default.
    #[must_use]
    pub fn is_default(content_type: Option<&str>) -> bool {
        content_type.is_none_or(Self::is_form_url_encoded)
    }

    /// Reports whether a value begins with the form-urlencoded media type.
    #[must_use]
    pub fn is_form_url_encoded(content_type: &str) -> bool {
        content_type
            .get(..Self::FormUrlEncoded.value().len())
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case(Self::FormUrlEncoded.value()))
    }

    /// Infers JSON or XML from the first non-whitespace body character.
    #[must_use]
    pub fn detect(body: &str) -> Option<Self> {
        match body.trim_start().chars().next()? {
            '{' | '[' => Some(Self::Json),
            '<' => Some(Self::Xml),
            _ => None,
        }
    }

    /// Builds a media type with a charset parameter.
    #[must_use]
    pub fn build(content_type: &str, charset: &str) -> String {
        format!("{content_type};charset={charset}")
    }

    /// Builds this media type with a charset parameter.
    #[must_use]
    pub fn build_type(content_type: Self, charset: &str) -> String {
        Self::build(content_type.value(), charset)
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.value())
    }
}

macro_rules! status_codes {
    ($($name:ident = $value:literal),+ $(,)?) => {
        $(
            #[doc = concat!("HTTP status code `", stringify!($value), "`.")]
            pub const $name: u16 = $value;
        )+
    };
}

/// Hutool-compatible HTTP status-code namespace.
#[derive(Debug, Clone, Copy, Default)]
pub struct HttpStatus;

impl HttpStatus {
    status_codes! {
        HTTP_CONTINUE = 100,
        HTTP_SWITCHING_PROTOCOLS = 101,
        HTTP_PROCESSING = 102,
        HTTP_CHECKPOINT = 103,
        HTTP_OK = 200,
        HTTP_CREATED = 201,
        HTTP_ACCEPTED = 202,
        HTTP_NOT_AUTHORITATIVE = 203,
        HTTP_NO_CONTENT = 204,
        HTTP_RESET = 205,
        HTTP_PARTIAL = 206,
        HTTP_MULTI_STATUS = 207,
        HTTP_ALREADY_REPORTED = 208,
        HTTP_IM_USED = 226,
        HTTP_MULT_CHOICE = 300,
        HTTP_MOVED_PERM = 301,
        HTTP_MOVED_TEMP = 302,
        HTTP_SEE_OTHER = 303,
        HTTP_NOT_MODIFIED = 304,
        HTTP_USE_PROXY = 305,
        HTTP_TEMP_REDIRECT = 307,
        HTTP_PERMANENT_REDIRECT = 308,
        HTTP_BAD_REQUEST = 400,
        HTTP_UNAUTHORIZED = 401,
        HTTP_PAYMENT_REQUIRED = 402,
        HTTP_FORBIDDEN = 403,
        HTTP_NOT_FOUND = 404,
        HTTP_BAD_METHOD = 405,
        HTTP_NOT_ACCEPTABLE = 406,
        HTTP_PROXY_AUTH = 407,
        HTTP_CLIENT_TIMEOUT = 408,
        HTTP_CONFLICT = 409,
        HTTP_GONE = 410,
        HTTP_LENGTH_REQUIRED = 411,
        HTTP_PRECON_FAILED = 412,
        HTTP_ENTITY_TOO_LARGE = 413,
        HTTP_REQ_TOO_LONG = 414,
        HTTP_UNSUPPORTED_TYPE = 415,
        HTTP_REQUESTED_RANGE_NOT_SATISFIABLE = 416,
        HTTP_EXPECTATION_FAILED = 417,
        HTTP_I_AM_A_TEAPOT = 418,
        HTTP_UNPROCESSABLE_ENTITY = 422,
        HTTP_LOCKED = 423,
        HTTP_FAILED_DEPENDENCY = 424,
        HTTP_TOO_EARLY = 425,
        HTTP_UPGRADE_REQUIRED = 426,
        HTTP_PRECONDITION_REQUIRED = 428,
        HTTP_TOO_MANY_REQUESTS = 429,
        HTTP_REQUEST_HEADER_FIELDS_TOO_LARGE = 431,
        HTTP_UNAVAILABLE_FOR_LEGAL_REASONS = 451,
        HTTP_INTERNAL_ERROR = 500,
        HTTP_NOT_IMPLEMENTED = 501,
        HTTP_BAD_GATEWAY = 502,
        HTTP_UNAVAILABLE = 503,
        HTTP_GATEWAY_TIMEOUT = 504,
        HTTP_VERSION = 505,
        HTTP_VARIANT_ALSO_NEGOTIATES = 506,
        HTTP_INSUFFICIENT_STORAGE = 507,
        HTTP_LOOP_DETECTED = 508,
        HTTP_BANDWIDTH_LIMIT_EXCEEDED = 509,
        HTTP_NOT_EXTENDED = 510,
        HTTP_NETWORK_AUTHENTICATION_REQUIRED = 511,
    }

    /// Reports whether Hutool treats `response_code` as a redirect.
    #[must_use]
    pub const fn is_redirected(response_code: u16) -> bool {
        matches!(response_code, 301 | 302 | 303 | 307 | 308)
    }
}

/// Legacy status constants retained for Hutool migration code.
#[derive(Debug, Clone, Copy, Default)]
pub struct Status;

impl Status {
    status_codes! {
        HTTP_OK = 200,
        HTTP_CREATED = 201,
        HTTP_ACCEPTED = 202,
        HTTP_NOT_AUTHORITATIVE = 203,
        HTTP_NO_CONTENT = 204,
        HTTP_RESET = 205,
        HTTP_PARTIAL = 206,
        HTTP_MULT_CHOICE = 300,
        HTTP_MOVED_PERM = 301,
        HTTP_MOVED_TEMP = 302,
        HTTP_SEE_OTHER = 303,
        HTTP_NOT_MODIFIED = 304,
        HTTP_USE_PROXY = 305,
        HTTP_BAD_REQUEST = 400,
        HTTP_UNAUTHORIZED = 401,
        HTTP_PAYMENT_REQUIRED = 402,
        HTTP_FORBIDDEN = 403,
        HTTP_NOT_FOUND = 404,
        HTTP_BAD_METHOD = 405,
        HTTP_NOT_ACCEPTABLE = 406,
        HTTP_PROXY_AUTH = 407,
        HTTP_CLIENT_TIMEOUT = 408,
        HTTP_CONFLICT = 409,
        HTTP_GONE = 410,
        HTTP_LENGTH_REQUIRED = 411,
        HTTP_PRECON_FAILED = 412,
        HTTP_ENTITY_TOO_LARGE = 413,
        HTTP_REQ_TOO_LONG = 414,
        HTTP_UNSUPPORTED_TYPE = 415,
        HTTP_INTERNAL_ERROR = 500,
        HTTP_NOT_IMPLEMENTED = 501,
        HTTP_BAD_GATEWAY = 502,
        HTTP_UNAVAILABLE = 503,
        HTTP_GATEWAY_TIMEOUT = 504,
        HTTP_VERSION = 505,
    }
}

/// Explicitly owned equivalent of Hutool's mutable global header collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalHeaders {
    headers: HashMap<String, Vec<String>>,
}

impl Default for GlobalHeaders {
    fn default() -> Self {
        let mut headers = Self {
            headers: HashMap::new(),
        };
        headers.put_default(false);
        headers
    }
}

impl GlobalHeaders {
    /// Creates an owned collection populated with Hutool's safe defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Installs default headers, optionally clearing custom values first.
    pub fn put_default(&mut self, reset: bool) -> &mut Self {
        if reset {
            self.headers.clear();
        }
        self.insert(
            Header::Accept.value(),
            "text/html,application/json,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            true,
        );
        self.insert(Header::AcceptEncoding.value(), "gzip, deflate", true);
        self.insert(
            Header::UserAgent.value(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.142 Safari/537.36 Hutool",
            true,
        );
        self
    }

    /// Returns the first value for a non-blank header name.
    #[must_use]
    pub fn header(&self, name: &str) -> Option<&str> {
        self.header_list(name)?.first().map(String::as_str)
    }

    /// Returns all values for a non-blank header name.
    #[must_use]
    pub fn header_list(&self, name: &str) -> Option<&[String]> {
        let name = name.trim();
        (!name.is_empty())
            .then(|| self.headers.get(name).map(Vec::as_slice))
            .flatten()
    }

    /// Returns the first value for a typed header name.
    #[must_use]
    pub fn named_header(&self, name: Header) -> Option<&str> {
        self.header(name.value())
    }

    /// Inserts or appends a string header.
    pub fn insert(&mut self, name: &str, value: &str, overwrite: bool) -> &mut Self {
        let name = name.trim();
        if name.is_empty() {
            return self;
        }
        match self.headers.entry(name.to_owned()) {
            Entry::Occupied(mut entry) if !overwrite => {
                entry.get_mut().push(value.trim().to_owned());
            }
            Entry::Occupied(mut entry) => {
                entry.insert(vec![value.to_owned()]);
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![value.to_owned()]);
            }
        }
        self
    }

    /// Inserts or appends a typed header.
    pub fn insert_header(&mut self, name: Header, value: &str, overwrite: bool) -> &mut Self {
        self.insert(name.value(), value, overwrite)
    }

    /// Overwrites a typed header value.
    pub fn set_header(&mut self, name: Header, value: &str) -> &mut Self {
        self.insert_header(name, value, true)
    }

    /// Overwrites a string header value.
    pub fn set(&mut self, name: &str, value: &str) -> &mut Self {
        self.insert(name, value, true)
    }

    /// Appends every supplied header value.
    pub fn extend(&mut self, headers: &HashMap<String, Vec<String>>) -> &mut Self {
        for (name, values) in headers {
            for value in values {
                self.insert(name, value, false);
            }
        }
        self
    }

    /// Removes a string header.
    pub fn remove(&mut self, name: &str) -> &mut Self {
        self.headers.remove(name.trim());
        self
    }

    /// Removes a typed header.
    pub fn remove_header(&mut self, name: Header) -> &mut Self {
        self.remove(name.value())
    }

    /// Returns the owned collection's read-only map.
    #[must_use]
    pub const fn headers(&self) -> &HashMap<String, Vec<String>> {
        &self.headers
    }

    /// Clears every header, including defaults.
    pub fn clear_headers(&mut self) -> &mut Self {
        self.headers.clear();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Method;

    #[test]
    fn methods_headers_and_statuses_match_hutool_catalogs() {
        let methods = [
            Method::GET,
            Method::POST,
            Method::HEAD,
            Method::OPTIONS,
            Method::PUT,
            Method::DELETE,
            Method::TRACE,
            Method::CONNECT,
            Method::PATCH,
        ];
        assert_eq!(methods.len(), 9);

        assert_eq!(Header::ALL.len(), 28);
        for header in Header::ALL {
            assert_eq!(header.to_string(), header.value());
            assert!(!header.value().is_empty());
        }

        for code in [301, 302, 303, 307, 308] {
            assert!(HttpStatus::is_redirected(code));
        }
        for code in [200, 300, 304, 305, 306, 400] {
            assert!(!HttpStatus::is_redirected(code));
        }
        assert_eq!(HttpStatus::HTTP_CONTINUE, 100);
        assert_eq!(HttpStatus::HTTP_NETWORK_AUTHENTICATION_REQUIRED, 511);
        assert_eq!(Status::HTTP_OK, HttpStatus::HTTP_OK);
        assert_eq!(Status::HTTP_VERSION, HttpStatus::HTTP_VERSION);
    }

    #[test]
    fn content_types_format_detect_and_classify_every_shape() {
        let expected = [
            "application/x-www-form-urlencoded",
            "multipart/form-data",
            "application/json",
            "application/xml",
            "text/plain",
            "text/xml",
            "text/html",
            "application/octet-stream",
            "text/event-stream",
        ];
        assert_eq!(ContentType::ALL.len(), expected.len());
        for (content_type, expected) in ContentType::ALL.iter().zip(expected) {
            assert_eq!(content_type.value(), expected);
            assert_eq!(content_type.to_string(), expected);
        }

        assert_eq!(
            ContentType::Json.with_charset("UTF-8"),
            "application/json;charset=UTF-8"
        );
        assert_eq!(
            ContentType::build_type(ContentType::TextPlain, "GBK"),
            "text/plain;charset=GBK"
        );
        assert!(ContentType::is_default(None));
        assert!(ContentType::is_default(Some(
            "Application/X-Www-Form-Urlencoded;charset=UTF-8"
        )));
        assert!(!ContentType::is_default(Some("application/json")));
        assert!(!ContentType::is_form_url_encoded("short"));
        assert_eq!(
            ContentType::detect("  {\"ok\":true}"),
            Some(ContentType::Json)
        );
        assert_eq!(ContentType::detect("\n[1]"), Some(ContentType::Json));
        assert_eq!(ContentType::detect("\t<root/>"), Some(ContentType::Xml));
        assert_eq!(ContentType::detect("plain"), None);
        assert_eq!(ContentType::detect("   "), None);
    }

    #[test]
    fn owned_global_headers_cover_defaults_overloads_mutation_and_reset() {
        let mut headers = GlobalHeaders::new();
        assert_eq!(headers.headers().len(), 3);
        assert_eq!(
            headers.named_header(Header::AcceptEncoding),
            Some("gzip, deflate")
        );
        assert_eq!(headers.header("  Accept-Encoding  "), Some("gzip, deflate"));
        assert_eq!(headers.header(""), None);
        assert_eq!(headers.header("missing"), None);

        headers.insert(" ", "ignored", true);
        headers.insert("X-Test", " first ", true);
        headers.insert("X-Test", " second ", false);
        assert_eq!(
            headers.header_list("X-Test"),
            Some([" first ".to_owned(), "second".to_owned()].as_slice())
        );
        headers.set("X-Test", "third");
        assert_eq!(headers.header("X-Test"), Some("third"));
        headers.insert_header(Header::ContentType, "text/plain", false);
        headers.set_header(Header::ContentType, "application/json");
        assert_eq!(
            headers.named_header(Header::ContentType),
            Some("application/json")
        );

        let additions = HashMap::from([
            ("X-Multi".to_owned(), vec!["a".to_owned(), " b ".to_owned()]),
            ("X-Empty".to_owned(), Vec::new()),
        ]);
        headers.extend(&additions);
        assert_eq!(
            headers.header_list("X-Multi"),
            Some(["a".to_owned(), "b".to_owned()].as_slice())
        );
        headers.remove("missing").remove_header(Header::ContentType);
        assert_eq!(headers.named_header(Header::ContentType), None);

        headers.put_default(false);
        assert!(headers.header("X-Test").is_some());
        headers.put_default(true);
        assert_eq!(headers.headers().len(), 3);
        assert_eq!(headers.header("X-Test"), None);
        headers.clear_headers();
        assert!(headers.headers().is_empty());
    }
}
