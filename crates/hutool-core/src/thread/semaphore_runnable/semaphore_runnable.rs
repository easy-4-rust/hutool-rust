//! 对齐: `cn.hutool.core.thread.SemaphoreRunnable`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/SemaphoreRunnable.java
//!
//! 带有信号量控制的任务包装: 在执行前 `acquire`, 结束后(含 panic) `release`,
//! 从而限制可同时执行该逻辑的线程数。
//!
//! # 设计选择 (sync, 非 Tokio)
//!
//! `hutool-core` 刻意不引入 async runtime (`lib.rs` / Cargo.toml 均无 tokio)。
//! Java `java.util.concurrent.Semaphore` 本身是阻塞式计数信号量, 与 Hutool
//! 线程模型一致。因此本模块用 `parking_lot::{Mutex, Condvar}` 实现同步
//! [`Semaphore`], 而非 `tokio::sync::Semaphore`。

use std::sync::Arc;

use parking_lot::{Condvar, Mutex};

use super::semaphore::Semaphore;

/// 对齐 Java 类: `cn.hutool.core.thread.SemaphoreRunnable`
///
/// 包装一个可执行任务: 若信号量非空, 则先获取许可再执行, 结束后释放许可。
/// 若信号量为 `None`(对齐 Java `null`), `run` 为空操作且**不**执行任务
/// (与 Hutool 源码一致)。
pub struct SemaphoreRunnable<F> {
    /// 实际执行的逻辑 — 对齐 Java 字段 `runnable`。
    runnable: F,
    /// 信号量 — 对齐 Java 字段 `semaphore`; `None` 对应 Java `null`。
    semaphore: Option<Arc<Semaphore>>,
}

impl<F> SemaphoreRunnable<F> {
    /// 构造带可选信号量的包装任务。
    ///
    /// 对齐 Java: `SemaphoreRunnable(Runnable runnable, Semaphore semaphore)`。
    ///
    /// # 参数
    /// - `runnable`: 实际执行逻辑
    /// - `semaphore`: 多个线程必须共享同一信号量; 传 `None` 对齐 Java `null`
    #[must_use]
    pub fn new(runnable: F, semaphore: Option<Arc<Semaphore>>) -> Self {
        Self {
            runnable,
            semaphore,
        }
    }

    /// 使用非空共享信号量构造包装任务(常见路径)。
    ///
    /// # 参数
    /// - `runnable`: 实际执行逻辑
    /// - `semaphore`: 共享信号量(`Arc`), 对齐 Java「多个线程必须共享同一信号量」
    #[must_use]
    pub fn with_semaphore(runnable: F, semaphore: Arc<Semaphore>) -> Self {
        Self::new(runnable, Some(semaphore))
    }

    /// 获得信号量引用。
    ///
    /// 对齐 Java: `getSemaphore()`。
    #[must_use]
    pub fn semaphore(&self) -> Option<&Arc<Semaphore>> {
        self.semaphore.as_ref()
    }

    /// 获得信号量引用(Java 命名别名)。
    ///
    /// 对齐 Java: `getSemaphore()`。
    #[must_use]
    pub fn get_semaphore(&self) -> Option<&Arc<Semaphore>> {
        self.semaphore()
    }
}

impl<F> SemaphoreRunnable<F>
where
    F: FnMut(),
{
    /// 在信号量控制下执行任务。
    ///
    /// 对齐 Java: `run()`。
    ///
    /// 逻辑:
    /// 1. `semaphore == None` → 直接返回(不执行任务, 对齐 Hutool)
    /// 2. `acquire` → 执行 `runnable` → `release`(经 RAII, panic 时也会释放)
    pub fn run(&mut self) {
        let Some(semaphore) = self.semaphore.as_ref() else {
            return;
        };

        // 获取许可; 无可用许可时阻塞
        semaphore.acquire();
        // RAII 保证 runnable panic 时仍释放许可(对齐 Java finally)
        let _guard = PermitGuard {
            semaphore: semaphore.as_ref(),
        };
        (self.runnable)();
    }
}

struct PermitGuard<'a> {
    semaphore: &'a Semaphore,
}
