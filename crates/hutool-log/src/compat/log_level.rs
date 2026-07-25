//! 对齐: `cn.hutool.log.Level`
//! 来源: hutool-log/src/main/java/cn/hutool/log/Level.java
//! 中文说明: Hutool 五级可移植日志级别枚举（Trace/Debug/Info/Warn/Error）。

use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, OnceLock, RwLock},
};

/// Hutool 的五级可移植日志级别，对应 Java 的 `cn.hutool.log.Level`。
///
/// 对齐 Java 类: `cn.hutool.log.Level`
/// 来源: hutool-log/src/main/java/cn/hutool/log/Level.java
///
/// Hutool's five portable logging levels.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogLevel {
    /// Highly detailed diagnostic events.
    #[default]
    Trace,
    /// Developer-oriented diagnostic events.
    Debug,
    /// Normal application events.
    Info,
    /// Recoverable or potentially harmful events.
    Warn,
    /// Failed operations.
    Error,
}

impl fmt::Display for LogLevel {
    /// Formats as the uppercase Hutool `Level` enum name (`DEBUG`, `INFO`, …).
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Trace => "TRACE",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
        })
    }
}

use super::{format_message, global_slot};
