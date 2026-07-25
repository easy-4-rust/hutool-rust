//! Hutool-aligned HTTP configuration backed by Reqwest and Rustls.

use reqwest::{Method, StatusCode, Url, header::HeaderMap, tls::Version};
use std::{fmt, sync::Arc, time::Duration};
use thiserror::Error;

/// Error returned by a configured request or response interceptor.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("HTTP interceptor rejected operation: {message}")]
pub struct HttpInterceptorError {
    message: String,
}

impl HttpInterceptorError {
    /// Creates an interceptor failure with a bounded owned message.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Returns the failure message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

use super::{duration};
