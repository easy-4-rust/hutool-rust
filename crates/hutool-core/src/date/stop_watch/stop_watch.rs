//! 对齐: `cn.hutool.core.date.StopWatch`
//! 来源: hutool-core StopWatch（Spring Framework 风格秒表）

use std::fmt;
use std::time::{Duration, Instant};

use super::task_info::TaskInfo;
use super::time_unit::TimeUnit;

/// 对齐 Java: `cn.hutool.core.date.StopWatch`
#[derive(Debug, Clone)]
pub struct StopWatch {
    id: String,
    keep_task_list: bool,
    task_list: Vec<TaskInfo>,
    current_task_name: Option<String>,
    start_instant: Option<Instant>,
    last_task_info: Option<TaskInfo>,
    task_count: i32,
    total_time_nanos: u128,
}

impl Default for StopWatch {
    fn default() -> Self {
        Self::new()
    }
}

impl StopWatch {
    /// 对齐 Java: `StopWatch.create(String)`
    pub fn create(id: impl Into<String>) -> Self {
        Self::with_id(id)
    }

    /// 对齐 Java: `new StopWatch()`
    pub fn new() -> Self {
        Self::with_keep("", true)
    }

    /// 对齐 Java: `new StopWatch(String)`
    pub fn with_id(id: impl Into<String>) -> Self {
        Self::with_keep(id, true)
    }

    /// 对齐 Java: `new StopWatch(String, boolean)`
    pub fn with_keep(id: impl Into<String>, keep_task_list: bool) -> Self {
        Self {
            id: id.into(),
            keep_task_list,
            task_list: Vec::new(),
            current_task_name: None,
            start_instant: None,
            last_task_info: None,
            task_count: 0,
            total_time_nanos: 0,
        }
    }

    /// 对齐 Java: `StopWatch.getId()`
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// 对齐 Java: `StopWatch.setKeepTaskList(boolean)`
    pub fn set_keep_task_list(&mut self, keep_task_list: bool) {
        self.keep_task_list = keep_task_list;
        if !keep_task_list {
            self.task_list.clear();
        }
    }

    /// 对齐 Java: `StopWatch.start()`
    pub fn start(&mut self) {
        self.start_named("");
    }

    /// 对齐 Java: `StopWatch.start(String)`
    pub fn start_named(&mut self, task_name: impl Into<String>) {
        if self.current_task_name.is_some() {
            panic!("Can't start StopWatch: it's already running");
        }
        self.current_task_name = Some(task_name.into());
        self.start_instant = Some(Instant::now());
    }

    /// 对齐 Java: `StopWatch.stop()`
    pub fn stop(&mut self) {
        let name = self
            .current_task_name
            .take()
            .expect("Can't stop StopWatch: it's not running");
        let elapsed = self
            .start_instant
            .take()
            .map(|s| s.elapsed())
            .unwrap_or(Duration::ZERO);
        let nanos = elapsed.as_nanos();
        self.total_time_nanos = self.total_time_nanos.saturating_add(nanos);
        let info = TaskInfo::new(name, nanos);
        if self.keep_task_list {
            self.task_list.push(info.clone());
        }
        self.last_task_info = Some(info);
        self.task_count += 1;
    }

    /// 对齐 Java: `StopWatch.isRunning()`
    pub fn is_running(&self) -> bool {
        self.current_task_name.is_some()
    }

    /// 对齐 Java: `StopWatch.currentTaskName()`
    pub fn current_task_name(&self) -> Option<&str> {
        self.current_task_name.as_deref()
    }

    /// 对齐 Java: `StopWatch.getLastTaskTimeNanos()`
    pub fn get_last_task_time_nanos(&self) -> u128 {
        self.last_task_info
            .as_ref()
            .map(TaskInfo::get_time_nanos)
            .expect("No tasks run: can't get last task interval")
    }

    /// 对齐 Java: `StopWatch.getLastTaskTimeMillis()`
    pub fn get_last_task_time_millis(&self) -> u128 {
        self.get_last_task_time_nanos() / 1_000_000
    }

    /// 对齐 Java: `StopWatch.getLastTaskName()`
    pub fn get_last_task_name(&self) -> &str {
        self.last_task_info
            .as_ref()
            .map(TaskInfo::get_task_name)
            .expect("No tasks run: can't get last task name")
    }

    /// 对齐 Java: `StopWatch.getLastTaskInfo()`
    pub fn get_last_task_info(&self) -> &TaskInfo {
        self.last_task_info
            .as_ref()
            .expect("No tasks run: can't get last task info")
    }

    /// 对齐 Java: `StopWatch.getTotal(TimeUnit)`
    pub fn get_total(&self, unit: TimeUnit) -> f64 {
        unit.convert_from_nanos(self.total_time_nanos)
    }

    /// 对齐 Java: `StopWatch.getTotalTimeNanos()`
    pub fn get_total_time_nanos(&self) -> u128 {
        self.total_time_nanos
    }

    /// 对齐 Java: `StopWatch.getTotalTimeMillis()`
    pub fn get_total_time_millis(&self) -> u128 {
        self.total_time_nanos / 1_000_000
    }

    /// 对齐 Java: `StopWatch.getTotalTimeSeconds()`
    pub fn get_total_time_seconds(&self) -> f64 {
        self.total_time_nanos as f64 / 1_000_000_000.0
    }

    /// 对齐 Java: `StopWatch.getTaskCount()`
    pub fn get_task_count(&self) -> i32 {
        self.task_count
    }

    /// 对齐 Java: `StopWatch.getTaskInfo()`
    pub fn get_task_info(&self) -> &[TaskInfo] {
        &self.task_list
    }

    /// 对齐 Java: `StopWatch.shortSummary()`
    pub fn short_summary(&self) -> String {
        self.short_summary_unit(TimeUnit::Nanos)
    }

    /// 对齐 Java: `StopWatch.shortSummary(TimeUnit)`
    pub fn short_summary_unit(&self, unit: TimeUnit) -> String {
        format!(
            "StopWatch '{}': running time = {} {}",
            self.id,
            self.get_total(unit),
            unit.short_name()
        )
    }

    /// 对齐 Java: `StopWatch.prettyPrint()`
    pub fn pretty_print(&self) -> String {
        self.pretty_print_unit(TimeUnit::Nanos)
    }

    /// 对齐 Java: `StopWatch.prettyPrint(TimeUnit)`
    pub fn pretty_print_unit(&self, unit: TimeUnit) -> String {
        let mut sb = String::new();
        sb.push_str(&self.short_summary_unit(unit));
        sb.push('\n');
        if self.task_list.is_empty() {
            sb.push_str("No task info kept");
        } else {
            sb.push_str("---------------------------------------------\n");
            sb.push_str(&format!("{:>11}  %{:>5}  Task name\n", unit.short_name(), ""));
            sb.push_str("---------------------------------------------\n");
            let total = self.total_time_nanos.max(1) as f64;
            for task in &self.task_list {
                let pct = task.time_nanos as f64 * 100.0 / total;
                sb.push_str(&format!(
                    "{:>11.3}  {:>6.1}  {}\n",
                    task.get_time(unit),
                    pct,
                    task.get_task_name()
                ));
            }
        }
        sb
    }
}

impl fmt::Display for StopWatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sb = self.short_summary();
        if self.task_list.is_empty() {
            sb.push_str("; no task info kept");
        } else {
            for task in &self.task_list {
                sb.push_str(&format!(
                    "; [{}] took {} ns",
                    task.get_task_name(),
                    task.get_time_nanos()
                ));
            }
        }
        f.write_str(&sb)
    }
}
