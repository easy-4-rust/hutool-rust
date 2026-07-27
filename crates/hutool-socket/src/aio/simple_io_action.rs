//! `SimpleIoAction` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.aio.SimpleIoAction`
//! 作用：Java 中是抽象类，默认 accept 为空、failed 打印日志；
//! Rust 中以适配器 `SimpleIoAction<F>` 形式承载一个闭包，
//! 由调用方决定 accept / failed 行为。

use crate::aio::aio_session::AioSession;
use crate::aio::io_action::IoAction;

/// 仅实现 `do_action`，其他回调默认 no-op 的便利适配器。
///
/// 对齐: cn.hutool.socket.aio.SimpleIoAction
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/aio/SimpleIoAction.java
/// 中文说明: Java 中的默认实现；Rust 中通过闭包适配。
pub struct SimpleIoAction<F>(pub F);

impl<F> IoAction for SimpleIoAction<F>
where
    F: Fn(&AioSession, &[u8]) + Send + Sync,
{
    fn do_action(&self, session: &AioSession, data: &[u8]) {
        (self.0)(session, data);
    }
}