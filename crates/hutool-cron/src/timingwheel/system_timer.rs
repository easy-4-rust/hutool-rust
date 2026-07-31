//! 对齐: `cn.hutool.cron.timingwheel.SystemTimer`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/timingwheel/SystemTimer.java
//! 中文说明: 显式启动和停止的一次性定时器服务。

#![allow(clippy::missing_panics_doc)]

use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
    time::Duration,
};

use crate::CronError;

use super::timer_task::TimerTask;

/// 对齐: `cn.hutool.cron.timingwheel.SystemTimer`
/// 中文说明: 显式启动和停止的一次性定时器服务。
///
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
    /// 中文说明: 创建已停止的定时器。
    /// 对齐 Java 方法: `new`
    #[must_use]
    pub fn new() -> Self {
        Self {
            delay_queue_timeout: Duration::from_millis(100),
            pending: Vec::new(),
            sender: None,
            worker: None,
        }
    }

    /// 中文说明: 设置工作线程的最大唤醒间隔。
    /// 对齐 Java 方法: `setDelayQueueTimeout`
    pub fn set_delay_queue_timeout(&mut self, timeout: Duration) -> Result<&mut Self, CronError> {
        if timeout.is_zero() || self.sender.is_some() {
            return Err(CronError::InvalidTimerState);
        }
        self.delay_queue_timeout = timeout;
        Ok(self)
    }

    /// 中文说明: 启动拥有的工作线程，重复启动会被拒绝。
    /// 对齐 Java 方法: `start`
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

    /// 中文说明: 在启动前或启动后添加任务。
    /// 对齐 Java 方法: `addTask`
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

    /// 中文说明: 停止工作线程并等待完成。
    /// 对齐 Java 方法: `stop`
    pub fn stop(&mut self) {
        if let Some(sender) = self.sender.take() {
            let _ = sender.send(TimerCommand::Stop);
        }
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }

    /// 中文说明: 返回工作线程是否正在运行。
    /// 对齐 Java 方法: `isStarted`
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

use super::{TimerCommand, run_timer};
