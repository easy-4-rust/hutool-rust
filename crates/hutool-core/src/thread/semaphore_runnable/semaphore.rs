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

use super::semaphore_runnable::SemaphoreRunnable;

/// 同步计数信号量, 对齐 Java `java.util.concurrent.Semaphore` 的核心语义。
///
/// 供 [`SemaphoreRunnable`] 与后续 `SegmentLock` / `LockUtil` 分段信号量复用。
/// 多个线程必须共享同一 [`Arc<Semaphore>`](Semaphore)。
#[derive(Debug)]
pub struct Semaphore {
    /// 当前可用许可数。
    permits: Mutex<usize>,
    /// 许可变为可用时唤醒等待者。
    available: Condvar,
}

impl Semaphore {
    /// 创建具有 `permits` 个初始许可的信号量。
    ///
    /// 对齐 Java: `new Semaphore(int permits)`。
    #[must_use]
    pub fn new(permits: usize) -> Self {
        Self {
            permits: Mutex::new(permits),
            available: Condvar::new(),
        }
    }

    /// 获取一个许可; 无可用许可时阻塞当前线程。
    ///
    /// 对齐 Java: `Semaphore#acquire()` (Rust 无线程中断, 故不抛出
    /// `InterruptedException`; 阻塞直至获得许可)。
    pub fn acquire(&self) {
        let mut permits = self.permits.lock();
        while *permits == 0 {
            self.available.wait(&mut permits);
        }
        *permits -= 1;
    }

    /// 尝试获取一个许可; 成功返回 `true`, 否则立即返回 `false`。
    ///
    /// 对齐 Java: `Semaphore#tryAcquire()`。
    #[must_use]
    pub fn try_acquire(&self) -> bool {
        let mut permits = self.permits.lock();
        if *permits == 0 {
            return false;
        }
        *permits -= 1;
        true
    }

    /// 释放一个许可, 可能唤醒一个等待中的获取者。
    ///
    /// 对齐 Java: `Semaphore#release()`。许可数可超过初始值(与 JDK 一致)。
    pub fn release(&self) {
        let mut permits = self.permits.lock();
        *permits += 1;
        self.available.notify_one();
    }

    /// 返回当前可用许可数。
    ///
    /// 对齐 Java: `Semaphore#availablePermits()`。
    #[must_use]
    pub fn available_permits(&self) -> usize {
        *self.permits.lock()
    }
}

impl Drop for PermitGuard<'_> {
    fn drop(&mut self) {
        self.semaphore.release();
    }
}

struct PermitGuard<'a> {
    semaphore: &'a Semaphore,
}
