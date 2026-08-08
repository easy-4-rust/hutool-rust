//! FTP facade，对齐 hutool 的 `cn.hutool.extra.ftp.*`。
//!
//! - POJO：`FtpConfig`/`FtpMode`/`FtpException`/`AbstractFtp` trait
//! - 实现：`FtpClient`（feature `ftp`，基于 [`suppaftp`]，对齐 commons-net `Ftp`）

mod abstract_ftp;
mod ftp_config;
mod ftp_exception;
mod ftp_mode;

#[cfg(feature = "ftp")]
mod ftp_client;

pub use abstract_ftp::AbstractFtp;
pub use ftp_config::FtpConfig;
pub use ftp_exception::FtpException;
pub use ftp_mode::FtpMode;

#[cfg(feature = "ftp")]
pub use ftp_client::FtpClient;
