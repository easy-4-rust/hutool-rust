//! `MsgDecoder` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.protocol.MsgDecoder`
//! 作用：从会话缓冲中解码出应用层消息。

use crate::aio::aio_session::AioSession;

/// 消息解码器。
///
/// 对齐: cn.hutool.socket.protocol.MsgDecoder
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/protocol/MsgDecoder.java
/// 中文说明: 把字节流解码为业务消息对象。
pub trait MsgDecoder<T>: Send + Sync {
    /// 解码一个应用消息；返回 `None` 表示尚不构成完整消息。
    fn decode(&self, session: &AioSession, input: &[u8]) -> Option<T>;
}