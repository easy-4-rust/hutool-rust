//! 对齐: `cn.hutool.cron.listener.TaskListenerManager`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/listener/TaskListenerManager.java
//! 中文说明: 线程安全的监听器集合，统一分发任务生命周期事件。

use std::{
    fmt,
    sync::{Arc, RwLock},
};

use crate::{CronError, TaskExecutor};

use super::TaskListener;

/// 对齐: `cn.hutool.cron.listener.TaskListenerManager`
/// 中文说明: 线程安全的监听器集合。
#[derive(Clone, Default)]
pub struct TaskListenerManager {
    listeners: Arc<RwLock<Vec<Arc<dyn TaskListener>>>>,
}

impl fmt::Debug for TaskListenerManager {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TaskListenerManager")
            .field(
                "listener_count",
                &self
                    .listeners
                    .read()
                    .expect("listener manager poisoned")
                    .len(),
            )
            .finish()
    }
}

impl TaskListenerManager {
    /// 中文说明: 添加监听器。
    /// 对齐 Java 方法: `addListener`
    pub fn add_listener(&self, listener: Arc<dyn TaskListener>) -> &Self {
        self.listeners
            .write()
            .expect("listener manager poisoned")
            .push(listener);
        self
    }

    /// 中文说明: 按共享身份移除监听器。
    /// 对齐 Java 方法: `removeListener`
    pub fn remove_listener(&self, listener: &Arc<dyn TaskListener>) -> bool {
        let mut listeners = self.listeners.write().expect("listener manager poisoned");
        if let Some(index) = listeners
            .iter()
            .position(|candidate| Arc::ptr_eq(candidate, listener))
        {
            listeners.remove(index);
            true
        } else {
            false
        }
    }

    fn snapshot(&self) -> Vec<Arc<dyn TaskListener>> {
        self.listeners
            .read()
            .expect("listener manager poisoned")
            .clone()
    }

    /// 中文说明: 通知所有监听器任务即将开始。
    /// 对齐 Java 方法: `notifyTaskStart`
    pub fn notify_task_start(&self, executor: &TaskExecutor) {
        for listener in self.snapshot() {
            listener.on_start(executor);
        }
    }

    /// 中文说明: 通知所有监听器任务执行成功。
    /// 对齐 Java 方法: `notifyTaskSucceeded`
    pub fn notify_task_succeeded(&self, executor: &TaskExecutor) {
        for listener in self.snapshot() {
            listener.on_succeeded(executor);
        }
    }

    /// 中文说明: 通知所有监听器任务执行失败。
    /// 对齐 Java 方法: `notifyTaskFailed`
    pub fn notify_task_failed(&self, executor: &TaskExecutor, error: &CronError) {
        for listener in self.snapshot() {
            listener.on_failed(executor, error);
        }
    }
}
