//! 对齐: `cn.hutool.http.interceptor.RequestInterceptor`
//! 来源: hutool-http/src/main/java/cn/hutool/http/interceptor/RequestInterceptor.java
//! 中文说明: 请求拦截器类型别名，在请求发送前修改请求上下文

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
