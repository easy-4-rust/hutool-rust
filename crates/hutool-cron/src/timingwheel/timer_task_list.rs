//! 对齐: `cn.hutool.cron.timingwheel.TimerTaskList`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/timingwheel/TimerTaskList.java
//! 中文说明: 时间轮桶，管理具有相同过期时间的任务集合。

#![allow(clippy::missing_panics_doc)]

use std::{
    cmp::Ordering,
    sync::Arc,
    time::Duration,
};

use super::timer_task::TimerTask;

/// 对齐: `cn.hutool.cron.timingwheel.TimerTaskList`
/// 中文说明: 时间轮桶，管理具有相同过期时间的任务集合。
///
/// A timing-wheel bucket.
#[derive(Debug, Default)]
pub struct TimerTaskList {
    expiration_ms: i64,
    tasks: Vec<TimerTask>,
}

impl TimerTaskList {
    /// 中文说明: 创建空的未调度桶。
    #[must_use]
    pub const fn new() -> Self {
        Self {
            expiration_ms: -1,
            tasks: Vec::new(),
        }
    }

    /// 中文说明: 修改过期时间，返回是否发生变化。
    /// 对齐 Java 方法: `setExpiration`
    pub fn set_expiration(&mut self, expiration_ms: i64) -> bool {
        if self.expiration_ms == expiration_ms {
            false
        } else {
            self.expiration_ms = expiration_ms;
            true
        }
    }

    /// 中文说明: 返回 Unix 毫秒过期时间。
    /// 对齐 Java 方法: `getExpiration`
    #[must_use]
    pub const fn expiration(&self) -> i64 {
        self.expiration_ms
    }

    /// 中文说明: 向桶中添加任务。
    /// 对齐 Java 方法: `addTask`
    pub fn add_task(&mut self, task: TimerTask) {
        self.tasks.push(task);
    }

    /// 中文说明: 按共享标识移除一个任务。
    /// 对齐 Java 方法: `removeTask`
    pub fn remove_task(&mut self, task: &TimerTask) -> bool {
        if let Some(index) = self
            .tasks
            .iter()
            .position(|candidate| Arc::ptr_eq(&candidate.0, &task.0))
        {
            self.tasks.remove(index);
            true
        } else {
            false
        }
    }

    /// 中文说明: 通过调用者提供的消费者排空桶中所有任务。
    /// 对齐 Java 方法: `flush`
    pub fn flush<F>(&mut self, mut consumer: F)
    where
        F: FnMut(TimerTask),
    {
        for task in self.tasks.drain(..) {
            consumer(task);
        }
        self.expiration_ms = -1;
    }

    /// 中文说明: 返回非负的剩余延迟。
    #[must_use]
    pub fn delay(&self, now_ms: i64) -> Duration {
        Duration::from_millis(
            u64::try_from(self.expiration_ms.saturating_sub(now_ms).max(0)).unwrap_or_default(),
        )
    }

    /// 中文说明: 比较桶的截止时间。
    #[must_use]
    pub fn compare_to(&self, other: &Self) -> Ordering {
        self.expiration_ms.cmp(&other.expiration_ms)
    }

    /// 中文说明: 返回当前任务数量。
    #[must_use]
    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    /// 中文说明: 返回桶是否为空。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}
