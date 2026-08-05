//! 对齐: `cn.hutool.core.thread.ThreadUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/ThreadUtil.java
//!
//! 以 `std::thread` 提供可移植子集；JVM `ThreadLocal` / `ThreadGroup` 全局语义保持 planned。

use std::sync::atomic::AtomicU64;
use std::sync::{Condvar, Mutex, OnceLock};

mod count_down_latch;
mod scheduled_handle;
mod scheduled_pool;
mod thread_util;

pub use count_down_latch::CountDownLatch;
pub use scheduled_handle::ScheduledHandle;
pub use scheduled_pool::ScheduledPool;
pub use thread_util::ThreadUtil;

static SCHEDULE_SEQ: AtomicU64 = AtomicU64::new(1);

static SYNC_SLOT: OnceLock<(Mutex<()>, Condvar)> = OnceLock::new();
