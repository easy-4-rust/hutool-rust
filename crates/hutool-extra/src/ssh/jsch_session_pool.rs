//! SSH POJO facade，对齐 hutool 的 `cn.hutool.extra.ssh.*`。
//!
//! **仅实现 POJO 部分**（Connector、ChannelType 等）。具体 SSH 协议操作（JschUtil/Sftp 等）
//! 需要外部 Java crate（com.jcraft.jsch / ssh2），属于 unsafe-to-copy。

use std::fmt;

use crate::HutoolException;

use super::jsch_util::JschUtil;

/// SSH 会话池接口，对齐 `cn.hutool.extra.ssh.JschSessionPool`。
///
/// 具体 Session 类型（jcraft.jsch.Session）是 Java-only，Rust 用 trait object 替代。
pub trait JschSessionPool: Send + Sync {
    /// 对齐 `JschSessionPool.get(String)`
    fn get(&self, key: &str) -> Option<Box<dyn std::any::Any>>;

    /// 对齐 `JschSessionPool.put(...)`（已在 JschUtil 中实现）
    fn put_session(&self, key: &str, session: Box<dyn std::any::Any>);

    /// 对齐 `JschSessionPool.remove(Session)`
    fn remove(&self, session: &dyn std::any::Any) -> bool;

    /// 对齐 `JschSessionPool.close(String)`
    fn close(&self, key: &str);

    /// 对齐 `JschSessionPool.closeAll()`
    fn close_all(&self);
}
