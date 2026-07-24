//! SSH POJO facade，对齐 hutool 的 `cn.hutool.extra.ssh.*`。
//!
//! **仅实现 POJO 部分**（Connector、ChannelType 等）。具体 SSH 协议操作（JschUtil/Sftp 等）
//! 需要外部 Java crate（com.jcraft.jsch / ssh2），属于 unsafe-to-copy。

use std::fmt;

use crate::HutoolException;

/// SSH 连接信息 POJO，对齐 `cn.hutool.extra.ssh.Connector`。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Connector {
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    password: Option<String>,
    group: Option<String>,
}

impl Connector {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_credentials(user: &str, password: &str, group: &str) -> Self {
        Self {
            user: Some(user.to_string()),
            password: Some(password.to_string()),
            group: Some(group.to_string()),
            ..Default::default()
        }
    }
    pub fn with_endpoint(host: &str, port: u16, user: &str, password: &str) -> Self {
        Self {
            host: Some(host.to_string()),
            port: Some(port),
            user: Some(user.to_string()),
            password: Some(password.to_string()),
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
    pub fn get_port(&self) -> Option<u16> {
        self.port
    }
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = Some(port);
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
    pub fn get_group(&self) -> Option<&str> {
        self.group.as_deref()
    }
    pub fn set_group(&mut self, group: &str) -> &mut Self {
        self.group = Some(group.to_string());
        self
    }
}

impl fmt::Display for Connector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Connector [host={}, port={}, user={}, password={}]",
            self.host.as_deref().unwrap_or("null"),
            self.port.map(|p| p.to_string()).unwrap_or_else(|| "null".into()),
            self.user.as_deref().unwrap_or("null"),
            self.password.as_deref().unwrap_or("null"),
        )
    }
}
