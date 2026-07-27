//! `ChannelUtil` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.ChannelUtil`
//! 作用：保留 Hutool 中“创建固定线程组”和“连接通道”的门面语义，
//! 但底层改为 Tokio 的 TCP 连接封装。

use std::time::Duration;

use tokio::net::{TcpStream, ToSocketAddrs};

use crate::{TcpConfig, connect_tcp};

use super::SocketRuntimeException;

/// Channel 相关封装。
pub struct ChannelUtil;

impl ChannelUtil {
    /// 校验并返回固定“线程组”大小。
    pub fn create_fixed_group(pool_size: usize) -> Result<usize, SocketRuntimeException> {
        if pool_size == 0 || pool_size > 1_024 {
            return Err(SocketRuntimeException::new(
                "thread pool size must be 1..=1024",
            ));
        }
        Ok(pool_size)
    }

    /// 连接到指定地址，返回 Tokio `TcpStream`。
    pub async fn connect(
        address: impl ToSocketAddrs,
        timeout: Duration,
    ) -> Result<TcpStream, SocketRuntimeException> {
        connect_tcp(
            address,
            TcpConfig {
                connect_timeout: timeout,
                no_delay: true,
            },
        )
        .await
        .map_err(Into::into)
    }
}
