//! 对齐: `cn.hutool.cron.TaskLauncher`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/TaskLauncher.java
//! 中文说明: 针对一个时间戳执行所有匹配的任务。Rust 版本维持
//! Java 版“一次性启动器”的边界，但直接返回每个任务的执行结果。
//!
//! One-shot launcher executing every task matching a timestamp.

use std::sync::{Arc, RwLock};

use crate::{CronError, Scheduler, TaskExecutor, TaskListenerManager, TaskTable};

/// 对齐: `cn.hutool.cron.TaskLauncher`
/// 中文说明: 执行匹配指定时间戳的所有任务。
///
/// Executes every task matching one timestamp.
#[derive(Debug, Clone)]
pub struct TaskLauncher {
    table: Arc<RwLock<TaskTable>>,
    listeners: TaskListenerManager,
    millis: i64,
    match_second: bool,
}

impl TaskLauncher {
    /// 中文说明: 创建一次性任务启动器。
    #[must_use]
    pub fn new(scheduler: &Scheduler, millis: i64) -> Self {
        Self {
            table: scheduler.task_table(),
            listeners: scheduler.listeners(),
            millis,
            match_second: scheduler.is_match_second(),
        }
    }

    /// 中文说明: 执行所有匹配的任务并返回结果。
    /// 对齐 Java 方法: `run`
    #[must_use]
    pub fn run(&self) -> Vec<Result<(), CronError>> {
        self.table
            .read()
            .expect("task table poisoned")
            .matching(self.millis, self.match_second)
            .into_iter()
            .map(|task| TaskExecutor::new(task, self.listeners.clone()).run())
            .collect()
    }

    /// 中文说明: 基于已捕获的共享资源构造启动器，供管理器复用。
    #[must_use]
    pub(crate) fn with_parts(
        table: Arc<RwLock<TaskTable>>,
        listeners: TaskListenerManager,
        millis: i64,
        match_second: bool,
    ) -> Self {
        Self {
            table,
            listeners,
            millis,
            match_second,
        }
    }
}
