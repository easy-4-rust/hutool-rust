//! Server facade，对齐 hutool 的 `cn.hutool.http.server.*`。
//!
//! 提供嵌入式 HTTP 服务器抽象。
//! 具体实现依赖 `com.sun.net.httpserver.HttpExchange`（JDK 内置），Rust 用 hyper/axum 替代。

use std::any::Any;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;

use crate::HttpException;

/// HTTP 服务器请求，对齐 `cn.hutool.http.server.HttpServerRequest`。
pub trait HttpServerRequest: Send + Sync {
    /// 对齐 `HttpServerRequest.getMethod()`
    fn get_method(&self) -> &str;

    /// 对齐 `HttpServerRequest.getURL()`
    fn get_url(&self) -> &str;

    /// 对齐 `HttpServerRequest.getPath()`
    fn get_path(&self) -> &str;

    /// 对齐 `HttpServerRequest.getQuery()`
    fn get_query(&self) -> &str;

    /// 对齐 `HttpServerRequest.getHeader(String)`
    fn get_header(&self, name: &str) -> Option<String>;

    /// 对齐 `HttpServerRequest.getHeaders()`
    fn get_headers(&self) -> HashMap<String, Vec<String>>;

    /// 对齐 `HttpServerRequest.getBody()`
    fn get_body(&self) -> Result<String, HttpException>;

    /// 对齐 `HttpServerRequest.getBodyBytes()`
    fn get_body_bytes(&self) -> Result<Vec<u8>, HttpException>;

    /// 对齐 `HttpServerRequest.getParam(String)`
    fn get_param(&self, name: &str) -> Option<String>;

    /// 对齐 `HttpServerRequest.getParamMap()`
    fn get_param_map(&self) -> HashMap<String, Vec<String>>;

    /// 对齐 `HttpServerRequest.getRemoteAddr()`
    fn get_remote_addr(&self) -> Option<SocketAddr>;

    /// 对齐 `HttpServerRequest.getRemoteClientIP()`
    fn get_remote_client_ip(&self) -> Option<String> {
        self.get_remote_addr().map(|a| a.ip().to_string())
    }

    /// 对齐 `HttpServerRequest.getCookieValue(String)`
    fn get_cookie_value(&self, name: &str) -> Option<String>;

    /// 对齐 `HttpServerRequest.isGetMethod()`
    fn is_get_method(&self) -> bool {
        self.get_method().eq_ignore_ascii_case("GET")
    }

    /// 对齐 `HttpServerRequest.isPostMethod()`
    fn is_post_method(&self) -> bool {
        self.get_method().eq_ignore_ascii_case("POST")
    }

    /// 对齐 `HttpServerRequest.isMultipart()`
    fn is_multipart(&self) -> bool {
        self.get_header("Content-Type")
            .map(|c| c.starts_with("multipart/"))
            .unwrap_or(false)
    }
}
