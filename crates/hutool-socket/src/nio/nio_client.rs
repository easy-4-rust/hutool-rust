//! `NioClient` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.nio.NioClient`
//! 作用：NIO 客户端，连接目标地址并通过 `ChannelHandler` 处理数据。
//! Rust 中基于 `AioClient` + `IoAction` 适配层实现。

use std::sync::Arc;

use tokio::net::ToSocketAddrs;

use crate::aio::{AioClient, AioSession, IoAction};
use crate::nio::channel_handler::ChannelHandler;
use crate::socket_config::SocketConfig;
use crate::socket_runtime_exception::SocketRuntimeException;

/// NIO 客户端。
///
/// 对齐: cn.hutool.socket.nio.NioClient
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/nio/NioClient.java
/// 中文说明: 基于 AioClient + ChannelHandler 的 NIO 客户端适配。
pub struct NioClient {
    client: AioClient,
    handler: Arc<dyn ChannelHandler>,
}

impl NioClient {
    /// 构造客户端并建立连接。
    pub async fn connect(
        address: impl ToSocketAddrs,
        handler: Arc<dyn ChannelHandler>,
        config: SocketConfig,
    ) -> Result<Self, SocketRuntimeException> {
        let adapter: HandlerAction = HandlerAction(Arc::clone(&handler));
        let action: Arc<dyn IoAction> = Arc::new(adapter);
        let client = AioClient::connect(address, action, config).await?;
        Ok(Self { client, handler })
    }

    /// 触发一次 handler.handle，等价 Java `listen()` 同步入口。
    pub fn listen(&self) -> Result<(), SocketRuntimeException> {
        self.handler.handle(self.client.session().clone())
    }

    /// 将多段字节合并后写入。
    pub async fn write(&self, data: &[&[u8]]) -> Result<usize, SocketRuntimeException> {
        let total: usize = data.iter().map(|part| part.len()).sum();
        let config = self.client.session().config();
        if total > config.write_buffer_size() {
            return Err(SocketRuntimeException::new(
                "write exceeds configured buffer size",
            ));
        }
        let mut bytes = Vec::with_capacity(total);
        for part in data {
            bytes.extend_from_slice(part);
        }
        self.client.write(&bytes).await
    }

    /// 返回底层会话引用。
    #[must_use]
    pub const fn session(&self) -> &AioSession {
        self.client.session()
    }

    /// 关闭客户端。
    pub async fn close(&self) -> Result<(), SocketRuntimeException> {
        self.client.close().await
    }
}

/// 内部 `ChannelHandler` → `IoAction` 适配层。
///
/// 对齐: NIO 适配层（Java 中通过 NioServer.handle 完成相同职责）
/// 中文说明: 把 `ChannelHandler` 包装成 `IoAction`，由 AIO 层消费。
pub struct HandlerAction(pub Arc<dyn ChannelHandler>);

impl IoAction for HandlerAction {
    fn accept(&self, session: &AioSession) {
        let _ = self.0.handle(session.clone());
    }
    fn do_action(&self, _session: &AioSession, _data: &[u8]) {}
}