//! 对齐: `cn.hutool.log.level.Level`
//! 来源: hutool-log/src/main/java/cn/hutool/log/level/Level.java
//!
//! 中文说明: Hutool 日志级别枚举，对齐 Java 的 `cn.hutool.log.level.Level`。
//! Rust 端常用别名 `LogLevel`（顶级 `crate::LogLevel`，与 Java `cn.hutool.log.LogLevel` 兼容），
//! 同时本路径下保留 `Level` 作为 `cn.hutool.log.level.Level` 的 1:1 命名。
//!
//! Hutool's portable logging levels.

use std::fmt;

/// Hutool 的可移植日志级别枚举，对齐 Java 类 `cn.hutool.log.level.Level`。
///
/// 对齐 Java 类: `cn.hutool.log.level.Level`
/// 来源: hutool-log/src/main/java/cn/hutool/log/level/Level.java
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Level {
    /// 'ALL' log level. Java `Level.ALL`
    All,
    /// 'TRACE' log level. Java `Level.TRACE`
    #[default]
    Trace,
    /// 'DEBUG' log level. Java `Level.DEBUG`
    Debug,
    /// 'INFO' log level. Java `Level.INFO`
    Info,
    /// 'WARN' log level. Java `Level.WARN`
    Warn,
    /// 'ERROR' log level. Java `Level.ERROR`
    Error,
    /// 'FATAL' log level. Java `Level.FATAL`
    Fatal,
    /// 'OFF'. Java `Level.OFF`
    Off,
}

/// 别名：保留 `cn.hutool.log.LogLevel` Java 风格的命名空间引用，
/// 同时提供 `crate::LogLevel` 顶层别名（与 Java `cn.hutool.log.LogLevel` 等价）。
pub type LogLevel = Level;

impl fmt::Display for Level {
    /// Formats as the uppercase Hutool `Level` enum name (`DEBUG`, `INFO`, ...).
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::All => "ALL",
            Self::Trace => "TRACE",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
            Self::Fatal => "FATAL",
            Self::Off => "OFF",
        })
    }
}
