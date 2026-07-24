//! HttpConnection facade，对齐 hutool 的 `cn.hutool.http.HttpConnection`。
//!
//! 提供 `java.net.HttpURLConnection` 的包装抽象。
//! 具体实现依赖 JDK HttpURLConnection，Rust 用 reqwest 替代。

use std::collections::HashMap;
use std::io;

use crate::HttpException;

use super::http_connection::HttpConnection;

/// 简单的 Stub 实现，所有方法返回 unsupported
pub struct StubHttpConnection {
    url: String,
    method: String,
    proxy: Option<String>,
}

impl StubHttpConnection {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            method: "GET".to_string(),
            proxy: None,
        }
    }
}

impl HttpConnection for StubHttpConnection {
    fn init_conn(&mut self) -> Result<(), HttpException> {
        Err(HttpException::new("init_conn requires HttpURLConnection"))
    }

    fn get_method(&self) -> &str {
        &self.method
    }

    fn set_method(&mut self, method: &str) -> Result<(), HttpException> {
        self.method = method.to_string();
        Ok(())
    }

    fn get_url(&self) -> &str {
        &self.url
    }

    fn get_proxy(&self) -> Option<&str> {
        self.proxy.as_deref()
    }

    fn get_raw_connection(&self) -> Option<Box<dyn std::any::Any>> {
        None
    }

    fn header_override(
        &mut self,
        _name: &str,
        _value: &str,
        _is_override: bool,
    ) -> Result<(), HttpException> {
        Err(HttpException::new("header requires HttpURLConnection"))
    }

    fn header(&self, _name: &str) -> Option<String> {
        None
    }

    fn headers(&self) -> HashMap<String, Vec<String>> {
        HashMap::new()
    }

    fn set_https_info(
        &mut self,
        _verifier: &dyn std::any::Any,
        _ssl_factory: &dyn std::any::Any,
    ) -> Result<(), HttpException> {
        Err(HttpException::new("set_https_info requires HttpURLConnection"))
    }

    fn disable_cache(&mut self) -> Result<(), HttpException> {
        Ok(())
    }

    fn set_connect_timeout(&mut self, _timeout_ms: u32) -> Result<(), HttpException> {
        Ok(())
    }

    fn set_read_timeout(&mut self, _timeout_ms: u32) -> Result<(), HttpException> {
        Ok(())
    }

    fn set_connection_and_read_timeout(&mut self, _timeout_ms: u32) -> Result<(), HttpException> {
        Ok(())
    }

    fn set_cookie(&mut self, _cookie: &str) -> Result<(), HttpException> {
        Ok(())
    }

    fn set_fixed_length_streaming_mode(
        &mut self,
        _content_length: u64,
    ) -> Result<(), HttpException> {
        Ok(())
    }

    fn set_chunked_streaming_mode(&mut self, _block_size: i32) -> Result<(), HttpException> {
        Ok(())
    }

    fn set_instance_follow_redirects(&mut self, _follow: bool) -> Result<(), HttpException> {
        Ok(())
    }

    fn connect(&mut self) -> Result<(), HttpException> {
        Err(HttpException::new("connect requires HttpURLConnection"))
    }

    fn disconnect_quietly(&mut self) {}

    fn disconnect(&mut self) -> Result<(), HttpException> {
        Ok(())
    }

    fn get_input_stream(&self) -> Result<Box<dyn io::Read>, HttpException> {
        Err(HttpException::new("get_input_stream requires HttpURLConnection"))
    }

    fn get_output_stream(&self) -> Result<Box<dyn io::Write>, HttpException> {
        Err(HttpException::new("get_output_stream requires HttpURLConnection"))
    }

    fn get_response_code(&self) -> Result<i32, HttpException> {
        Err(HttpException::new("get_response_code requires HttpURLConnection"))
    }

    fn get_response(&self) -> Result<String, HttpException> {
        Err(HttpException::new("get_response requires HttpURLConnection"))
    }

    fn get_header(&self, _name: &str) -> Option<String> {
        None
    }
}
