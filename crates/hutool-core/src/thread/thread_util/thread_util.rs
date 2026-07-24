//! 对齐: `cn.hutool.core.thread.ThreadUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/ThreadUtil.java
//!
//! 以 `std::thread` 提供可移植子集；JVM `ThreadLocal` / `ThreadGroup` 全局语义保持 planned。

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::thread::{self, JoinHandle, Thread, ThreadId};
use std::time::{Duration, Instant};

use crate::thread::concurrency_tester::ConcurrencyTester;
use crate::thread::executor_builder::{ExecutorBuilder, SimpleExecutor};
use crate::thread::global_thread_pool::GlobalThreadPool;
use crate::thread::named_thread_factory::NamedThreadFactory;
use crate::thread::reject_policy::RejectPolicy;
use crate::thread::thread_factory_builder::ThreadFactoryBuilder;

/// 对齐 Java 类: `cn.hutool.core.thread.ThreadUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ThreadUtil;

use super::{SCHEDULE_SEQ, SYNC_SLOT};
