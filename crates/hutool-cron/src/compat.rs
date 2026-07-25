//! 对齐: `cn.hutool.cron` (Task / TaskListener / CronTask / Scheduler / CronUtil)
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/*.java
//! 中文说明: 提供与 Hutool Java 定时任务框架对齐的任务、监听器、
//! 任务表、调度器等核心类型。
//!
//! Hutool-aligned task, listener, table, and scheduler facades.

#![allow(clippy::missing_fields_in_debug, clippy::missing_panics_doc)]

use std::{
    fmt,
    sync::{Arc, RwLock},
    time::Duration,
};

use chrono::Utc;
use tokio::{task::JoinHandle, time};

use crate::{CronError, CronPattern};

/// 对齐: `cn.hutool.cron.task.Task`
/// 中文说明: 同步任务契约，调度器在 Tokio 阻塞线程池上执行，
/// 任务不应阻塞调度循环。
///
/// Synchronous task contract. Scheduler execution happens on Tokio's blocking
/// pool so a task cannot block the scheduling loop.
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

/// 对齐: `cn.hutool.cron.task.RunnableTask`
/// 中文说明: 将不会出错的 Rust 闭包适配为 `Task` 接口。
///
/// Adapts an infallible Rust closure to `Task`.
pub struct RunnableTask<F> {
    runnable: F,
}

impl<F> RunnableTask<F>
where
    F: Fn() + Send + Sync + 'static,
{
    /// 中文说明: 创建任务适配器。
    /// 对齐 Java 方法: `new`
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

/// 对齐: `cn.hutool.cron.CronInvoke`
/// 中文说明: 显式方法注册表，替代 Java 反射和类路径查找机制。
///
/// Explicit method registry replacing Java reflection and classpath lookup.
#[derive(Clone, Default)]
pub struct InvokeRegistry {
    methods: Arc<RwLock<std::collections::HashMap<String, Arc<dyn Task>>>>,
}

impl fmt::Debug for InvokeRegistry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("InvokeRegistry")
            .field(
                "method_count",
                &self.methods.read().expect("invoke registry poisoned").len(),
            )
            .finish()
    }
}

impl InvokeRegistry {
    /// 中文说明: 创建空注册表。
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// 中文说明: 注册或替换一个命名任务。
    /// 对齐 Java 方法: `register`
    pub fn register<T>(&self, name: impl Into<String>, task: T) -> Option<Arc<dyn Task>>
    where
        T: Task,
    {
        self.methods
            .write()
            .expect("invoke registry poisoned")
            .insert(name.into(), Arc::new(task))
    }

    /// 中文说明: 按名称查找已注册的任务。
    /// 对齐 Java 方法: `resolve`
    #[must_use]
    pub fn resolve(&self, name: &str) -> Option<Arc<dyn Task>> {
        self.methods
            .read()
            .expect("invoke registry poisoned")
            .get(name)
            .cloned()
    }
}

/// 对齐: `cn.hutool.cron.CronInvoke`
/// 中文说明: 通过注入的注册表解析的命名调用任务。
///
/// A named invocation resolved through an injected registry.
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
    /// 中文说明: 解析 Hutool 风格的 `type::method` 名称（无需反射）。
    /// 对齐 Java 方法: `CronInvoke.invoke`
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

/// 对齐: `cn.hutool.cron.CronTask`
/// 中文说明: 带有稳定 ID 和可变调度表达式的定时任务。
///
/// A task together with its stable ID and mutable schedule.
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
    /// 对齐 Java 方法: `new`
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

    /// 中文说明: 返回当前调度表达式的快照。
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
    #[must_use]
    pub fn raw(&self) -> Arc<dyn Task> {
        Arc::clone(&self.task)
    }
}

/// 对齐: `cn.hutool.cron.TaskTable`
/// 中文说明: 稳定的按插入顺序排列的定时任务表。
///
/// Stable insertion-ordered scheduled-task table.
#[derive(Default)]
pub struct TaskTable {
    entries: Vec<Arc<CronTask>>,
}

impl fmt::Debug for TaskTable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_list().entries(&self.entries).finish()
    }
}

impl fmt::Display for TaskTable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = self
            .entries
            .iter()
            .map(|task| format!("{} {}", task.id(), task.pattern()))
            .collect::<Vec<_>>()
            .join("\n");
        formatter.write_str(&output)
    }
}

impl TaskTable {
    /// 中文说明: 创建空任务表。
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// 中文说明: 创建具有预分配容量的空任务表。
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
        }
    }

    /// 中文说明: 添加一个具有唯一 ID 的任务。
    /// 对齐 Java 方法: `addTask`
    pub fn add(&mut self, task: CronTask) -> Result<&mut Self, CronError> {
        if self.get_task(task.id()).is_some() {
            return Err(CronError::DuplicateTaskId(task.id().to_owned()));
        }
        self.entries.push(Arc::new(task));
        Ok(self)
    }

    /// 中文说明: 按插入顺序返回所有任务 ID。
    /// 对齐 Java 方法: `getIds`
    #[must_use]
    pub fn ids(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|task| task.id().to_owned())
            .collect()
    }

    /// 中文说明: 返回所有调度表达式的快照。
    /// 对齐 Java 方法: `getPatterns`
    #[must_use]
    pub fn patterns(&self) -> Vec<CronPattern> {
        self.entries.iter().map(|task| task.pattern()).collect()
    }

    /// 中文说明: 返回所有底层任务句柄。
    /// 对齐 Java 方法: `getTasks`
    #[must_use]
    pub fn tasks(&self) -> Vec<Arc<dyn Task>> {
        self.entries.iter().map(|task| task.raw()).collect()
    }

    /// 中文说明: 按 ID 移除任务。
    /// 对齐 Java 方法: `remove`
    pub fn remove(&mut self, id: &str) -> bool {
        if let Some(index) = self.entries.iter().position(|task| task.id() == id) {
            self.entries.remove(index);
            true
        } else {
            false
        }
    }

    /// 中文说明: 替换指定 ID 的调度表达式，返回 ID 是否存在。
    /// 对齐 Java 方法: `updatePattern`
    pub fn update_pattern(&self, id: &str, pattern: CronPattern) -> bool {
        self.get_task(id).is_some_and(|task| {
            task.set_pattern(pattern);
            true
        })
    }

    /// 中文说明: 按索引返回任务。
    #[must_use]
    pub fn task_at(&self, index: usize) -> Option<Arc<CronTask>> {
        self.entries.get(index).cloned()
    }

    /// 中文说明: 按 ID 返回任务。
    /// 对齐 Java 方法: `getTask`
    #[must_use]
    pub fn get_task(&self, id: &str) -> Option<Arc<CronTask>> {
        self.entries.iter().find(|task| task.id() == id).cloned()
    }

    /// 中文说明: 按索引返回调度表达式。
    #[must_use]
    pub fn pattern_at(&self, index: usize) -> Option<CronPattern> {
        self.task_at(index).map(|task| task.pattern())
    }

    /// 中文说明: 按 ID 返回调度表达式。
    /// 对齐 Java 方法: `getPattern`
    #[must_use]
    pub fn get_pattern(&self, id: &str) -> Option<CronPattern> {
        self.get_task(id).map(|task| task.pattern())
    }

    /// 中文说明: 返回任务数量。
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// 中文说明: 返回任务表是否为空。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn matching(&self, millis: i64, match_second: bool) -> Vec<Arc<CronTask>> {
        self.entries
            .iter()
            .filter(|task| {
                task.pattern()
                    .matches_millis(millis, match_second)
                    .unwrap_or(false)
            })
            .cloned()
            .collect()
    }
}

/// 对齐: `cn.hutool.cron.listener.TaskListener`
/// 中文说明: 任务生命周期事件监听器接口。
///
/// Listener for task lifecycle events.
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

/// 对齐: `cn.hutool.cron.listener.SimpleTaskListener`
/// 中文说明: 空操作监听器，便于选择性重写。
///
/// No-op listener convenient for selective overrides.
#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleTaskListener;

impl TaskListener for SimpleTaskListener {}

/// 对齐: `cn.hutool.cron.listener.TaskListenerManager`
/// 中文说明: 线程安全的监听器集合。
///
/// Thread-safe listener collection.
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
    /// Adds a listener.
    pub fn add_listener(&self, listener: Arc<dyn TaskListener>) -> &Self {
        self.listeners
            .write()
            .expect("listener manager poisoned")
            .push(listener);
        self
    }

    /// Removes a listener by shared identity.
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

    /// Notifies start listeners.
    pub fn notify_task_start(&self, executor: &TaskExecutor) {
        for listener in self.snapshot() {
            listener.on_start(executor);
        }
    }

    /// Notifies success listeners.
    pub fn notify_task_succeeded(&self, executor: &TaskExecutor) {
        for listener in self.snapshot() {
            listener.on_succeeded(executor);
        }
    }

    /// Notifies failure listeners.
    pub fn notify_task_failed(&self, executor: &TaskExecutor, error: &CronError) {
        for listener in self.snapshot() {
            listener.on_failed(executor, error);
        }
    }
}

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
    /// Creates an execution with an explicit listener manager.
    #[must_use]
    pub fn new(cron_task: Arc<CronTask>, listeners: TaskListenerManager) -> Self {
        Self {
            cron_task,
            listeners,
        }
    }

    /// Returns the raw task.
    #[must_use]
    pub fn task(&self) -> Arc<dyn Task> {
        self.cron_task.raw()
    }

    /// Returns the scheduled task.
    #[must_use]
    pub fn cron_task(&self) -> &Arc<CronTask> {
        &self.cron_task
    }

    /// Executes and emits lifecycle events.
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

/// Tracks currently spawned blocking executions.
#[derive(Debug, Clone)]
pub struct TaskExecutorManager {
    listeners: TaskListenerManager,
    executors: Arc<RwLock<Vec<TaskExecutor>>>,
}

impl TaskExecutorManager {
    /// Creates an empty manager.
    #[must_use]
    pub fn new(listeners: TaskListenerManager) -> Self {
        Self {
            listeners,
            executors: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Returns a snapshot of active executions.
    #[must_use]
    pub fn executors(&self) -> Vec<TaskExecutor> {
        self.executors
            .read()
            .expect("executor manager poisoned")
            .clone()
    }

    /// Creates and records an executor.
    pub fn spawn_executor(&self, task: Arc<CronTask>) -> TaskExecutor {
        let executor = TaskExecutor::new(task, self.listeners.clone());
        self.executors
            .write()
            .expect("executor manager poisoned")
            .push(executor.clone());
        executor
    }

    /// Removes a completed executor by task identity.
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

/// Scheduler configuration.
#[derive(Debug, Clone)]
pub struct CronConfig {
    timezone: chrono::FixedOffset,
    match_second: bool,
}

impl Default for CronConfig {
    fn default() -> Self {
        Self {
            timezone: chrono::FixedOffset::east_opt(0).expect("UTC offset is valid"),
            match_second: false,
        }
    }
}

impl CronConfig {
    /// Sets the fixed timezone offset.
    pub fn set_timezone(&mut self, timezone: chrono::FixedOffset) -> &mut Self {
        self.timezone = timezone;
        self
    }

    /// Returns the fixed timezone offset.
    #[must_use]
    pub const fn timezone(&self) -> chrono::FixedOffset {
        self.timezone
    }

    /// Returns whether seconds are matched.
    #[must_use]
    pub const fn is_match_second(&self) -> bool {
        self.match_second
    }

    /// Sets second matching.
    pub fn set_match_second(&mut self, match_second: bool) -> &mut Self {
        self.match_second = match_second;
        self
    }
}

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

/// One validated task entry used for explicit batch scheduling.
#[derive(Clone)]
pub struct CronSettingEntry {
    /// Stable task ID.
    pub id: String,
    /// Parsed pattern.
    pub pattern: CronPattern,
    /// Injected task implementation.
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
    /// Creates a stopped scheduler.
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

    /// Sets the timezone used for configuration/reporting.
    pub fn set_timezone(&mut self, timezone: chrono::FixedOffset) -> &mut Self {
        self.config.set_timezone(timezone);
        self
    }

    /// Returns the configured timezone.
    #[must_use]
    pub const fn timezone(&self) -> chrono::FixedOffset {
        self.config.timezone()
    }

    /// Sets daemon shutdown semantics metadata.
    pub fn set_daemon(&mut self, daemon: bool) -> &mut Self {
        self.daemon = daemon;
        self
    }

    /// Returns daemon mode.
    #[must_use]
    pub const fn is_daemon(&self) -> bool {
        self.daemon
    }

    /// Injects the Tokio runtime used for scheduling and blocking tasks.
    pub fn set_runtime(&mut self, runtime: tokio::runtime::Handle) -> Result<&mut Self, CronError> {
        if self.is_started() {
            return Err(CronError::SchedulerAlreadyStarted);
        }
        self.runtime = Some(runtime);
        Ok(self)
    }

    /// Returns whether seconds are matched.
    #[must_use]
    pub const fn is_match_second(&self) -> bool {
        self.config.is_match_second()
    }

    /// Sets second matching.
    pub fn set_match_second(&mut self, value: bool) -> Result<&mut Self, CronError> {
        if self.is_started() {
            return Err(CronError::SchedulerAlreadyStarted);
        }
        self.config.set_match_second(value);
        Ok(self)
    }

    /// Adds a listener.
    pub fn add_listener(&self, listener: Arc<dyn TaskListener>) -> &Self {
        self.listeners.add_listener(listener);
        self
    }

    /// Removes a listener.
    pub fn remove_listener(&self, listener: &Arc<dyn TaskListener>) -> bool {
        self.listeners.remove_listener(listener)
    }

    /// Schedules an auto-ID task.
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

    /// Schedules an explicit-ID task.
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
        self.task_table
            .write()
            .expect("task table poisoned")
            .add(CronTask::new(id, pattern, task))?;
        Ok(self)
    }

    /// Adds every entry from an explicitly parsed setting.
    pub fn schedule_setting(
        &self,
        entries: impl IntoIterator<Item = CronSettingEntry>,
    ) -> Result<&Self, CronError> {
        for entry in entries {
            self.schedule_owned(entry.id, entry.pattern, entry.task)?;
        }
        Ok(self)
    }

    /// Removes a task and ignores absence.
    pub fn deschedule(&self, id: &str) -> &Self {
        self.deschedule_with_status(id);
        self
    }

    /// Removes a task and reports whether it existed.
    pub fn deschedule_with_status(&self, id: &str) -> bool {
        self.task_table
            .write()
            .expect("task table poisoned")
            .remove(id)
    }

    /// Updates a task pattern.
    pub fn update_pattern(&self, id: &str, pattern: CronPattern) -> bool {
        self.task_table
            .read()
            .expect("task table poisoned")
            .update_pattern(id, pattern)
    }

    /// Returns the shared task table for read-only inspection.
    #[must_use]
    pub fn task_table(&self) -> Arc<RwLock<TaskTable>> {
        Arc::clone(&self.task_table)
    }

    /// Returns a task pattern.
    #[must_use]
    pub fn pattern(&self, id: &str) -> Option<CronPattern> {
        self.task_table
            .read()
            .expect("task table poisoned")
            .get_pattern(id)
    }

    /// Returns a scheduled task.
    #[must_use]
    pub fn task(&self, id: &str) -> Option<Arc<CronTask>> {
        self.task_table
            .read()
            .expect("task table poisoned")
            .get_task(id)
    }

    /// Returns whether the schedule is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the scheduled task count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.task_table.read().expect("task table poisoned").len()
    }

    /// Clears all tasks.
    pub fn clear(&self) -> &Self {
        *self.task_table.write().expect("task table poisoned") = TaskTable::new();
        self
    }

    /// Returns whether the scheduler worker is active.
    #[must_use]
    pub const fn is_started(&self) -> bool {
        self.worker.is_some()
    }

    /// Starts on the injected runtime or the current Tokio runtime.
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

    /// Stops the scheduler, optionally clearing tasks.
    pub fn stop(&mut self, clear_tasks: bool) -> &mut Self {
        if let Some(worker) = self.worker.take() {
            worker.abort();
        }
        if clear_tasks {
            self.clear();
        }
        self
    }
}

impl Drop for Scheduler {
    fn drop(&mut self) {
        self.stop(false);
    }
}

/// Executes every task matching one timestamp.
#[derive(Debug, Clone)]
pub struct TaskLauncher {
    table: Arc<RwLock<TaskTable>>,
    listeners: TaskListenerManager,
    millis: i64,
    match_second: bool,
}

impl TaskLauncher {
    /// Creates a one-shot launcher.
    #[must_use]
    pub fn new(scheduler: &Scheduler, millis: i64) -> Self {
        Self {
            table: scheduler.task_table(),
            listeners: scheduler.listeners.clone(),
            millis,
            match_second: scheduler.is_match_second(),
        }
    }

    /// Executes all matching tasks and returns their results.
    #[must_use]
    pub fn run(&self) -> Vec<Result<(), CronError>> {
        self.table
            .read()
            .expect("task table poisoned")
            .matching(self.millis, self.match_second)
            .into_iter()
            .map(|task| TaskExecutor::new(task, self.listeners.clone()).run())
            .collect()
    }
}

/// Factory for launchers.
#[derive(Debug, Clone)]
pub struct TaskLauncherManager {
    table: Arc<RwLock<TaskTable>>,
    listeners: TaskListenerManager,
    match_second: bool,
}

impl TaskLauncherManager {
    /// Captures a scheduler's shared resources.
    #[must_use]
    pub fn new(scheduler: &Scheduler) -> Self {
        Self {
            table: scheduler.task_table(),
            listeners: scheduler.listeners.clone(),
            match_second: scheduler.is_match_second(),
        }
    }

    /// Creates a launcher for one timestamp.
    #[must_use]
    pub fn launcher(&self, millis: i64) -> TaskLauncher {
        TaskLauncher {
            table: Arc::clone(&self.table),
            listeners: self.listeners.clone(),
            millis,
            match_second: self.match_second,
        }
    }
}

/// Owned facade corresponding to Hutool's static `CronUtil` surface.
#[derive(Debug, Default)]
pub struct CronUtil {
    scheduler: Scheduler,
}

impl CronUtil {
    /// Creates an isolated facade.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the owned scheduler.
    #[must_use]
    pub const fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }

    /// Returns the owned scheduler mutably.
    pub fn scheduler_mut(&mut self) -> &mut Scheduler {
        &mut self.scheduler
    }

    /// Configures second matching before start.
    pub fn set_match_second(&mut self, value: bool) -> Result<&mut Self, CronError> {
        self.scheduler.set_match_second(value)?;
        Ok(self)
    }

    /// Adds an auto-ID task.
    pub fn schedule<T>(&mut self, pattern: &str, task: T) -> Result<String, CronError>
    where
        T: Task,
    {
        self.scheduler.schedule(pattern, task)
    }

    /// Adds an explicit-ID task.
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

    /// Adds an explicit batch setting.
    pub fn schedule_setting(
        &self,
        entries: impl IntoIterator<Item = CronSettingEntry>,
    ) -> Result<&Self, CronError> {
        self.scheduler.schedule_setting(entries)?;
        Ok(self)
    }

    /// Removes a task.
    pub fn remove(&self, id: &str) -> bool {
        self.scheduler.deschedule_with_status(id)
    }

    /// Replaces a task pattern.
    pub fn update_pattern(&self, id: &str, pattern: CronPattern) -> bool {
        self.scheduler.update_pattern(id, pattern)
    }

    /// Starts the scheduler.
    pub fn start(&mut self) -> Result<&mut Self, CronError> {
        self.scheduler.start()?;
        Ok(self)
    }

    /// Restarts without clearing tasks.
    pub fn restart(&mut self) -> Result<&mut Self, CronError> {
        self.scheduler.stop(false).start()?;
        Ok(self)
    }

    /// Stops and clears tasks.
    pub fn stop(&mut self) -> &mut Self {
        self.scheduler.stop(true);
        self
    }
}

/// Thin owned timer facade for compatibility with Hutool's `CronTimer`.
#[derive(Debug)]
pub struct CronTimer<'a> {
    scheduler: &'a mut Scheduler,
}

impl<'a> CronTimer<'a> {
    /// Creates a timer for a scheduler.
    pub fn new(scheduler: &'a mut Scheduler) -> Self {
        Self { scheduler }
    }

    /// Starts the scheduler.
    pub fn run(&mut self) -> Result<(), CronError> {
        self.scheduler.start().map(|_| ())
    }

    /// Stops the scheduler without clearing tasks.
    pub fn stop_timer(&mut self) {
        self.scheduler.stop(false);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use chrono::TimeZone;

    use super::*;

    #[allow(clippy::unnecessary_wraps)]
    fn ok_task() -> Result<(), CronError> {
        Ok(())
    }

    #[derive(Default)]
    struct CountingListener {
        starts: AtomicUsize,
        successes: AtomicUsize,
        failures: AtomicUsize,
    }

    impl TaskListener for CountingListener {
        fn on_start(&self, _executor: &TaskExecutor) {
            self.starts.fetch_add(1, Ordering::SeqCst);
        }

        fn on_succeeded(&self, _executor: &TaskExecutor) {
            self.successes.fetch_add(1, Ordering::SeqCst);
        }

        fn on_failed(&self, _executor: &TaskExecutor, _error: &CronError) {
            self.failures.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn registry_runnable_cron_tasks_and_tables_are_explicit() {
        let runs = Arc::new(AtomicUsize::new(0));
        let registry = InvokeRegistry::new();
        let invoke_runs = Arc::clone(&runs);
        registry.register("demo::run", move || {
            invoke_runs.fetch_add(1, Ordering::SeqCst);
            Ok(())
        });
        assert!(format!("{registry:?}").contains("method_count: 1"));
        let invoke = InvokeTask::new("demo::run", &registry).unwrap();
        assert_eq!(invoke.name(), "demo::run");
        assert!(format!("{invoke:?}").contains("demo::run"));
        invoke.execute().unwrap();
        assert!(InvokeTask::new("missing", &registry).is_err());

        let runnable_runs = Arc::clone(&runs);
        RunnableTask::new(move || {
            runnable_runs.fetch_add(1, Ordering::SeqCst);
        })
        .execute()
        .unwrap();

        let pattern = CronPattern::parse("* * * * *").unwrap();
        let task = CronTask::new("one", pattern.clone(), Arc::new(invoke));
        assert!(format!("{task:?}").contains("one"));
        assert_eq!(task.id(), "one");
        assert_eq!(task.pattern().to_string(), "* * * * *");
        assert!(
            task.set_pattern(CronPattern::parse("*/2 * * * *").unwrap())
                .raw()
                .execute()
                .is_ok()
        );

        let mut table = TaskTable::with_capacity(2);
        table.add(task).unwrap();
        assert!(format!("{table:?}").contains("one"));
        assert_eq!(table.ids(), ["one"]);
        assert_eq!(table.patterns().len(), 1);
        assert_eq!(table.tasks().len(), 1);
        assert!(
            table
                .add(CronTask::new("one", pattern, Arc::new(ok_task)))
                .is_err()
        );
        assert!(table.task_at(0).is_some());
        assert!(table.pattern_at(0).is_some());
        assert!(table.get_pattern("one").is_some());
        assert!(table.update_pattern("one", CronPattern::parse("* * * * *").unwrap()));
        assert!(!table.update_pattern("missing", CronPattern::parse("* * * * *").unwrap()));
        assert!(table.to_string().contains("one"));
        table
            .add(CronTask::new(
                "two",
                CronPattern::parse("* * * * *").unwrap(),
                Arc::new(ok_task),
            ))
            .unwrap();
        assert!(table.to_string().contains("\ntwo"));
        assert!(table.remove("one"));
        assert!(!table.remove("one"));
        assert!(table.remove("two"));
        assert!(table.is_empty());
    }

    #[test]
    fn listeners_executors_and_launchers_report_success_and_failure() {
        let listener = Arc::new(CountingListener::default());
        let listener_dyn: Arc<dyn TaskListener> = listener.clone();
        let listeners = TaskListenerManager::default();
        listeners.add_listener(Arc::clone(&listener_dyn));
        assert!(format!("{listeners:?}").contains("listener_count: 1"));
        let success = Arc::new(CronTask::new(
            "ok",
            CronPattern::parse("* * * * *").unwrap(),
            Arc::new(ok_task),
        ));
        let failure = Arc::new(CronTask::new(
            "bad",
            CronPattern::parse("* * * * *").unwrap(),
            Arc::new(|| Err(CronError::Task("failed".to_owned()))),
        ));
        let manager = TaskExecutorManager::new(listeners.clone());
        let ok = manager.spawn_executor(success);
        assert!(format!("{ok:?}").contains("ok"));
        assert!(Arc::ptr_eq(&ok.task(), &ok.cron_task().raw()));
        assert!(ok.run().is_ok());
        assert!(manager.notify_executor_completed(&ok));
        assert!(!manager.notify_executor_completed(&ok));
        let bad = manager.spawn_executor(failure);
        assert!(bad.run().is_err());
        assert_eq!(manager.executors().len(), 1);
        assert_eq!(listener.starts.load(Ordering::SeqCst), 2);
        assert_eq!(listener.successes.load(Ordering::SeqCst), 1);
        assert_eq!(listener.failures.load(Ordering::SeqCst), 1);
        assert!(listeners.remove_listener(&listener_dyn));
        assert!(!listeners.remove_listener(&listener_dyn));
        let no_op = SimpleTaskListener;
        no_op.on_start(&bad);
        no_op.on_succeeded(&bad);
        no_op.on_failed(&bad, &CronError::Task("ignored".to_owned()));
    }

    #[tokio::test]
    #[allow(clippy::too_many_lines)]
    async fn scheduler_lifecycle_tables_launchers_and_owned_facade_are_bounded() {
        let runs = Arc::new(AtomicUsize::new(0));
        let mut scheduler = Scheduler::new();
        scheduler
            .set_timezone(chrono::FixedOffset::east_opt(3600).unwrap())
            .set_daemon(true);
        scheduler
            .set_runtime(tokio::runtime::Handle::current())
            .unwrap()
            .set_match_second(true)
            .unwrap();
        assert!(scheduler.is_daemon());
        assert!(scheduler.is_match_second());
        assert_eq!(scheduler.timezone().local_minus_utc(), 3600);
        let task_runs = Arc::clone(&runs);
        let id = scheduler
            .schedule("* * * * * *", move || {
                task_runs.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
            .unwrap();
        assert_eq!(scheduler.len(), 1);
        assert!(scheduler.task(&id).is_some());
        assert!(scheduler.pattern(&id).is_some());
        let now = Utc::now();
        let launcher = TaskLauncher::new(&scheduler, now.timestamp_millis());
        assert_eq!(launcher.run().len(), 1);
        let launchers = TaskLauncherManager::new(&scheduler);
        assert_eq!(launchers.launcher(now.timestamp_millis()).run().len(), 1);
        scheduler.start().unwrap();
        assert!(scheduler.is_started());
        assert!(format!("{scheduler:?}").contains("started: true"));
        assert!(scheduler.start().is_err());
        assert!(scheduler.set_match_second(false).is_err());
        assert!(
            scheduler
                .set_runtime(tokio::runtime::Handle::current())
                .is_err()
        );
        time::sleep(Duration::from_millis(20)).await;
        scheduler.stop(false);
        assert!(!scheduler.is_started());
        assert!(scheduler.update_pattern(&id, CronPattern::parse("*/2 * * * *").unwrap()));
        assert!(scheduler.schedule("invalid", ok_task).is_err());
        scheduler.next_id = 1;
        assert!(scheduler.schedule("* * * * * *", ok_task).is_err());
        assert!(
            scheduler
                .schedule_with_id(
                    id.clone(),
                    CronPattern::parse("* * * * * *").unwrap(),
                    Arc::new(ok_task),
                )
                .is_err()
        );
        assert!(
            scheduler
                .schedule_setting([CronSettingEntry {
                    id: id.clone(),
                    pattern: CronPattern::parse("* * * * * *").unwrap(),
                    task: Arc::new(ok_task),
                }])
                .is_err()
        );
        assert!(scheduler.deschedule_with_status(&id));
        assert!(!scheduler.deschedule_with_status(&id));
        assert!(scheduler.is_empty());
        scheduler.deschedule("missing").clear();

        let listener: Arc<dyn TaskListener> = Arc::new(SimpleTaskListener);
        scheduler.add_listener(Arc::clone(&listener));
        assert!(scheduler.remove_listener(&listener));

        let mut facade = CronUtil::new();
        facade
            .scheduler_mut()
            .set_runtime(tokio::runtime::Handle::current())
            .unwrap();
        assert_eq!(facade.scheduler().len(), 0);
        facade.set_match_second(true).unwrap();
        let facade_id = facade.schedule("* * * * * *", ok_task).unwrap();
        facade
            .schedule_with_id(
                "batch",
                CronPattern::parse("* * * * * *").unwrap(),
                Arc::new(ok_task),
            )
            .unwrap();
        facade
            .schedule_setting([CronSettingEntry {
                id: "setting".to_owned(),
                pattern: CronPattern::parse("* * * * * *").unwrap(),
                task: Arc::new(ok_task),
            }])
            .unwrap();
        let setting = CronSettingEntry {
            id: "debug".to_owned(),
            pattern: CronPattern::parse("* * * * * *").unwrap(),
            task: Arc::new(ok_task),
        };
        assert!(format!("{setting:?}").contains("debug"));
        assert!(facade.update_pattern(&facade_id, CronPattern::parse("*/2 * * * * *").unwrap()));
        assert!(facade.remove("batch"));
        facade.start().unwrap();
        assert!(facade.set_match_second(false).is_err());
        assert!(
            facade
                .schedule_with_id(
                    facade_id.clone(),
                    CronPattern::parse("* * * * * *").unwrap(),
                    Arc::new(ok_task),
                )
                .is_err()
        );
        assert!(
            facade
                .schedule_setting([CronSettingEntry {
                    id: facade_id,
                    pattern: CronPattern::parse("* * * * * *").unwrap(),
                    task: Arc::new(ok_task),
                }])
                .is_err()
        );
        facade.restart().unwrap().stop();

        let mut timer_scheduler = Scheduler::new();
        timer_scheduler
            .set_runtime(tokio::runtime::Handle::current())
            .unwrap();
        let mut timer = CronTimer::new(&mut timer_scheduler);
        timer.run().unwrap();
        timer.stop_timer();
        assert!(runs.load(Ordering::SeqCst) >= 2);
        let _ = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
    }

    #[test]
    fn scheduler_requires_a_runtime_and_config_is_mutable_before_start() {
        let mut config = CronConfig::default();
        assert!(!config.is_match_second());
        config
            .set_timezone(chrono::FixedOffset::west_opt(3600).unwrap())
            .set_match_second(true);
        assert_eq!(config.timezone().local_minus_utc(), -3600);
        assert!(config.is_match_second());

        let mut scheduler = Scheduler::new();
        assert!(scheduler.start().is_err());

        let mut facade = CronUtil::new();
        assert!(facade.start().is_err());
        assert!(facade.restart().is_err());
    }

    #[tokio::test]
    async fn minute_scheduler_enters_its_worker_and_drop_stops_it() {
        let mut scheduler = Scheduler::new();
        scheduler
            .set_runtime(tokio::runtime::Handle::current())
            .unwrap();
        scheduler.start().unwrap();
        time::sleep(Duration::from_millis(5)).await;
        drop(scheduler);
    }
}
