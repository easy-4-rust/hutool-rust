//! HttpConnection facade，对齐 hutool 的 `cn.hutool.http.HttpConnection`。
//!
//! 提供 `java.net.HttpURLConnection` 的包装抽象。
//! 具体实现依赖 JDK HttpURLConnection，Rust 用 reqwest 替代。

use std::collections::HashMap;
use std::io;

use crate::HttpException;

/// HTTP 连接，对齐 `cn.hutool.http.HttpConnection`。
///
/// Java 包装 `java.net.HttpURLConnection`；Rust 用 trait 提供形状。
pub trait HttpConnection: Send + Sync {
    /// 对齐 `HttpConnection.initConn()`
    fn init_conn(&mut self) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.getMethod()`
    fn get_method(&self) -> &str;

    /// 对齐 `HttpConnection.setMethod(Method)`
    fn set_method(&mut self, method: &str) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.getUrl()`
    fn get_url(&self) -> &str;

    /// 对齐 `HttpConnection.getProxy()`
    fn get_proxy(&self) -> Option<&str>;

    /// 对齐 `HttpConnection.getHttpURLConnection()` — Rust 版返回 Option
    fn get_raw_connection(&self) -> Option<Box<dyn std::any::Any>>;

    /// 对齐 `HttpConnection.header(String, String, boolean)`
    fn header_override(
        &mut self,
        name: &str,
        value: &str,
        is_override: bool,
    ) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.header(String)` 单参数读取
    fn header(&self, name: &str) -> Option<String>;

    /// 对齐 `HttpConnection.headers()`
    fn headers(&self) -> HashMap<String, Vec<String>>;

    /// 对齐 `HttpConnection.setHttpsInfo(HostnameVerifier, SSLSocketFactory)`
    fn set_https_info(
        &mut self,
        verifier: &dyn std::any::Any,
        ssl_factory: &dyn std::any::Any,
    ) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.disableCache()`
    fn disable_cache(&mut self) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.setConnectTimeout(int)`
    fn set_connect_timeout(&mut self, timeout_ms: u32) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.setReadTimeout(int)`
    fn set_read_timeout(&mut self, timeout_ms: u32) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.setConnectionAndReadTimeout(int)`
    fn set_connection_and_read_timeout(&mut self, timeout_ms: u32) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.setCookie(String)`
    fn set_cookie(&mut self, cookie: &str) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.setFixedLengthStreamingMode(long)`
    fn set_fixed_length_streaming_mode(&mut self, content_length: u64) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.setChunkedStreamingMode(int)`
    fn set_chunked_streaming_mode(&mut self, block_size: i32) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.setInstanceFollowRedirects(boolean)`
    fn set_instance_follow_redirects(&mut self, follow: bool) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.connect()`
    fn connect(&mut self) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.disconnectQuietly()`
    fn disconnect_quietly(&mut self);

    /// 对齐 `HttpConnection.disconnect()`
    fn disconnect(&mut self) -> Result<(), HttpException>;

    /// 对齐 `HttpConnection.getInputStream()`
    fn get_input_stream(&self) -> Result<Box<dyn io::Read>, HttpException>;

    /// 对齐 `HttpConnection.getOutputStream()`
    fn get_output_stream(&self) -> Result<Box<dyn io::Write>, HttpException>;

    /// 对齐 `HttpConnection.getResponseCode()`
    fn get_response_code(&self) -> Result<i32, HttpException>;

    /// 对齐 `HttpConnection.getResponse()`
    fn get_response(&self) -> Result<String, HttpException>;

    /// 对齐 `HttpConnection.getHeader(String)` (在 response 中查找)
    fn get_header(&self, name: &str) -> Option<String>;
}
