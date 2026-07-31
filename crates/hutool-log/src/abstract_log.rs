//! 对齐: `cn.hutool.log.AbstractLog`
//! 来源: hutool-log/src/main/java/cn/hutool/log/AbstractLog.java
//! 中文说明: 日志的抽象基类，持有日志名称和后端接收器（LogSink），提供各级别启用状态查询。

use std::{fmt, sync::Arc};

use crate::log::Log;
use crate::level::LogLevel;
use crate::log_record::LogRecord;
use crate::log_sink::LogSink;

/// 日志的抽象基类，对应 Hutool 的 `AbstractLog`。
///
/// 对齐 Java 类: `cn.hutool.log.AbstractLog`
/// 来源: hutool-log/src/main/java/cn/hutool/log/AbstractLog.java
///
/// 持有日志名称和可插拔的 `LogSink` 后端，提供各级别启用状态查询。
/// Shared implementation corresponding to Hutool's `AbstractLog`.
#[derive(Clone)]
pub struct AbstractLog {
    name: String,
    sink: Arc<dyn LogSink>,
}

impl fmt::Debug for AbstractLog {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AbstractLog")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

impl AbstractLog {
    /// Creates a logger backed by an explicitly owned sink.
    #[must_use]
    pub fn new(name: &str, sink: Arc<dyn LogSink>) -> Self {
        Self {
            name: name.to_owned(),
            sink,
        }
    }

    /// Returns whether one level is accepted by the sink.
    #[must_use]
    pub fn is_trace_enabled(&self) -> bool {
        self.is_enabled(LogLevel::Trace)
    }
    /// Returns whether debug events are enabled.
    #[must_use]
    pub fn is_debug_enabled(&self) -> bool {
        self.is_enabled(LogLevel::Debug)
    }
    /// Returns whether informational events are enabled.
    #[must_use]
    pub fn is_info_enabled(&self) -> bool {
        self.is_enabled(LogLevel::Info)
    }
    /// Returns whether warning events are enabled.
    #[must_use]
    pub fn is_warn_enabled(&self) -> bool {
        self.is_enabled(LogLevel::Warn)
    }
    /// Returns whether error events are enabled.
    #[must_use]
    pub fn is_error_enabled(&self) -> bool {
        self.is_enabled(LogLevel::Error)
    }
}

impl Log for AbstractLog {
    fn name(&self) -> &str {
        &self.name
    }
    fn is_enabled(&self, level: LogLevel) -> bool {
        self.sink.enabled(&self.name, level)
    }
    fn log_record(&self, record: LogRecord) {
        if self.is_enabled(record.level) {
            self.sink.emit(&record);
        }
    }
}
