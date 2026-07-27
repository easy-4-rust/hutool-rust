//! 对齐: `cn.hutool.cron.TaskExecutorManager`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/TaskExecutorManager.java
//! 中文说明: 跟踪当前已派生的阻塞执行实例，提供与 Java 版一致的
//! 执行器登记与完成回收语义。
//!
//! Tracks spawned task executors aligned with Hutool's executor manager.

use std::sync::{Arc, RwLock};

use crate::{CronTask, TaskExecutor, TaskListenerManager};

/// 对齐: `cn.hutool.cron.TaskExecutorManager`
/// 中文说明: 跟踪当前已派生的阻塞执行实例。
///
/// Tracks currently spawned blocking executions.
#[derive(Debug, Clone)]
pub struct TaskExecutorManager {
    listeners: TaskListenerManager,
    executors: Arc<RwLock<Vec<TaskExecutor>>>,
}

impl TaskExecutorManager {
    /// 中文说明: 创建空的执行管理器。
    #[must_use]
    pub fn new(listeners: TaskListenerManager) -> Self {
        Self {
            listeners,
            executors: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 中文说明: 返回当前活跃执行的快照。
    #[must_use]
    pub fn executors(&self) -> Vec<TaskExecutor> {
        self.executors
            .read()
            .expect("executor manager poisoned")
            .clone()
    }

    /// 中文说明: 创建并记录一个执行实例。
    pub fn spawn_executor(&self, task: Arc<CronTask>) -> TaskExecutor {
        let executor = TaskExecutor::new(task, self.listeners.clone());
        self.executors
            .write()
            .expect("executor manager poisoned")
            .push(executor.clone());
        executor
    }

    /// 中文说明: 按任务标识移除已完成的执行实例。
    pub fn notify_executor_completed(&self, executor: &TaskExecutor) -> bool {
        let mut executors = self.executors.write().expect("executor manager poisoned");
        if let Some(index) = executors
            .iter()
            .position(|candidate| Arc::ptr_eq(candidate.cron_task(), executor.cron_task()))
        {
            executors.remove(index);
            true
        } else {
            false
        }
    }
}
