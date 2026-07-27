//! `SocketUtil` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.SocketUtil`
//! 作用：保留 Hutool 风格的 socket 工具类入口，对外提供连接、远端地址
//! 查询与连接状态判断，内部复用 Tokio `TcpStream`。

use std::net::SocketAddr;
use std::time::Duration;

use tokio::net::{TcpStream, ToSocketAddrs};

use super::{ChannelUtil, SocketRuntimeException};

/// Socket 相关工具类。
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
