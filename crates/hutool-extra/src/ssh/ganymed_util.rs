//! SSH POJO facade，对齐 hutool 的 `cn.hutool.extra.ssh.*`。
//!
//! **仅实现 POJO 部分**（Connector、ChannelType 等）。具体 SSH 协议操作（JschUtil/Sftp 等）
//! 需要外部 Java crate（com.jcraft.jsch / ssh2），属于 unsafe-to-copy。

use std::fmt;

use crate::HutoolException;

/// Ganymed SSH 工具类，对齐 `cn.hutool.extra.ssh.GanymedUtil`。
///
/// 依赖 ch.ethz.ssh2（Ganymed SSH-2 for Java），属于 unsafe-to-copy。
pub struct GanymedUtil;

impl GanymedUtil {
    /// 对齐 `GanymedUtil.close(Session)`
    pub fn close(_session: &dyn std::any::Any) {}
}
