//! `AioSession` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.aio.AioSession`
//! 作用：AIO 会话对象，封装底层通道、读写缓冲与 `IoAction` 回调；
//! Rust 中以共享 Tokio `TcpStream` + Mutex/Notify 协调的会话形式提供，
//! 修复 `MutexGuard` 跨 `await` 反模式（见 `take_stream` / `restore_stream`）。

use std::fmt;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, Notify};

use crate::aio::io_action::IoAction;
use crate::internal::{finish_write_and_close, with_timeout};
use crate::socket_config::SocketConfig;
use crate::socket_runtime_exception::SocketRuntimeException;

/// AIO 会话：每个客户端对应一个会话对象。
///
/// 对齐: cn.hutool.socket.aio.AioSession
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/aio/AioSession.java
/// 中文说明: 封装底层 TCP 流、读写缓冲与回调动作。
#[derive(Clone)]
pub struct AioSession {
    /// `Option` enables take/restore so I/O `.await` never holds `MutexGuard`.
    stream: Arc<Mutex<Option<TcpStream>>>,
    /// Wakes waiters after the stream is restored (serializes concurrent I/O).
    stream_available: Arc<Notify>,
    action: Arc<dyn IoAction>,
    config: SocketConfig,
    remote: SocketAddr,
}

impl fmt::Debug for AioSession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AioSession")
            .field("remote", &self.remote)
            .finish_non_exhaustive()
    }
}

impl AioSession {
    /// 构造一个新会话。
    pub fn new(stream: TcpStream, action: Arc<dyn IoAction>, config: SocketConfig) -> Self {
        let remote = stream
            .peer_addr()
            .expect("Tokio TcpStream instances are connected");
        Self {
            stream: Arc::new(Mutex::new(Some(stream))),
            stream_available: Arc::new(Notify::new()),
            action,
            config,
            remote,
        }
    }

    /// 取出 TCP 流以执行单次 I/O，避免在 await 期间持有 MutexGuard。
    /// rust-async-patterns 修复：先注册 notified() 再检查，防止漏掉唤醒。
    pub(crate) async fn take_stream(&self) -> TcpStream {
        loop {
            let wait = self.stream_available.notified();
            if let Some(stream) = self.stream.lock().await.take() {
                return stream;
            }
            wait.await;
        }
    }

    /// I/O 完成后归还 TCP 流并唤醒等待者。
    pub(crate) async fn restore_stream(&self, stream: TcpStream) {
        *self.stream.lock().await = Some(stream);
        self.stream_available.notify_waiters();
    }

    /// 返回配置的读取缓冲容量。
    #[must_use]
    pub const fn read_buffer_size(&self) -> usize {
        self.config.read_buffer_size()
    }
    /// 返回配置的写缓冲容量。
    #[must_use]
    pub const fn write_buffer_size(&self) -> usize {
        self.config.write_buffer_size()
    }
    /// 返回回调动作。
    #[must_use]
    pub fn io_action(&self) -> &dyn IoAction {
        self.action.as_ref()
    }
    /// 返回对端地址。
    #[must_use]
    pub const fn remote_address(&self) -> SocketAddr {
        self.remote
    }
    /// 返回底层配置的引用。
    #[must_use]
    pub const fn config(&self) -> &SocketConfig {
        &self.config
    }

    /// 读取一块有界数据并回调；I/O await 时不持有 MutexGuard。
    pub async fn read(&self) -> Result<usize, SocketRuntimeException> {
        let mut buffer = vec![0; self.config.read_buffer_size()];
        let mut stream = self.take_stream().await;
        let result = with_timeout(self.config.read_timeout(), stream.read(&mut buffer)).await;
        self.restore_stream(stream).await;
        let count = match result {
            Ok(count) => count,
            Err(error) => {
                self.action.failed(&error, self);
                return Err(error);
            }
        };
        buffer.truncate(count);
        self.action.do_action(self, &buffer);
        Ok(count)
    }

    /// 写入有界字节；I/O await 时不持有 MutexGuard。
    pub async fn write(&self, data: &[u8]) -> Result<usize, SocketRuntimeException> {
        if data.len() > self.config.write_buffer_size() {
            return Err(SocketRuntimeException::new(
                "write exceeds configured buffer size",
            ));
        }
        let mut stream = self.take_stream().await;
        let result = with_timeout(self.config.write_timeout(), stream.write_all(data)).await;
        self.restore_stream(stream).await;
        result?;
        Ok(data.len())
    }

    /// 写入并关闭会话。
    pub async fn write_and_close(&self, data: &[u8]) -> Result<usize, SocketRuntimeException> {
        finish_write_and_close(self.write(data).await, self.close().await)
    }

    /// 流临时取出进行 I/O 时视为仍打开。
    pub async fn is_open(&self) -> bool {
        match self.stream.lock().await.as_ref() {
            Some(stream) => stream.take_error().is_ok_and(|e| e.is_none()),
            None => true,
        }
    }

    /// 关闭会话；shutdown await 时不持有 MutexGuard。
    pub async fn close(&self) -> Result<(), SocketRuntimeException> {
        let mut stream = self.take_stream().await;
        let result = stream.shutdown().await;
        self.restore_stream(stream).await;
        result.map_err(Into::into)
    }

    /// 关闭会话输入端的别名（Java `shutdownInput`）。
    pub async fn close_in(&self) -> Result<(), SocketRuntimeException> {
        self.close().await
    }
    /// 关闭会话输出端的别名（Java `shutdownOutput`）。
    pub async fn close_out(&self) -> Result<(), SocketRuntimeException> {
        self.close().await
    }

    /// 暴露给其他 aio 模块内部使用（用于 `MutexGuard` 不跨 await 模式的
    /// 测试与构造辅助）。
    #[cfg(test)]
    pub(crate) fn stream_slot(&self) -> &Arc<Mutex<Option<TcpStream>>> {
        &self.stream
    }
}