//! SSH POJO facade，对齐 hutool 的 `cn.hutool.extra.ssh.*`。
//!
//! **仅实现 POJO 部分**（Connector、ChannelType 等）。具体 SSH 协议操作（JschUtil/Sftp 等）
//! 需要外部 Java crate（com.jcraft.jsch / ssh2），属于 unsafe-to-copy。

use std::fmt;

use crate::HutoolException;

mod channel_type;
mod connector;
mod jsch_runtime_exception;
mod jsch_session_pool;
mod jsch_util;
mod ganymed_util;

pub use channel_type::ChannelType;
pub use connector::Connector;
pub use jsch_runtime_exception::JschRuntimeException;
pub use jsch_session_pool::JschSessionPool;
pub use jsch_util::JschUtil;
pub use ganymed_util::GanymedUtil;
