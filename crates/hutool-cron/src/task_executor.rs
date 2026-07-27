//! 对齐: `cn.hutool.cron.TaskExecutor`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/TaskExecutor.java
//! 中文说明: 单次具体任务执行实例，负责触发监听器生命周期事件，
//! 与 Java 版一样围绕一个 `CronTask` 管理一次运行过程。
//!
//! One concrete task execution aligned with Hutool's task executor.

use std::{fmt, sync::Arc};

use crate::{CronError, CronTask, Task, TaskListenerManager};

/// 对齐: `cn.hutool.cron.TaskExecutor`
/// 中文说明: 单次具体任务执行实例，负责触发监听器生命周期事件。
///
/// One concrete task execution.
#[derive(Clone)]
pub struct TaskExecutor {
    cron_task: Arc<CronTask>,
    listeners: TaskListenerManager,
}

impl fmt::Debug for TaskExecutor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TaskExecutor")
            .field("task_id", &self.cron_task.id())
            .finish_non_exhaustive()
    }
}

impl TaskExecutor {
    /// 中文说明: 使用指定的监听器管理器创建执行实例。
    #[must_use]
    pub fn new(cron_task: Arc<CronTask>, listeners: TaskListenerManager) -> Self {
        Self {
            cron_task,
            listeners,
        }
    }

    /// 中文说明: 返回底层任务。
    #[must_use]
    pub fn task(&self) -> Arc<dyn Task> {
        self.cron_task.raw()
    }

    /// 中文说明: 返回关联的定时任务。
    #[must_use]
    pub fn cron_task(&self) -> &Arc<CronTask> {
        &self.cron_task
    }

    /// 中文说明: 执行任务并触发生命周期事件。
    /// 对齐 Java 方法: `run`
    pub fn run(&self) -> Result<(), CronError> {
        self.listeners.notify_task_start(self);
        match self.cron_task.execute() {
            Ok(()) => {
                self.listeners.notify_task_succeeded(self);
                Ok(())
            }
            Err(error) => {
                self.listeners.notify_task_failed(self, &error);
                Err(error)
            }
        }
    }
}
