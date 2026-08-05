//! 对齐: `cn.hutool.core.thread.GlobalThreadPool`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/GlobalThreadPool.java

use crate::thread::executor_builder::{ExecutorBuilder, SimpleExecutor};
use std::sync::{Arc, OnceLock};
use std::thread::JoinHandle;

static POOL: OnceLock<Arc<SimpleExecutor>> = OnceLock::new();

/// 对齐 Java 类: `cn.hutool.core.thread.GlobalThreadPool`
#[derive(Debug, Clone, Default)]
pub struct GlobalThreadPool;

impl GlobalThreadPool {
    /// 对齐 `init()` — 懒初始化全局池。
    pub fn init() {
        let _ = Self::get_executor();
    }

    /// 对齐 `getExecutor()`。
    pub fn get_executor() -> Arc<SimpleExecutor> {
        POOL.get_or_init(|| {
            Arc::new(
                ExecutorBuilder::create()
                    .set_core_pool_size(2)
                    .set_max_pool_size(
                        std::thread::available_parallelism()
                            .map(|n| n.get())
                            .unwrap_or(4)
                            .max(2),
                    )
                    .build(),
            )
        })
        .clone()
    }

    /// 对齐 `execute(Runnable)`。
    pub fn execute<F>(f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        Self::get_executor().execute(f);
    }

    /// 对齐 `submit(Callable)` / `submit(Runnable)`。
    pub fn submit<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        Self::get_executor().submit(f)
    }

    /// 对齐 `shutdown(boolean isNow)`。
    pub fn shutdown(_is_now: bool) {
        if let Some(pool) = POOL.get() {
            pool.shutdown();
        }
    }
}
