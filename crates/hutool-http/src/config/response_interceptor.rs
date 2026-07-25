//! 对齐: `cn.hutool.http.interceptor.ResponseInterceptor`
//! 来源: hutool-http/src/main/java/cn/hutool/http/interceptor/ResponseInterceptor.java
//! 中文说明: 响应拦截器类型别名，在响应返回前修改响应上下文

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
