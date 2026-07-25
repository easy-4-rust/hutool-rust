//! Hutool-aligned HTTP configuration backed by Reqwest and Rustls.

use reqwest::{Method, StatusCode, Url, header::HeaderMap, tls::Version};
use std::{fmt, sync::Arc, time::Duration};
use thiserror::Error;

use super::http_interceptor_error::HttpInterceptorError;
use super::http_request_context::HttpRequestContext;

/// Shared request-interceptor callback.
pub type RequestInterceptor = Arc<
    dyn Fn(&mut HttpRequestContext) -> Result<(), HttpInterceptorError> + Send + Sync + 'static,
>;

use super::{duration};
