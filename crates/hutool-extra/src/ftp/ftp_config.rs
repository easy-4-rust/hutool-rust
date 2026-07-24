//! FTP POJO facade，对齐 hutool 的 `cn.hutool.extra.ftp.*`。
//!
//! **仅实现 POJO 部分**（FtpConfig、FtpMode、FtpException、AbstractFtp trait）。
//! 具体 FTP 协议操作（Ftp/SimpleFtpServer）需要外部 Java crate（Apache Commons Net /
//! Apache FtpServer），属于 unsafe-to-copy。

use std::fmt;

use crate::HutoolException;

/// FTP 配置 POJO，对齐 `cn.hutool.extra.ftp.FtpConfig`。
///
/// 所有 setter 返回 `&mut Self` 以匹配 Java 的链式 API（`FtpConfig.setXxx` 返回 this）。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FtpConfig {
    host: Option<String>,
    port: u16,
    user: Option<String>,
    password: Option<String>,
    charset: Option<String>,
    /// 连接超时（毫秒）
    connection_timeout: i64,
    /// Socket 超时（毫秒）
    so_timeout: i64,
    server_language_code: Option<String>,
    system_key: Option<String>,
}

impl FtpConfig {
    /// 对齐 `FtpConfig.create()` 静态工厂
    pub fn create() -> Self {
        Self::default()
    }

    /// 对齐 `FtpConfig()` 默认构造
    pub fn new() -> Self {
        Self::default()
    }

    /// 对齐 `FtpConfig(String host, int port, String user, String password, Charset charset)`
    pub fn new_simple(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        charset: &str,
    ) -> Self {
        Self {
            host: Some(host.to_string()),
            port,
            user: Some(user.to_string()),
            password: Some(password.to_string()),
            charset: Some(charset.to_string()),
            ..Default::default()
        }
    }

    /// 对齐 `FtpConfig(String, int, String, String, Charset, String, String)` 全参数构造
    pub fn new_full(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        charset: &str,
        server_language_code: &str,
        system_key: &str,
    ) -> Self {
        Self {
            host: Some(host.to_string()),
            port,
            user: Some(user.to_string()),
            password: Some(password.to_string()),
            charset: Some(charset.to_string()),
            server_language_code: Some(server_language_code.to_string()),
            system_key: Some(system_key.to_string()),
            ..Default::default()
        }
    }

    pub fn get_host(&self) -> Option<&str> {
        self.host.as_deref()
    }
    pub fn set_host(&mut self, host: &str) -> &mut Self {
        self.host = Some(host.to_string());
        self
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }

    pub fn get_user(&self) -> Option<&str> {
        self.user.as_deref()
    }
    pub fn set_user(&mut self, user: &str) -> &mut Self {
        self.user = Some(user.to_string());
        self
    }

    pub fn get_password(&self) -> Option<&str> {
        self.password.as_deref()
    }
    pub fn set_password(&mut self, password: &str) -> &mut Self {
        self.password = Some(password.to_string());
        self
    }

    pub fn get_charset(&self) -> Option<&str> {
        self.charset.as_deref()
    }
    pub fn set_charset(&mut self, charset: &str) -> &mut Self {
        self.charset = Some(charset.to_string());
        self
    }

    pub fn get_connection_timeout(&self) -> i64 {
        self.connection_timeout
    }
    pub fn set_connection_timeout(&mut self, timeout_ms: i64) -> &mut Self {
        self.connection_timeout = timeout_ms;
        self
    }

    pub fn get_so_timeout(&self) -> i64 {
        self.so_timeout
    }
    pub fn set_so_timeout(&mut self, timeout_ms: i64) -> &mut Self {
        self.so_timeout = timeout_ms;
        self
    }

    pub fn get_server_language_code(&self) -> Option<&str> {
        self.server_language_code.as_deref()
    }
    pub fn set_server_language_code(&mut self, code: &str) -> &mut Self {
        self.server_language_code = Some(code.to_string());
        self
    }

    pub fn get_system_key(&self) -> Option<&str> {
        self.system_key.as_deref()
    }
    pub fn set_system_key(&mut self, key: &str) -> &mut Self {
        self.system_key = Some(key.to_string());
        self
    }
}
