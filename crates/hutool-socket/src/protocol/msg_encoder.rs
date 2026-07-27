//! `MsgEncoder` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.protocol.MsgEncoder`
//! 作用：将应用层消息编码为字节。

use crate::aio::aio_session::AioSession;
use crate::socket_runtime_exception::SocketRuntimeException;

/// 消息编码器。
///
/// 对齐: cn.hutool.socket.protocol.MsgEncoder
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/protocol/MsgEncoder.java
/// 中文说明: 把业务消息编码为字节流。
pub trait MsgEncoder<T>: Send + Sync {
    /// 编码一个应用消息为字节。
    fn encode(
        &self,
        session: &AioSession,
        value: &T,
    ) -> Result<Vec<u8>, SocketRuntimeException>;
}