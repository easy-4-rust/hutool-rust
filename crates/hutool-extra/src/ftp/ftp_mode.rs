//! FTP POJO facade，对齐 hutool 的 `cn.hutool.extra.ftp.*`。
//!
//! **仅实现 POJO 部分**（FtpConfig、FtpMode、FtpException、AbstractFtp trait）。
//! 具体 FTP 协议操作（Ftp/SimpleFtpServer）需要外部 Java crate（Apache Commons Net /
//! Apache FtpServer），属于 unsafe-to-copy。

use std::fmt;

use crate::HutoolException;

/// FTP 连接模式，对齐 `cn.hutool.extra.ftp.FtpMode`。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FtpMode {
    /// 主动模式（PORT）
    #[default]
    Active,
    /// 被动模式（PASV）
    Passive,
}

impl fmt::Display for FtpMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FtpMode::Active => write!(f, "Active"),
            FtpMode::Passive => write!(f, "Passive"),
        }
    }
}
