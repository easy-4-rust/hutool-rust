//! 对齐: `cn.hutool.http.interceptor.HttpInterceptorException`
//! 来源: hutool-http/src/main/java/cn/hutool/http/interceptor/HttpInterceptorException.java
//! 中文说明: HTTP拦截器异常类型，拦截器拒绝操作时抛出

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
