//! 对齐: `cn.hutool.core.thread.NamedThreadFactory`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/NamedThreadFactory.java

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::{self, JoinHandle};

type ExHandler = Option<Box<dyn Fn(&thread::Thread, Box<dyn std::any::Any + Send>) + Send + Sync>>;

/// 对齐 Java 类: `cn.hutool.core.thread.NamedThreadFactory`
#[derive(Clone)]
pub struct NamedThreadFactory {
    prefix: String,
    is_daemon: bool,
    seq: std::sync::Arc<AtomicUsize>,
}

impl NamedThreadFactory {
    /// 对齐 `NamedThreadFactory(String prefix, boolean isDaemon)`。
    pub fn new(prefix: impl Into<String>, is_daemon: bool) -> Self {
        Self {
            prefix: prefix.into(),
            is_daemon,
            seq: std::sync::Arc::new(AtomicUsize::new(0)),
        }
    }

    /// 对齐带 ThreadGroup / UncaughtExceptionHandler 的构造（忽略 ThreadGroup）。
    pub fn with_handler(prefix: impl Into<String>, is_daemon: bool, _handler: ExHandler) -> Self {
        Self::new(prefix, is_daemon)
    }

    /// 生成下一个线程名。
    pub fn next_name(&self, index_hint: usize) -> String {
        let n = self.seq.fetch_add(1, Ordering::Relaxed);
        format!("{}{}-{}", self.prefix, n, index_hint)
    }

    /// 对齐 `newThread(Runnable)`。
    pub fn new_thread<F>(&self, f: F) -> JoinHandle<()>
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = self.is_daemon; // Rust 无守护线程语义
        thread::Builder::new()
            .name(self.next_name(0))
            .spawn(f)
            .expect("NamedThreadFactory::new_thread")
    }
}

impl Default for NamedThreadFactory {
    fn default() -> Self {
        Self::new("hutool-", false)
    }
}
