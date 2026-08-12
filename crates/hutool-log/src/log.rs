//! 对齐: `cn.hutool.log.Log`
//! 来源: hutool-log/src/main/java/cn/hutool/log/Log.java
//! 中文说明: 面向对象安全的 Hutool 日志接口 trait，提供各级别日志记录方法。

use std::fmt;

use crate::format_message::format_message;
use crate::level::LogLevel;
use crate::log_record::LogRecord;

/// 面向对象安全的 Hutool 兼容日志接口，对应 Java 的 `cn.hutool.log.Log`。
///
/// 对齐 Java 类: `cn.hutool.log.Log`
/// 来源: hutool-log/src/main/java/cn/hutool/log/Log.java
///
/// Object-safe Hutool-compatible logger contract.
pub trait Log: Send + Sync {
    /// Returns the logger/category name.
    fn name(&self) -> &str;
    /// Returns whether the logger accepts this level.
    fn is_enabled(&self, level: LogLevel) -> bool;
    /// Emits a fully-built record when enabled.
    fn log_record(&self, record: LogRecord);

    /// Logs a plain message at an arbitrary level.
    fn log(&self, level: LogLevel, message: &str) {
        self.log_record(LogRecord::new(self.name(), level, message));
    }
    /// Logs a Hutool-style templated message at an arbitrary level.
    fn log_fmt(&self, level: LogLevel, template: &str, arguments: &[&dyn fmt::Display]) {
        self.log(level, &format_message(template, arguments));
    }
    /// Logs a message and error description.
    fn log_error(&self, level: LogLevel, error: &str, message: &str) {
        self.log_record(LogRecord::new(self.name(), level, message).with_error(error));
    }
    /// Logs a message with facade/caller metadata and an optional error.
    fn log_fqcn(&self, fqcn: &str, level: LogLevel, error: Option<&str>, message: &str) {
        let mut record = LogRecord::new(self.name(), level, message).with_fqcn(fqcn);
        if let Some(error) = error {
            record = record.with_error(error);
        }
        self.log_record(record);
    }
    /// Logs a nullable message; `None` becomes the literal `"null"` (Hutool `null` arg).
    fn log_nullable(&self, level: LogLevel, message: Option<&str>) {
        self.log(level, message.unwrap_or("null"));
    }
    /// Logs a trace message.
    fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message);
    }
    /// Logs a Hutool-style templated trace message.
    fn trace_fmt(&self, template: &str, arguments: &[&dyn fmt::Display]) {
        self.log_fmt(LogLevel::Trace, template, arguments);
    }
    /// Logs a debug message.
    fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }
    /// Logs a Hutool-style templated debug message.
    fn debug_fmt(&self, template: &str, arguments: &[&dyn fmt::Display]) {
        self.log_fmt(LogLevel::Debug, template, arguments);
    }
    /// Logs an informational message.
    fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }
    /// Logs a Hutool-style templated informational message.
    fn info_fmt(&self, template: &str, arguments: &[&dyn fmt::Display]) {
        self.log_fmt(LogLevel::Info, template, arguments);
    }
    /// Logs a warning message.
    fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }
    /// Logs a Hutool-style templated warning message.
    fn warn_fmt(&self, template: &str, arguments: &[&dyn fmt::Display]) {
        self.log_fmt(LogLevel::Warn, template, arguments);
    }
    /// Logs an error message.
    fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
    /// Logs a Hutool-style templated error message.
    fn error_fmt(&self, template: &str, arguments: &[&dyn fmt::Display]) {
        self.log_fmt(LogLevel::Error, template, arguments);
    }
    /// Logs an error message with a separate throwable/error description.
    fn error_with_cause(&self, message: &str, error: &str) {
        self.log_error(LogLevel::Error, error, message);
    }
}
