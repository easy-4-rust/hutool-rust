//! 对齐: `cn.hutool.cron.timingwheel`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/timingwheel/
//! 中文说明: 显式拥有的定时器和时间轮原语，提供高效的延迟任务调度。
//!
//! Explicitly owned timer and timing-wheel primitives.

#![allow(clippy::missing_panics_doc)]

use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    fmt,
    sync::{Arc, Mutex, mpsc},
    thread::{self, JoinHandle},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::CronError;

mod timer_task;
mod timer_task_list;
mod timing_wheel;
mod system_timer;

pub use timer_task::TimerTask;
pub use timer_task_list::TimerTaskList;
pub use timing_wheel::TimingWheel;
pub use system_timer::SystemTimer;

type TaskFn = Box<dyn FnOnce() + Send + 'static>;

fn now_millis() -> i64 {
    i64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis(),
    )
    .unwrap_or(i64::MAX)
}

struct TimerTaskInner {
    delay_ms: u64,
    deadline_ms: i64,
    task: Mutex<Option<TaskFn>>,
}

struct ScheduledTask {
    deadline_ms: i64,
    sequence: u64,
    task: TimerTask,
}

enum TimerCommand {
    Add(TimerTask),
    Stop,
}

fn run_timer(receiver: &mpsc::Receiver<TimerCommand>, initial: Vec<TimerTask>, max_wait: Duration) {
    let mut queue = BinaryHeap::new();
    let mut sequence = 0_u64;
    for task in initial {
        queue.push(ScheduledTask {
            deadline_ms: task.deadline_ms(),
            sequence,
            task,
        });
        sequence = sequence.wrapping_add(1);
    }
    loop {
        while queue
            .peek()
            .is_some_and(|entry| entry.deadline_ms <= now_millis())
        {
            let entry = queue
                .pop()
                .expect("a successful heap peek guarantees one queued task");
            entry.task.execute();
        }
        let wait = queue.peek().map_or(max_wait, |entry| {
            bounded_wait(entry.deadline_ms, now_millis(), max_wait)
        });
        match receiver.recv_timeout(wait) {
            Ok(TimerCommand::Add(task)) => {
                queue.push(ScheduledTask {
                    deadline_ms: task.deadline_ms(),
                    sequence,
                    task,
                });
                sequence = sequence.wrapping_add(1);
            }
            Ok(TimerCommand::Stop) | Err(mpsc::RecvTimeoutError::Disconnected) => return,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
    }
}

fn bounded_wait(deadline_ms: i64, now_ms: i64, max_wait: Duration) -> Duration {
    let remaining = deadline_ms.saturating_sub(now_ms).max(0);
    Duration::from_millis(
        u64::try_from(remaining).expect("a non-negative i64 is representable as u64"),
    )
    .min(max_wait)
}
