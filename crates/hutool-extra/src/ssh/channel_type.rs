//! SSH POJO facade，对齐 hutool 的 `cn.hutool.extra.ssh.*`。
//!
//! **仅实现 POJO 部分**（Connector、ChannelType 等）。具体 SSH 协议操作（JschUtil/Sftp 等）
//! 需要外部 Java crate（com.jcraft.jsch / ssh2），属于 unsafe-to-copy。

use std::fmt;

use crate::HutoolException;

/// SSH 通道类型枚举，对齐 `cn.hutool.extra.ssh.ChannelType`。
///
/// Java 是 enum + getValue()；Rust 用 enum + `as_str()` 方法。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChannelType {
    /// Shell 通道
    Shell,
    /// SFTP 通道
    Sftp,
    /// Exec 通道
    Exec,
    /// Direct TCP/IP 通道
    DirectTcpip,
}

impl ChannelType {
    /// 对齐 `getValue()`
    pub fn get_value(&self) -> &'static str {
        match self {
            ChannelType::Shell => "shell",
            ChannelType::Sftp => "sftp",
            ChannelType::Exec => "exec",
            ChannelType::DirectTcpip => "direct-tcpip",
        }
    }
}

impl fmt::Display for ChannelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_value())
    }
}
