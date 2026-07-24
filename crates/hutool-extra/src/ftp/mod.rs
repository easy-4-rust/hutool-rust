//! FTP POJO facade，对齐 hutool 的 `cn.hutool.extra.ftp.*`。
//!
//! **仅实现 POJO 部分**（FtpConfig、FtpMode、FtpException、AbstractFtp trait）。
//! 具体 FTP 协议操作（Ftp/SimpleFtpServer）需要外部 Java crate（Apache Commons Net /
//! Apache FtpServer），属于 unsafe-to-copy。

use std::fmt;

use crate::HutoolException;

mod ftp_mode;
mod ftp_config;
mod ftp_exception;
mod abstract_ftp;

pub use ftp_mode::FtpMode;
pub use ftp_config::FtpConfig;
pub use ftp_exception::FtpException;
pub use abstract_ftp::AbstractFtp;
