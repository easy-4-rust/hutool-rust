//! Server facade，对齐 hutool 的 `cn.hutool.http.server.*`。
//!
//! 提供嵌入式 HTTP 服务器抽象。
//! 具体实现依赖 `com.sun.net.httpserver.HttpExchange`（JDK 内置），Rust 用 hyper/axum 替代。

use std::any::Any;
use std::collections::HashMap;
use std::io;
use std::net::SocketAddr;

use crate::HttpException;

mod simple_server;
mod http_server_base;
mod http_server_request;
mod http_server_response;
mod http_server_response_ext;
mod action;
mod root_action;
mod filter;
mod filter_chain;
mod default_exception_filter;
mod http_exchange_wrapper;

pub use simple_server::SimpleServer;
pub use http_server_base::HttpServerBase;
pub use http_server_request::HttpServerRequest;
pub use http_server_response::HttpServerResponse;
pub use http_server_response_ext::HttpServerResponseExt;
pub use action::Action;
pub use root_action::RootAction;
pub use filter::Filter;
pub use filter_chain::FilterChain;
pub use default_exception_filter::DefaultExceptionFilter;
pub use http_exchange_wrapper::HttpExchangeWrapper;
