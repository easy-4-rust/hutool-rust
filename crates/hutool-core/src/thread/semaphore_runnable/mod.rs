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

mod semaphore;
mod semaphore_runnable;

pub use semaphore::Semaphore;
pub use semaphore_runnable::SemaphoreRunnable;
