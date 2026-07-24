//! SSL facade，对齐 hutool 的 `cn.hutool.http.ssl.*`。
//!
//! 提供定制协议的 SSLSocketFactory 抽象 + 信任任意主机名验证器。
//! 具体实现依赖 `javax.net.ssl.SSLSocketFactory`，Rust 用 reqwest 内置 rustls 替代。

use std::any::Any;
use std::io;

use super::custom_protocols_ssl_factory::CustomProtocolsSslFactory;

/// 简单实现：仅记录 protocol 列表，不创建真实 SSLSocket
pub struct CustomProtocolsSslFactoryImpl {
    protocols: Vec<String>,
}

impl CustomProtocolsSslFactoryImpl {
    /// 对齐 `CustomProtocolsSSLFactory(String... protocols)`
    pub fn new(protocols: &[&str]) -> Self {
        Self {
            protocols: protocols.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// 获取配置的协议列表
    pub fn protocols(&self) -> &[String] {
        &self.protocols
    }
}

impl CustomProtocolsSslFactory for CustomProtocolsSslFactoryImpl {
    fn get_default_cipher_suites(&self) -> Vec<String> {
        // 返回常见 cipher suites（与 rustls 默认对齐）
        vec![
            "TLS_AES_128_GCM_SHA256".into(),
            "TLS_AES_256_GCM_SHA384".into(),
            "TLS_CHACHA20_POLY1305_SHA256".into(),
        ]
    }

    fn get_supported_cipher_suites(&self) -> Vec<String> {
        self.get_default_cipher_suites()
    }

    fn create_socket(&self) -> io::Result<Box<dyn Any>> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "CustomProtocolsSslFactoryImpl::create_socket requires javax.net.ssl; use reqwest rustls in Rust",
        ))
    }

    fn create_socket_wrapped(
        &self,
        _socket: Box<dyn Any>,
        _host: &str,
        _port: u16,
        _auto_close: bool,
    ) -> io::Result<Box<dyn Any>> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "CustomProtocolsSslFactoryImpl::create_socket_wrapped requires javax.net.ssl",
        ))
    }

    fn create_socket_host(&self, _host: &str, _port: u16) -> io::Result<Box<dyn Any>> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "create_socket_host requires javax.net.ssl",
        ))
    }

    fn create_socket_host_with_client(
        &self,
        _host: &str,
        _port: u16,
        _client_host: &str,
        _client_port: u16,
    ) -> io::Result<Box<dyn Any>> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "create_socket_host_with_client requires javax.net.ssl",
        ))
    }

    fn create_socket_inet(&self, _host: &str, _port: u16) -> io::Result<Box<dyn Any>> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "create_socket_inet requires javax.net.ssl",
        ))
    }

    fn create_socket_inet_with_client(
        &self,
        _host: &str,
        _port: u16,
        _client_host: &str,
        _client_port: u16,
    ) -> io::Result<Box<dyn Any>> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "create_socket_inet_with_client requires javax.net.ssl",
        ))
    }
}
