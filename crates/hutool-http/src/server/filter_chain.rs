//! Server facade，对齐 hutool 的 `cn.hutool.http.server.*`。
//!
//! 提供嵌入式 HTTP 服务器抽象。
//! 具体实现依赖 `com.sun.net.httpserver.HttpExchange`（JDK 内置），Rust 用 hyper/axum 替代。

use std::any::Any;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;

use crate::HttpException;

use super::filter::Filter;
use super::http_server_request::HttpServerRequest;
use super::http_server_response::HttpServerResponse;

/// 过滤器链，对齐 `cn.hutool.http.server.filter.Filter.Chain`。
pub trait FilterChain: Send + Sync {
    /// 对齐 `Filter.Chain.doFilter(HttpServerRequest, HttpServerResponse)`
    fn do_filter(
        &self,
        req: &dyn HttpServerRequest,
        resp: &mut dyn HttpServerResponse,
    ) -> Result<(), HttpException>;
}
