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
use super::http_server_request::HttpServerRequest;
use super::http_server_response::HttpServerResponse;

/// 根路径 Action，对齐 `cn.hutool.http.server.action.RootAction`。
///
/// 处理根路径 `/` 的默认 action。
pub struct RootAction;

impl Action for RootAction {
    fn do_action(
        &self,
        _req: &dyn HttpServerRequest,
        resp: &mut dyn HttpServerResponse,
    ) -> Result<(), HttpException> {
        resp.set_status(200);
        resp.write(b"Hello from HiTool RootAction")
    }
}
