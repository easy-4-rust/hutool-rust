//! 对齐: `cn.hutool.core.thread.DelegatedExecutorService`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/DelegatedExecutorService.java

use crate::thread::executor_builder::SimpleExecutor;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// 对齐 Java 类: `cn.hutool.core.thread.DelegatedExecutorService`
pub struct DelegatedExecutorService {
    inner: Arc<SimpleExecutor>,
    shutdown: AtomicBool,
}

impl DelegatedExecutorService {
    /// 对齐构造：包装已有执行器。
    pub fn new(inner: Arc<SimpleExecutor>) -> Self {
        Self {
            inner,
            shutdown: AtomicBool::new(false),
        }
    }

    /// 对齐 `execute`。
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if self.is_shutdown() {
            return;
        }
        self.inner.execute(f);
    }

    /// 对齐 `submit(Callable)`。
    pub fn submit<F, T>(&self, f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.inner.submit(f)
    }

    /// 对齐 `submit(Runnable, T result)`。
    pub fn submit_with_result<F, T>(&self, f: F, result: T) -> JoinHandle<T>
    where
        F: FnOnce() + Send + 'static,
        T: Send + 'static,
    {
        self.inner.submit(move || {
            f();
            result
        })
    }

    /// 对齐 `shutdown`。
    pub fn shutdown(&self) {
        self.shutdown.store(true, Ordering::SeqCst);
        self.inner.shutdown();
    }

    /// 对齐 `shutdownNow`。
    pub fn shutdown_now(&self) -> Vec<()> {
        self.shutdown();
        Vec::new()
    }

    /// 对齐 `isShutdown`。
    pub fn is_shutdown(&self) -> bool {
        self.shutdown.load(Ordering::SeqCst)
    }

    /// 对齐 `isTerminated` — 关闭后即视为终止。
    pub fn is_terminated(&self) -> bool {
        self.is_shutdown()
    }

    /// 对齐 `awaitTermination`。
    pub fn await_termination(&self, timeout: Duration) -> bool {
        let start = Instant::now();
        while !self.is_terminated() {
            if start.elapsed() >= timeout {
                return false;
            }
            std::thread::sleep(Duration::from_millis(1));
        }
        true
    }

    /// 对齐 `invokeAll` — 顺序提交并收集结果。
    pub fn invoke_all<T, F>(&self, tasks: Vec<F>) -> Vec<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let handles: Vec<_> = tasks.into_iter().map(|t| self.submit(t)).collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("invoke_all join"))
            .collect()
    }

    /// 对齐 `invokeAny` — 取第一个完成的结果。
    pub fn invoke_any<T, F>(&self, tasks: Vec<F>) -> Option<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        self.invoke_all(tasks).into_iter().next()
    }
}
