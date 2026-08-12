//! `AcceptHandler` 的 Rust 对齐实现（NIO 子包）。
//!
//! Java 来源：`cn.hutool.socket.nio.AcceptHandler`
//! 作用：Java 中用于接收客户端连接并把 `SocketChannel` 注册到 selector；
//! Rust 中由 `NioServer` 通过 `ChannelHandler` 适配，
//! 本结构保留 `completed` / `failed` 形态作为桥接。

use crate::aio::aio_session::AioSession;
use crate::nio::channel_handler::ChannelHandler;
use crate::socket_runtime_exception::SocketRuntimeException;

/// NIO 接入完成回调。
///
/// 对齐: cn.hutool.socket.nio.AcceptHandler
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/nio/AcceptHandler.java
/// 中文说明: 桥接到 `ChannelHandler::handle`，保留 failed 形态。
#[derive(Debug, Default, Clone, Copy)]
pub struct AcceptHandler;

impl AcceptHandler {
    /// 接入成功：把会话交给 `ChannelHandler`。
    pub fn completed(
        &self,
        session: AioSession,
        handler: &dyn ChannelHandler,
    ) -> Result<(), SocketRuntimeException> {
        handler.handle(session)
    }

    /// 接入失败：包装为 `SocketRuntimeException`。
    pub fn failed(&self, error: SocketRuntimeException) -> Result<(), SocketRuntimeException> {
        Err(error)
    }
}
