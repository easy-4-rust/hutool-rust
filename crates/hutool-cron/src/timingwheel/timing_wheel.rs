//! 对齐: `cn.hutool.cron.timingwheel.TimingWheel`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/timingwheel/TimingWheel.java
//! 中文说明: 单级时间轮，超出其间隔的任务会被拒绝，调用者可转发至溢出轮。

#![allow(clippy::missing_panics_doc)]

use std::{cmp::Ordering, fmt, sync::Arc, time::Duration};

use crate::CronError;

use super::timer_task::TimerTask;
use super::timer_task_list::TimerTaskList;

/// 对齐: `cn.hutool.cron.timingwheel.TimingWheel`
/// 中文说明: 单级时间轮，超出其间隔的任务会被拒绝，调用者可转发至溢出轮。
///
/// A single-level timing wheel. Tasks beyond its interval are rejected so a
/// caller can forward them to an overflow wheel or another bounded queue.
pub struct TimingWheel {
    tick_ms: i64,
    wheel_size: usize,
    interval_ms: i64,
    current_time_ms: i64,
    buckets: Vec<TimerTaskList>,
    consumer: Arc<dyn Fn(&TimerTaskList) + Send + Sync>,
}

impl fmt::Debug for TimingWheel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TimingWheel")
            .field("tick_ms", &self.tick_ms)
            .field("wheel_size", &self.wheel_size)
            .field("current_time_ms", &self.current_time_ms)
            .finish_non_exhaustive()
    }
}

impl TimingWheel {
    /// 中文说明: 在当前系统时间创建时间轮。
    /// 对齐 Java 方法: `new`
    pub fn new<F>(tick: Duration, wheel_size: usize, consumer: F) -> Result<Self, CronError>
    where
        F: Fn(&TimerTaskList) + Send + Sync + 'static,
    {
        Self::with_current_time(tick, wheel_size, now_millis(), consumer)
    }

    /// 中文说明: 在指定的 Unix 毫秒时间创建时间轮。
    pub fn with_current_time<F>(
        tick: Duration,
        wheel_size: usize,
        current_time_ms: i64,
        consumer: F,
    ) -> Result<Self, CronError>
    where
        F: Fn(&TimerTaskList) + Send + Sync + 'static,
    {
        Self::with_consumer(tick, wheel_size, current_time_ms, Arc::new(consumer))
    }

    fn with_consumer(
        tick: Duration,
        wheel_size: usize,
        current_time_ms: i64,
        consumer: Arc<dyn Fn(&TimerTaskList) + Send + Sync>,
    ) -> Result<Self, CronError> {
        let tick_ms = i64::try_from(tick.as_millis()).unwrap_or(i64::MAX);
        if tick_ms == 0 || wheel_size == 0 {
            return Err(CronError::InvalidTimingWheel);
        }
        let wheel_size_i64 =
            i64::try_from(wheel_size).map_err(|_| CronError::InvalidTimingWheel)?;
        let interval_ms = tick_ms
            .checked_mul(wheel_size_i64)
            .ok_or(CronError::InvalidTimingWheel)?;
        Ok(Self {
            tick_ms,
            wheel_size,
            interval_ms,
            current_time_ms: current_time_ms - current_time_ms.rem_euclid(tick_ms),
            buckets: (0..wheel_size).map(|_| TimerTaskList::new()).collect(),
            consumer,
        })
    }

    /// 中文说明: 当任务截止时间在当前时间轮间隔内时添加任务。
    /// 对齐 Java 方法: `addTask`
    pub fn add_task(&mut self, task: TimerTask) -> bool {
        let deadline = task.deadline_ms();
        if deadline < self.current_time_ms.saturating_add(self.tick_ms)
            || deadline >= self.current_time_ms.saturating_add(self.interval_ms)
        {
            return false;
        }
        let virtual_id = deadline / self.tick_ms;
        let wheel_size = i64::try_from(self.wheel_size).unwrap_or(i64::MAX);
        let index = usize::try_from(virtual_id.rem_euclid(wheel_size)).unwrap_or_default();
        let expiration = virtual_id.saturating_mul(self.tick_ms);
        let bucket = &mut self.buckets[index];
        bucket.set_expiration(expiration);
        bucket.add_task(task);
        (self.consumer)(bucket);
        true
    }

    /// 中文说明: 推进时间轮并刷新所有已过期的桶。
    /// 对齐 Java 方法: `advanceClock`
    pub fn advance_clock<F>(&mut self, timestamp_ms: i64, mut flush: F)
    where
        F: FnMut(TimerTask),
    {
        if timestamp_ms < self.current_time_ms.saturating_add(self.tick_ms) {
            return;
        }
        self.current_time_ms = timestamp_ms - timestamp_ms.rem_euclid(self.tick_ms);
        for bucket in &mut self.buckets {
            if bucket.expiration() >= 0 && bucket.expiration() <= self.current_time_ms {
                bucket.flush(&mut flush);
            }
        }
    }
}

impl PartialEq for ScheduledTask {
    fn eq(&self, other: &Self) -> bool {
        (self.deadline_ms, self.sequence) == (other.deadline_ms, other.sequence)
    }
}

impl Eq for ScheduledTask {}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.deadline_ms, other.sequence).cmp(&(self.deadline_ms, self.sequence))
    }
}

use super::{ScheduledTask, now_millis};
