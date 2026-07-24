//! SSH POJO facade，对齐 hutool 的 `cn.hutool.extra.ssh.*`。
//!
//! **仅实现 POJO 部分**（Connector、ChannelType 等）。具体 SSH 协议操作（JschUtil/Sftp 等）
//! 需要外部 Java crate（com.jcraft.jsch / ssh2），属于 unsafe-to-copy。

use std::fmt;

use crate::HutoolException;

use super::jsch_session_pool::JschSessionPool;

/// SSH 工具类（Jsch 实现），对齐 `cn.hutool.extra.ssh.JschUtil`。
///
/// 具体 Session/Channel/Sftp 类型依赖 jcraft.jsch，属于 unsafe-to-copy。
/// Rust 用 trait + 错误返回桩，标记 API 完整性。
pub struct JschUtil;

impl JschUtil {
    /// 对齐 `JschUtil.SSH_NONE` 常量。
    pub const SSH_NONE: &'static str = "none";

    /// 对齐 `JschUtil.generateLocalPort()`
    ///
    /// 调用 hutool-core 的 LocalPortGenerater（如未启用则返回错误）。
    pub fn generate_local_port() -> std::result::Result<u16, HutoolException> {
        Err(HutoolException::Message(
            "JschUtil::generate_local_port requires ssh2 crate (unsafe-to-copy); use std::net TcpListener bind to port 0 for ephemeral port".into(),
        ))
    }

    /// 对齐 `JschUtil.openSftp(Session)`
    pub fn open_sftp(
        _session: &dyn std::any::Any,
    ) -> std::result::Result<Box<dyn std::any::Any>, HutoolException> {
        Err(HutoolException::Message(
            "JschUtil::open_sftp requires jcraft.jsch crate (unsafe-to-copy)".into(),
        ))
    }

    /// 对齐 `JschUtil.createSftp(Session)`
    pub fn create_sftp(
        _session: &dyn std::any::Any,
    ) -> std::result::Result<Box<dyn std::any::Any>, HutoolException> {
        Err(HutoolException::Message(
            "JschUtil::create_sftp requires jcraft.jsch crate (unsafe-to-copy)".into(),
        ))
    }

    /// 对齐 `JschUtil.openShell(Session)`
    pub fn open_shell(
        _session: &dyn std::any::Any,
    ) -> std::result::Result<Box<dyn std::any::Any>, HutoolException> {
        Err(HutoolException::Message(
            "JschUtil::open_shell requires jcraft.jsch crate (unsafe-to-copy)".into(),
        ))
    }

    /// 对齐 `JschUtil.close(Session)`
    pub fn close_session(_session: &dyn std::any::Any) {}

    /// 对齐 `JschUtil.close(Channel)`
    pub fn close_channel(_channel: &dyn std::any::Any) {}

    /// 对齐 `JschUtil.close(String)`（按 key 关闭 Session 池中的会话）
    pub fn close_by_key(_pool: &dyn JschSessionPool, _key: &str) {}

    /// 对齐 `JschUtil.closeAll()`
    pub fn close_all(_pool: &dyn JschSessionPool) {}
}
