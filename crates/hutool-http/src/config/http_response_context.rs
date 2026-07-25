//! Hutool-aligned HTTP configuration backed by Reqwest and Rustls.

use reqwest::{Method, StatusCode, Url, header::HeaderMap, tls::Version};
use std::{fmt, sync::Arc, time::Duration};
use thiserror::Error;

/// Mutable response metadata exposed to application interceptors.
#[derive(Debug, Clone)]
pub struct HttpResponseContext {
    status: StatusCode,
    headers: HeaderMap,
}

impl HttpResponseContext {
    pub(crate) fn new(status: StatusCode, headers: HeaderMap) -> Self {
        Self { status, headers }
    }

    /// Returns the immutable response status.
    #[must_use]
    pub const fn status(&self) -> StatusCode {
        self.status
    }

    /// Returns response headers.
    #[must_use]
    pub const fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Returns mutable response headers.
    pub const fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    pub(crate) fn into_headers(self) -> HeaderMap {
        self.headers
    }
}

use super::{duration};
