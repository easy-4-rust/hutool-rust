//! Server facade，对齐 hutool 的 `cn.hutool.http.server.*`。
//!
//! 提供嵌入式 HTTP 服务器抽象。
//! 具体实现依赖 `com.sun.net.httpserver.HttpExchange`（JDK 内置），Rust 用 hyper/axum 替代。

use std::any::Any;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;

use crate::HttpException;

/// HttpExchange 包装器，对齐 `cn.hutool.http.server.HttpExchangeWrapper`。
///
/// 依赖 `com.sun.net.httpserver.HttpExchange`，unsafe-to-copy。
pub trait HttpExchangeWrapper: Send + Sync {
    /// 对齐 `HttpExchangeWrapper.getExchange()`
    fn get_exchange(&self) -> Option<Box<dyn Any>>;
}
