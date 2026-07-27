//! `ChannelHandler` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.nio.ChannelHandler`
//! 作用：Java 中是基于 `SocketChannel` 的单方法函数式接口；
//! Rust 中提供 trait + 为闭包实现的 blanket impl。

use crate::aio::aio_session::AioSession;
use crate::socket_runtime_exception::SocketRuntimeException;

/// NIO 数据处理接口。
///
/// 对齐: cn.hutool.socket.nio.ChannelHandler
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/nio/ChannelHandler.java
/// 中文说明: 从已建立会话中处理读写的接口（trait 形式）。
pub trait ChannelHandler: Send + Sync {
    /// 处理一个已建立的会话。
    fn handle(&self, session: AioSession) -> Result<(), SocketRuntimeException>;
}

impl<F> ChannelHandler for F
where
    F: Fn(AioSession) -> Result<(), SocketRuntimeException> + Send + Sync,
{
    fn handle(&self, session: AioSession) -> Result<(), SocketRuntimeException> {
        self(session)
    }
}