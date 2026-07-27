//! Bounded asynchronous TCP and UDP helpers backed by Tokio.
//!
//! 模块结构（与 Hutool `cn.hutool.socket.*` 1:1 对齐）：
//! - 顶级：`socket_util`, `channel_util`, `socket_config`, `socket_runtime_exception`
//! - 子包：`aio::{aio_client, aio_server, aio_session, aio_accept_handler, io_action,
//!   read_handler, simple_io_action}`
//! - 子包：`nio::{nio_client, nio_server, nio_accept_handler, channel_handler,
//!   nio_util, operation}`
//! - 子包：`protocol::{protocol, msg_decoder, msg_encoder}`

#![forbid(unsafe_code)]

use std::{io, time::Duration};

use thiserror::Error;
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader},
    net::{TcpStream, ToSocketAddrs, UdpSocket},
    time,
};

// 1:1 对齐的 Hutool 模块（cn.hutool.socket.*）
mod channel_util;
mod internal;
mod socket_config;
mod socket_runtime_exception;
mod socket_util;

// Hutool 子包（cn.hutool.socket.{aio,nio,protocol}）
pub mod aio;
pub mod nio;
pub mod protocol;

// 顶层公共 re-export：保留原公开 API 路径，crate 用户代码不需要改。
pub use aio::{
    AioClient, AioServer, AioSession, IoAction, ReadHandler, SimpleIoAction,
};
pub use aio::AcceptHandler as AioAcceptHandler;
pub use channel_util::ChannelUtil;
pub use nio::{ChannelHandler, NioClient, NioServer, NioUtil, Operation};
pub use nio::AcceptHandler as NioAcceptHandler;
pub use protocol::{MsgDecoder, MsgEncoder, Protocol};
pub use socket_config::SocketConfig;
pub use socket_runtime_exception::SocketRuntimeException;
pub use socket_util::SocketUtil;

/// Socket helper failures.
#[derive(Debug, Error)]
pub enum SocketError {
    /// An operating-system I/O operation failed.
    #[error(transparent)]
    Io(#[from] io::Error),
    /// A connection attempt exceeded its deadline.
    #[error("socket connection timed out")]
    ConnectTimeout,
    /// An incoming frame exceeded the configured limit.
    #[error("frame contains {actual} bytes, exceeding limit {limit}")]
    FrameTooLarge {
        /// Configured limit.
        limit: usize,
        /// Observed bytes before termination.
        actual: usize,
    },
}

/// TCP connection policy.
#[derive(Debug, Clone, Copy)]
pub struct TcpConfig {
    /// Maximum connection establishment time.
    pub connect_timeout: Duration,
    /// Whether to disable Nagle's algorithm.
    pub no_delay: bool,
}

impl Default for TcpConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(10),
            no_delay: true,
        }
    }
}

/// Connects a TCP stream with timeout and socket policy.
pub async fn connect_tcp(
    address: impl ToSocketAddrs,
    config: TcpConfig,
) -> Result<TcpStream, SocketError> {
    #[cfg(not(test))]
    return connect_tcp_inner(address, config).await;
    #[cfg(test)]
    connect_tcp_inner(address, config, ConnectFaults::default()).await
}

#[cfg(test)]
#[derive(Clone, Copy, Default)]
struct ConnectFaults {
    timeout: bool,
    policy: bool,
}

async fn connect_tcp_inner(
    address: impl ToSocketAddrs,
    config: TcpConfig,
    #[cfg(test)] faults: ConnectFaults,
) -> Result<TcpStream, SocketError> {
    if config.connect_timeout.is_zero() {
        return Err(SocketError::ConnectTimeout);
    }
    #[cfg(test)]
    let timed = if faults.timeout {
        None
    } else {
        time::timeout(config.connect_timeout, TcpStream::connect(address))
            .await
            .ok()
    };
    #[cfg(not(test))]
    let timed = time::timeout(config.connect_timeout, TcpStream::connect(address))
        .await
        .ok();
    let stream = timed.ok_or(SocketError::ConnectTimeout)??;
    #[cfg(test)]
    let policy = if faults.policy {
        Err(io::Error::other("injected socket policy failure"))
    } else {
        stream.set_nodelay(config.no_delay)
    };
    #[cfg(not(test))]
    let policy = stream.set_nodelay(config.no_delay);
    policy?;
    Ok(stream)
}

/// Binds an asynchronous UDP socket.
pub async fn bind_udp(address: impl ToSocketAddrs) -> Result<UdpSocket, SocketError> {
    Ok(UdpSocket::bind(address).await?)
}

/// Reads one delimiter-terminated frame while enforcing a byte limit.
pub async fn read_frame<R: AsyncRead + Unpin>(
    reader: R,
    delimiter: u8,
    max_bytes: usize,
) -> Result<Vec<u8>, SocketError> {
    let mut reader = BufReader::new(reader);
    let mut output = Vec::new();
    reader.read_until(delimiter, &mut output).await?;
    if output.len() > max_bytes {
        return Err(SocketError::FrameTooLarge {
            limit: max_bytes,
            actual: output.len(),
        });
    }
    Ok(output)
}

/// Writes all frame bytes and flushes the writer.
pub async fn write_frame<W: AsyncWrite + Unpin>(
    mut writer: W,
    frame: &[u8],
) -> Result<(), SocketError> {
    writer.write_all(frame).await?;
    writer.flush().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error as _;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::task::{Context, Poll};

    use tokio::io::ReadBuf;
    use tokio::sync::Notify;
    use tokio::{io::AsyncReadExt as _, net::TcpListener};

    use crate::internal::finish_write_and_close;
    use crate::nio::{NioClient, NioServer};

    use super::*;

    #[derive(Default)]
    struct TestIo {
        input: Vec<u8>,
        offset: usize,
        output: Vec<u8>,
        read_error: bool,
        write_error: bool,
        flush_error: bool,
    }

    impl TestIo {
        fn reader(input: &[u8]) -> Self {
            Self {
                input: input.to_vec(),
                ..Self::default()
            }
        }
    }

    impl AsyncRead for TestIo {
        fn poll_read(
            mut self: Pin<&mut Self>,
            _context: &mut Context<'_>,
            buffer: &mut ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            if self.read_error {
                return Poll::Ready(Err(io::Error::other("read failed")));
            }
            let remaining = &self.input[self.offset..];
            let count = remaining.len().min(buffer.remaining());
            buffer.put_slice(&remaining[..count]);
            self.offset += count;
            Poll::Ready(Ok(()))
        }
    }

    impl AsyncWrite for TestIo {
        fn poll_write(
            mut self: Pin<&mut Self>,
            _context: &mut Context<'_>,
            data: &[u8],
        ) -> Poll<io::Result<usize>> {
            if self.write_error {
                return Poll::Ready(Err(io::Error::other("write failed")));
            }
            self.output.extend_from_slice(data);
            Poll::Ready(Ok(data.len()))
        }

        fn poll_flush(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<()>> {
            if self.flush_error {
                Poll::Ready(Err(io::Error::other("flush failed")))
            } else {
                Poll::Ready(Ok(()))
            }
        }

        fn poll_shutdown(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<()>> {
            Poll::Ready(Ok(()))
        }
    }

    #[tokio::test]
    async fn framed_io_is_bounded() {
        let mut input = TestIo::reader(b"hello\nrest");
        assert_eq!(read_frame(&mut input, b'\n', 6).await.unwrap(), b"hello\n");
        let mut input = TestIo::reader(b"too-long\n");
        assert!(read_frame(&mut input, b'\n', 4).await.is_err());
        let mut input = TestIo {
            read_error: true,
            ..TestIo::default()
        };
        assert!(read_frame(&mut input, b'\n', 4).await.is_err());

        let mut output = TestIo::default();
        write_frame(&mut output, b"hello").await.unwrap();
        assert_eq!(output.output, b"hello");
        output.shutdown().await.unwrap();
        let mut output = TestIo {
            write_error: true,
            ..TestIo::default()
        };
        assert!(write_frame(&mut output, b"hello").await.is_err());
        let mut output = TestIo {
            flush_error: true,
            ..TestIo::default()
        };
        assert!(write_frame(&mut output, b"hello").await.is_err());
    }

    #[tokio::test]
    async fn tcp_udp_and_default_policy_use_real_loopback_sockets() {
        let config = TcpConfig::default();
        assert_eq!(config.connect_timeout, Duration::from_secs(10));
        assert!(config.no_delay);

        let udp = bind_udp("127.0.0.1:0").await.unwrap();
        assert!(udp.local_addr().unwrap().port() > 0);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let accepted = tokio::spawn(async move { listener.accept().await.unwrap() });
        let stream = connect_tcp(address, config).await.unwrap();
        assert!(stream.nodelay().unwrap());
        drop(accepted.await.unwrap());

        assert!(
            connect_tcp(
                address,
                TcpConfig {
                    connect_timeout: Duration::ZERO,
                    no_delay: false,
                }
            )
            .await
            .is_err()
        );
        assert!(connect_tcp(address, config).await.is_err());
        assert!(bind_udp("not a socket address").await.is_err());

        assert!(
            connect_tcp_inner(
                address,
                config,
                ConnectFaults {
                    timeout: true,
                    policy: false,
                },
            )
            .await
            .is_err()
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let accepted = tokio::spawn(async move { listener.accept().await.unwrap() });
        assert!(
            connect_tcp_inner(
                address,
                config,
                ConnectFaults {
                    timeout: false,
                    policy: true,
                },
            )
            .await
            .is_err()
        );
        drop(accepted.await.unwrap());
    }

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

    impl crate::nio::ChannelHandler for CountingHandler {
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
        let handler: Arc<dyn crate::nio::ChannelHandler> =
            Arc::new(CountingHandler(Arc::clone(&callback_count)));
        let adapter = crate::nio::nio_client::HandlerAction(Arc::clone(&handler));
        adapter.do_action(&session, b"ignored");
        adapter.failed(&error, &session);
        NioAcceptHandler
            .completed(session.clone(), handler.as_ref())
            .unwrap();
        let closure = |_session: AioSession| Ok(());
        <_ as crate::nio::ChannelHandler>::handle(&closure, session.clone()).unwrap();
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
        let handler: Arc<dyn crate::nio::ChannelHandler> =
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
            let guard = client.session().stream_slot().lock().await;
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

        assert!(session.stream_slot().lock().await.is_some());
        assert_eq!(session.write(b"hello").await.unwrap(), 5);
        assert!(session.stream_slot().lock().await.is_some());
        assert_eq!(session.read().await.unwrap(), 5);
        assert_eq!(action.bytes.load(Ordering::SeqCst), 5);
        assert!(session.is_open().await);
        session.close().await.unwrap();
        peer.await.unwrap();
    }
}