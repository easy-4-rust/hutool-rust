use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, OnceLock, RwLock},
};

use crate::global_log_factory::GlobalLogFactory;
use crate::log::Log;
use crate::level::LogLevel;

/// Static facade matching Hutool's convenience entry points.
pub struct StaticLog;

impl StaticLog {
    /// Gets or creates a logger from the compatibility global.
    #[must_use]
    pub fn get(name: &str) -> Arc<dyn Log> {
        GlobalLogFactory::get().get(name)
    }
    /// 对齐 Java: `Log.get()` / `StaticLog.get()` — caller-inferred name becomes `"default"`.
    #[must_use]
    pub fn get_default() -> Arc<dyn Log> {
        GlobalLogFactory::get().get("default")
    }
    /// Logs a message at an arbitrary level.
    pub fn log(level: LogLevel, message: &str) {
        Self::get("static").log(level, message);
    }
    /// Logs a Hutool-style templated message at an arbitrary level.
    pub fn log_fmt(level: LogLevel, template: &str, arguments: &[&dyn fmt::Display]) {
        Self::get("static").log_fmt(level, template, arguments);
    }
    /// Logs a trace message.
    pub fn trace(message: &str) {
        Self::log(LogLevel::Trace, message);
    }
    /// 对齐 Java: `StaticLog.trace(String, Object...)`.
    pub fn trace_fmt(template: &str, arguments: &[&dyn fmt::Display]) {
        Self::log_fmt(LogLevel::Trace, template, arguments);
    }
    /// Logs a debug message.
    pub fn debug(message: &str) {
        Self::log(LogLevel::Debug, message);
    }
    /// 对齐 Java: `StaticLog.debug(String, Object...)`.
    pub fn debug_fmt(template: &str, arguments: &[&dyn fmt::Display]) {
        Self::log_fmt(LogLevel::Debug, template, arguments);
    }
    /// Logs an informational message.
    pub fn info(message: &str) {
        Self::log(LogLevel::Info, message);
    }
    /// 对齐 Java: `StaticLog.info(String, Object...)`.
    pub fn info_fmt(template: &str, arguments: &[&dyn fmt::Display]) {
        Self::log_fmt(LogLevel::Info, template, arguments);
    }
    /// Logs a warning message.
    pub fn warn(message: &str) {
        Self::log(LogLevel::Warn, message);
    }
    /// 对齐 Java: `StaticLog.warn(String, Object...)`.
    pub fn warn_fmt(template: &str, arguments: &[&dyn fmt::Display]) {
        Self::log_fmt(LogLevel::Warn, template, arguments);
    }
    /// Logs an error message.
    pub fn error(message: &str) {
        Self::log(LogLevel::Error, message);
    }
    /// 对齐 Java: `StaticLog.error(String, Object...)`.
    pub fn error_fmt(template: &str, arguments: &[&dyn fmt::Display]) {
        Self::log_fmt(LogLevel::Error, template, arguments);
    }
}
