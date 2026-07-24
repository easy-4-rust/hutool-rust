//! SSL facade，对齐 hutool 的 `cn.hutool.http.ssl.*`。
//!
//! 提供定制协议的 SSLSocketFactory 抽象 + 信任任意主机名验证器。
//! 具体实现依赖 `javax.net.ssl.SSLSocketFactory`，Rust 用 reqwest 内置 rustls 替代。

use std::any::Any;
use std::io;

/// 自定义协议 SSL 工厂，对齐 `cn.hutool.http.ssl.CustomProtocolsSSLFactory`。
///
/// Java 继承 `SSLSocketFactory`；Rust 用 trait 提供形状。
pub trait CustomProtocolsSslFactory: Send + Sync {
    /// 对齐 `getDefaultCipherSuites()`
    fn get_default_cipher_suites(&self) -> Vec<String>;

    /// 对齐 `getSupportedCipherSuites()`
    fn get_supported_cipher_suites(&self) -> Vec<String>;

    /// 对齐 `createSocket()`
    fn create_socket(&self) -> io::Result<Box<dyn Any>>;

    /// 对齐 `createSocket(Socket, String, int, boolean)`
    fn create_socket_wrapped(
        &self,
        socket: Box<dyn Any>,
        host: &str,
        port: u16,
        auto_close: bool,
    ) -> io::Result<Box<dyn Any>>;

    /// 对齐 `createSocket(String, int)`
    fn create_socket_host(&self, host: &str, port: u16) -> io::Result<Box<dyn Any>>;

    /// 对齐 `createSocket(String, int, InetAddress, int)` (InetAddress 用 &str 表示)
    fn create_socket_host_with_client(
        &self,
        host: &str,
        port: u16,
        client_host: &str,
        client_port: u16,
    ) -> io::Result<Box<dyn Any>>;

    /// 对齐 `createSocket(InetAddress, int)` (InetAddress 用 &str 表示)
    fn create_socket_inet(&self, host: &str, port: u16) -> io::Result<Box<dyn Any>>;

    /// 对齐 `createSocket(InetAddress, int, InetAddress, int)` (InetAddress 用 &str 表示)
    fn create_socket_inet_with_client(
        &self,
        host: &str,
        port: u16,
        client_host: &str,
        client_port: u16,
    ) -> io::Result<Box<dyn Any>>;
}
