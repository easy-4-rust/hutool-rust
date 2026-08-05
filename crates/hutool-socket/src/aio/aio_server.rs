//! `AioServer` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.aio.AioServer`
//! 作用：AIO 服务端；Rust 实现以 Tokio 监听 + 后台任务为骨架，
//! 通过 `watch::Sender<bool>` 实现优雅关闭，并通过 `Semaphore`
//! 实现 Hutool `threadPoolSize` 并发上限（背压）。

use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::sync::{RwLock, Semaphore, watch};
use tokio::task::{JoinHandle, JoinSet};

use crate::aio::aio_session::AioSession;
use crate::aio::io_action::IoAction;
use crate::internal::accept_connection;
use crate::socket_config::SocketConfig;
use crate::socket_runtime_exception::SocketRuntimeException;

/// AIO 服务端。
///
/// 对齐: cn.hutool.socket.aio.AioServer
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/aio/AioServer.java
/// 中文说明: 异步 Socket 服务端，通过 Semaphore 限制 Hutool threadPoolSize。
pub struct AioServer {
    listener: Arc<TcpListener>,
    action: Arc<RwLock<Option<Arc<dyn IoAction>>>>,
    config: SocketConfig,
    shutdown: watch::Sender<bool>,
    #[cfg(test)]
    pub(crate) fail_accept: bool,
}

impl AioServer {
    /// 绑定服务端。
    pub async fn bind(
        address: impl ToSocketAddrs,
        config: SocketConfig,
    ) -> Result<Self, SocketRuntimeException> {
        let listener = TcpListener::bind(address).await?;
        let (shutdown, _) = watch::channel(false);
        Ok(Self {
            listener: Arc::new(listener),
            action: Arc::new(RwLock::new(None)),
            config,
            shutdown,
            #[cfg(test)]
            fail_accept: false,
        })
    }

    /// 设置单例 `IoAction`。
    pub async fn set_io_action(&self, action: Arc<dyn IoAction>) {
        *self.action.write().await = Some(action);
    }

    /// 返回本地绑定地址。
    pub fn local_address(&self) -> Result<SocketAddr, SocketRuntimeException> {
        self.listener.local_addr().map_err(Into::into)
    }

    /// 返回是否未请求关闭。
    #[must_use]
    pub fn is_open(&self) -> bool {
        !*self.shutdown.borrow()
    }

    /// 启动 accept 循环（返回 JoinHandle 可被取消）。
    pub fn start(&self) -> JoinHandle<Result<(), SocketRuntimeException>> {
        let listener = Arc::clone(&self.listener);
        let actions = Arc::clone(&self.action);
        let config = self.config;
        let mut shutdown = self.shutdown.subscribe();
        let semaphore = Arc::new(Semaphore::new(config.thread_pool_size()));
        #[cfg(test)]
        let fail_accept = self.fail_accept;
        #[cfg(not(test))]
        let fail_accept = false;
        let fail_accept_arg = fail_accept;
        tokio::spawn(async move {
            let mut sessions = JoinSet::new();
            loop {
                tokio::select! {
                    changed = shutdown.changed() => {
                        changed.map_err(|_| SocketRuntimeException::new("shutdown channel closed"))?;
                        sessions.shutdown().await;
                        return Ok(());
                    }
                    accepted = accept_connection(&listener, fail_accept_arg) => {
                        let (stream, _) = accepted?;
                        if let Some(action) = actions.read().await.clone() {
                            let permit = tokio::select! {
                                changed = shutdown.changed() => {
                                    changed.map_err(|_| {
                                        SocketRuntimeException::new("shutdown channel closed")
                                    })?;
                                    drop(stream);
                                    sessions.shutdown().await;
                                    return Ok(());
                                }
                                permit = Arc::clone(&semaphore).acquire_owned() => {
                                    permit.map_err(|_| {
                                        SocketRuntimeException::new("connection semaphore closed")
                                    })?
                                }
                            };
                            let session = AioSession::new(stream, action, config);
                            session.io_action().accept(&session);
                            sessions.spawn(async move {
                                let _permit = permit;
                                let _ = session.read().await;
                            });
                        }
                    }
                }
            }
        })
    }

    /// 请求关闭服务端。
    pub fn close(&self) {
        let _ = self.shutdown.send(true);
    }
}
