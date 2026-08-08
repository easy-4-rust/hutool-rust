//! ssh2 客户端封装，对齐 hutool `cn.hutool.extra.ssh.JschUtil` 的 exec/sftp 语义。
//!
//! Java hutool 的 `JschUtil` 通过 `JSch` 建立 Session/Channel/Sftp；Rust 侧用
//! [`ssh2`]（libssh2 绑定）承载协议，方法名对齐 Java（`exec`/`upload`/`download`）。

use std::io::Read;
use std::net::TcpStream;

use crate::HutoolException;

/// SSH 会话封装，对齐 Java `com.jcraft.jsch.Session`。
pub struct SshSession {
    session: ssh2::Session,
}

impl SshSession {
    /// 对齐 `JschUtil.openSession(host, port, user, password, timeout)`。
    ///
    /// 建立 TCP + SSH 握手 + 密码认证。`timeout` 为 TCP 连接超时（毫秒）。
    pub fn connect_password(
        host: &str,
        port: u16,
        user: &str,
        password: &str,
        timeout_ms: u64,
    ) -> std::result::Result<Self, HutoolException> {
        let addr = format!("{host}:{port}");
        let tcp = if timeout_ms > 0 {
            let socket = addr.to_socket_addrs_ssh().map_err(|e| Self::io_err(&e))?;
            TcpStream::connect_timeout(&socket, std::time::Duration::from_millis(timeout_ms))
                .map_err(|e| Self::io_err(&e))?
        } else {
            TcpStream::connect(&addr).map_err(|e| Self::io_err(&e))?
        };
        let mut session = ssh2::Session::new().map_err(|e| Self::ssh_err(&e))?;
        session.set_tcp_stream(tcp);
        session.handshake().map_err(|e| Self::ssh_err(&e))?;
        session
            .userauth_password(user, password)
            .map_err(|e| Self::ssh_err(&e))?;
        if !session.authenticated() {
            return Err(HutoolException::Message("ssh auth failed".into()));
        }
        Ok(Self { session })
    }

    /// 对齐 `JschUtil.exec(Session, cmd, charset)`：执行命令并返回 stdout 字符串。
    pub fn exec(&mut self, cmd: &str) -> std::result::Result<String, HutoolException> {
        let mut channel = self
            .session
            .channel_session()
            .map_err(|e| Self::ssh_err(&e))?;
        channel.exec(cmd).map_err(|e| Self::ssh_err(&e))?;
        let mut output = String::new();
        channel
            .read_to_string(&mut output)
            .map_err(|e| Self::io_err(&e))?;
        channel.wait_close().ok();
        Ok(output)
    }

    /// 对齐 `JschUtil.openSftp(Session)` + `Sftp.put`：上传本地文件到远程路径。
    pub fn upload_file(
        &mut self,
        local: &std::path::Path,
        remote: &str,
    ) -> std::result::Result<(), HutoolException> {
        let sftp = self.session.sftp().map_err(|e| Self::ssh_err(&e))?;
        let mut local_file = std::fs::File::open(local).map_err(|e| Self::io_err(&e))?;
        let remote_path = std::path::Path::new(remote);
        let mut remote_file = sftp.create(remote_path).map_err(|e| Self::ssh_err(&e))?;
        std::io::copy(&mut local_file, &mut remote_file).map_err(|e| Self::io_err(&e))?;
        Ok(())
    }

    /// 对齐 `Sftp.get`：下载远程文件到本地路径。
    pub fn download_file(
        &mut self,
        remote: &str,
        local: &std::path::Path,
    ) -> std::result::Result<(), HutoolException> {
        let sftp = self.session.sftp().map_err(|e| Self::ssh_err(&e))?;
        let remote_path = std::path::Path::new(remote);
        let mut remote_file = sftp.open(remote_path).map_err(|e| Self::ssh_err(&e))?;
        let mut local_file = std::fs::File::create(local).map_err(|e| Self::io_err(&e))?;
        std::io::copy(&mut remote_file, &mut local_file).map_err(|e| Self::io_err(&e))?;
        Ok(())
    }

    /// 远程文件读取到内存（对齐 `Sftp.get` 的 `InputStream` 变体）。
    pub fn read_remote(&mut self, remote: &str) -> std::result::Result<Vec<u8>, HutoolException> {
        let sftp = self.session.sftp().map_err(|e| Self::ssh_err(&e))?;
        let remote_path = std::path::Path::new(remote);
        let mut remote_file = sftp.open(remote_path).map_err(|e| Self::ssh_err(&e))?;
        let mut buf = Vec::new();
        remote_file
            .read_to_end(&mut buf)
            .map_err(|e| Self::io_err(&e))?;
        Ok(buf)
    }

    fn ssh_err(error: &ssh2::Error) -> HutoolException {
        HutoolException::Message(format!("ssh2 error: {error}"))
    }

    fn io_err(error: &std::io::Error) -> HutoolException {
        HutoolException::FromCause {
            message: format!("io error: {error}"),
            source: Box::new(std::io::Error::new(error.kind(), error.to_string())),
        }
    }
}

impl Drop for SshSession {
    fn drop(&mut self) {
        // 对齐 Java Session.disconnect()
        let _ = self.session.disconnect(None, "bye", None);
    }
}

trait ToSocketAddrsSsh {
    fn to_socket_addrs_ssh(&self) -> std::io::Result<std::net::SocketAddr>;
}

impl ToSocketAddrsSsh for String {
    fn to_socket_addrs_ssh(&self) -> std::io::Result<std::net::SocketAddr> {
        use std::net::ToSocketAddrs;
        self.to_socket_addrs()?
            .next()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "no addr"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_refused_propagates_error() {
        let result = SshSession::connect_password("127.0.0.1", 1, "u", "p", 500);
        assert!(result.is_err());
    }

    #[test]
    fn connect_no_timeout_refused_propagates_error() {
        let result = SshSession::connect_password("127.0.0.1", 1, "u", "p", 0);
        assert!(result.is_err());
        let err = result.err().unwrap();
        // TCP 失败走 FromCause 包装
        assert!(matches!(err, HutoolException::FromCause { .. }));
    }

    #[test]
    fn io_err_kind_boxed() {
        let err = SshSession::io_err(&std::io::Error::new(
            std::io::ErrorKind::ConnectionRefused,
            "test",
        ));
        assert!(matches!(err, HutoolException::FromCause { .. }));
    }
}
