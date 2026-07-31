//! HttpConnection facade，对齐 hutool 的 `cn.hutool.http.HttpConnection`。
//!
//! 提供 `java.net.HttpURLConnection` 的包装抽象。
//! 具体实现依赖 JDK HttpURLConnection，Rust 用 reqwest 替代。

use crate::HttpException;

mod http_connection;
mod stub_http_connection;

pub use http_connection::HttpConnection;
pub use stub_http_connection::StubHttpConnection;

/// 对齐 Java `HttpConnection.create` 的脚手架工厂，Rust 环境下不可用。
#[allow(dead_code)]
pub fn http_connection_create(
    _url: &str,
    _proxy: Option<&str>,
) -> Result<Box<dyn HttpConnection>, HttpException> {
    Err(HttpException::new(
        "http_connection_create requires java.net.HttpURLConnection; use reqwest in Rust",
    ))
}
