//! SSH facade，对齐 hutool 的 `cn.hutool.extra.ssh.*`。
//!
//! - POJO：`Connector`/`ChannelType`/`JschRuntimeException`/`JschSessionPool`/`JschUtil`/`GanymedUtil`
//! - 实现：`SshSession`（feature `ssh`，基于 [`ssh2`]，对齐 `JSch` exec/sftp 语义）

mod channel_type;
mod connector;
mod ganymed_util;
mod jsch_runtime_exception;
mod jsch_session_pool;
mod jsch_util;

#[cfg(feature = "ssh")]
mod ssh_client;

pub use channel_type::ChannelType;
pub use connector::Connector;
pub use ganymed_util::GanymedUtil;
pub use jsch_runtime_exception::JschRuntimeException;
pub use jsch_session_pool::JschSessionPool;
pub use jsch_util::JschUtil;

#[cfg(feature = "ssh")]
pub use ssh_client::SshSession;
