//! Shared Hutool-style request and response metadata.

use crate::Header;
use encoding_rs::{Encoding, UTF_8};
use std::{
    collections::{HashMap, hash_map::Entry},
    fmt,
};
use thiserror::Error;

use super::http_base_error::HttpBaseError;

/// Shared, explicitly owned metadata for Hutool-style requests and responses.
#[derive(Debug, Clone)]
pub struct HttpBase {
    header_aggregated: bool,
    headers: HashMap<String, Vec<String>>,
    charset: &'static Encoding,
    http_version: String,
    body: Option<Vec<u8>>,
}

impl Default for HttpBase {
    fn default() -> Self {
        Self {
            header_aggregated: false,
            headers: HashMap::new(),
            charset: UTF_8,
            http_version: HTTP_1_1.to_owned(),
            body: None,
        }
    }
}

impl HttpBase {
    /// Creates empty UTF-8 HTTP/1.1 metadata.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the first value for a non-blank, case-insensitive header name.
    #[must_use]
    pub fn header(&self, name: &str) -> Option<&str> {
        self.header_list(name)?.first().map(String::as_str)
    }

    /// Returns all values for a non-blank, case-insensitive header name.
    #[must_use]
    pub fn header_list(&self, name: &str) -> Option<&[String]> {
        let name = name.trim();
        if name.is_empty() {
            return None;
        }
        self.headers
            .iter()
            .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
            .map(|(_, values)| values.as_slice())
    }

    /// Returns the first value for a typed header name.
    #[must_use]
    pub fn named_header(&self, name: Header) -> Option<&str> {
        self.header(name.value())
    }

    /// Inserts or appends a header while treating its name case-insensitively.
    pub fn insert(&mut self, name: &str, value: &str, overwrite: bool) -> &mut Self {
        let name = name.trim();
        if name.is_empty() {
            return self;
        }
        let canonical = self
            .headers
            .keys()
            .find(|candidate| candidate.eq_ignore_ascii_case(name))
            .cloned()
            .unwrap_or_else(|| name.to_owned());
        match self.headers.entry(canonical) {
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

    /// Inserts a flat header map using the requested overwrite mode.
    pub fn extend_flat(&mut self, headers: &HashMap<String, String>, overwrite: bool) -> &mut Self {
        for (name, value) in headers {
            self.insert(name, value, overwrite);
        }
        self
    }

    /// Inserts a multi-value header map using the requested overwrite mode.
    pub fn extend(&mut self, headers: &HashMap<String, Vec<String>>, overwrite: bool) -> &mut Self {
        for (name, values) in headers {
            for value in values {
                self.insert(name, value, overwrite);
            }
        }
        self
    }

    /// Appends every value from a flat header map.
    pub fn add_headers(&mut self, headers: &HashMap<String, String>) -> &mut Self {
        self.extend_flat(headers, false)
    }

    /// Removes a header by a case-insensitive string name.
    pub fn remove(&mut self, name: &str) -> &mut Self {
        let name = name.trim();
        if let Some(key) = self
            .headers
            .keys()
            .find(|candidate| candidate.eq_ignore_ascii_case(name))
            .cloned()
        {
            self.headers.remove(&key);
        }
        self
    }

    /// Removes a typed header.
    pub fn remove_header(&mut self, name: Header) -> &mut Self {
        self.remove(name.value())
    }

    /// Returns the read-only header map.
    #[must_use]
    pub const fn headers(&self) -> &HashMap<String, Vec<String>> {
        &self.headers
    }

    /// Clears every header.
    pub fn clear_headers(&mut self) -> &mut Self {
        self.headers.clear();
        self
    }

    /// Enables or disables comma aggregation of repeated wire headers.
    pub const fn header_aggregation(&mut self, aggregate: bool) -> &mut Self {
        self.header_aggregated = aggregate;
        self
    }

    /// Reports whether repeated wire headers are comma-aggregated.
    #[must_use]
    pub const fn is_header_aggregated(&self) -> bool {
        self.header_aggregated
    }

    /// Materializes headers as they should be applied to an HTTP request.
    #[must_use]
    pub fn wire_headers(&self) -> Vec<(String, String)> {
        if self.header_aggregated {
            self.headers
                .iter()
                .map(|(name, values)| (name.clone(), values.join(",")))
                .collect()
        } else {
            self.headers
                .iter()
                .flat_map(|(name, values)| values.iter().map(|value| (name.clone(), value.clone())))
                .collect()
        }
    }

    /// Returns the metadata-only HTTP protocol label.
    #[must_use]
    pub fn http_version(&self) -> &str {
        &self.http_version
    }

    /// Sets the metadata-only HTTP protocol label.
    pub fn set_http_version(&mut self, http_version: impl Into<String>) -> &mut Self {
        self.http_version = http_version.into();
        self
    }

    /// Returns the optional body bytes.
    #[must_use]
    pub fn body_bytes(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }

    /// Replaces the optional body bytes.
    pub fn set_body(&mut self, body: impl Into<Vec<u8>>) -> &mut Self {
        self.body = Some(body.into());
        self
    }

    /// Clears the body.
    pub fn clear_body(&mut self) -> &mut Self {
        self.body = None;
        self
    }

    /// Returns the canonical character-set name.
    #[must_use]
    pub fn charset(&self) -> &'static str {
        self.charset.name()
    }

    /// Resolves and sets a non-blank character-set label.
    pub fn set_charset_name(&mut self, charset: &str) -> Result<&mut Self, HttpBaseError> {
        let charset = charset.trim();
        if charset.is_empty() {
            return Ok(self);
        }
        let encoding = Encoding::for_label(charset.as_bytes())
            .ok_or_else(|| HttpBaseError::UnsupportedCharset(charset.to_owned()))?;
        Ok(self.set_charset(encoding))
    }

    /// Sets an Encoding Standard character set.
    pub const fn set_charset(&mut self, charset: &'static Encoding) -> &mut Self {
        self.charset = charset;
        self
    }
}

impl fmt::Display for HttpBase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Headers: \r\n")?;
        for (name, values) in &self.headers {
            formatter.write_str("    ")?;
            formatter.write_str(name)?;
            formatter.write_str(": ")?;
            formatter.write_str(&values.join(","))?;
            formatter.write_str("\r\n")?;
        }
        formatter.write_str("Body: \r\n    ")?;
        if let Some(body) = &self.body {
            formatter.write_str(&self.charset.decode(body).0)?;
        } else {
            formatter.write_str("null")?;
        }
        formatter.write_str("\r\n")
    }
}

use super::{HTTP_1_0, HTTP_1_1};
