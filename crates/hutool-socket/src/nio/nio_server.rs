//! `NioServer` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.nio.NioServer`
//! 作用：NIO 服务端，依赖 selector 模型。
//! Rust 端以 `AioServer` + `HandlerAction` 适配层实现。

use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::ToSocketAddrs;
use tokio::task::JoinHandle;

use crate::aio::{AioServer, IoAction};
use crate::nio::channel_handler::ChannelHandler;
use crate::nio::nio_client::HandlerAction;
use crate::socket_config::SocketConfig;
use crate::socket_runtime_exception::SocketRuntimeException;

/// NIO 服务端。
///
/// 对齐: cn.hutool.socket.nio.NioServer
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/nio/NioServer.java
/// 中文说明: 基于 AioServer 的 NIO 服务端适配。
pub struct NioServer {
    server: AioServer,
}

impl NioServer {
    /// 绑定服务端。
    pub async fn bind(
        address: impl ToSocketAddrs,
        config: SocketConfig,
    ) -> Result<Self, SocketRuntimeException> {
        Ok(Self {
            server: AioServer::bind(address, config).await?,
        })
    }

    /// 设置 `ChannelHandler`。
    pub async fn set_channel_handler(&self, handler: Arc<dyn ChannelHandler>) {
        let action: Arc<dyn IoAction> = Arc::new(HandlerAction(handler));
        self.server.set_io_action(action).await;
    }

    /// 返回本地地址（Java selector 身份）。
    pub fn selector(&self) -> Result<SocketAddr, SocketRuntimeException> {
        self.server.local_address()
    }

    /// 启动服务端，返回 JoinHandle。
    pub fn start(&self) -> JoinHandle<Result<(), SocketRuntimeException>> {
        self.server.start()
    }

    /// `start` 的别名，对齐 Java `listen()`。
    pub fn listen(&self) -> JoinHandle<Result<(), SocketRuntimeException>> {
        self.start()
    }

    /// 关闭服务端。
    pub fn close(&self) {
        self.server.close();
    }
}
