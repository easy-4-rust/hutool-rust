//! 对齐: `cn.hutool.core.thread.ThreadUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/thread/ThreadUtil.java
//!
//! 以 `std::thread` 提供可移植子集；JVM `ThreadLocal` / `ThreadGroup` 全局语义保持 planned。

use std::sync::atomic::Ordering;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use super::scheduled_handle::ScheduledHandle;
use super::thread_util::ThreadUtil;

/// 简易定时任务池。
#[derive(Debug, Default)]
pub struct ScheduledPool;

impl ScheduledPool {
    /// 创建调度池。
    pub fn new() -> Self {
        Self
    }

    /// fixedRate / fixedDelay 循环。
    pub fn schedule<F>(
        &self,
        command: F,
        initial_delay: Duration,
        period: Duration,
        fixed_rate: bool,
    ) -> ScheduledHandle
    where
        F: Fn() + Send + Sync + 'static,
    {
        let stop = Arc::new((Mutex::new(false), Condvar::new()));
        let stop2 = Arc::clone(&stop);
        let command = Arc::new(command);
        let join = thread::Builder::new()
            .name(format!(
                "hutool-schedule-{}",
                SCHEDULE_SEQ.fetch_add(1, Ordering::Relaxed)
            ))
            .spawn(move || {
                {
                    let (lock, cv) = &*stop2;
                    let mut g = lock.lock().unwrap();
                    let deadline = Instant::now() + initial_delay;
                    while !*g {
                        let now = Instant::now();
                        if now >= deadline {
                            break;
                        }
                        let (gg, _) = cv.wait_timeout(g, deadline - now).unwrap();
                        g = gg;
                    }
                    if *g {
                        return;
                    }
                }
                loop {
                    let started = Instant::now();
                    command();
                    let (lock, cv) = &*stop2;
                    let mut g = lock.lock().unwrap();
                    let wait = if fixed_rate {
                        period.saturating_sub(started.elapsed())
                    } else {
                        period
                    };
                    let deadline = Instant::now() + wait;
                    while !*g {
                        let now = Instant::now();
                        if now >= deadline {
                            break;
                        }
                        let (gg, _) = cv.wait_timeout(g, deadline - now).unwrap();
                        g = gg;
                    }
                    if *g {
                        break;
                    }
                }
            })
            .expect("spawn schedule thread");
        ScheduledHandle {
            stop,
            join: Some(join),
        }
    }
}

impl ThreadUtil {
    /// 对齐 `sync(Object)` 近似 —— 永久等待直至 [`Self::notify_sync`]。
    pub fn sync() {
        let (lock, cv) = SYNC_SLOT.get_or_init(|| (Mutex::new(()), Condvar::new()));
        let g = lock.lock().unwrap();
        let _guard = cv.wait(g).unwrap();
    }

    /// 唤醒所有 `sync()` 等待者。
    pub fn notify_sync() {
        let (lock, cv) = SYNC_SLOT.get_or_init(|| (Mutex::new(()), Condvar::new()));
        let _g = lock.lock().unwrap();
        cv.notify_all();
    }
}

use super::{SCHEDULE_SEQ, SYNC_SLOT};
