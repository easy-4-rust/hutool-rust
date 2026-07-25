//! 对齐: `cn.hutool.log.LogSink` (自定义抽象)
//! 来源: hutool-log/src/main/java/cn/hutool/log/
//! 中文说明: 日志后端接收器 trait，所有兼容方言共享的可注入目标接口。

use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, OnceLock, RwLock},
};

use super::log_level::LogLevel;
use super::log_record::LogRecord;

/// 日志后端接收器 trait，对应 Hutool 日志体系中各实现后端的抽象接口。
///
/// 对齐: `cn.hutool.log` 中各 `Log` 实现的输出目标
/// 来源: hutool-log/src/main/java/cn/hutool/log/
///
/// Injectable destination used by all compatibility dialects.
pub trait LogSink: Send + Sync {
    /// Returns whether the destination accepts this category and level.
    fn enabled(&self, _name: &str, _level: LogLevel) -> bool {
        true
    }
    /// Emits one accepted record.
    fn emit(&self, record: &LogRecord);
}

use super::{format_message, global_slot};
