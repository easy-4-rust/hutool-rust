//! 对齐: `cn.hutool.core.date.StopWatch`
//! 来源: hutool-core StopWatch（Spring Framework 风格秒表）

use std::fmt;
use std::time::{Duration, Instant};

use super::stop_watch::StopWatch;
use super::time_unit::TimeUnit;

/// 对齐 Java: `StopWatch.TaskInfo`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskInfo {
    task_name: String,
    pub(crate) time_nanos: u128,
}

impl TaskInfo {
    /// 构造任务信息。
    pub fn new(task_name: impl Into<String>, time_nanos: u128) -> Self {
        Self {
            task_name: task_name.into(),
            time_nanos,
        }
    }

    /// 对齐 Java: `TaskInfo.getTaskName()`
    pub fn get_task_name(&self) -> &str {
        &self.task_name
    }

    /// 对齐 Java: `TaskInfo.getTime(TimeUnit)`
    pub fn get_time(&self, unit: TimeUnit) -> f64 {
        unit.convert_from_nanos(self.time_nanos)
    }

    /// 对齐 Java: `TaskInfo.getTimeNanos()`
    pub fn get_time_nanos(&self) -> u128 {
        self.time_nanos
    }

    /// 对齐 Java: `TaskInfo.getTimeMillis()`
    pub fn get_time_millis(&self) -> u128 {
        self.time_nanos / 1_000_000
    }

    /// 对齐 Java: `TaskInfo.getTimeSeconds()`
    pub fn get_time_seconds(&self) -> f64 {
        self.time_nanos as f64 / 1_000_000_000.0
    }
}
