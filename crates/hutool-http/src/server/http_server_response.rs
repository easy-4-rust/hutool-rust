//! Server facade，对齐 hutool 的 `cn.hutool.http.server.*`。
//!
//! 提供嵌入式 HTTP 服务器抽象。
//! 具体实现依赖 `com.sun.net.httpserver.HttpExchange`（JDK 内置），Rust 用 hyper/axum 替代。

use std::any::Any;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;

use crate::HttpException;

/// HTTP 服务器响应，对齐 `cn.hutool.http.server.HttpServerResponse`。
pub trait HttpServerResponse: Send + Sync {
    /// 对齐 `HttpServerResponse.setStatus(int)`
    fn set_status(&mut self, status: u16);

    /// 对齐 `HttpServerResponse.getStatus()`
    fn get_status(&self) -> u16;

    /// 对齐 `HttpServerResponse.setHeader(String, String)`
    fn set_header(&mut self, name: &str, value: &str);

    /// 对齐 `HttpServerResponse.addHeader(String, String)`
    fn add_header(&mut self, name: &str, value: &str);

    /// 对齐 `HttpServerResponse.setHeaders(Map)`
    fn set_headers(&mut self, headers: HashMap<String, String>);

    /// 对齐 `HttpServerResponse.setCookie(String, String)` (简化版)
    fn set_cookie(&mut self, name: &str, value: &str);

    /// 对齐 `HttpServerResponse.setContentType(String)`
    fn set_content_type(&mut self, content_type: &str);

    /// 对齐 `HttpServerResponse.setContentLength(int)` / `setContentLengthLong(long)`
    fn set_content_length(&mut self, length: u64);

    /// 对齐 `HttpServerResponse.write(String)` / `write(byte[])`
    fn write(&mut self, body: &[u8]) -> Result<(), HttpException>;
}
