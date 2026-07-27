//! 对齐: `cn.hutool.log` 后端适配（tracing 桥接）
//! 来源: hutool-log/src/main/java/cn/hutool/log/
//! 中文说明: 基于 Rust tracing 生产级后端的 LogSink 实现，将 Hutool 日志事件桥接到 tracing 宏。

use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, OnceLock, RwLock},
};

use crate::level::LogLevel;
use crate::log_record::LogRecord;
use crate::log_sink::LogSink;

/// 基于 tracing 生态的生产级 LogSink 实现。
///
/// 对齐: `cn.hutool.log` 中各后端的输出实现
/// 来源: hutool-log/src/main/java/cn/hutool/log/
///
/// Production sink backed by the mature `tracing` ecosystem.
#[derive(Debug, Default)]
pub struct TracingSink;

impl LogSink for TracingSink {
    fn enabled(&self, _name: &str, level: LogLevel) -> bool {
        match level {
            LogLevel::All | LogLevel::Trace | LogLevel::Debug => {
                tracing::enabled!(target: "hutool", tracing::Level::TRACE)
            }
            LogLevel::Info => tracing::enabled!(target: "hutool", tracing::Level::INFO),
            LogLevel::Warn => tracing::enabled!(target: "hutool", tracing::Level::WARN),
            LogLevel::Error | LogLevel::Fatal => {
                tracing::enabled!(target: "hutool", tracing::Level::ERROR)
            }
            LogLevel::Off => false,
        }
    }

    fn emit(&self, record: &LogRecord) {
        let error = record.error.as_deref().unwrap_or_default();
        let fqcn = record.fqcn.as_deref().unwrap_or_default();
        match record.level {
            LogLevel::All | LogLevel::Trace => {
                tracing::trace!(target: "hutool", logger = %record.name, error, fqcn, "{}", record.message);
            }
            LogLevel::Debug => {
                tracing::debug!(target: "hutool", logger = %record.name, error, fqcn, "{}", record.message);
            }
            LogLevel::Info => {
                tracing::info!(target: "hutool", logger = %record.name, error, fqcn, "{}", record.message);
            }
            LogLevel::Warn => {
                tracing::warn!(target: "hutool", logger = %record.name, error, fqcn, "{}", record.message);
            }
            LogLevel::Error | LogLevel::Fatal => {
                tracing::error!(target: "hutool", logger = %record.name, error, fqcn, "{}", record.message);
            }
            LogLevel::Off => {}
        }
    }
}
