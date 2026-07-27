//! 私有内部辅助：跨 aio / nio / protocol 子模块共享的工具函数。

use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use tokio::net::TcpListener;
use tokio::time;

use crate::socket_runtime_exception::SocketRuntimeException;

/// 在 `timeout` 内执行一个 I/O future；`timeout` 为零时表示不施加超时。
pub(crate) async fn with_timeout<T>(
    timeout: Duration,
    future: impl Future<Output = io::Result<T>>,
) -> Result<T, SocketRuntimeException> {
    if timeout.is_zero() {
        return future.await.map_err(Into::into);
    }
    time::timeout(timeout, future)
        .await
        .map_err(|_| SocketRuntimeException::new("socket operation timed out"))?
        .map_err(Into::into)
}

/// 合并 write + close 两个结果，任一失败即返回错误。
pub(crate) fn finish_write_and_close(
    write: Result<usize, SocketRuntimeException>,
    close: Result<(), SocketRuntimeException>,
) -> Result<usize, SocketRuntimeException> {
    write.and_then(|count| close.map(|()| count))
}

/// `TcpListener::accept` 的封装，便于测试期间注入失败。
#[cfg(test)]
pub(crate) async fn accept_connection(
    listener: &TcpListener,
    fail_accept: bool,
) -> io::Result<(tokio::net::TcpStream, SocketAddr)> {
    if fail_accept {
        return Err(io::Error::other("injected accept failure"));
    }
    listener.accept().await
}

/// 真实环境使用的 accept（无测试钩子）。
#[cfg(not(test))]
pub(crate) async fn accept_connection(
    listener: &TcpListener,
    _fail_accept: bool,
) -> io::Result<(tokio::net::TcpStream, SocketAddr)> {
    let _ = _fail_accept;
    listener.accept().await
}