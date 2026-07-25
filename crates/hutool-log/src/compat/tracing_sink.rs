//! 对齐: `cn.hutool.log` 后端适配（tracing 桥接）
//! 来源: hutool-log/src/main/java/cn/hutool/log/
//! 中文说明: 基于 Rust tracing 生产级后端的 LogSink 实现，将 Hutool 日志事件桥接到 tracing 宏。

use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, OnceLock, RwLock},
};

use super::log_level::LogLevel;
use super::log_record::LogRecord;
use super::log_sink::LogSink;

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
            LogLevel::Trace => tracing::enabled!(target: "hutool", tracing::Level::TRACE),
            LogLevel::Debug => tracing::enabled!(target: "hutool", tracing::Level::DEBUG),
            LogLevel::Info => tracing::enabled!(target: "hutool", tracing::Level::INFO),
            LogLevel::Warn => tracing::enabled!(target: "hutool", tracing::Level::WARN),
            LogLevel::Error => tracing::enabled!(target: "hutool", tracing::Level::ERROR),
        }
    }

    fn emit(&self, record: &LogRecord) {
        let error = record.error.as_deref().unwrap_or_default();
        let fqcn = record.fqcn.as_deref().unwrap_or_default();
        match record.level {
            LogLevel::Trace => {
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
            LogLevel::Error => {
                tracing::error!(target: "hutool", logger = %record.name, error, fqcn, "{}", record.message);
            }
        }
    }
}

use super::{format_message, global_slot};
