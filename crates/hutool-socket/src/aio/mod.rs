//! Hutool `cn.hutool.socket.aio` AIO 子包对齐。
//!
//! 文件拆分：
//! - `aio_client.rs` → AioClient
//! - `aio_server.rs` → AioServer
//! - `aio_session.rs` → AioSession
//! - `aio_accept_handler.rs` → AcceptHandler
//! - `io_action.rs` → IoAction
//! - `read_handler.rs` → ReadHandler
//! - `simple_io_action.rs` → SimpleIoAction

mod aio_accept_handler;
mod aio_client;
mod aio_server;
pub(crate) mod aio_session;
mod io_action;
mod read_handler;
mod simple_io_action;

// crate 内部以及外部均可访问（AioClient 等需要在 aio:: 命名空间下也可达）。
pub use aio_accept_handler::AcceptHandler;
pub use aio_client::AioClient;
pub use aio_server::AioServer;
pub use aio_session::AioSession;
pub use io_action::IoAction;
pub use read_handler::ReadHandler;
pub use simple_io_action::SimpleIoAction;