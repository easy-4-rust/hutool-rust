//! Hutool-aligned HTTP configuration backed by Reqwest and Rustls.

use reqwest::{Method, StatusCode, Url, header::HeaderMap, tls::Version};
use std::{fmt, sync::Arc, time::Duration};
use thiserror::Error;

/// Explicit hostname-verification policy for TLS connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HostnameVerification {
    /// Verify certificate hostnames using Rustls and `WebPKI`.
    #[default]
    Strict,
    /// Accept invalid certificate hostnames. This is dangerous outside tests.
    DangerousAcceptInvalid,
}

use super::{duration};
