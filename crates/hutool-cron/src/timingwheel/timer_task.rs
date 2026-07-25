//! 对齐: `cn.hutool.cron.timingwheel.TimerTask`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/timingwheel/TimerTask.java
//! 中文说明: 具有相对延迟的一次性定时任务。

#![allow(clippy::missing_panics_doc)]

use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt,
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::CronError;

/// 对齐: `cn.hutool.cron.timingwheel.TimerTask`
/// 中文说明: 具有相对延迟的一次性定时任务。
///
/// A one-shot task with a relative delay.
#[derive(Clone)]
pub struct TimerTask(pub(crate) Arc<TimerTaskInner>);

impl TimerTask {
    /// 中文说明: 创建延迟的一次性任务。
    /// 对齐 Java 方法: `new`
    pub fn new<F>(task: F, delay: Duration) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        let delay_ms = u64::try_from(delay.as_millis()).unwrap_or(u64::MAX);
        let deadline_delay = i64::try_from(delay_ms).unwrap_or(i64::MAX);
        Self(Arc::new(TimerTaskInner {
            delay_ms,
            deadline_ms: now_millis().saturating_add(deadline_delay),
            task: Mutex::new(Some(Box::new(task))),
        }))
    }

    /// 中文说明: 返回配置的相对延迟。
    /// 对齐 Java 方法: `getDelay`
    #[must_use]
    pub fn delay(&self) -> Duration {
        Duration::from_millis(self.0.delay_ms)
    }

    /// 中文说明: 返回配置的延迟毫秒数。
    #[must_use]
    pub fn delay_ms(&self) -> u64 {
        self.0.delay_ms
    }

    /// Returns the absolute deadline used by timing wheels.
    #[must_use]
    pub(crate) fn deadline_ms(&self) -> i64 {
        self.0.deadline_ms
    }

    /// 中文说明: 最多执行一次任务，返回是否实际执行。
    /// 对齐 Java 方法: `run`
    pub fn execute(&self) -> bool {
        let task = self
            .0
            .task
            .lock()
            .expect("timer task mutex poisoned")
            .take();
        if let Some(task) = task {
            task();
            true
        } else {
            false
        }
    }
}

impl fmt::Debug for TimerTask {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TimerTask")
            .field("delay_ms", &self.0.delay_ms)
            .field("deadline_ms", &self.0.deadline_ms)
            .field(
                "pending",
                &self
                    .0
                    .task
                    .lock()
                    .expect("timer task mutex poisoned")
                    .is_some(),
            )
            .finish()
    }
}

use super::{ScheduledTask, TaskFn, TimerCommand, TimerTaskInner, bounded_wait, now_millis, run_timer};
