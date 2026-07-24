//! Explicitly owned timer and timing-wheel primitives.

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

use super::timer_task::TimerTask;
use super::timer_task_list::TimerTaskList;

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
    /// Creates a wheel at the current system time.
    pub fn new<F>(tick: Duration, wheel_size: usize, consumer: F) -> Result<Self, CronError>
    where
        F: Fn(&TimerTaskList) + Send + Sync + 'static,
    {
        Self::with_current_time(tick, wheel_size, now_millis(), consumer)
    }

    /// Creates a wheel at an explicit Unix-millisecond time.
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

    /// Adds a task when its deadline fits this wheel's current interval.
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

    /// Advances the wheel and flushes every elapsed bucket.
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

use super::{ScheduledTask, TaskFn, TimerCommand, TimerTaskInner, bounded_wait, now_millis, run_timer};
