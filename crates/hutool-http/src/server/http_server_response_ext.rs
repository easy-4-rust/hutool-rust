//! Server facade，对齐 hutool 的 `cn.hutool.http.server.*`。
//!
//! 提供嵌入式 HTTP 服务器抽象。
//! 具体实现依赖 `com.sun.net.httpserver.HttpExchange`（JDK 内置），Rust 用 hyper/axum 替代。

use std::any::Any;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;

use crate::HttpException;

use super::http_server_response::HttpServerResponse;

/// HttpServerResponse 扩展 trait，提供链式 API（独立 trait 保持 dyn-compatibility）
pub trait HttpServerResponseExt: HttpServerResponse {
    /// 对齐 `HttpServerResponse.print(String)`
    fn print(&mut self, body: &str) -> Result<(), HttpException> {
        self.write(body.as_bytes())
    }

    /// 对齐 `HttpServerResponse.send(int, String)` 简化
    fn send(&mut self, status: u16, body: &str) -> Result<(), HttpException> {
        self.set_status(status);
        self.print(body)
    }
}

impl<T: HttpServerResponse + ?Sized> HttpServerResponseExt for T {}
