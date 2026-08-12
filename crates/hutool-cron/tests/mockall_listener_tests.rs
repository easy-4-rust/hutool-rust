//! mockall 驱动的任务生命周期监听测试。
//!
//! `mock!` 生成 `MockTaskListener`（`TaskListener`）与 `MockTask`（`Task`），
//! 验证 `TaskExecutor::run` 按 Java 语义触发 `on_start` → `on_succeeded`/`on_failed`，
//! 以及 `TaskListenerManager` 的注册/移除通知行为。
// mock! 生成的回调字段以 `on_` 前缀命名（Java onStart/onSucceeded/onFailed 映射）。
#![allow(clippy::struct_field_names)]

use std::sync::Arc;

use hutool_cron::listener::{TaskListener, TaskListenerManager};
use hutool_cron::pattern::CronPattern;
use hutool_cron::{CronError, CronTask, Task, TaskExecutor};
use mockall::mock;

mock! {
    /// `TaskListener` 的 mock：三个生命周期回调均可设置期望。
    pub Listener {}
    impl TaskListener for Listener {
        fn on_start(&self, executor: &TaskExecutor);
        fn on_succeeded(&self, executor: &TaskExecutor);
        fn on_failed(&self, executor: &TaskExecutor, error: &CronError);
    }
}

mock! {
    /// `Task` 的 mock：`execute` 成功/失败可设置。
    pub Task {}
    impl Task for Task {
        fn execute(&self) -> Result<(), CronError>;
    }
}

fn pattern() -> CronPattern {
    CronPattern::of("0 0 * * * *").expect("valid cron")
}

fn executor_with(manager: TaskListenerManager, task: Arc<dyn Task>) -> TaskExecutor {
    TaskExecutor::new(
        Arc::new(CronTask::new("mock-task", pattern(), task)),
        manager,
    )
}

/// `监听器管理器：add_listener` 后三个通知事件均送达；
/// 未注册的监听器收不到任何事件。
#[test]
fn manager_notifies_registered_listener() {
    let mut listener = MockListener::new();
    listener.expect_on_start().once().return_const(());
    listener.expect_on_succeeded().once().return_const(());
    listener.expect_on_failed().once().return_const(());

    let manager = TaskListenerManager::default();
    manager.add_listener(Arc::new(listener));
    let executor = executor_with(manager.clone(), Arc::new(|| Ok(())));
    manager.notify_task_start(&executor);
    manager.notify_task_succeeded(&executor);
    manager.notify_task_failed(&executor, &CronError::InvalidTimestamp);
}

/// `remove_listener` 后该监听器不再收到通知（Java `removeListener` 语义）。
#[test]
fn manager_removed_listener_receives_nothing() {
    let mut keep = MockListener::new();
    keep.expect_on_start().once().return_const(());
    let mut removed = MockListener::new();
    removed.expect_on_start().never();

    let manager = TaskListenerManager::default();
    let keep: Arc<dyn TaskListener> = Arc::new(keep);
    let removed: Arc<dyn TaskListener> = Arc::new(removed);
    manager.add_listener(keep.clone());
    manager.add_listener(removed.clone());
    assert!(manager.remove_listener(&removed));

    let executor = executor_with(manager.clone(), Arc::new(|| Ok(())));
    manager.notify_task_start(&executor);
}

/// 任务执行成功：run 触发 `on_start` 后 `on_succeeded，绝不触发` `on_failed`。
#[test]
fn executor_run_success_triggers_start_then_succeeded() {
    let mut listener = MockListener::new();
    // 顺序验证：on_start 必须发生在 on_succeeded 之前
    listener.expect_on_start().once().returning(|_| {});
    listener.expect_on_succeeded().once().return_const(());
    listener.expect_on_failed().never();

    let manager = TaskListenerManager::default();
    manager.add_listener(Arc::new(listener));
    let executor = executor_with(manager, Arc::new(|| Ok(())));
    executor.run().expect("task succeeds");
}

/// 任务执行失败：run 触发 `on_start` 后 `on_failed（携带底层错误`），
/// 绝不触发 `on_succeeded`。
#[test]
fn executor_run_failure_triggers_on_failed_with_error() {
    let mut listener = MockListener::new();
    listener.expect_on_start().once().return_const(());
    listener.expect_on_succeeded().never();
    listener
        .expect_on_failed()
        .once()
        .withf(|_, error| matches!(error, CronError::InvalidPattern(msg) if msg == "boom"))
        .return_const(());

    let mut task = MockTask::new();
    task.expect_execute()
        .once()
        .returning(|| Err(CronError::InvalidPattern("boom".to_string())));

    let manager = TaskListenerManager::default();
    manager.add_listener(Arc::new(listener));
    let executor = executor_with(manager, Arc::new(task));
    let error = executor.run().expect_err("task fails");
    assert!(matches!(error, CronError::InvalidPattern(_)));
}
