//! 对齐: `cn.hutool.cron.Scheduler` / `cn.hutool.cron.CronSetting`
//! 来源:
//! - hutool-cron/src/main/java/cn/hutool/cron/Scheduler.java
//! - hutool-cron/src/main/java/cn/hutool/cron/CronSetting.java（语义对齐为 `CronSettingEntry`）
//! 中文说明: 从 `compat.rs` 拆出的显式调度器实现，负责任务注册、
//! 调度循环、监听器接线以及批量配置条目的承载。
//!
//! Extracted scheduler implementation aligned with Hutool's scheduler facade.

use std::{
    fmt,
    sync::{Arc, RwLock},
    time::Duration,
};

use chrono::Utc;
use tokio::{task::JoinHandle, time};

use crate::{
    CronConfig, CronError, CronPattern, CronTask, Task, TaskExecutorManager, TaskListener,
    TaskListenerManager, TaskTable,
};

/// 对齐: `cn.hutool.cron.Cron`
/// 中文说明: 显式拥有的定时任务调度器，不创建隐藏的运行时或全局状态。
///
/// Explicitly owned scheduler; it never creates a hidden runtime or global.
pub struct Scheduler {
    config: CronConfig,
    daemon: bool,
    runtime: Option<tokio::runtime::Handle>,
    task_table: Arc<RwLock<TaskTable>>,
    listeners: TaskListenerManager,
    worker: Option<JoinHandle<()>>,
    next_id: u64,
}

/// 对齐: `cn.hutool.cron.CronSetting`
/// 中文说明: 用于批量调度的已验证任务条目。
///
/// One validated task entry used for explicit batch scheduling.
#[derive(Clone)]
pub struct CronSettingEntry {
    /// 中文说明: 稳定的任务 ID。
    pub id: String,
    /// 中文说明: 已解析的调度表达式。
    pub pattern: CronPattern,
    /// 中文说明: 注入的任务实现。
    pub task: Arc<dyn Task>,
}

impl fmt::Debug for CronSettingEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CronSettingEntry")
            .field("id", &self.id)
            .field("pattern", &self.pattern)
            .finish_non_exhaustive()
    }
}

impl fmt::Debug for Scheduler {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Scheduler")
            .field("config", &self.config)
            .field("daemon", &self.daemon)
            .field("started", &self.is_started())
            .field("task_count", &self.len())
            .finish()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    /// 中文说明: 创建已停止的调度器。
    /// 对齐 Java 方法: `new`
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: CronConfig::default(),
            daemon: false,
            runtime: None,
            task_table: Arc::new(RwLock::new(TaskTable::new())),
            listeners: TaskListenerManager::default(),
            worker: None,
            next_id: 1,
        }
    }

    /// 中文说明: 设置用于配置和报告的时区。
    /// 对齐 Java 方法: `setTimezone`
    pub fn set_timezone(&mut self, timezone: chrono::FixedOffset) -> &mut Self {
        self.config.set_timezone(timezone);
        self
    }

    /// 中文说明: 返回配置的时区。
    /// 对齐 Java 方法: `getTimezone`
    #[must_use]
    pub const fn timezone(&self) -> chrono::FixedOffset {
        self.config.timezone()
    }

    /// 中文说明: 设置守护进程关闭语义。
    /// 对齐 Java 方法: `setDaemon`
    pub fn set_daemon(&mut self, daemon: bool) -> &mut Self {
        self.daemon = daemon;
        self
    }

    /// 中文说明: 返回是否为守护模式。
    /// 对齐 Java 方法: `isDaemon`
    #[must_use]
    pub const fn is_daemon(&self) -> bool {
        self.daemon
    }

    /// 中文说明: 注入用于调度和阻塞任务的 Tokio 运行时。
    pub fn set_runtime(&mut self, runtime: tokio::runtime::Handle) -> Result<&mut Self, CronError> {
        if self.is_started() {
            return Err(CronError::SchedulerAlreadyStarted);
        }
        self.runtime = Some(runtime);
        Ok(self)
    }

    /// 中文说明: 返回是否匹配秒字段。
    /// 对齐 Java 方法: `isMatchSecond`
    #[must_use]
    pub const fn is_match_second(&self) -> bool {
        self.config.is_match_second()
    }

    /// 中文说明: 设置是否匹配秒字段。
    /// 对齐 Java 方法: `setMatchSecond`
    pub fn set_match_second(&mut self, value: bool) -> Result<&mut Self, CronError> {
        if self.is_started() {
            return Err(CronError::SchedulerAlreadyStarted);
        }
        self.config.set_match_second(value);
        Ok(self)
    }

    /// 中文说明: 添加任务监听器。
    /// 对齐 Java 方法: `addListener`
    pub fn add_listener(&self, listener: Arc<dyn TaskListener>) -> &Self {
        self.listeners.add_listener(listener);
        self
    }

    /// 中文说明: 移除任务监听器。
    /// 对齐 Java 方法: `removeListener`
    pub fn remove_listener(&self, listener: &Arc<dyn TaskListener>) -> bool {
        self.listeners.remove_listener(listener)
    }

    /// 中文说明: 返回监听器管理器快照，供同 crate 启动器和执行器复用。
    #[must_use]
    pub(crate) fn listeners(&self) -> TaskListenerManager {
        self.listeners.clone()
    }

    /// 中文说明: 调度一个自动分配 ID 的任务。
    /// 对齐 Java 方法: `schedule`
    pub fn schedule<T>(&mut self, pattern: &str, task: T) -> Result<String, CronError>
    where
        T: Task,
    {
        self.schedule_arc(pattern, Arc::new(task))
    }

    fn schedule_arc(&mut self, pattern: &str, task: Arc<dyn Task>) -> Result<String, CronError> {
        let id = format!("hutool-cron-{}", self.next_id);
        self.next_id = self.next_id.wrapping_add(1);
        self.schedule_owned(id.clone(), CronPattern::parse(pattern)?, task)?;
        Ok(id)
    }

    /// 中文说明: 调度一个指定 ID 的任务。
    /// 对齐 Java 方法: `schedule`
    pub fn schedule_with_id(
        &self,
        id: impl Into<String>,
        pattern: CronPattern,
        task: Arc<dyn Task>,
    ) -> Result<&Self, CronError> {
        self.schedule_owned(id.into(), pattern, task)
    }

    pub(crate) fn schedule_owned(
        &self,
        id: String,
        pattern: CronPattern,
        task: Arc<dyn Task>,
    ) -> Result<&Self, CronError> {
        self.task_table
            .write()
            .expect("task table poisoned")
            .add(CronTask::new(id, pattern, task))?;
        Ok(self)
    }

    /// 中文说明: 批量添加已解析的调度设置。
    /// 对齐 Java 方法: `scheduleSetting`
    pub fn schedule_setting(
        &self,
        entries: impl IntoIterator<Item = CronSettingEntry>,
    ) -> Result<&Self, CronError> {
        for entry in entries {
            self.schedule_owned(entry.id, entry.pattern, entry.task)?;
        }
        Ok(self)
    }

    /// 中文说明: 移除任务（忽略不存在的情况）。
    /// 对齐 Java 方法: `deschedule`
    pub fn deschedule(&self, id: &str) -> &Self {
        self.deschedule_with_status(id);
        self
    }

    /// 中文说明: 移除任务并报告是否存在。
    pub fn deschedule_with_status(&self, id: &str) -> bool {
        self.task_table
            .write()
            .expect("task table poisoned")
            .remove(id)
    }

    /// 中文说明: 更新任务的调度表达式。
    /// 对齐 Java 方法: `updatePattern`
    pub fn update_pattern(&self, id: &str, pattern: CronPattern) -> bool {
        self.task_table
            .read()
            .expect("task table poisoned")
            .update_pattern(id, pattern)
    }

    /// 中文说明: 返回共享的任务表（只读检查）。
    #[must_use]
    pub fn task_table(&self) -> Arc<RwLock<TaskTable>> {
        Arc::clone(&self.task_table)
    }

    /// 中文说明: 返回指定任务的调度表达式。
    /// 对齐 Java 方法: `getPattern`
    #[must_use]
    pub fn pattern(&self, id: &str) -> Option<CronPattern> {
        self.task_table
            .read()
            .expect("task table poisoned")
            .get_pattern(id)
    }

    /// 中文说明: 返回指定 ID 的定时任务。
    /// 对齐 Java 方法: `getTask`
    #[must_use]
    pub fn task(&self, id: &str) -> Option<Arc<CronTask>> {
        self.task_table
            .read()
            .expect("task table poisoned")
            .get_task(id)
    }

    /// 中文说明: 返回调度表是否为空。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 中文说明: 返回已调度的任务数量。
    #[must_use]
    pub fn len(&self) -> usize {
        self.task_table.read().expect("task table poisoned").len()
    }

    /// 中文说明: 清除所有任务。
    /// 对齐 Java 方法: `clear`
    pub fn clear(&self) -> &Self {
        *self.task_table.write().expect("task table poisoned") = TaskTable::new();
        self
    }

    /// 中文说明: 返回调度器工作线程是否活跃。
    /// 对齐 Java 方法: `isStarted`
    #[must_use]
    pub const fn is_started(&self) -> bool {
        self.worker.is_some()
    }

    /// 中文说明: 在注入的运行时或当前 Tokio 运行时上启动调度器。
    /// 对齐 Java 方法: `start`
    pub fn start(&mut self) -> Result<&mut Self, CronError> {
        if self.is_started() {
            return Err(CronError::SchedulerAlreadyStarted);
        }
        let runtime = self
            .runtime
            .clone()
            .or_else(|| tokio::runtime::Handle::try_current().ok())
            .ok_or(CronError::MissingRuntime)?;
        let table = Arc::clone(&self.task_table);
        let manager = TaskExecutorManager::new(self.listeners.clone());
        let match_second = self.config.is_match_second();
        let worker_runtime = runtime.clone();
        let worker = runtime.spawn(async move {
            let period = if match_second {
                Duration::from_secs(1)
            } else {
                Duration::from_secs(60)
            };
            let mut ticks = time::interval(period);
            loop {
                ticks.tick().await;
                let tasks = table
                    .read()
                    .expect("task table poisoned")
                    .matching(Utc::now().timestamp_millis(), match_second);
                for task in tasks {
                    let executor = manager.spawn_executor(task);
                    let completed = manager.clone();
                    worker_runtime.spawn_blocking(move || {
                        let _ = executor.run();
                        completed.notify_executor_completed(&executor);
                    });
                }
            }
        });
        self.worker = Some(worker);
        Ok(self)
    }

    /// 中文说明: 停止调度器，可选清除任务。
    /// 对齐 Java 方法: `stop`
    pub fn stop(&mut self, clear_tasks: bool) -> &mut Self {
        if let Some(worker) = self.worker.take() {
            worker.abort();
        }
        if clear_tasks {
            self.clear();
        }
        self
    }

    #[cfg(test)]
    pub(crate) fn set_next_id_for_test(&mut self, next_id: u64) {
        self.next_id = next_id;
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        self.stop(false);
    }
}
