//! 对齐: `cn.hutool.cron.timingwheel`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/timingwheel/
//! 中文说明: 显式拥有的定时器和时间轮原语，提供高效的延迟任务调度。
//!
//! Explicitly owned timer and timing-wheel primitives.

#![allow(clippy::missing_panics_doc)]

use std::{
    collections::BinaryHeap,
    sync::{Mutex, mpsc},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

mod system_timer;
mod timer_task;
mod timer_task_list;
mod timing_wheel;

pub use system_timer::SystemTimer;
pub use timer_task::TimerTask;
pub use timer_task_list::TimerTaskList;
pub use timing_wheel::TimingWheel;

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

pub(crate) struct TimerTaskInner {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduled_task_equality_by_deadline_and_sequence() {
        let task = |delay: u64| TimerTask::new(|| {}, Duration::from_millis(delay));
        let a = ScheduledTask {
            deadline_ms: 1000,
            sequence: 0,
            task: task(1),
        };
        let b = ScheduledTask {
            deadline_ms: 1000,
            sequence: 1,
            task: task(1),
        };
        // PartialEq：deadline + sequence 都相等才相等（对齐 Java compareTo）
        assert!(a != b);
        let c = ScheduledTask {
            deadline_ms: 1000,
            sequence: 0,
            task: task(2),
        };
        assert!(a == c);
        // Ord 是反转的（BinaryHeap 最大堆实现最小堆）：`x < y` 表示 y 先执行。
        // 同 deadline 时 seq 小者先执行（先进先出）→ a(seq 0) 先于 b(seq 1)。
        assert!(b < a);
        // deadline 更小的 d 先执行 → 反转序中 a < d。
        let d = ScheduledTask {
            deadline_ms: 999,
            sequence: 0,
            task: task(1),
        };
        assert!(a < d);
        assert!(b < d);
    }

    #[test]
    fn bounded_wait_clamps_to_max() {
        assert_eq!(
            bounded_wait(0, 1000, Duration::from_millis(5)),
            Duration::from_millis(0)
        );
        assert_eq!(
            bounded_wait(1_000_000, 0, Duration::from_millis(5)),
            Duration::from_millis(5)
        );
    }
}
