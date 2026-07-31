//! 对齐: `cn.hutool.http.HttpRequestContext`
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpRequestContext.java
//! 中文说明: 请求拦截器上下文，暴露可变的请求方法、URL和头部信息

use reqwest::{Method, Url, header::HeaderMap};

/// Mutable request metadata exposed to application interceptors.
#[derive(Debug, Clone)]
pub struct HttpRequestContext {
    method: Method,
    url: Url,
    headers: HeaderMap,
}

impl HttpRequestContext {
    pub(crate) fn new(method: Method, url: Url, headers: HeaderMap) -> Self {
        Self {
            method,
            url,
            headers,
        }
    }

    /// Returns the request method.
    #[must_use]
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// Replaces the request method.
    pub fn set_method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }

    /// Returns the destination URL.
    #[must_use]
    pub const fn url(&self) -> &Url {
        &self.url
    }

    /// Replaces the destination URL.
    pub fn set_url(&mut self, url: Url) -> &mut Self {
        self.url = url;
        self
    }

    /// Returns request headers.
    #[must_use]
    pub const fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Returns mutable request headers.
    pub const fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    pub(crate) fn into_parts(self) -> (Method, Url, HeaderMap) {
        (self.method, self.url, self.headers)
    }
}
