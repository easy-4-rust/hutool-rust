//! Hutool-aligned HTTP configuration backed by Reqwest and Rustls.

use reqwest::{Method, StatusCode, Url, header::HeaderMap, tls::Version};
use std::{fmt, sync::Arc, time::Duration};
use thiserror::Error;

mod hostname_verification;
mod tls_protocol;
mod http_config_error;
mod http_interceptor_error;
mod http_request_context;
mod http_response_context;
mod request_interceptor;
mod response_interceptor;
mod http_config;

pub use hostname_verification::HostnameVerification;
pub use tls_protocol::TlsProtocol;
pub use http_config_error::HttpConfigError;
pub use http_interceptor_error::HttpInterceptorError;
pub use http_request_context::HttpRequestContext;
pub use http_response_context::HttpResponseContext;
pub use request_interceptor::RequestInterceptor;
pub use response_interceptor::ResponseInterceptor;
pub use http_config::HttpConfig;

fn duration(milliseconds: i64) -> Result<Duration, HttpConfigError> {
    let milliseconds =
        u64::try_from(milliseconds).map_err(|_| HttpConfigError::NegativeTimeout(milliseconds))?;
    Ok(Duration::from_millis(milliseconds))
}
