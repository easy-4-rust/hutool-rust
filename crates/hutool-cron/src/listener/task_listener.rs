//! 对齐: `cn.hutool.cron.listener.TaskListener`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/listener/TaskListener.java
//! 中文说明: 任务生命周期事件监听器接口。

use crate::{CronError, TaskExecutor};

/// 对齐: `cn.hutool.cron.listener.TaskListener`
/// 中文说明: 监听任务执行的开始、成功与失败事件。
pub trait TaskListener: Send + Sync + 'static {
    /// 中文说明: 任务执行前调用。
    /// 对齐 Java 方法: `onStart`
    fn on_start(&self, _executor: &TaskExecutor) {}

    /// 中文说明: 任务执行成功后调用。
    /// 对齐 Java 方法: `onSucceeded`
    fn on_succeeded(&self, _executor: &TaskExecutor) {}

    /// 中文说明: 任务执行失败后调用。
    /// 对齐 Java 方法: `onFailed`
    fn on_failed(&self, _executor: &TaskExecutor, _error: &CronError) {}
}
