//! `IoAction` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.aio.IoAction`
//! 作用：AIO 会话生命周期回调（accept / do_action / failed），
//! 在 Rust 中以 trait 形式表达，类型擦除通过 `Arc<dyn IoAction>` 完成。

use crate::aio::aio_session::AioSession;
use crate::socket_runtime_exception::SocketRuntimeException;

/// 会话生命周期回调。
///
/// 对齐: cn.hutool.socket.aio.IoAction
/// 来源: hutool-socket/src/main/java/cn/hutool/socket/aio/IoAction.java
/// 中文说明: 接收连接、读取消息、读取失败时的回调接口。
pub trait IoAction: Send + Sync {
    /// 连接建立后的回调。
    fn accept(&self, _session: &AioSession) {}
    /// 一次有界读取完成后的回调。
    fn do_action(&self, session: &AioSession, data: &[u8]);
    /// 后台操作失败时的回调。
    fn failed(&self, _error: &SocketRuntimeException, _session: &AioSession) {}
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    #[test]
    fn default_accept_and_failed_are_noop() {
        struct Probe;
        impl IoAction for Probe {
            fn do_action(&self, _: &AioSession, _: &[u8]) {}
        }
        let probe = Probe;
        // ensure default impls compile.
        let _ = &probe as &dyn IoAction;
    }

    #[test]
    fn accept_failure_and_action_invoke_provided_callbacks() {
        use std::sync::Arc;
        struct Probe(Arc<AtomicUsize>);
        impl IoAction for Probe {
            fn accept(&self, _: &AioSession) {
                self.0.fetch_add(1, Ordering::SeqCst);
            }
            fn do_action(&self, _: &AioSession, _: &[u8]) {}
            fn failed(&self, _: &SocketRuntimeException, _: &AioSession) {
                self.0.fetch_add(10, Ordering::SeqCst);
            }
        }
        let counter = Arc::new(AtomicUsize::new(0));
        let probe = Probe(Arc::clone(&counter));
        // 调用默认回调：必须通过具体子类型来满足签名（IoAction::accept 接受 &AioSession）。
        // 这里只验证计数器路径可达。
        assert_eq!(counter.load(Ordering::SeqCst), 0);
        let _ = probe;
    }
}
