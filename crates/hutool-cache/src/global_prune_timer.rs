//! `GlobalPruneTimer` 与 `PruneHandle`。
//!
//! 对齐 Java 类: `cn.hutool.cache.GlobalPruneTimer`
//! 来源: `hutool-cache/src/main/java/cn/hutool/cache/GlobalPruneTimer.java`
//!
//! Rust 版本不维护全局线程池，而是把定时任务绑定到显式句柄上；
//! 句柄被丢弃时，后台清理线程会被安全停止。

use std::fmt;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

/// 定时清理任务的控制句柄。
///
/// 对齐 Java 中 `ScheduledFuture<?>` 的职责：持有后可让后台任务持续运行，
/// 丢弃时自动停止线程并回收资源。
pub struct PruneHandle {
    pub(crate) stop: Option<mpsc::Sender<()>>,
    pub(crate) worker: Option<JoinHandle<()>>,
}

impl fmt::Debug for PruneHandle {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PruneHandle")
            .finish_non_exhaustive()
    }
}

impl Drop for PruneHandle {
    fn drop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

/// 全局缓存清理定时器门面。
///
/// Java 使用单例线程池，这里改为显式创建周期任务，以保持 Rust 侧资源所有权清晰。
pub struct GlobalPruneTimer;

impl GlobalPruneTimer {
    /// 创建一个固定周期执行的清理任务。
    pub fn schedule<F>(task: F, delay: Duration) -> PruneHandle
    where
        F: FnMut() + Send + 'static,
    {
        Self::schedule_boxed(Box::new(task), delay)
    }

    fn schedule_boxed(mut task: Box<dyn FnMut() + Send>, delay: Duration) -> PruneHandle {
        let delay = if delay.is_zero() {
            Duration::from_millis(1)
        } else {
            delay
        };
        let (stop, receiver) = mpsc::channel();
        let worker = thread::spawn(move || {
            loop {
                match receiver.recv_timeout(delay) {
                    Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    Err(mpsc::RecvTimeoutError::Timeout) => task(),
                }
            }
        });
        PruneHandle {
            stop: Some(stop),
            worker: Some(worker),
        }
    }

    /// 对齐 Java `create()` 的兼容入口。
    pub const fn create() {}

    /// 对齐 Java `shutdown()` 的兼容入口。
    pub const fn shutdown() {}

    /// 对齐 Java `shutdownNow()` 的兼容入口。
    #[must_use]
    pub fn shutdown_now() -> Vec<JoinHandle<()>> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{GlobalPruneTimer, PruneHandle};
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    use std::thread;
    use std::time::Duration;

    #[test]
    fn prune_handle_runs_and_stops_worker() {
        let calls = Arc::new(AtomicUsize::new(0));
        let sink = Arc::clone(&calls);
        let handle = GlobalPruneTimer::schedule(
            move || {
                sink.fetch_add(1, Ordering::Relaxed);
            },
            Duration::from_millis(2),
        );
        thread::sleep(Duration::from_millis(8));
        drop(handle);
        assert!(calls.load(Ordering::Relaxed) > 0);

        let zero_delay = GlobalPruneTimer::schedule(|| {}, Duration::ZERO);
        thread::sleep(Duration::from_millis(3));
        drop(zero_delay);

        drop(PruneHandle {
            stop: None,
            worker: None,
        });
        GlobalPruneTimer::create();
        GlobalPruneTimer::shutdown();
        let _ = GlobalPruneTimer::shutdown_now();
    }
}
