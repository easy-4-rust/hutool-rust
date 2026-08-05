//! `AioClient` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.aio.AioClient`
//! 作用：AIO 客户端，连接目标地址后立即触发 `IoAction::accept`，
//! 并对外暴露 `read` / `write` / `close`。

use std::sync::Arc;
use std::time::Duration;

use tokio::net::ToSocketAddrs;

use crate::aio::aio_session::AioSession;
use crate::aio::io_action::IoAction;
use crate::channel_util::ChannelUtil;
use crate::socket_config::SocketConfig;
use crate::socket_runtime_exception::SocketRuntimeException;

/// AIO 客户端。
///
/// 对齐: cn.hutool.socket.aio.AioClient
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/aio/AioClient.java
/// 中文说明: 异步 Socket 客户端，封装 AioSession 与连接建立逻辑。
pub struct AioClient {
    session: AioSession,
}

impl AioClient {
    /// 连接并触发 `IoAction::accept`。
    pub async fn connect(
        address: impl ToSocketAddrs,
        action: Arc<dyn IoAction>,
        config: SocketConfig,
    ) -> Result<Self, SocketRuntimeException> {
        let stream = ChannelUtil::connect(address, Duration::from_secs(10)).await?;
        let session = AioSession::new(stream, action, config);
        session.io_action().accept(&session);
        Ok(Self { session })
    }

    /// 返回底层会话引用。
    #[must_use]
    pub const fn session(&self) -> &AioSession {
        &self.session
    }

    /// 读取并派发一块数据。
    pub async fn read(&self) -> Result<usize, SocketRuntimeException> {
        self.session.read().await
    }

    /// 写入字节。
    pub async fn write(&self, data: &[u8]) -> Result<usize, SocketRuntimeException> {
        self.session.write(data).await
    }

    /// 关闭客户端。
    pub async fn close(&self) -> Result<(), SocketRuntimeException> {
        self.session.close().await
    }
}
