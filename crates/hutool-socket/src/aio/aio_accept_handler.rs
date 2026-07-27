//! `AcceptHandler` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.aio.AcceptHandler`
//! 作用：Java 中是 `CompletionHandler<AsynchronousSocketChannel, AioServer>`，
//! 服务端在接收连接后立即触发；Rust 中 `AioAcceptHandler::completed`
//! 把已建立的 `TcpStream` 包装成 `AioSession` 并调用 `IoAction::accept`。

use tokio::net::TcpStream;

use crate::aio::aio_session::AioSession;
use crate::aio::io_action::IoAction;
use crate::socket_config::SocketConfig;
use crate::socket_runtime_exception::SocketRuntimeException;

/// AIO 接入完成回调。
///
/// 对齐: cn.hutool.socket.aio.AcceptHandler
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/aio/AcceptHandler.java
/// 中文说明: 单例的接入完成回调，桥接到 IoAction.accept。
#[derive(Debug, Default, Clone, Copy)]
pub struct AcceptHandler;

impl AcceptHandler {
    /// 完成接入：将 stream 包装为 session 并触发 `IoAction::accept`。
    pub fn completed(
        &self,
        stream: TcpStream,
        action: std::sync::Arc<dyn IoAction>,
        config: SocketConfig,
    ) -> AioSession {
        let session = AioSession::new(stream, action, config);
        session.io_action().accept(&session);
        session
    }

    /// 接入失败：通过 action.failed 派发。
    pub fn failed(&self, error: &SocketRuntimeException, session: &AioSession) {
        session.io_action().failed(error, session);
    }
}