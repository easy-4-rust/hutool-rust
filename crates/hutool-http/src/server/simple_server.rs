//! Server facade，对齐 hutool 的 `cn.hutool.http.server.*`。
//!
//! 提供嵌入式 HTTP 服务器抽象。
//! 具体实现依赖 `com.sun.net.httpserver.HttpExchange`（JDK 内置），Rust 用 hyper/axum 替代。

use std::any::Any;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;

use crate::HttpException;

use super::action::Action;
use super::filter::Filter;

/// 简单 HTTP 服务器，对齐 `cn.hutool.http.server.SimpleServer`。
///
/// Java 基于 `com.sun.net.httpserver.HttpServer`；Rust 推荐 axum/actix-web。
pub trait SimpleServer: Send + Sync {
    /// 对齐 `SimpleServer.addAction(String path, Action action)`
    fn add_action(&mut self, path: &str, action: Box<dyn Action>) -> Result<(), HttpException>;

    /// 对齐 `SimpleServer.addFilter(Filter filter)`
    fn add_filter(&mut self, filter: Box<dyn Filter>) -> Result<(), HttpException>;

    /// 对齐 `SimpleServer.setPort(int)`
    fn set_port(&mut self, port: u16) -> &mut Self;

    /// 对齐 `SimpleServer.getPort()`
    fn get_port(&self) -> u16;

    /// 对齐 `SimpleServer.start()`
    fn start(&self) -> Result<(), HttpException>;

    /// 对齐 `SimpleServer.stop()`
    fn stop(&self) -> Result<(), HttpException>;
}
