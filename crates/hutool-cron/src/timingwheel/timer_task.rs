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

/// A one-shot task with a relative delay.
#[derive(Clone)]
pub struct TimerTask(pub(crate) Arc<TimerTaskInner>);

impl TimerTask {
    /// Creates a delayed one-shot task.
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

    /// Returns the configured relative delay.
    #[must_use]
    pub fn delay(&self) -> Duration {
        Duration::from_millis(self.0.delay_ms)
    }

    /// Returns the configured delay in milliseconds.
    #[must_use]
    pub fn delay_ms(&self) -> u64 {
        self.0.delay_ms
    }

    /// Returns the absolute deadline used by timing wheels.
    #[must_use]
    pub(crate) fn deadline_ms(&self) -> i64 {
        self.0.deadline_ms
    }

    /// Executes the task at most once and reports whether it ran.
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
