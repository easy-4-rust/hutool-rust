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
use super::filter_chain::FilterChain;
use super::http_server_request::HttpServerRequest;
use super::http_server_response::HttpServerResponse;

/// 默认异常过滤器，对齐 `cn.hutool.http.server.filter.DefaultExceptionFilter`。
///
/// 捕获异常并返回 500。
pub struct DefaultExceptionFilter;

impl Filter for DefaultExceptionFilter {
    fn do_filter(
        &self,
        req: &dyn HttpServerRequest,
        resp: &mut dyn HttpServerResponse,
        chain: &dyn FilterChain,
    ) -> Result<(), HttpException> {
        // 先走 chain，出错则改 status
        match chain.do_filter(req, resp) {
            Ok(()) => Ok(()),
            Err(e) => {
                resp.set_status(500);
                let _ = resp.write(e.to_string().as_bytes());
                Ok(())
            }
        }
    }
}
