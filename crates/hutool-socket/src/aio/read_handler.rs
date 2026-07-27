//! `ReadHandler` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.aio.ReadHandler`
//! 作用：Java 中是 `CompletionHandler<Integer, AioSession>` 单例，
//! 将读取到的字节数回传给 `AioSession::callbackRead`；
//! Rust 中通过 `ReadHandler.completed` / `ReadHandler.failed`
//! 直接桥接到 `IoAction::do_action` / `IoAction::failed`。

use crate::aio::aio_session::AioSession;
use crate::socket_runtime_exception::SocketRuntimeException;

/// 读取完成回调。
///
/// 对齐: cn.hutool.socket.aio.ReadHandler
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/aio/ReadHandler.java
/// 中文说明: 单例的读取完成回调，桥接到 IoAction。
#[derive(Debug, Default, Clone, Copy)]
pub struct ReadHandler;

impl ReadHandler {
    /// 派发读取结果（字节切片）。
    pub fn completed(&self, session: &AioSession, data: &[u8]) {
        session.io_action().do_action(session, data);
    }

    /// 派发读取失败。
    pub fn failed(&self, error: &SocketRuntimeException, session: &AioSession) {
        session.io_action().failed(error, session);
    }
}