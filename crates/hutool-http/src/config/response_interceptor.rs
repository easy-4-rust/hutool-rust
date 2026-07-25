//! Hutool-aligned HTTP configuration backed by Reqwest and Rustls.

use reqwest::{Method, StatusCode, Url, header::HeaderMap, tls::Version};
use std::{fmt, sync::Arc, time::Duration};
use thiserror::Error;

use super::http_interceptor_error::HttpInterceptorError;
use super::http_response_context::HttpResponseContext;

/// Shared response-interceptor callback.
pub type ResponseInterceptor = Arc<
    dyn Fn(&mut HttpResponseContext) -> Result<(), HttpInterceptorError> + Send + Sync + 'static,
>;

use super::{duration};
