//! `NioUtil` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.nio.NioUtil`
//! 作用：提供注册通道到 selector 的能力。
//! Rust 端没有 selector 模型，仅保留 `register_channel` 用于校验
//! `TcpStream` 处于有效状态并能参与请求的操作。

use tokio::net::TcpStream;

use crate::nio::operation::Operation;
use crate::socket_runtime_exception::SocketRuntimeException;

/// NIO 工具类。
///
/// 对齐: cn.hutool.socket.nio.NioUtil
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/nio/NioUtil.java
/// 中文说明: 注册通道到 selector 的工具门面（Rust 端简化为状态校验）。
pub struct NioUtil;

impl NioUtil {
    /// 校验流可以参与请求的操作；Java 中会调用
    /// `channel.configureBlocking(false)` + `register(selector, ops)`。
    pub fn register_channel(
        stream: &TcpStream,
        _operation: Operation,
    ) -> Result<(), SocketRuntimeException> {
        stream.local_addr().map(|_| ()).map_err(Into::into)
    }
}