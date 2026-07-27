//! 对齐: `cn.hutool.cron.CronUtil`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/CronUtil.java
//! 中文说明: 对应 Hutool 静态 `CronUtil` 的拥有式门面，
//! 对外包装 `Scheduler` 的常用调度能力。
//!
//! Owned facade corresponding to Hutool's static `CronUtil` surface.

use std::sync::Arc;

use crate::{CronError, CronPattern, CronSettingEntry, Scheduler, Task};

/// 对齐: `cn.hutool.cron.CronUtil`
/// 中文说明: 对应 Hutool 静态 `CronUtil` 的拥有式门面。
///
/// Owned facade corresponding to Hutool's static `CronUtil` surface.
#[derive(Debug, Default)]
pub struct CronUtil {
    scheduler: Scheduler,
}

impl CronUtil {
    /// 中文说明: 创建隔离的门面实例。
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 中文说明: 返回拥有的调度器引用。
    /// 对齐 Java 方法: `getScheduler`
    #[must_use]
    pub const fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }

    /// 中文说明: 返回拥有的调度器可变引用。
    pub fn scheduler_mut(&mut self) -> &mut Scheduler {
        &mut self.scheduler
    }

    /// 中文说明: 启动前配置是否匹配秒字段。
    /// 对齐 Java 方法: `setMatchSecond`
    pub fn set_match_second(&mut self, value: bool) -> Result<&mut Self, CronError> {
        self.scheduler.set_match_second(value)?;
        Ok(self)
    }

    /// 中文说明: 添加自动分配 ID 的任务。
    /// 对齐 Java 方法: `schedule`
    pub fn schedule<T>(&mut self, pattern: &str, task: T) -> Result<String, CronError>
    where
        T: Task,
    {
        self.scheduler.schedule(pattern, task)
    }

    /// 中文说明: 添加指定 ID 的任务。
    /// 对齐 Java 方法: `schedule`
    pub fn schedule_with_id(
        &self,
        id: impl Into<String>,
        pattern: CronPattern,
        task: Arc<dyn Task>,
    ) -> Result<&Self, CronError> {
        self.schedule_owned(id.into(), pattern, task)
    }

    fn schedule_owned(
        &self,
        id: String,
        pattern: CronPattern,
        task: Arc<dyn Task>,
    ) -> Result<&Self, CronError> {
        self.scheduler.schedule_owned(id, pattern, task)?;
        Ok(self)
    }

    /// 中文说明: 批量添加已解析的调度设置。
    /// 对齐 Java 方法: `scheduleSetting`
    pub fn schedule_setting(
        &self,
        entries: impl IntoIterator<Item = CronSettingEntry>,
    ) -> Result<&Self, CronError> {
        self.scheduler.schedule_setting(entries)?;
        Ok(self)
    }

    /// 中文说明: 移除任务。
    /// 对齐 Java 方法: `remove`
    pub fn remove(&self, id: &str) -> bool {
        self.scheduler.deschedule_with_status(id)
    }

    /// 中文说明: 替换任务的调度表达式。
    /// 对齐 Java 方法: `updatePattern`
    pub fn update_pattern(&self, id: &str, pattern: CronPattern) -> bool {
        self.scheduler.update_pattern(id, pattern)
    }

    /// 中文说明: 启动调度器。
    /// 对齐 Java 方法: `start`
    pub fn start(&mut self) -> Result<&mut Self, CronError> {
        self.scheduler.start()?;
        Ok(self)
    }

    /// 中文说明: 重启调度器（不清除任务）。
    /// 对齐 Java 方法: `restart`
    pub fn restart(&mut self) -> Result<&mut Self, CronError> {
        self.scheduler.stop(false).start()?;
        Ok(self)
    }

    /// 中文说明: 停止并清除任务。
    /// 对齐 Java 方法: `stop`
    pub fn stop(&mut self) -> &mut Self {
        self.scheduler.stop(true);
        self
    }
}
