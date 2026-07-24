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

/// Explicitly started and stopped one-shot timer service.
#[derive(Debug)]
pub struct SystemTimer {
    delay_queue_timeout: Duration,
    pending: Vec<TimerTask>,
    sender: Option<mpsc::Sender<TimerCommand>>,
    worker: Option<JoinHandle<()>>,
}

impl Default for SystemTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemTimer {
    /// Creates a stopped timer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            delay_queue_timeout: Duration::from_millis(100),
            pending: Vec::new(),
            sender: None,
            worker: None,
        }
    }

    /// Sets the maximum worker wake-up interval.
    pub fn set_delay_queue_timeout(&mut self, timeout: Duration) -> Result<&mut Self, CronError> {
        if timeout.is_zero() || self.sender.is_some() {
            return Err(CronError::InvalidTimerState);
        }
        self.delay_queue_timeout = timeout;
        Ok(self)
    }

    /// Starts the owned worker. Starting twice is rejected.
    pub fn start(&mut self) -> Result<&mut Self, CronError> {
        self.start_with(|receiver, initial, timeout| {
            thread::Builder::new()
                .name("hutool-system-timer".to_owned())
                .spawn(move || run_timer(&receiver, initial, timeout))
        })
    }

    fn start_with<S>(&mut self, spawner: S) -> Result<&mut Self, CronError>
    where
        S: FnOnce(
            mpsc::Receiver<TimerCommand>,
            Vec<TimerTask>,
            Duration,
        ) -> std::io::Result<JoinHandle<()>>,
    {
        if self.sender.is_some() {
            return Err(CronError::TimerAlreadyStarted);
        }
        let (sender, receiver) = mpsc::channel();
        let timeout = self.delay_queue_timeout;
        let initial = std::mem::take(&mut self.pending);
        self.install_worker(sender, spawner(receiver, initial, timeout))
    }

    fn install_worker(
        &mut self,
        sender: mpsc::Sender<TimerCommand>,
        worker: std::io::Result<JoinHandle<()>>,
    ) -> Result<&mut Self, CronError> {
        let worker = worker.map_err(CronError::TimerThread)?;
        self.sender = Some(sender);
        self.worker = Some(worker);
        Ok(self)
    }

    /// Adds a task before or after start.
    pub fn add_task(&mut self, task: TimerTask) -> Result<(), CronError> {
        if let Some(sender) = &self.sender {
            sender
                .send(TimerCommand::Add(task))
                .map_err(|_| CronError::TimerStopped)
        } else {
            self.pending.push(task);
            Ok(())
        }
    }

    /// Stops the worker and waits for completion.
    pub fn stop(&mut self) {
        if let Some(sender) = self.sender.take() {
            let _ = sender.send(TimerCommand::Stop);
        }
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }

    /// Returns whether the worker is running.
    #[must_use]
    pub const fn is_started(&self) -> bool {
        self.sender.is_some()
    }
}

impl Drop for SystemTimer {
    fn drop(&mut self) {
        self.stop();
    }
}

use super::{ScheduledTask, TaskFn, TimerCommand, TimerTaskInner, bounded_wait, now_millis, run_timer};
