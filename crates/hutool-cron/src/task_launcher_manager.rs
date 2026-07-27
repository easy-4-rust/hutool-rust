//! 对齐: `cn.hutool.cron.TaskLauncherManager`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/TaskLauncherManager.java
//! 中文说明: 任务启动器工厂，提前捕获调度器共享资源并为指定
//! 时间戳生成一次性启动器。
//!
//! Factory for one-shot task launchers aligned with Hutool.

use std::sync::{Arc, RwLock};

use crate::{Scheduler, TaskLauncher, TaskListenerManager, TaskTable};

/// 对齐: `cn.hutool.cron.TaskLauncherManager`
/// 中文说明: 任务启动器工厂。
///
/// Factory for launchers.
#[derive(Debug, Clone)]
pub struct TaskLauncherManager {
    table: Arc<RwLock<TaskTable>>,
    listeners: TaskListenerManager,
    match_second: bool,
}

impl TaskLauncherManager {
    /// 中文说明: 捕获调度器的共享资源。
    #[must_use]
    pub fn new(scheduler: &Scheduler) -> Self {
        Self {
            table: scheduler.task_table(),
            listeners: scheduler.listeners(),
            match_second: scheduler.is_match_second(),
        }
    }

    /// 中文说明: 为指定时间戳创建启动器。
    #[must_use]
    pub fn launcher(&self, millis: i64) -> TaskLauncher {
        TaskLauncher::with_parts(
            Arc::clone(&self.table),
            self.listeners.clone(),
            millis,
            self.match_second,
        )
    }
}
