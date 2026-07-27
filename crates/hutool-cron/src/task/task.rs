//! 对齐: `cn.hutool.cron.task.Task`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/task/Task.java
//! 中文说明: 同步任务契约，调度器在 Tokio 阻塞线程池上执行，
//! 任务实现不应阻塞调度循环。

use crate::CronError;

/// 对齐: `cn.hutool.cron.task.Task`
/// 中文说明: 同步任务契约。
pub trait Task: Send + Sync + 'static {
    /// 中文说明: 执行一次任务调用。
    /// 对齐 Java 方法: `execute`
    fn execute(&self) -> Result<(), CronError>;
}

impl<F> Task for F
where
    F: Fn() -> Result<(), CronError> + Send + Sync + 'static,
{
    fn execute(&self) -> Result<(), CronError> {
        self()
    }
}
