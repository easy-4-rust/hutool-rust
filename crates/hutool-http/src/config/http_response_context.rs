//! 对齐: `cn.hutool.http.HttpResponseContext`
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpResponseContext.java
//! 中文说明: 响应拦截器上下文，暴露可变的响应状态码和头部信息

use reqwest::{StatusCode, header::HeaderMap};

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
