//! SSH POJO facade，对齐 hutool 的 `cn.hutool.extra.ssh.*`。
//!
//! **仅实现 POJO 部分**（Connector、ChannelType 等）。具体 SSH 协议操作（JschUtil/Sftp 等）
//! 需要外部 Java crate（com.jcraft.jsch / ssh2），属于 unsafe-to-copy。

use std::fmt;

use crate::HutoolException;

/// SSH 运行时异常，对齐 `cn.hutool.extra.ssh.JschRuntimeException`。
///
/// Rust 用 `HutoolException` 别名实现统一异常层次。
pub type JschRuntimeException = HutoolException;
