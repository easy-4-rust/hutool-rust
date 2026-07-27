//! Hutool 风格 Socket 兼容层。

mod channel_util;
mod socket_config;
mod socket_runtime_exception;
mod socket_util;

pub use channel_util::ChannelUtil;
pub use socket_config::SocketConfig;
pub use socket_runtime_exception::SocketRuntimeException;
pub use socket_util::SocketUtil;

use std::fmt;
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::sync::{Mutex, Notify, RwLock, Semaphore, watch};
use tokio::task::{JoinHandle, JoinSet};
use tokio::time;

/// Interest operation corresponding to Hutool's NIO enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Operation {
    /// Read readiness.
    Read = 1,
    /// Write readiness.
    Write = 4,
    /// Connect readiness.
    Connect = 8,
    /// Accept readiness.
    Accept = 16,
}

impl Operation {
    /// Returns the Java NIO-compatible bit value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self as u8
    }
}

/// Decodes one application message from a session buffer.
pub trait MsgDecoder<T>: Send + Sync {
    /// Returns `None` when a complete message is not available yet.
    fn decode(&self, session: &AioSession, input: &[u8]) -> Option<T>;
}

/// Encodes one application message into bounded output bytes.
pub trait MsgEncoder<T>: Send + Sync {
    /// Encodes a message.
    fn encode(&self, session: &AioSession, value: &T) -> Result<Vec<u8>, SocketRuntimeException>;
}

/// Combined message protocol.
pub trait Protocol<T>: MsgDecoder<T> + MsgEncoder<T> {}

impl<T, P: MsgDecoder<T> + MsgEncoder<T>> Protocol<T> for P {}

/// Session lifecycle callback.
pub trait IoAction: Send + Sync {
    /// Called after a connection is established.
    fn accept(&self, _session: &AioSession) {}
    /// Called after one bounded read completes.
    fn do_action(&self, session: &AioSession, data: &[u8]);
    /// Called when a background operation fails.
    fn failed(&self, _error: &SocketRuntimeException, _session: &AioSession) {}
}

/// Convenience action with no-op accept/failure callbacks.
pub struct SimpleIoAction<F>(pub F);

impl<F> IoAction for SimpleIoAction<F>
where
    F: Fn(&AioSession, &[u8]) + Send + Sync,
{
    fn do_action(&self, session: &AioSession, data: &[u8]) {
        (self.0)(session, data);
    }
}

/// Shared Tokio TCP session.
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
    fn new(stream: TcpStream, action: Arc<dyn IoAction>, config: SocketConfig) -> Self {
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

    /// Takes the stream for one I/O op without holding `MutexGuard` across `.await`.
    /// 取出 TCP 流以执行单次 I/O，避免在 await 期间持有 MutexGuard（rust-async-patterns 修复）。
    async fn take_stream(&self) -> TcpStream {
        loop {
            // Register before checking so a restore notification cannot be missed.
            let wait = self.stream_available.notified();
            if let Some(stream) = self.stream.lock().await.take() {
                return stream;
            }
            wait.await;
        }
    }

    /// Restores the stream after I/O and wakes any waiter.
    /// I/O 完成后归还 TCP 流并唤醒等待者。
    async fn restore_stream(&self, stream: TcpStream) {
        *self.stream.lock().await = Some(stream);
        self.stream_available.notify_waiters();
    }

    /// Returns the configured read-buffer capacity.
    #[must_use]
    pub const fn read_buffer_size(&self) -> usize {
        self.config.read_buffer_size()
    }
    /// Returns the configured write-buffer capacity.
    #[must_use]
    pub const fn write_buffer_size(&self) -> usize {
        self.config.write_buffer_size()
    }
    /// Returns the action callback.
    #[must_use]
    pub fn io_action(&self) -> &dyn IoAction {
        self.action.as_ref()
    }
    /// Returns the peer address.
    #[must_use]
    pub const fn remote_address(&self) -> SocketAddr {
        self.remote
    }

    /// Reads one bounded chunk and dispatches it to the action.
    /// 读取一块有界数据并回调；I/O await 时不持有 MutexGuard。
    pub async fn read(&self) -> Result<usize, SocketRuntimeException> {
        let mut buffer = vec![0; self.config.read_buffer_size()];
        // Anti-pattern fix: never `lock().await.read(...).await` (MutexGuard across await).
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

    /// Writes one bounded byte slice.
    /// 写入有界字节；I/O await 时不持有 MutexGuard。
    pub async fn write(&self, data: &[u8]) -> Result<usize, SocketRuntimeException> {
        if data.len() > self.config.write_buffer_size() {
            return Err(SocketRuntimeException::new(
                "write exceeds configured buffer size",
            ));
        }
        // Anti-pattern fix: never `lock().await.write_all(...).await` (MutexGuard across await).
        let mut stream = self.take_stream().await;
        let result = with_timeout(self.config.write_timeout(), stream.write_all(data)).await;
        self.restore_stream(stream).await;
        result?;
        Ok(data.len())
    }

    /// Writes bytes then shuts down the stream.
    pub async fn write_and_close(&self, data: &[u8]) -> Result<usize, SocketRuntimeException> {
        finish_write_and_close(self.write(data).await, self.close().await)
    }

    /// Returns whether the stream has no pending socket error.
    /// 流临时取出进行 I/O 时视为仍打开。
    pub async fn is_open(&self) -> bool {
        match self.stream.lock().await.as_ref() {
            Some(stream) => stream.take_error().is_ok_and(|e| e.is_none()),
            // Stream is checked out for in-flight I/O — session is still open.
            None => true,
        }
    }

    /// Shuts down the session. Tokio exposes a full async shutdown instead of Java half-close methods.
    /// 关闭会话；shutdown await 时不持有 MutexGuard。
    pub async fn close(&self) -> Result<(), SocketRuntimeException> {
        let mut stream = self.take_stream().await;
        let result = stream.shutdown().await;
        self.restore_stream(stream).await;
        result.map_err(Into::into)
    }

    /// Rust compatibility alias for closing input.
    pub async fn close_in(&self) -> Result<(), SocketRuntimeException> {
        self.close().await
    }
    /// Rust compatibility alias for closing output.
    pub async fn close_out(&self) -> Result<(), SocketRuntimeException> {
        self.close().await
    }
}

fn finish_write_and_close(
    write: Result<usize, SocketRuntimeException>,
    close: Result<(), SocketRuntimeException>,
) -> Result<usize, SocketRuntimeException> {
    write.and_then(|count| close.map(|()| count))
}

async fn with_timeout<T>(
    timeout: Duration,
    future: impl Future<Output = io::Result<T>>,
) -> Result<T, SocketRuntimeException> {
    if timeout.is_zero() {
        return future.await.map_err(Into::into);
    }
    time::timeout(timeout, future)
        .await
        .map_err(|_| SocketRuntimeException::new("socket operation timed out"))?
        .map_err(Into::into)
}

/// Tokio registers readiness with the runtime, so registration is an explicit validation operation.
pub struct NioUtil;

impl NioUtil {
    /// Validates that a stream can participate in the requested operation.
    pub fn register_channel(
        stream: &TcpStream,
        _operation: Operation,
    ) -> Result<(), SocketRuntimeException> {
        stream.local_addr().map(|_| ()).map_err(Into::into)
    }
}

/// Handler used by the NIO-shaped client/server facades.
pub trait ChannelHandler: Send + Sync {
    /// Handles one established session.
    fn handle(&self, session: AioSession) -> Result<(), SocketRuntimeException>;
}

/// AIO accept completion adapter.
#[derive(Debug, Default, Clone, Copy)]
pub struct AioAcceptHandler;

impl AioAcceptHandler {
    /// Converts an accepted Tokio stream into a session and dispatches `accept`.
    pub fn completed(
        &self,
        stream: TcpStream,
        action: Arc<dyn IoAction>,
        config: SocketConfig,
    ) -> AioSession {
        let session = AioSession::new(stream, action, config);
        session.action.accept(&session);
        session
    }

    /// Dispatches an accept failure through the action callback.
    pub fn failed(&self, error: &SocketRuntimeException, session: &AioSession) {
        session.action.failed(error, session);
    }
}

/// Read completion adapter.
#[derive(Debug, Default, Clone, Copy)]
pub struct ReadHandler;

impl ReadHandler {
    /// Dispatches completed bytes.
    pub fn completed(&self, session: &AioSession, data: &[u8]) {
        session.action.do_action(session, data);
    }

    /// Dispatches a read failure.
    pub fn failed(&self, error: &SocketRuntimeException, session: &AioSession) {
        session.action.failed(error, session);
    }
}

/// NIO accept completion adapter.
#[derive(Debug, Default, Clone, Copy)]
pub struct NioAcceptHandler;

impl NioAcceptHandler {
    /// Passes an accepted session to the configured channel handler.
    pub fn completed(
        &self,
        session: AioSession,
        handler: &dyn ChannelHandler,
    ) -> Result<(), SocketRuntimeException> {
        handler.handle(session)
    }

    /// Preserves the failure as a structured result.
    pub fn failed(&self, error: SocketRuntimeException) -> Result<(), SocketRuntimeException> {
        Err(error)
    }
}

impl<F> ChannelHandler for F
where
    F: Fn(AioSession) -> Result<(), SocketRuntimeException> + Send + Sync,
{
    fn handle(&self, session: AioSession) -> Result<(), SocketRuntimeException> {
        self(session)
    }
}

/// AIO-shaped Tokio client.
pub struct AioClient {
    session: AioSession,
}

impl AioClient {
    /// Connects and invokes the accept callback.
    pub async fn connect(
        address: impl ToSocketAddrs,
        action: Arc<dyn IoAction>,
        config: SocketConfig,
    ) -> Result<Self, SocketRuntimeException> {
        let stream = ChannelUtil::connect(address, Duration::from_secs(10)).await?;
        let session = AioSession::new(stream, action, config);
        session.action.accept(&session);
        Ok(Self { session })
    }
    /// Returns the session.
    #[must_use]
    pub const fn session(&self) -> &AioSession {
        &self.session
    }
    /// Reads and dispatches one chunk.
    pub async fn read(&self) -> Result<usize, SocketRuntimeException> {
        self.session.read().await
    }
    /// Writes bytes.
    pub async fn write(&self, data: &[u8]) -> Result<usize, SocketRuntimeException> {
        self.session.write(data).await
    }
    /// Closes the client.
    pub async fn close(&self) -> Result<(), SocketRuntimeException> {
        self.session.close().await
    }
}

/// AIO-shaped Tokio server with explicit task shutdown.
pub struct AioServer {
    listener: Arc<TcpListener>,
    action: Arc<RwLock<Option<Arc<dyn IoAction>>>>,
    config: SocketConfig,
    shutdown: watch::Sender<bool>,
    #[cfg(test)]
    fail_accept: bool,
}

impl AioServer {
    /// Binds a server.
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
    /// Sets the singleton I/O action.
    pub async fn set_io_action(&self, action: Arc<dyn IoAction>) {
        *self.action.write().await = Some(action);
    }
    /// Returns the bound address.
    pub fn local_address(&self) -> Result<SocketAddr, SocketRuntimeException> {
        self.listener.local_addr().map_err(Into::into)
    }
    /// Returns whether shutdown has not been requested.
    #[must_use]
    pub fn is_open(&self) -> bool {
        !*self.shutdown.borrow()
    }
    /// Starts the accept loop in a managed Tokio task.
    ///
    /// Concurrent sessions are capped by [`SocketConfig::thread_pool_size`] using a
    /// `Semaphore`. When no permit is available the loop waits (backpressure) while
    /// still honouring shutdown via `select!`; the permit is held for the session
    /// task lifetime and released on drop.
    pub fn start(&self) -> JoinHandle<Result<(), SocketRuntimeException>> {
        let listener = Arc::clone(&self.listener);
        let actions = Arc::clone(&self.action);
        let config = self.config;
        let mut shutdown = self.shutdown.subscribe();
        let semaphore = Arc::new(Semaphore::new(config.thread_pool_size()));
        #[cfg(test)]
        let fail_accept = self.fail_accept;
        tokio::spawn(async move {
            let mut sessions = JoinSet::new();
            loop {
                tokio::select! {
                    changed = shutdown.changed() => {
                        changed.map_err(|_| SocketRuntimeException::new("shutdown channel closed"))?;
                        sessions.shutdown().await;
                        return Ok(());
                    }
                    accepted = accept_connection(&listener, #[cfg(test)] fail_accept) => {
                        let (stream, _) = accepted?;
                        if let Some(action) = actions.read().await.clone() {
                            // Backpressure: wait for a free slot, or exit on shutdown.
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
                            session.action.accept(&session);
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
    /// Requests shutdown.
    pub fn close(&self) {
        let _ = self.shutdown.send(true);
    }
}

async fn accept_connection(
    listener: &TcpListener,
    #[cfg(test)] fail_accept: bool,
) -> io::Result<(TcpStream, SocketAddr)> {
    #[cfg(test)]
    if fail_accept {
        return Err(io::Error::other("injected accept failure"));
    }
    listener.accept().await
}

struct HandlerAction(Arc<dyn ChannelHandler>);

impl IoAction for HandlerAction {
    fn accept(&self, session: &AioSession) {
        let _ = self.0.handle(session.clone());
    }

    fn do_action(&self, _session: &AioSession, _data: &[u8]) {}
}

/// NIO-shaped Tokio client.
pub struct NioClient {
    client: AioClient,
    handler: Arc<dyn ChannelHandler>,
}

impl NioClient {
    /// Connects a client with a channel handler.
    pub async fn connect(
        address: impl ToSocketAddrs,
        handler: Arc<dyn ChannelHandler>,
        config: SocketConfig,
    ) -> Result<Self, SocketRuntimeException> {
        let action: Arc<dyn IoAction> = Arc::new(HandlerAction(Arc::clone(&handler)));
        let client = AioClient::connect(address, action, config).await?;
        Ok(Self { client, handler })
    }

    /// Dispatches the current session to the handler.
    pub fn listen(&self) -> Result<(), SocketRuntimeException> {
        self.handler.handle(self.client.session.clone())
    }

    /// Writes gathered byte slices in order.
    pub async fn write(&self, data: &[&[u8]]) -> Result<usize, SocketRuntimeException> {
        let total = data.iter().map(|part| part.len()).sum();
        if total > self.client.session.config.write_buffer_size() {
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

    /// Returns the underlying session.
    #[must_use]
    pub const fn session(&self) -> &AioSession {
        self.client.session()
    }

    /// Closes the client.
    pub async fn close(&self) -> Result<(), SocketRuntimeException> {
        self.client.close().await
    }
}

/// NIO-shaped Tokio server.
pub struct NioServer {
    server: AioServer,
}

impl NioServer {
    /// Binds a NIO facade.
    pub async fn bind(
        address: impl ToSocketAddrs,
        config: SocketConfig,
    ) -> Result<Self, SocketRuntimeException> {
        Ok(Self {
            server: AioServer::bind(address, config).await?,
        })
    }

    /// Sets the channel handler.
    pub async fn set_channel_handler(&self, handler: Arc<dyn ChannelHandler>) {
        self.server
            .set_io_action(Arc::new(HandlerAction(handler)))
            .await;
    }

    /// Returns the bound address as the Tokio selector identity.
    pub fn selector(&self) -> Result<SocketAddr, SocketRuntimeException> {
        self.server.local_address()
    }

    /// Starts listening.
    pub fn start(&self) -> JoinHandle<Result<(), SocketRuntimeException>> {
        self.server.start()
    }

    /// Alias for `start`.
    pub fn listen(&self) -> JoinHandle<Result<(), SocketRuntimeException>> {
        self.start()
    }

    /// Closes the server.
    pub fn close(&self) {
        self.server.close();
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error as _;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use tokio::sync::Notify;

    use crate::SocketError;

    use super::*;

    #[derive(Default)]
    struct RecordingAction {
        accepted: AtomicUsize,
        bytes: AtomicUsize,
        failures: AtomicUsize,
        notify: Notify,
    }

    impl IoAction for RecordingAction {
        fn accept(&self, _session: &AioSession) {
            self.accepted.fetch_add(1, Ordering::SeqCst);
        }

        fn do_action(&self, _session: &AioSession, data: &[u8]) {
            self.bytes.fetch_add(data.len(), Ordering::SeqCst);
            self.notify.notify_one();
        }

        fn failed(&self, _error: &SocketRuntimeException, _session: &AioSession) {
            self.failures.fetch_add(1, Ordering::SeqCst);
            self.notify.notify_one();
        }
    }

    struct CountingHandler(Arc<AtomicUsize>);

    impl ChannelHandler for CountingHandler {
        fn handle(&self, _session: AioSession) -> Result<(), SocketRuntimeException> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    struct LengthProtocol;

    impl MsgDecoder<usize> for LengthProtocol {
        fn decode(&self, _session: &AioSession, input: &[u8]) -> Option<usize> {
            (!input.is_empty()).then_some(input.len())
        }
    }

    impl MsgEncoder<usize> for LengthProtocol {
        fn encode(
            &self,
            _session: &AioSession,
            value: &usize,
        ) -> Result<Vec<u8>, SocketRuntimeException> {
            Ok(value.to_string().into_bytes())
        }
    }

    fn assert_protocol<P: Protocol<usize>>(_protocol: &P) {}

    #[test]
    fn config_errors_operations_and_formatting_are_explicit() {
        let mut config = SocketConfig::new();
        assert!(config.thread_pool_size() > 0);
        assert_eq!(config.read_buffer_size(), 8_192);
        assert_eq!(config.write_buffer_size(), 8_192);
        assert_eq!(config.read_timeout(), Duration::ZERO);
        assert_eq!(config.write_timeout(), Duration::ZERO);
        assert!(config.set_thread_pool_size(0).is_err());
        assert!(config.set_thread_pool_size(1_025).is_err());
        config.set_thread_pool_size(2).unwrap();
        config
            .set_read_timeout(Duration::from_millis(1))
            .set_write_timeout(Duration::from_millis(2));
        assert!(config.set_read_buffer_size(0).is_err());
        assert!(config.set_write_buffer_size(16 * 1024 * 1024 + 1).is_err());
        config.set_read_buffer_size(32).unwrap();
        config.set_write_buffer_size(64).unwrap();
        assert_eq!(config.thread_pool_size(), 2);
        assert_eq!(config.read_buffer_size(), 32);
        assert_eq!(config.write_buffer_size(), 64);
        assert_eq!(config.read_timeout(), Duration::from_millis(1));
        assert_eq!(config.write_timeout(), Duration::from_millis(2));

        assert_eq!(Operation::Read.value(), 1);
        assert_eq!(Operation::Write.value(), 4);
        assert_eq!(Operation::Connect.value(), 8);
        assert_eq!(Operation::Accept.value(), 16);
        assert_eq!(ChannelUtil::create_fixed_group(2).unwrap(), 2);
        assert!(ChannelUtil::create_fixed_group(0).is_err());
        assert!(ChannelUtil::create_fixed_group(1_025).is_err());

        let formatted = SocketRuntimeException::formatted("{} + {}", &[&1, &2]);
        assert_eq!(formatted.to_string(), "1 + 2");
        let trailing = SocketRuntimeException::formatted("plain", &[&1]);
        assert_eq!(trailing.to_string(), "plain");
        let sourced = SocketRuntimeException::from(io::Error::other("boom"));
        assert!(sourced.source().is_some());
        assert!(
            SocketRuntimeException::from(SocketError::ConnectTimeout)
                .to_string()
                .contains("timed out")
        );
        assert!(finish_write_and_close(Err(SocketRuntimeException::new("write")), Ok(())).is_err());
        assert!(finish_write_and_close(Ok(1), Err(SocketRuntimeException::new("close"))).is_err());
    }

    #[tokio::test]
    async fn completion_handlers_and_nio_facades_delegate_to_tokio() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let connected = tokio::spawn(async move { TcpStream::connect(address).await.unwrap() });
        let (stream, _) = listener.accept().await.unwrap();
        let peer = connected.await.unwrap();
        let action = Arc::new(RecordingAction::default());
        let session = AioAcceptHandler.completed(stream, action.clone(), SocketConfig::default());
        assert!(format!("{session:?}").contains("AioSession"));
        ReadHandler.completed(&session, b"abc");
        let error = SocketRuntimeException::new("failed");
        ReadHandler.failed(&error, &session);
        AioAcceptHandler.failed(&error, &session);
        assert_eq!(action.accepted.load(Ordering::SeqCst), 1);
        assert_eq!(action.bytes.load(Ordering::SeqCst), 3);
        assert_eq!(action.failures.load(Ordering::SeqCst), 2);

        let callback_count = Arc::new(AtomicUsize::new(0));
        let handler: Arc<dyn ChannelHandler> =
            Arc::new(CountingHandler(Arc::clone(&callback_count)));
        let adapter = HandlerAction(Arc::clone(&handler));
        adapter.do_action(&session, b"ignored");
        adapter.failed(&error, &session);
        NioAcceptHandler
            .completed(session.clone(), handler.as_ref())
            .unwrap();
        let closure = |_session: AioSession| Ok(());
        ChannelHandler::handle(&closure, session.clone()).unwrap();
        assert_eq!(callback_count.load(Ordering::SeqCst), 1);
        assert!(
            NioAcceptHandler
                .failed(SocketRuntimeException::new("nio"))
                .is_err()
        );
        drop(peer);

        let server = NioServer::bind("127.0.0.1:0", SocketConfig::default())
            .await
            .unwrap();
        server.set_channel_handler(Arc::clone(&handler)).await;
        let address = server.selector().unwrap();
        let task = server.listen();
        let client = NioClient::connect(address, handler, SocketConfig::default())
            .await
            .unwrap();
        client.listen().unwrap();
        assert_eq!(client.write(&[b"a", b"b"]).await.unwrap(), 2);
        let oversized = vec![0; 9_000];
        assert!(client.write(&[&oversized]).await.is_err());
        assert_eq!(client.session().remote_address(), address);
        client.close().await.unwrap();
        server.close();
        task.await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn aio_client_read_dispatches_server_bytes() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let peer = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            stream.write_all(b"reply").await.unwrap();
        });
        let bytes = Arc::new(AtomicUsize::new(0));
        let output = Arc::clone(&bytes);
        let action = Arc::new(SimpleIoAction(move |_: &AioSession, data: &[u8]| {
            output.fetch_add(data.len(), Ordering::SeqCst);
        }));
        let client = AioClient::connect(address, action, SocketConfig::default())
            .await
            .unwrap();
        assert_eq!(client.read().await.unwrap(), 5);
        assert_eq!(bytes.load(Ordering::SeqCst), 5);
        client
            .session()
            .io_action()
            .failed(&SocketRuntimeException::new("observed"), client.session());
        peer.await.unwrap();
    }

    #[tokio::test]
    async fn connection_and_server_error_paths_are_explicit() {
        assert!(
            AioServer::bind("not a socket address", SocketConfig::default())
                .await
                .is_err()
        );
        assert!(
            NioServer::bind("not a socket address", SocketConfig::default())
                .await
                .is_err()
        );

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let unused = listener.local_addr().unwrap();
        drop(listener);
        let action: Arc<dyn IoAction> = Arc::new(RecordingAction::default());
        assert!(
            AioClient::connect(unused, Arc::clone(&action), SocketConfig::default())
                .await
                .is_err()
        );
        let handler: Arc<dyn ChannelHandler> =
            Arc::new(CountingHandler(Arc::new(AtomicUsize::new(0))));
        assert!(
            NioClient::connect(unused, handler, SocketConfig::default())
                .await
                .is_err()
        );

        let server = AioServer::bind("127.0.0.1:0", SocketConfig::default())
            .await
            .unwrap();
        let address = server.local_address().unwrap();
        let task = server.start();
        let stream = TcpStream::connect(address).await.unwrap();
        drop(stream);
        server.close();
        task.await.unwrap().unwrap();

        let server = AioServer::bind("127.0.0.1:0", SocketConfig::default())
            .await
            .unwrap();
        let task = server.start();
        drop(server);
        assert!(task.await.unwrap().is_err());

        let mut server = AioServer::bind("127.0.0.1:0", SocketConfig::default())
            .await
            .unwrap();
        server.fail_accept = true;
        assert!(server.start().await.unwrap().is_err());

        let mut config = SocketConfig::default();
        config.set_read_timeout(Duration::from_millis(1));
        let action = Arc::new(RecordingAction::default());
        let server = AioServer::bind("127.0.0.1:0", config).await.unwrap();
        server.set_io_action(action.clone()).await;
        let task = server.start();
        let client = TcpStream::connect(server.local_address().unwrap())
            .await
            .unwrap();
        time::timeout(Duration::from_secs(1), action.notify.notified())
            .await
            .unwrap();
        assert_eq!(action.failures.load(Ordering::SeqCst), 1);
        tokio::task::yield_now().await;
        drop(client);
        server.close();
        task.await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn aio_server_limits_concurrent_sessions_to_thread_pool_size() {
        /// Tracks accept callbacks; sessions stay open until the peer is dropped.
        struct LimitingAction {
            accepted: AtomicUsize,
            notify: Notify,
        }

        impl IoAction for LimitingAction {
            fn accept(&self, _session: &AioSession) {
                self.accepted.fetch_add(1, Ordering::SeqCst);
                self.notify.notify_waiters();
            }

            fn do_action(&self, _session: &AioSession, _data: &[u8]) {}
        }

        let action = Arc::new(LimitingAction {
            accepted: AtomicUsize::new(0),
            notify: Notify::new(),
        });
        let mut config = SocketConfig::default();
        // Cap concurrent session tasks at 2 (Hutool `threadPoolSize` mapping).
        config.set_thread_pool_size(2).unwrap();
        let server = AioServer::bind("127.0.0.1:0", config).await.unwrap();
        server.set_io_action(action.clone()).await;
        let address = server.local_address().unwrap();
        let task = server.start();

        let client1 = TcpStream::connect(address).await.unwrap();
        let client2 = TcpStream::connect(address).await.unwrap();
        while action.accepted.load(Ordering::SeqCst) < 2 {
            time::timeout(Duration::from_secs(1), action.notify.notified())
                .await
                .unwrap();
        }
        assert_eq!(action.accepted.load(Ordering::SeqCst), 2);

        // Third connect succeeds at TCP level but must not be accepted until a slot frees.
        let pending = tokio::spawn(async move { TcpStream::connect(address).await.unwrap() });
        time::sleep(Duration::from_millis(100)).await;
        assert_eq!(
            action.accepted.load(Ordering::SeqCst),
            2,
            "accept loop must wait when thread_pool_size permits are exhausted"
        );

        drop(client1);
        let client3 = time::timeout(Duration::from_secs(1), pending)
            .await
            .expect("third connect should complete once a permit is free")
            .unwrap();
        while action.accepted.load(Ordering::SeqCst) < 3 {
            time::timeout(Duration::from_secs(1), action.notify.notified())
                .await
                .unwrap();
        }
        assert_eq!(action.accepted.load(Ordering::SeqCst), 3);

        drop(client2);
        drop(client3);
        server.close();
        task.await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn aio_server_client_session_and_protocol_use_real_loopback_io() {
        let action = Arc::new(RecordingAction::default());
        let server = AioServer::bind("127.0.0.1:0", SocketConfig::default())
            .await
            .unwrap();
        server.set_io_action(action.clone()).await;
        let address = server.local_address().unwrap();
        assert!(server.is_open());
        let task = server.start();

        let client_action = Arc::new(SimpleIoAction(|_: &AioSession, _: &[u8]| {}));
        let client = AioClient::connect(address, client_action, SocketConfig::default())
            .await
            .unwrap();
        client
            .session()
            .io_action()
            .do_action(client.session(), b"");
        client
            .session()
            .io_action()
            .failed(&SocketRuntimeException::new("observed"), client.session());
        assert_eq!(client.session().remote_address(), address);
        assert_eq!(client.session().read_buffer_size(), 8_192);
        assert_eq!(client.session().write_buffer_size(), 8_192);
        assert!(client.session().is_open().await);
        {
            let guard = client.session().stream.lock().await;
            let stream = guard.as_ref().expect("stream present when idle");
            assert!(SocketUtil::is_connected(stream));
            assert_eq!(SocketUtil::remote_address(stream).unwrap(), address);
            NioUtil::register_channel(stream, Operation::Read).unwrap();
        }

        assert_eq!(client.write(b"hello").await.unwrap(), 5);
        time::timeout(Duration::from_secs(1), action.notify.notified())
            .await
            .unwrap();
        assert_eq!(action.accepted.load(Ordering::SeqCst), 1);
        assert_eq!(action.bytes.load(Ordering::SeqCst), 5);
        assert_eq!(action.failures.load(Ordering::SeqCst), 0);

        let protocol = LengthProtocol;
        assert_protocol(&protocol);
        assert_eq!(protocol.decode(client.session(), b"abc"), Some(3));
        assert_eq!(protocol.decode(client.session(), b""), None);
        assert_eq!(protocol.encode(client.session(), &12).unwrap(), b"12");
        client.session().io_action().accept(client.session());

        client.close().await.unwrap();
        server.close();
        task.await.unwrap().unwrap();
        assert!(!server.is_open());
    }

    #[tokio::test]
    async fn session_limits_timeouts_and_close_aliases_are_bounded() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let accepted = tokio::spawn(async move { listener.accept().await.unwrap().0 });
        let stream = SocketUtil::connect(address).await.unwrap();
        let peer = accepted.await.unwrap();
        let action = Arc::new(RecordingAction::default());
        let mut config = SocketConfig::default();
        config.set_read_timeout(Duration::from_millis(1));
        config.set_write_buffer_size(2).unwrap();
        let session = AioSession::new(stream, action, config);
        assert!(session.write(b"abc").await.is_err());
        assert!(session.read().await.is_err());
        drop(peer);
        session.close_in().await.unwrap();

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let accepted = tokio::spawn(async move { listener.accept().await.unwrap().0 });
        let stream = SocketUtil::connect_timeout(address, Duration::from_secs(1))
            .await
            .unwrap();
        let peer = accepted.await.unwrap();
        let session = AioSession::new(
            stream,
            Arc::new(RecordingAction::default()),
            SocketConfig::default(),
        );
        assert_eq!(session.write_and_close(b"x").await.unwrap(), 1);
        drop(peer);

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let accepted = tokio::spawn(async move { listener.accept().await.unwrap().0 });
        let stream = ChannelUtil::connect(address, Duration::from_secs(1))
            .await
            .unwrap();
        let peer = accepted.await.unwrap();
        let session = AioSession::new(stream, Arc::new(RecordingAction::default()), {
            let mut config = SocketConfig::default();
            config.set_write_timeout(Duration::from_secs(1));
            config
        });
        assert_eq!(session.write(b"x").await.unwrap(), 1);
        session.close_out().await.unwrap();
        drop(peer);

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let accepted = tokio::spawn(async move { listener.accept().await.unwrap().0 });
        let stream = SocketUtil::connect(address).await.unwrap();
        let peer = accepted.await.unwrap();
        let session = AioSession::new(stream, Arc::new(RecordingAction::default()), {
            let mut config = SocketConfig::default();
            config.set_write_timeout(Duration::from_nanos(1));
            config.set_write_buffer_size(16 * 1024 * 1024).unwrap();
            config
        });
        assert!(session.write(&vec![0; 16 * 1024 * 1024]).await.is_err());
        drop(peer);
    }

    /// Proves read/write still work after the MutexGuard-across-await anti-pattern fix.
    /// 验证 take/restore 修复后 read/write 行为保持正确。
    #[tokio::test]
    async fn session_read_write_without_mutex_across_await() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let peer = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buf = [0_u8; 5];
            stream.read_exact(&mut buf).await.unwrap();
            assert_eq!(&buf, b"hello");
            stream.write_all(b"world").await.unwrap();
        });

        let action = Arc::new(RecordingAction::default());
        let session = AioSession::new(
            SocketUtil::connect(address).await.unwrap(),
            action.clone() as Arc<dyn IoAction>,
            SocketConfig::default(),
        );

        // Stream is present while idle; I/O uses take/restore (no MutexGuard across await).
        assert!(session.stream.lock().await.is_some());
        assert_eq!(session.write(b"hello").await.unwrap(), 5);
        assert!(session.stream.lock().await.is_some());
        assert_eq!(session.read().await.unwrap(), 5);
        assert_eq!(action.bytes.load(Ordering::SeqCst), 5);
        assert!(session.is_open().await);
        session.close().await.unwrap();
        peer.await.unwrap();
    }
}
