//! 对齐: `cn.hutool.cron.task.CronTask`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/task/CronTask.java
//! 中文说明: 绑定稳定 ID、调度表达式与底层任务实现的 cron 任务对象。

use std::{
    fmt,
    sync::{Arc, RwLock},
};

use crate::{CronError, CronPattern};

use super::Task;

/// 对齐: `cn.hutool.cron.task.CronTask`
/// 中文说明: 带有稳定 ID 和可变调度表达式的定时任务。
pub struct CronTask {
    id: String,
    pattern: RwLock<CronPattern>,
    task: Arc<dyn Task>,
}

impl fmt::Debug for CronTask {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CronTask")
            .field("id", &self.id)
            .field(
                "pattern",
                &self.pattern.read().expect("cron pattern poisoned"),
            )
            .finish_non_exhaustive()
    }
}

impl CronTask {
    /// 中文说明: 创建带 ID 和调度表达式的定时任务。
    #[must_use]
    pub fn new(id: impl Into<String>, pattern: CronPattern, task: Arc<dyn Task>) -> Self {
        Self {
            id: id.into(),
            pattern: RwLock::new(pattern),
            task,
        }
    }

    /// 中文说明: 执行底层任务。
    /// 对齐 Java 方法: `execute`
    pub fn execute(&self) -> Result<(), CronError> {
        self.task.execute()
    }

    /// 中文说明: 返回任务 ID。
    /// 对齐 Java 方法: `getId`
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// 中文说明: 返回当前调度表达式快照。
    /// 对齐 Java 方法: `getPattern`
    #[must_use]
    pub fn pattern(&self) -> CronPattern {
        self.pattern.read().expect("cron pattern poisoned").clone()
    }

    /// 中文说明: 替换调度表达式。
    /// 对齐 Java 方法: `setPattern`
    pub fn set_pattern(&self, pattern: CronPattern) -> &Self {
        *self.pattern.write().expect("cron pattern poisoned") = pattern;
        self
    }

    /// 中文说明: 返回底层任务的共享引用。
    /// 对齐 Java 方法: `getRaw`
    #[must_use]
    pub fn raw(&self) -> Arc<dyn Task> {
        Arc::clone(&self.task)
    }
}
