//! Server facade，对齐 hutool 的 `cn.hutool.http.server.*`。
//!
//! 提供嵌入式 HTTP 服务器抽象。
//! 具体实现依赖 `com.sun.net.httpserver.HttpExchange`（JDK 内置），Rust 用 hyper/axum 替代。

use std::any::Any;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;

use crate::HttpException;

/// HTTP Server 基类，对齐 `cn.hutool.http.server.HttpServerBase`。
pub trait HttpServerBase: Send + Sync {
    /// 对齐 `HttpServerBase.getServer()`
    fn get_server(&self) -> Option<Box<dyn Any>>;

    /// 对齐 `HttpServerBase.init(String host, int port)`
    fn init(&mut self, host: &str, port: u16) -> Result<(), HttpException>;
}
