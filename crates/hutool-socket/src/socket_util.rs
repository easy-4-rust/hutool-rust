//! `SocketUtil` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.SocketUtil`
//! 作用：保留 Hutool 风格的 socket 工具类入口，对外提供连接、远端地址
//! 查询与连接状态判断，内部复用 Tokio `TcpStream`。

use std::net::SocketAddr;
use std::time::Duration;

use tokio::net::{TcpStream, ToSocketAddrs};

use crate::channel_util::ChannelUtil;
use crate::socket_runtime_exception::SocketRuntimeException;

/// Socket 相关工具类。
///
/// 对齐: cn.hutool.socket.SocketUtil
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/SocketUtil.java
/// 中文说明: 提供带超时的连接、远端地址查询与连接状态判断。
pub struct SocketUtil;

impl SocketUtil {
    /// 使用默认 10 秒超时连接到目标地址。
    pub async fn connect(address: impl ToSocketAddrs) -> Result<TcpStream, SocketRuntimeException> {
        ChannelUtil::connect(address, Duration::from_secs(10)).await
    }

    /// 使用显式超时连接到目标地址。
    pub async fn connect_timeout(
        address: impl ToSocketAddrs,
        timeout: Duration,
    ) -> Result<TcpStream, SocketRuntimeException> {
        ChannelUtil::connect(address, timeout).await
    }

    /// 获取远端地址。
    pub fn remote_address(stream: &TcpStream) -> Result<SocketAddr, SocketRuntimeException> {
        stream.peer_addr().map_err(Into::into)
    }

    /// 判断连接是否仍然可用。
    pub fn is_connected(stream: &TcpStream) -> bool {
        stream.take_error().is_ok_and(|e| e.is_none())
    }
}

#[cfg(test)]
mod tests {
    use tokio::net::TcpListener;

    use super::*;

    #[tokio::test]
    async fn connect_helpers_reach_a_loopback_listener() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let accepted = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            // Hold the peer open until the test finishes.
            let mut buf = [0_u8; 1];
            let _ = tokio::io::AsyncReadExt::read(&mut stream, &mut buf).await;
        });
        let stream = SocketUtil::connect(address).await.unwrap();
        assert!(SocketUtil::is_connected(&stream));
        assert_eq!(SocketUtil::remote_address(&stream).unwrap(), address);
        drop(stream);
        let _ = accepted.await;
    }

    #[tokio::test]
    async fn connect_timeout_honors_explicit_duration() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        drop(listener);
        assert!(
            SocketUtil::connect_timeout(address, Duration::from_millis(1))
                .await
                .is_err()
        );
    }
}