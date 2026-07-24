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

/// A timing-wheel bucket.
#[derive(Debug, Default)]
pub struct TimerTaskList {
    expiration_ms: i64,
    tasks: Vec<TimerTask>,
}

impl TimerTaskList {
    /// Creates an empty, unscheduled bucket.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            expiration_ms: -1,
            tasks: Vec::new(),
        }
    }

    /// Changes the expiration and reports whether it changed.
    pub fn set_expiration(&mut self, expiration_ms: i64) -> bool {
        if self.expiration_ms == expiration_ms {
            false
        } else {
            self.expiration_ms = expiration_ms;
            true
        }
    }

    /// Returns the expiration in Unix milliseconds.
    #[must_use]
    pub const fn expiration(&self) -> i64 {
        self.expiration_ms
    }

    /// Adds a task to this bucket.
    pub fn add_task(&mut self, task: TimerTask) {
        self.tasks.push(task);
    }

    /// Removes one task by shared identity.
    pub fn remove_task(&mut self, task: &TimerTask) -> bool {
        if let Some(index) = self
            .tasks
            .iter()
            .position(|candidate| Arc::ptr_eq(&candidate.0, &task.0))
        {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }

    /// Drains the bucket through a caller-supplied consumer.
    pub fn flush<F>(&mut self, mut consumer: F)
    where
        F: FnMut(TimerTask),
    {
        for task in self.tasks.drain(..) {
            consumer(task);
        }
        self.expiration_ms = -1;
    }

    /// Returns the non-negative remaining delay.
    #[must_use]
    pub fn delay(&self, now_ms: i64) -> Duration {
        Duration::from_millis(
            u64::try_from(self.expiration_ms.saturating_sub(now_ms).max(0)).unwrap_or_default(),
        )
    }

    /// Compares bucket deadlines.
    #[must_use]
    pub fn compare_to(&self, other: &Self) -> Ordering {
        self.expiration_ms.cmp(&other.expiration_ms)
    }

    /// Returns the current task count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// Returns whether the bucket is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

use super::{ScheduledTask, TaskFn, TimerCommand, TimerTaskInner, bounded_wait, now_millis, run_timer};
