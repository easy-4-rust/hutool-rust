//! hutool-cron timingwheel `模块单元测试（RUST_OBLIGATION`）。
//!
//! Java 侧 `Issue3090Test` 为 `main` 方法（无断言），本文件以 Rust 本地测试
//! 承载 `SystemTimer`/`TimingWheel`/`TimerTask`/`TimerTaskList` 的
//! 延迟、过期、桶调度、启动/停止生命周期语义。

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use hutool_cron::timingwheel::{SystemTimer, TimerTask, TimerTaskList, TimingWheel};

/// 当前 Unix 毫秒（测试辅助，对应实现内部 `now_millis`）。
fn now_millis() -> i64 {
    i64::try_from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis(),
    )
    .unwrap_or(i64::MAX)
}

/// TimerTask：延迟、截止时间、最多执行一次（对齐 Java `run`）
#[test]
fn timer_task_delay_and_single_execution() {
    let calls = Arc::new(AtomicUsize::new(0));
    let slot = Arc::clone(&calls);
    let task = TimerTask::new(
        move || {
            slot.fetch_add(1, Ordering::SeqCst);
        },
        Duration::from_millis(500),
    );
    assert_eq!(task.delay(), Duration::from_millis(500));
    assert_eq!(task.delay_ms(), 500);
    // 第一次执行返回 true
    assert!(task.execute());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    // 第二次执行返回 false（任务已消费）
    assert!(!task.execute());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    // Debug 输出含延迟信息
    assert!(format!("{task:?}").contains("delay_ms"));
}

/// TimerTaskList：桶的过期时间、添加/移除/排空
#[test]
fn timer_task_list_bucket_semantics() {
    let mut bucket = TimerTaskList::new();
    assert_eq!(bucket.expiration(), -1);
    assert!(bucket.is_empty());

    let task = TimerTask::new(|| {}, Duration::from_millis(100));
    // 设置过期时间：变化返回 true，重复设置返回 false
    assert!(bucket.set_expiration(1000));
    assert!(!bucket.set_expiration(1000));
    assert_eq!(bucket.expiration(), 1000);
    assert_eq!(bucket.delay(500), Duration::from_millis(500));
    assert_eq!(bucket.delay(1500), Duration::ZERO);

    bucket.add_task(task.clone());
    assert_eq!(bucket.len(), 1);
    // remove_task 移除指定任务
    assert!(bucket.remove_task(&task));
    assert!(bucket.is_empty());
    assert!(!bucket.remove_task(&task));

    // flush 排空并复位过期时间
    bucket.add_task(task);
    bucket.set_expiration(2000);
    let flushed = Arc::new(Mutex::new(Vec::new()));
    let slot = Arc::clone(&flushed);
    bucket.flush(move |t| slot.lock().unwrap().push(t));
    assert!(bucket.is_empty());
    assert_eq!(bucket.expiration(), -1);
    assert_eq!(flushed.lock().unwrap().len(), 1);
    // compare_to 按过期时间比较
    let mut other = TimerTaskList::new();
    other.set_expiration(3000);
    assert_eq!(bucket.compare_to(&other), std::cmp::Ordering::Less);
}

/// `TimingWheel：桶选择、add_task` `边界、advance_clock` 触发
#[test]
fn timing_wheel_add_and_advance() {
    // tick=100ms, size=10 → interval=1000ms（真实系统时间）
    let executed = Arc::new(Mutex::new(Vec::new()));
    let slot = Arc::clone(&executed);
    let mut wheel = TimingWheel::new(Duration::from_millis(100), 10, move |bucket| {
        let _ = bucket;
    })
    .unwrap();

    let task = TimerTask::new(
        move || slot.lock().unwrap().push("done".to_string()),
        Duration::from_millis(500),
    );
    // 500ms 在 [100, 1000) 间隔内 → 可添加
    assert!(wheel.add_task(task.clone()));
    // 超出间隔的任务被拒绝（对齐 Java 单级轮拒绝语义）
    let far = TimerTask::new(|| {}, Duration::from_secs(10));
    assert!(!wheel.add_task(far));
    // 早于下一 tick 的任务被拒绝
    let early = TimerTask::new(|| {}, Duration::from_millis(0));
    assert!(!wheel.add_task(early));

    // 当前时刻 +50ms：未到下一 tick，不推进
    let now = now_millis();
    wheel.advance_clock(now + 50, |_| {});
    assert!(executed.lock().unwrap().is_empty());
    // 推进到任务截止（now+500ms）之后触发 flush
    let mut flushed: Vec<TimerTask> = Vec::new();
    wheel.advance_clock(now + 600, |t| flushed.push(t));
    assert_eq!(flushed.len(), 1);
    // flush 出来的任务执行
    flushed[0].execute();
    assert_eq!(executed.lock().unwrap().as_slice(), &["done"]);
}

/// `TimingWheel` 构造参数校验
#[test]
fn timing_wheel_constructor_validation() {
    // tick 为 0 报错
    assert!(TimingWheel::with_current_time(Duration::ZERO, 10, 0, |_| {}).is_err());
    // wheel_size 为 0 报错
    assert!(TimingWheel::with_current_time(Duration::from_millis(100), 0, 0, |_| {}).is_err());
    // Debug 输出
    let wheel =
        TimingWheel::with_current_time(Duration::from_millis(100), 10, 1_000_000, |_| {}).unwrap();
    assert!(format!("{wheel:?}").contains("wheel_size"));
}

/// SystemTimer：完整生命周期（start/add/执行/stop）
#[test]
fn system_timer_lifecycle() {
    let calls = Arc::new(AtomicUsize::new(0));
    let slot = Arc::clone(&calls);
    let mut timer = SystemTimer::new();
    assert!(!timer.is_started());

    // 未启动时 add_task 进入 pending
    timer
        .add_task(TimerTask::new(
            move || {
                slot.fetch_add(1, Ordering::SeqCst);
            },
            Duration::from_millis(50),
        ))
        .unwrap();

    timer.start().unwrap();
    assert!(timer.is_started());
    // 重复启动报错
    assert!(timer.start().is_err());
    // 启动后 set_delay_queue_timeout 报错
    assert!(
        timer
            .set_delay_queue_timeout(Duration::from_millis(200))
            .is_err()
    );

    // 等待任务执行（事件驱动轮询）
    awaitility::at_most(Duration::from_secs(2))
        .poll_interval(Duration::from_millis(5))
        .until(|| calls.load(Ordering::SeqCst) > 0);

    // 启动后仍可添加任务
    let more = Arc::new(AtomicUsize::new(0));
    let slot2 = Arc::clone(&more);
    timer
        .add_task(TimerTask::new(
            move || {
                slot2.fetch_add(1, Ordering::SeqCst);
            },
            Duration::from_millis(30),
        ))
        .unwrap();
    awaitility::at_most(Duration::from_secs(2))
        .poll_interval(Duration::from_millis(5))
        .until(|| more.load(Ordering::SeqCst) > 0);

    timer.stop();
    assert!(!timer.is_started());
    // stop 后可重新启动（pending 为空）
    timer.start().unwrap();
    timer.stop();
}

/// `SystemTimer` 非法状态
#[test]
fn system_timer_invalid_state() {
    let mut timer = SystemTimer::new();
    // set_delay_queue_timeout(0) 报错
    assert!(timer.set_delay_queue_timeout(Duration::ZERO).is_err());
    // 正常设置
    timer
        .set_delay_queue_timeout(Duration::from_millis(50))
        .unwrap();
    // Drop 自动停止
    drop(timer);
}

/// `advance_clock` 早于下一 tick 时直接返回（不推进 `current_time、不` flush）。
#[test]
fn advance_clock_early_return_before_tick() {
    let mut wheel =
        TimingWheel::with_current_time(Duration::from_millis(100), 10, 1_000_000, |_| {}).unwrap();
    let mut flushed = false;
    // 1_000_050 < current(1_000_000) + tick(100) → 早退，flush 不触发
    wheel.advance_clock(1_000_050, |_| flushed = true);
    assert!(!flushed);
    // 推进到 current+200：current 更新到 1_000_200，但桶内无任务 → flush 仍不触发
    wheel.advance_clock(1_000_200, |_| flushed = true);
    assert!(!flushed);
}
