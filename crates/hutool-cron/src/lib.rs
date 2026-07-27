//! 对齐: `cn.hutool.cron` (CronUtil / Cron)
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/CronUtil.java
//! 中文说明: Hutool 定时任务模块的 Rust 入口，提供 cron 表达式解析、
//! 调度器、时间轮等核心功能的对外导出。
//!
//! Cron expression parsing and occurrence calculation.

#![forbid(unsafe_code)]

use chrono::{DateTime, TimeZone, Utc};
use cron::Schedule;
use std::str::FromStr;
use std::{future::Future, pin::Pin, sync::Arc, time::Duration};
use thiserror::Error;
use tokio::{sync::oneshot, task::JoinHandle, time};
use tracing::Instrument;

type UnitFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
type UnitJob = Arc<dyn Fn() -> UnitFuture + Send + Sync + 'static>;
type FallibleFuture = Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'static>>;
type FallibleJob = Arc<dyn Fn() -> FallibleFuture + Send + Sync + 'static>;

fn box_unit_job<F, Fut>(job: F) -> UnitJob
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    Arc::new(move || Box::pin(job()))
}

fn box_fallible_job<F, Fut>(job: F) -> FallibleJob
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), String>> + Send + 'static,
{
    Arc::new(move || Box::pin(job()))
}

mod compat;
mod cron_config;
mod cron_timer;
mod cron_util;
pub mod listener;
pub mod pattern;
pub mod prelude;
mod scheduler;
pub mod task;
mod task_executor;
mod task_executor_manager;
mod task_launcher;
mod task_launcher_manager;
pub mod timingwheel;

pub use compat::{InvokeRegistry, TaskTable};
pub use listener::{SimpleTaskListener, TaskListener, TaskListenerManager};
pub use cron_config::CronConfig;
pub use cron_timer::CronTimer;
pub use cron_util::CronUtil;
pub use pattern::{
    AlwaysTrueMatcher, BoolArrayMatcher, CronPattern, CronPatternBuilder, CronPatternUtil,
    DayOfMonthMatcher, Part, PartMatcher, PartParser, PatternMatcher, PatternParser,
    YearValueMatcher,
};
pub use scheduler::{CronSettingEntry, Scheduler};
pub use task::{CronTask, InvokeTask, RunnableTask, Task};
pub use task_executor::TaskExecutor;
pub use task_executor_manager::TaskExecutorManager;
pub use task_launcher::TaskLauncher;
pub use task_launcher_manager::TaskLauncherManager;
pub use timingwheel::{SystemTimer, TimerTask, TimerTaskList, TimingWheel};

/// 对齐: `cn.hutool.cron.CronException`
/// 中文说明: Cron 模块统一错误类型，涵盖表达式解析、任务调度、
/// 定时器生命周期等各类异常。
///
/// Errors returned by cron utilities.
#[derive(Debug, Error)]
pub enum CronError {
    /// The cron expression is invalid.
    #[error("invalid cron expression: {0}")]
    Parse(#[from] cron::error::Error),
    /// The timestamp cannot be represented by `chrono`.
    #[error("timestamp is outside the supported range")]
    InvalidTimestamp,
    /// A retry policy must contain at least one attempt and a valid delay range.
    #[error("retry policy must have at least one attempt and initial delay <= maximum delay")]
    InvalidRetryPolicy,
    /// The expression has an unsupported field shape.
    #[error("invalid Hutool cron pattern: {0}")]
    InvalidPattern(String),
    /// A field index is outside the seven cron parts.
    #[error("invalid cron part index: {0}")]
    InvalidPartIndex(usize),
    /// A value is outside a cron field's valid range.
    #[error("invalid {part:?} value {value}")]
    InvalidPartValue {
        /// The field being validated.
        part: Part,
        /// The rejected value.
        value: i32,
    },
    /// A field range must be ascending.
    #[error("invalid {part:?} range {begin}..={end}")]
    InvalidPartRange {
        /// The field being validated.
        part: Part,
        /// Inclusive range start.
        begin: i32,
        /// Inclusive range end.
        end: i32,
    },
    /// A builder field requires at least one value.
    #[error("{0:?} requires at least one value")]
    EmptyPartValues(Part),
    /// A finite matcher requires at least one value.
    #[error("a finite cron matcher requires at least one value")]
    EmptyMatcher,
    /// Date range bounds are reversed.
    #[error("cron date range end precedes start")]
    InvalidDateRange,
    /// Timing-wheel dimensions must be non-zero and fit in an `i64` interval.
    #[error("timing wheel requires a non-zero tick and wheel size")]
    InvalidTimingWheel,
    /// Timer configuration cannot be changed in its current lifecycle state.
    #[error("timer configuration is invalid in its current state")]
    InvalidTimerState,
    /// The timer has already been started.
    #[error("timer has already been started")]
    TimerAlreadyStarted,
    /// The timer worker has stopped accepting tasks.
    #[error("timer worker has stopped")]
    TimerStopped,
    /// The timer thread could not be created.
    #[error("failed to create timer thread: {0}")]
    TimerThread(#[source] std::io::Error),
    /// A named invocation was not present in the injected registry.
    #[error("unknown registered cron invocation: {0}")]
    UnknownInvokeTask(String),
    /// Task IDs are unique within a scheduler.
    #[error("duplicate cron task ID: {0}")]
    DuplicateTaskId(String),
    /// A task reported an application error.
    #[error("cron task failed: {0}")]
    Task(String),
    /// A scheduler requires an injected or current Tokio runtime.
    #[error("no Tokio runtime is available for the cron scheduler")]
    MissingRuntime,
    /// A running scheduler cannot be started or reconfigured.
    #[error("cron scheduler has already been started")]
    SchedulerAlreadyStarted,
}

/// 对齐: `cn.hutool.cron.pattern.CronPattern`
/// 中文说明: 已解析的 cron 调度计划，线程安全，可计算下一次触发时间。
///
/// A parsed cron schedule that can be shared safely between threads.
#[derive(Debug, Clone)]
pub struct CronSchedule {
    expression: String,
    inner: Schedule,
}

impl CronSchedule {
    /// 中文说明: 解析 cron 表达式，支持秒级精度（第一位为秒字段）。
    /// 对齐 Java 方法: `of`
    pub fn parse(expression: impl Into<String>) -> Result<Self, CronError> {
        let expression = expression.into();
        let inner = Schedule::from_str(&expression)?;
        Ok(Self { expression, inner })
    }

    /// Returns the original expression.
    #[must_use]
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// 中文说明: 返回指定 UTC 时间之后的下一次触发时刻。
    /// 对齐 Java 方法: `nextMatchAfter`
    #[must_use]
    pub fn next_after(&self, after: &DateTime<Utc>) -> Option<DateTime<Utc>> {
        self.inner.after(after).next()
    }

    /// 中文说明: 返回指定时间之后最多 `limit` 个触发时刻。
    /// 对齐 Java 方法: `matchedDates`
    #[must_use]
    pub fn upcoming(&self, after: &DateTime<Utc>, limit: usize) -> Vec<DateTime<Utc>> {
        self.inner.after(after).take(limit).collect()
    }

    /// Resolves a millisecond timestamp and returns the next occurrence.
    pub fn next_after_millis(&self, timestamp: i64) -> Result<Option<DateTime<Utc>>, CronError> {
        let after = Utc
            .timestamp_millis_opt(timestamp)
            .single()
            .ok_or(CronError::InvalidTimestamp)?;
        Ok(self.next_after(&after))
    }
}

/// 中文说明: 运行中的定时任务句柄，丢弃时自动取消后续执行。
///
/// A running cron job. Dropping the handle aborts future executions.
#[derive(Debug)]
pub struct JobHandle {
    cancellation: Option<oneshot::Sender<()>>,
    task: JoinHandle<()>,
}

impl JobHandle {
    /// Requests cancellation and waits for the task to finish.
    pub async fn cancel(mut self) {
        if let Some(cancellation) = self.cancellation.take() {
            let _ = cancellation.send(());
        }
        let _ = (&mut self.task).await;
    }
}

impl Drop for JobHandle {
    fn drop(&mut self) {
        if let Some(cancellation) = self.cancellation.take() {
            let _ = cancellation.send(());
        }
        self.task.abort();
    }
}

/// 中文说明: 定时任务的运行时执行策略（如超时时间）。
///
/// Runtime execution policy for a cron job.
#[derive(Debug, Clone, Copy, Default)]
pub struct JobPolicy {
    /// Per-execution deadline. `None` allows the job to run until completion
    /// or cancellation.
    pub timeout: Option<Duration>,
}

/// 中文说明: 失败任务的重试策略，支持指数退避。
///
/// Retry policy for fallible jobs, kept separate from the job implementation.
#[derive(Debug, Clone, Copy)]
pub struct RetryPolicy {
    max_attempts: u32,
    initial_delay: Duration,
    max_delay: Duration,
}

impl RetryPolicy {
    /// 中文说明: 创建有界指数退避重试策略，`max_attempts` 包含首次执行。
    /// 对齐 Java 方法: `new`
    pub fn new(
        max_attempts: u32,
        initial_delay: Duration,
        max_delay: Duration,
    ) -> Result<Self, CronError> {
        if max_attempts == 0 || initial_delay > max_delay {
            return Err(CronError::InvalidRetryPolicy);
        }
        Ok(Self {
            max_attempts,
            initial_delay,
            max_delay,
        })
    }

    /// 中文说明: 返回仅执行一次的策略（不重试）。
    #[must_use]
    pub const fn none() -> Self {
        Self {
            max_attempts: 1,
            initial_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
        }
    }

    fn delay_after(&self, failed_attempt: u32) -> Duration {
        let exponent = failed_attempt.saturating_sub(1).min(31);
        self.initial_delay
            .checked_mul(1_u32 << exponent)
            .unwrap_or(self.max_delay)
            .min(self.max_delay)
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self::none()
    }
}

/// 中文说明: 在当前 Tokio 运行时上启动异步定时任务，
/// 每次触发仅在上次完成后才开始，避免任务重叠。
/// 对齐 Java 方法: `CronUtil.schedule`
#[must_use]
pub fn spawn<F, Fut>(schedule: CronSchedule, job: F) -> JobHandle
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    spawn_on(
        &tokio::runtime::Handle::current(),
        schedule,
        JobPolicy::default(),
        job,
    )
}

/// 中文说明: 在指定的 Tokio 运行时句柄上启动定时任务。
/// 对齐 Java 方法: `CronUtil.schedule`
#[must_use]
pub fn spawn_on<F, Fut>(
    runtime: &tokio::runtime::Handle,
    schedule: CronSchedule,
    policy: JobPolicy,
    job: F,
) -> JobHandle
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let job = box_unit_job(job);
    spawn_on_boxed(runtime, schedule, policy, job)
}

fn spawn_on_boxed(
    runtime: &tokio::runtime::Handle,
    schedule: CronSchedule,
    policy: JobPolicy,
    job: UnitJob,
) -> JobHandle {
    let (cancellation, mut cancelled) = oneshot::channel();
    let task = runtime.spawn(async move {
        loop {
            let now = Utc::now();
            let Some(next) = schedule.next_after(&now) else {
                return;
            };
            let delay = (next - now).to_std().unwrap_or(Duration::ZERO);
            tokio::select! {
                () = time::sleep(delay) => {},
                _ = &mut cancelled => return,
            }
            let span = tracing::info_span!(
                "hutool.cron.job",
                scheduled_at = %next,
                timed_out = tracing::field::Empty,
            );
            let run = (job)().instrument(span.clone());
            if let Some(timeout) = policy.timeout {
                tokio::select! {
                    result = time::timeout(timeout, run) => {
                        if result.is_err() {
                            record_timeout(&span);
                        }
                    },
                    _ = &mut cancelled => return,
                }
            } else {
                tokio::select! {
                    () = run => {},
                    _ = &mut cancelled => return,
                }
            }
        }
    });
    JobHandle {
        cancellation: Some(cancellation),
        task,
    }
}

fn record_timeout(span: &tracing::Span) {
    span.record("timed_out", true);
    tracing::warn!(parent: span, "cron job exceeded its execution timeout");
}

/// 中文说明: 在当前 Tokio 运行时上启动可失败定时任务，
/// 支持独立的有界重试策略，执行不会重叠。
/// 对齐 Java 方法: `CronUtil.schedule`
#[must_use]
pub fn spawn_fallible<F, Fut>(
    schedule: CronSchedule,
    policy: JobPolicy,
    retry: RetryPolicy,
    job: F,
) -> JobHandle
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), String>> + Send + 'static,
{
    spawn_fallible_on(
        &tokio::runtime::Handle::current(),
        schedule,
        policy,
        retry,
        job,
    )
}

/// 中文说明: 在指定的 Tokio 运行时句柄上启动可失败定时任务。
/// 对齐 Java 方法: `CronUtil.schedule`
#[must_use]
pub fn spawn_fallible_on<F, Fut>(
    runtime: &tokio::runtime::Handle,
    schedule: CronSchedule,
    policy: JobPolicy,
    retry: RetryPolicy,
    job: F,
) -> JobHandle
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), String>> + Send + 'static,
{
    let job = box_fallible_job(job);
    spawn_fallible_on_boxed(runtime, schedule, policy, retry, job)
}

fn spawn_fallible_on_boxed(
    runtime: &tokio::runtime::Handle,
    schedule: CronSchedule,
    policy: JobPolicy,
    retry: RetryPolicy,
    job: FallibleJob,
) -> JobHandle {
    let (cancellation, mut cancelled) = oneshot::channel();
    let task = runtime.spawn(async move {
        loop {
            let now = Utc::now();
            let Some(next) = schedule.next_after(&now) else {
                return;
            };
            let delay = (next - now).to_std().unwrap_or(Duration::ZERO);
            tokio::select! {
                () = time::sleep(delay) => {},
                _ = &mut cancelled => return,
            }
            let span = tracing::info_span!(
                "hutool.cron.fallible_job",
                scheduled_at = %next,
                attempts = tracing::field::Empty,
                outcome = tracing::field::Empty,
            );
            tokio::select! {
                result = run_with_retry(&job, policy, retry, &span) => {
                    match result {
                        Ok(attempts) => {
                            span.record("attempts", attempts);
                            span.record("outcome", "success");
                        }
                        Err(failure) => {
                            span.record("attempts", failure.attempts);
                            span.record("outcome", failure.kind);
                            tracing::warn!(parent: &span, error = %failure.message, "cron job exhausted its retry policy");
                        }
                    }
                },
                _ = &mut cancelled => return,
            }
        }
    });
    JobHandle {
        cancellation: Some(cancellation),
        task,
    }
}

#[derive(Debug)]
struct RunFailure {
    attempts: u32,
    kind: &'static str,
    message: String,
}

async fn run_with_retry(
    job: &FallibleJob,
    policy: JobPolicy,
    retry: RetryPolicy,
    span: &tracing::Span,
) -> Result<u32, RunFailure> {
    for attempt in 1..=retry.max_attempts {
        let outcome = if let Some(timeout) = policy.timeout {
            match time::timeout(timeout, (job)().instrument(span.clone())).await {
                Ok(result) => result.map_err(|error| ("error", error)),
                Err(_) => Err(("timeout", format!("execution exceeded {timeout:?}"))),
            }
        } else {
            (job)()
                .instrument(span.clone())
                .await
                .map_err(|error| ("error", error))
        };
        match outcome {
            Ok(()) => return Ok(attempt),
            Err((kind, message)) if attempt == retry.max_attempts => {
                return Err(RunFailure {
                    attempts: attempt,
                    kind,
                    message,
                });
            }
            Err(_) => time::sleep(retry.delay_after(attempt)).await,
        }
    }
    Err(RunFailure {
        attempts: 0,
        kind: "invalid_retry_policy",
        message: "retry policy did not permit an attempt".to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    type SignalSender = Arc<std::sync::Mutex<Option<oneshot::Sender<()>>>>;
    type SignalReceiver = oneshot::Receiver<()>;

    async fn noop_job() {}

    fn ready_success() -> std::future::Ready<Result<(), String>> {
        std::future::ready(Ok(()))
    }

    #[test]
    fn calculates_deterministic_occurrences() {
        let schedule = CronSchedule::parse("0 */5 * * * * *").unwrap();
        assert_eq!(schedule.expression(), "0 */5 * * * * *");
        let start = Utc.with_ymd_and_hms(2026, 1, 1, 0, 1, 0).unwrap();
        let values = schedule.upcoming(&start, 2);
        assert_eq!(
            values[0],
            Utc.with_ymd_and_hms(2026, 1, 1, 0, 5, 0).unwrap()
        );
        assert_eq!(
            values[1],
            Utc.with_ymd_and_hms(2026, 1, 1, 0, 10, 0).unwrap()
        );
        assert_eq!(
            schedule
                .next_after_millis(start.timestamp_millis())
                .unwrap(),
            Some(values[0])
        );
        assert!(schedule.next_after_millis(i64::MAX).is_err());
        assert!(CronSchedule::parse("invalid").is_err());
        assert!(RetryPolicy::new(0, Duration::ZERO, Duration::ZERO).is_err());
        assert!(RetryPolicy::new(1, Duration::from_secs(2), Duration::from_secs(1)).is_err());
        assert_eq!(RetryPolicy::default().max_attempts, 1);
        let saturating = RetryPolicy {
            max_attempts: 2,
            initial_delay: Duration::MAX,
            max_delay: Duration::MAX,
        };
        assert_eq!(saturating.delay_after(32), Duration::MAX);
    }

    #[tokio::test]
    async fn spawned_jobs_can_be_cancelled() {
        let schedule = CronSchedule::parse("0/1 * * * * * *").unwrap();
        let handle = spawn_on(
            &tokio::runtime::Handle::current(),
            schedule.clone(),
            JobPolicy {
                timeout: Some(std::time::Duration::from_millis(10)),
            },
            noop_job,
        );
        handle.cancel().await;

        let (signal, observed) = one_shot_signal();
        let run_signal = Arc::clone(&signal);
        let handle = spawn_on(
            &tokio::runtime::Handle::current(),
            schedule.clone(),
            JobPolicy {
                timeout: Some(Duration::from_secs(1)),
            },
            move || {
                signal_once(&run_signal);
                std::future::ready(())
            },
        );
        time::timeout(Duration::from_secs(2), observed)
            .await
            .unwrap()
            .unwrap();
        time::sleep(Duration::from_millis(5)).await;
        handle.cancel().await;
    }

    fn one_shot_signal() -> (SignalSender, SignalReceiver) {
        let (sender, receiver) = oneshot::channel();
        (Arc::new(std::sync::Mutex::new(Some(sender))), receiver)
    }

    fn signal_once(signal: &SignalSender) {
        if let Some(sender) = signal.lock().unwrap().take() {
            let _ = sender.send(());
        }
    }

    #[tokio::test]
    async fn spawned_jobs_execute_timeout_and_cancel_in_each_run_state() {
        let schedule = CronSchedule::parse("0/1 * * * * * *").unwrap();

        let (signal, observed) = one_shot_signal();
        let run_signal = Arc::clone(&signal);
        let handle = spawn(schedule.clone(), move || {
            let run_signal = Arc::clone(&run_signal);
            async move {
                signal_once(&run_signal);
            }
        });
        time::timeout(Duration::from_secs(2), observed)
            .await
            .unwrap()
            .unwrap();
        handle.cancel().await;

        let (signal, observed) = one_shot_signal();
        let run_signal = Arc::clone(&signal);
        let handle = spawn_on(
            &tokio::runtime::Handle::current(),
            schedule.clone(),
            JobPolicy {
                timeout: Some(Duration::from_millis(5)),
            },
            move || {
                signal_once(&run_signal);
                time::sleep(Duration::from_millis(100))
            },
        );
        time::timeout(Duration::from_secs(2), observed)
            .await
            .unwrap()
            .unwrap();
        time::sleep(Duration::from_millis(20)).await;
        handle.cancel().await;

        let (signal, observed) = one_shot_signal();
        let run_signal = Arc::clone(&signal);
        let handle = spawn_on(
            &tokio::runtime::Handle::current(),
            schedule.clone(),
            JobPolicy {
                timeout: Some(Duration::from_secs(5)),
            },
            move || {
                signal_once(&run_signal);
                std::future::pending::<()>()
            },
        );
        time::timeout(Duration::from_secs(2), observed)
            .await
            .unwrap()
            .unwrap();
        handle.cancel().await;

        let (signal, observed) = one_shot_signal();
        let run_signal = Arc::clone(&signal);
        let handle = spawn_on(
            &tokio::runtime::Handle::current(),
            schedule,
            JobPolicy::default(),
            move || {
                signal_once(&run_signal);
                std::future::pending::<()>()
            },
        );
        time::timeout(Duration::from_secs(2), observed)
            .await
            .unwrap()
            .unwrap();
        handle.cancel().await;
    }

    #[tokio::test]
    async fn handles_cover_no_future_schedule_and_drop_shutdown() {
        let expired = CronSchedule::parse("0 0 0 1 1 * 1970").unwrap();
        spawn_on(
            &tokio::runtime::Handle::current(),
            expired,
            JobPolicy::default(),
            noop_job,
        )
        .cancel()
        .await;

        let handle = spawn_on(
            &tokio::runtime::Handle::current(),
            CronSchedule::parse("0/1 * * * * * *").unwrap(),
            JobPolicy::default(),
            noop_job,
        );
        drop(handle);
        time::sleep(Duration::from_millis(1)).await;

        JobHandle {
            cancellation: None,
            task: tokio::spawn(noop_job()),
        }
        .cancel()
        .await;
        drop(JobHandle {
            cancellation: None,
            task: tokio::spawn(noop_job()),
        });
    }

    #[tokio::test]
    async fn fallible_jobs_retry_with_independent_policy() {
        let attempts = Arc::new(AtomicUsize::new(0));
        let job_attempts = Arc::clone(&attempts);
        let job = box_fallible_job(move || {
            let job_attempts = Arc::clone(&job_attempts);
            async move {
                if job_attempts.fetch_add(1, Ordering::SeqCst) < 2 {
                    Err("transient".to_owned())
                } else {
                    Ok(())
                }
            }
        });
        let retry =
            RetryPolicy::new(3, Duration::from_millis(1), Duration::from_millis(2)).unwrap();
        let result = run_with_retry(
            &job,
            JobPolicy {
                timeout: Some(Duration::from_secs(1)),
            },
            retry,
            &tracing::Span::none(),
        )
        .await;
        assert_eq!(result.unwrap(), 3);
        assert_eq!(attempts.load(Ordering::SeqCst), 3);

        let no_timeout = box_fallible_job(|| async { Err("permanent".to_owned()) });
        let failure = run_with_retry(
            &no_timeout,
            JobPolicy::default(),
            RetryPolicy::none(),
            &tracing::Span::none(),
        )
        .await
        .unwrap_err();
        assert_eq!(failure.attempts, 1);
        assert_eq!(failure.kind, "error");
        assert_eq!(failure.message, "permanent");
        assert!(format!("{failure:?}").contains("permanent"));

        let timeout_job = box_fallible_job(std::future::pending::<Result<(), String>>);
        let timeout = run_with_retry(
            &timeout_job,
            JobPolicy {
                timeout: Some(Duration::from_millis(1)),
            },
            RetryPolicy::none(),
            &tracing::Span::none(),
        )
        .await
        .unwrap_err();
        assert_eq!(timeout.kind, "timeout");

        let invalid = RetryPolicy {
            max_attempts: 0,
            initial_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
        };
        assert_eq!(
            run_with_retry(
                &no_timeout,
                JobPolicy::default(),
                invalid,
                &tracing::Span::none()
            )
            .await
            .unwrap_err()
            .kind,
            "invalid_retry_policy"
        );
        let ready = box_fallible_job(ready_success);
        assert_eq!(
            run_with_retry(
                &ready,
                JobPolicy::default(),
                RetryPolicy::none(),
                &tracing::Span::none(),
            )
            .await
            .unwrap(),
            1
        );
    }

    #[tokio::test]
    async fn fallible_scheduler_reports_success_failure_and_cancellation() {
        let schedule = CronSchedule::parse("0/1 * * * * * *").unwrap();
        let (signal, observed) = one_shot_signal();
        let run_signal = Arc::clone(&signal);
        let handle = spawn_fallible(
            schedule.clone(),
            JobPolicy::default(),
            RetryPolicy::none(),
            move || {
                signal_once(&run_signal);
                std::future::ready(Ok::<(), String>(()))
            },
        );
        time::timeout(Duration::from_secs(2), observed)
            .await
            .unwrap()
            .unwrap();
        time::sleep(Duration::from_millis(5)).await;
        handle.cancel().await;

        let (signal, observed) = one_shot_signal();
        let run_signal = Arc::clone(&signal);
        let handle = spawn_fallible_on(
            &tokio::runtime::Handle::current(),
            schedule.clone(),
            JobPolicy::default(),
            RetryPolicy::none(),
            move || {
                signal_once(&run_signal);
                std::future::ready(Err("failed".to_owned()))
            },
        );
        time::timeout(Duration::from_secs(2), observed)
            .await
            .unwrap()
            .unwrap();
        time::sleep(Duration::from_millis(5)).await;
        handle.cancel().await;

        let (signal, observed) = one_shot_signal();
        let run_signal = Arc::clone(&signal);
        let handle = spawn_fallible_on(
            &tokio::runtime::Handle::current(),
            schedule,
            JobPolicy::default(),
            RetryPolicy::none(),
            move || {
                signal_once(&run_signal);
                std::future::pending::<Result<(), String>>()
            },
        );
        time::timeout(Duration::from_secs(2), observed)
            .await
            .unwrap()
            .unwrap();
        handle.cancel().await;

        spawn_fallible_on(
            &tokio::runtime::Handle::current(),
            CronSchedule::parse("0 0 0 1 1 * 1970").unwrap(),
            JobPolicy::default(),
            RetryPolicy::none(),
            ready_success,
        )
        .cancel()
        .await;

        record_timeout(&tracing::Span::none());
        noop_job().await;
        ready_success().await.unwrap();
        let (signal, _receiver) = one_shot_signal();
        signal_once(&signal);
        signal_once(&signal);
    }
}
