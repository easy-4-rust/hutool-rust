//! Hutool-aligned HTTP configuration backed by Reqwest and Rustls.

use reqwest::{Method, StatusCode, Url, header::HeaderMap, tls::Version};
use std::{fmt, sync::Arc, time::Duration};
use thiserror::Error;

/// TLS protocol versions supported by the Rustls transport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsProtocol {
    /// TLS 1.2 only.
    Tls12,
    /// TLS 1.3 only.
    Tls13,
}

impl TlsProtocol {
    pub(crate) const fn reqwest(self) -> Version {
        match self {
            Self::Tls12 => Version::TLS_1_2,
            Self::Tls13 => Version::TLS_1_3,
        }
    }
}

use super::{duration};
