//! FTP POJO facade，对齐 hutool 的 `cn.hutool.extra.ftp.*`。
//!
//! **仅实现 POJO 部分**（FtpConfig、FtpMode、FtpException、AbstractFtp trait）。
//! 具体 FTP 协议操作（Ftp/SimpleFtpServer）需要外部 Java crate（Apache Commons Net /
//! Apache FtpServer），属于 unsafe-to-copy。

use std::fmt;

use crate::HutoolException;

/// 抽象 FTP 操作 trait，对齐 `cn.hutool.extra.ftp.AbstractFtp`。
///
/// 具体 Ftp / SshjSftp 实现需要外部 Java crate，属于 unsafe-to-copy。
pub trait AbstractFtp: Send + Sync {
    /// 对齐 `AbstractFtp.reconnectIfTimeout()`
    fn reconnect_if_timeout(&self) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.cd(String)`
    fn cd(&mut self, dir: &str) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.toParent()`
    fn to_parent(&mut self) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.pwd()`
    fn pwd(&self) -> std::result::Result<String, HutoolException>;

    /// 对齐 `AbstractFtp.isDir(String)`
    fn is_dir(&self, dir: &str) -> bool;

    /// 对齐 `AbstractFtp.mkdir(String)`
    fn mkdir(&mut self, dir: &str) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.exist(String)`
    fn exist(&self, path: &str) -> bool;

    /// 对齐 `AbstractFtp.delFile(String)`
    fn del_file(&mut self, path: &str) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.delDir(String)`
    fn del_dir(&mut self, dir: &str) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.mkDirs(String)`
    fn mk_dirs(&mut self, dir: &str) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.upload(...)`
    fn upload(&mut self, dest: &str, _data: &[u8]) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.recursiveDownloadFolder(String, File)`
    fn recursive_download_folder(
        &mut self,
        remote: &str,
        local: &std::path::Path,
    ) -> std::result::Result<(), HutoolException>;

    /// 对齐 `AbstractFtp.rename(String, String)`
    fn rename(&mut self, from: &str, to: &str) -> std::result::Result<(), HutoolException>;
}
