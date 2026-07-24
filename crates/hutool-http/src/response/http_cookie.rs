//! Hutool-aligned buffered HTTP response facade (`cn.hutool.http.HttpResponse`).

use crate::{HttpError, StatusCode, header};
use encoding_rs::Encoding;
use std::{
    fmt,
    io::Cursor,
    path::{Path, PathBuf},
};

/// Simple cookie name/value pair parsed from `Set-Cookie` (Hutool `HttpCookie` subset).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpCookie {
    name: String,
    value: String,
}

impl HttpCookie {
    /// Creates a cookie with the given name and value.
    ///
    /// Java: `java.net.HttpCookie` name/value accessors used by Hutool.
    #[must_use]
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    /// Returns the cookie name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the cookie value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

use super::{decode_rfc5987, filename_from_dispositions, from_hex, parse_set_cookie, percent_decode_lightweight, strip_quotes};
