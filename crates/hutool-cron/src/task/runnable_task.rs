//! 对齐: `cn.hutool.cron.task.RunnableTask`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/task/RunnableTask.java
//! 中文说明: 将不会出错的 Rust 闭包适配为 `Task` 接口。

use crate::CronError;

use super::Task;

/// 对齐: `cn.hutool.cron.task.RunnableTask`
/// 中文说明: 将 `Fn()` 适配为 `Task`。
pub struct RunnableTask<F> {
    runnable: F,
}

impl<F> RunnableTask<F>
where
    F: Fn() + Send + Sync + 'static,
{
    /// 中文说明: 创建任务适配器。
    #[must_use]
    pub const fn new(runnable: F) -> Self {
        Self { runnable }
    }
}

impl<F> Task for RunnableTask<F>
where
    F: Fn() + Send + Sync + 'static,
{
    fn execute(&self) -> Result<(), CronError> {
        (self.runnable)();
        Ok(())
    }
}
