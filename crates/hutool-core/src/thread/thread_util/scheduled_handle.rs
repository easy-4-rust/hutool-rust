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

use super::count_down_latch::CountDownLatch;
use super::scheduled_pool::ScheduledPool;
use super::thread_util::ThreadUtil;

/// 简易定时调度句柄（对齐 `ScheduledThreadPoolExecutor` 可运行子集）。
pub struct ScheduledHandle {
    pub(crate) stop: Arc<(Mutex<bool>, Condvar)>,
    pub(crate) join: Option<JoinHandle<()>>,
}

impl ScheduledHandle {
    /// 停止定时任务并等待调度线程退出。
    pub fn shutdown(mut self) {
        {
            let (lock, cv) = &*self.stop;
            *lock.lock().unwrap() = true;
            cv.notify_all();
        }
        if let Some(h) = self.join.take() {
            let _ = h.join();
        }
    }
}

impl Drop for ScheduledHandle {
    fn drop(&mut self) {
        let (lock, cv) = &*self.stop;
        *lock.lock().unwrap() = true;
        cv.notify_all();
    }
}

impl ThreadUtil {
    /// 对齐 Java: `MAIN_THREAD_NAME`
    pub const MAIN_THREAD_NAME: &'static str = "main";

    /// 对齐 Java: `sleep(long)` —— 返回 bool 以对齐中断语义（Rust 恒为 true）。
    pub fn sleep(millis: u64) -> bool {
        if millis > 0 {
            thread::sleep(Duration::from_millis(millis));
        }
        true
    }

    /// 对齐 Java: `sleep(Number, TimeUnit)` —— Duration 形态。
    pub fn sleep_duration(timeout: Duration) -> bool {
        if !timeout.is_zero() {
            thread::sleep(timeout);
        }
        true
    }

    /// 对齐 Java: `safeSleep(long)` —— 保证实际休眠不少于给定毫秒。
    pub fn safe_sleep(millis: u64) -> bool {
        let target = Duration::from_millis(millis);
        let start = Instant::now();
        while start.elapsed() < target {
            let remain = target.saturating_sub(start.elapsed());
            if !Self::sleep_duration(remain) {
                return false;
            }
        }
        true
    }

    /// 对齐 Java: `getMainThread()` 近似 —— 返回当前线程。
    #[must_use]
    pub fn current_thread() -> Thread {
        thread::current()
    }

    /// 对齐 Java: `getCurrentThreadId` 近似。
    #[must_use]
    pub fn current_thread_id() -> ThreadId {
        thread::current().id()
    }

    /// 对齐 Java: 线程名比较 `MAIN_THREAD_NAME`。
    #[must_use]
    pub fn is_main_thread() -> bool {
        thread::current().name() == Some(Self::MAIN_THREAD_NAME)
    }

    /// 对齐 Java: `newThread(Runnable, String)`。
    pub fn new_thread<F>(f: F, name: impl Into<String>) -> JoinHandle<()>
    where
        F: FnOnce() + Send + 'static,
    {
        Self::new_thread_daemon(f, name, false)
    }

    /// 对齐 Java: `newThread(Runnable, String, boolean)`。
    pub fn new_thread_daemon<F>(f: F, name: impl Into<String>, is_daemon: bool) -> JoinHandle<()>
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = is_daemon;
        thread::Builder::new()
            .name(name.into())
            .spawn(f)
            .expect("failed to spawn thread")
    }

    /// 对齐 Java: `execAsync(Runnable, boolean)`。
    pub fn exec_async_daemon<F>(f: F, is_daemon: bool) -> JoinHandle<()>
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = is_daemon;
        thread::spawn(f)
    }

    /// 对齐 Java: `execAsync(Runnable)` —— fire-and-forget spawn。
    pub fn exec_async<F>(f: F) -> JoinHandle<()>
    where
        F: FnOnce() + Send + 'static,
    {
        thread::spawn(f)
    }

    /// 对齐 Java: `execAsync(Callable)` —— 经全局池提交。
    pub fn exec_async_call<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        GlobalThreadPool::submit(f)
    }

    /// 对齐 Java: `execute(Runnable)` —— 全局池。
    pub fn execute<F>(f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        GlobalThreadPool::execute(f);
    }

    /// 对齐 Java: `newExecutor(int)` —— 构建并返回执行器。
    #[must_use]
    pub fn new_executor(core_pool_size: usize) -> SimpleExecutor {
        let mut b = ExecutorBuilder::create();
        if core_pool_size > 0 {
            b = b
                .set_core_pool_size(core_pool_size)
                .set_max_pool_size(core_pool_size);
        }
        b.build()
    }

    /// 对齐 Java: `newExecutor()` —— SynchronousQueue 语义。
    #[must_use]
    pub fn new_executor_default() -> SimpleExecutor {
        ExecutorBuilder::create().use_synchronous_queue().build()
    }

    /// 对齐 Java: `newExecutor(int, int)`。
    #[must_use]
    pub fn new_executor_sized(core: usize, max: usize) -> SimpleExecutor {
        ExecutorBuilder::create()
            .set_core_pool_size(core)
            .set_max_pool_size(max)
            .build()
    }

    /// 对齐 Java: `newExecutor(int, int, int)`。
    #[must_use]
    pub fn new_executor_queued(core: usize, max: usize, queue: usize) -> SimpleExecutor {
        ExecutorBuilder::create()
            .set_core_pool_size(core)
            .set_max_pool_size(max)
            .use_array_blocking_queue(queue)
            .build()
    }

    /// 对齐 Java: `newSingleExecutor()`。
    #[must_use]
    pub fn new_single_executor() -> SimpleExecutor {
        ExecutorBuilder::create()
            .set_core_pool_size(1)
            .set_max_pool_size(1)
            .set_keep_alive_time(Duration::ZERO)
            .build_finalizable()
    }

    /// 对齐 Java: `newExecutorByBlockingCoefficient(float)`。
    #[must_use]
    pub fn new_executor_by_blocking_coefficient(blocking_coefficient: f32) -> SimpleExecutor {
        assert!(
            (0.0..1.0).contains(&blocking_coefficient),
            "[blockingCoefficient] must between 0 and 1, or equals 0."
        );
        let cpus = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1) as f32;
        let pool = (cpus / (1.0 - blocking_coefficient)).max(1.0) as usize;
        ExecutorBuilder::create()
            .set_core_pool_size(pool)
            .set_max_pool_size(pool)
            .set_keep_alive_time(Duration::ZERO)
            .build()
    }

    /// 对齐 Java: `newFixedExecutor(nThreads, threadNamePrefix, isBlocked)`。
    #[must_use]
    pub fn new_fixed_executor(
        n_threads: usize,
        thread_name_prefix: &str,
        is_blocked: bool,
    ) -> SimpleExecutor {
        Self::new_fixed_executor_queued(n_threads, 1024, thread_name_prefix, is_blocked)
    }

    /// 对齐 Java: `newFixedExecutor(..., maximumQueueSize, ...)`。
    #[must_use]
    pub fn new_fixed_executor_queued(
        n_threads: usize,
        maximum_queue_size: usize,
        thread_name_prefix: &str,
        is_blocked: bool,
    ) -> SimpleExecutor {
        let policy = if is_blocked {
            RejectPolicy::BLOCK
        } else {
            RejectPolicy::ABORT
        };
        ExecutorBuilder::create()
            .set_core_pool_size(n_threads)
            .set_max_pool_size(n_threads)
            .use_array_blocking_queue(maximum_queue_size)
            .set_thread_factory(Self::create_thread_factory(thread_name_prefix))
            .set_handler(policy)
            .build()
    }

    /// 对齐 Java: `createThreadFactoryBuilder()`。
    #[must_use]
    pub fn create_thread_factory_builder() -> ThreadFactoryBuilder {
        ThreadFactoryBuilder::create()
    }

    /// 对齐 Java: `createThreadFactory(String)`。
    #[must_use]
    pub fn create_thread_factory(thread_name_prefix: &str) -> NamedThreadFactory {
        ThreadFactoryBuilder::create()
            .set_name_prefix(thread_name_prefix)
            .build()
    }

    /// 对齐 Java: `newNamedThreadFactory(prefix, isDaemon)`。
    #[must_use]
    pub fn new_named_thread_factory(prefix: &str, is_daemon: bool) -> NamedThreadFactory {
        NamedThreadFactory::new(prefix, is_daemon)
    }

    /// 对齐 Java: `newCountDownLatch(int)`。
    #[must_use]
    pub fn new_count_down_latch(thread_count: usize) -> Arc<CountDownLatch> {
        Arc::new(CountDownLatch::new(thread_count))
    }

    /// 对齐 Java: `getStackTrace()`。
    #[must_use]
    pub fn get_stack_trace() -> Vec<String> {
        let bt = std::backtrace::Backtrace::force_capture();
        format!("{bt}").lines().map(str::to_string).collect()
    }

    /// 对齐 Java: `getStackTraceElement(int)`。
    #[must_use]
    pub fn get_stack_trace_element(i: isize) -> Option<String> {
        let frames = Self::get_stack_trace();
        let idx = if i < 0 {
            frames.len().wrapping_add(i as usize)
        } else {
            i as usize
        };
        frames.get(idx).cloned()
    }

    /// 对齐 Java: `waitForDie(Thread)` —— join。
    pub fn wait_for_die(handle: JoinHandle<()>) {
        let _ = handle.join();
    }

    /// 对齐 Java: `joinThread` 近似 —— join JoinHandle 列表。
    pub fn join_handles(handles: Vec<JoinHandle<()>>) {
        for h in handles {
            let _ = h.join();
        }
    }

    /// 对齐 Java: `concurrencyTest(int, Runnable)`。
    pub fn concurrency_test<F>(thread_size: usize, f: F) -> ConcurrencyTester
    where
        F: Fn() + Send + Sync + 'static,
    {
        let mut tester = ConcurrencyTester::new(thread_size);
        tester.test(f);
        tester
    }

    /// 对齐 Java: `createScheduledExecutor(int)`。
    #[must_use]
    pub fn create_scheduled_executor(_core_pool_size: usize) -> Arc<ScheduledPool> {
        Arc::new(ScheduledPool::new())
    }

    /// 对齐 Java: `schedule(..., fixedRateOrFixedDelay)` —— 毫秒。
    pub fn schedule<F>(
        pool: Option<Arc<ScheduledPool>>,
        command: F,
        initial_delay_ms: u64,
        period_ms: u64,
        fixed_rate: bool,
    ) -> ScheduledHandle
    where
        F: Fn() + Send + Sync + 'static,
    {
        let pool = pool.unwrap_or_else(|| Self::create_scheduled_executor(2));
        pool.schedule(
            command,
            Duration::from_millis(initial_delay_ms),
            Duration::from_millis(period_ms),
            fixed_rate,
        )
    }
}

use super::{SCHEDULE_SEQ, SYNC_SLOT};
