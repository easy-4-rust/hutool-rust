//! 对齐: `cn.hutool.cron.TaskTable` / `cn.hutool.cron.task.InvokeTask`
//! 来源:
//! - hutool-cron/src/main/java/cn/hutool/cron/TaskTable.java
//! - hutool-cron/src/main/java/cn/hutool/cron/task/InvokeTask.java（Rust 侧以显式注册表替代反射）
//! 中文说明: 兼容层剩余的共享对象，目前承载显式调用注册表与任务表；
//! 任务、监听器及其实现已拆到独立模块。
//!
//! Remaining compatibility-layer types shared by the extracted task modules.

#![allow(clippy::missing_fields_in_debug, clippy::missing_panics_doc)]

use std::{
    fmt,
    sync::{Arc, RwLock},
};

use crate::{CronError, CronPattern, CronTask, Task};

/// 对齐: `cn.hutool.cron.CronInvoke`
/// 中文说明: 显式方法注册表，替代 Java 反射和类路径查找机制。
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
    pub fn register<T>(&self, name: impl Into<String>, task: T) -> Option<Arc<dyn Task>>
    where
        T: Task,
    {
        self.methods
            .write()
            .expect("invoke registry poisoned")
            .insert(name.into(), Arc::new(task))
    }

    /// 中文说明: 按名称查找已注册任务。
    #[must_use]
    pub fn resolve(&self, name: &str) -> Option<Arc<dyn Task>> {
        self.methods
            .read()
            .expect("invoke registry poisoned")
            .get(name)
            .cloned()
    }
}

/// 对齐: `cn.hutool.cron.TaskTable`
/// 中文说明: 稳定的按插入顺序排列的定时任务表。
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

    pub(crate) fn matching(&self, millis: i64, match_second: bool) -> Vec<Arc<CronTask>> {
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

#[cfg(test)]
mod tests {
    use std::{
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
        time::Duration,
    };

    use chrono::{TimeZone, Utc};
    use tokio::time;

    use super::*;
    use crate::{
        CronConfig, CronSettingEntry, CronTimer, CronUtil, InvokeTask, RunnableTask, Scheduler,
        SimpleTaskListener, TaskExecutor, TaskExecutorManager, TaskLauncher, TaskLauncherManager,
        TaskListener, TaskListenerManager,
    };

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
        assert!(table.to_string().contains(
            "
two"
        ));
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
        scheduler.set_next_id_for_test(1);
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
