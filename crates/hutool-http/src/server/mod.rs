//! Server facade，对齐 hutool 的 `cn.hutool.http.server.*`。
//!
//! 提供嵌入式 HTTP 服务器抽象。
//! 具体实现依赖 `com.sun.net.httpserver.HttpExchange`（JDK 内置），Rust 用 hyper/axum 替代。

mod action;
mod default_exception_filter;
mod filter;
mod filter_chain;
mod http_exchange_wrapper;
mod http_server_base;
mod http_server_request;
mod http_server_response;
mod http_server_response_ext;
mod root_action;
mod simple_server;

pub use action::Action;
pub use default_exception_filter::DefaultExceptionFilter;
pub use filter::Filter;
pub use filter_chain::FilterChain;
pub use http_exchange_wrapper::HttpExchangeWrapper;
pub use http_server_base::HttpServerBase;
pub use http_server_request::HttpServerRequest;
pub use http_server_response::HttpServerResponse;
pub use http_server_response_ext::HttpServerResponseExt;
pub use root_action::RootAction;
pub use simple_server::SimpleServer;
