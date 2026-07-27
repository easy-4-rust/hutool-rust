//! 对齐: `cn.hutool.cron.CronTimer`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/CronTimer.java
//! 中文说明: 从 `compat.rs` 拆出的轻量定时器门面，保留 Hutool `CronTimer`
//! 的启动与停止语义，但内部复用 Rust `Scheduler` 的运行时调度实现。
//!
//! Thin timer facade aligned with Hutool's `CronTimer`.

use crate::{CronError, Scheduler};

/// 对齐: `cn.hutool.cron.CronTimer`
/// 中文说明: 兼容 Hutool `CronTimer` 的轻量拥有的定时器门面。
///
/// Thin owned timer facade for compatibility with Hutool's `CronTimer`.
#[derive(Debug)]
pub struct CronTimer<'a> {
    scheduler: &'a mut Scheduler,
}

impl<'a> CronTimer<'a> {
    /// 中文说明: 为调度器创建定时器。
    /// 对齐 Java 方法: `new`
    pub fn new(scheduler: &'a mut Scheduler) -> Self {
        Self { scheduler }
    }

    /// 中文说明: 启动调度器。
    /// 对齐 Java 方法: `start`
    pub fn run(&mut self) -> Result<(), CronError> {
        self.scheduler.start().map(|_| ())
    }

    /// 中文说明: 停止调度器（不清除任务）。
    /// 对齐 Java 方法: `stop`
    pub fn stop_timer(&mut self) {
        self.scheduler.stop(false);
    }
}
