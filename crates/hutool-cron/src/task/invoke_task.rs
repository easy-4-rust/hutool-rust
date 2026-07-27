//! 对齐: `cn.hutool.cron.task.InvokeTask`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/task/InvokeTask.java
//! 中文说明: 通过显式注入的注册表解析命名任务，替代 Java 反射调用。

use std::{fmt, sync::Arc};

use crate::{CronError, InvokeRegistry};

use super::Task;

/// 对齐: `cn.hutool.cron.task.InvokeTask`
/// 中文说明: 通过注入注册表解析的命名调用任务。
#[derive(Clone)]
pub struct InvokeTask {
    name: String,
    task: Arc<dyn Task>,
}

impl fmt::Debug for InvokeTask {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InvokeTask")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl InvokeTask {
    /// 中文说明: 解析 Hutool 风格的方法名；Rust 版本改为从注册表查找。
    /// 对齐 Java 方法: `InvokeTask(String classNameWithMethodName)`
    pub fn new(name: impl Into<String>, registry: &InvokeRegistry) -> Result<Self, CronError> {
        let name = name.into();
        let task = registry
            .resolve(&name)
            .ok_or_else(|| CronError::UnknownInvokeTask(name.clone()))?;
        Ok(Self { name, task })
    }

    /// 中文说明: 返回已注册的方法名称。
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Task for InvokeTask {
    fn execute(&self) -> Result<(), CronError> {
        self.task.execute()
    }
}
