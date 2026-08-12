//! 对齐: `cn.hutool.log.LogRecord`
//! 来源: hutool-log/src/main/java/cn/hutool/log/LogRecord.java
//! 中文说明: 后端无关的日志事件记录，包含名称、级别、消息和可选的错误描述。

use crate::level::LogLevel;

/// 后端无关的日志事件记录，对应 Hutool 的 `LogRecord`。
///
/// 对齐 Java 类: `cn.hutool.log.LogRecord`
/// 来源: hutool-log/src/main/java/cn/hutool/log/LogRecord.java
///
/// A backend-neutral logging event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogRecord {
    /// Logger/category name.
    pub name: String,
    /// Event severity.
    pub level: LogLevel,
    /// Rendered event message.
    pub message: String,
    /// Optional error description.
    pub error: Option<String>,
    /// Optional fully-qualified facade or caller name.
    pub fqcn: Option<String>,
}

impl LogRecord {
    /// Creates a record with owned data suitable for asynchronous sinks.
    #[must_use]
    pub fn new(name: &str, level: LogLevel, message: &str) -> Self {
        Self {
            name: name.to_owned(),
            level,
            message: message.to_owned(),
            error: None,
            fqcn: None,
        }
    }

    /// Attaches an error description.
    #[must_use]
    pub fn with_error(mut self, error: &str) -> Self {
        self.error = Some(error.to_owned());
        self
    }

    /// Attaches the original facade/caller name.
    #[must_use]
    pub fn with_fqcn(mut self, fqcn: &str) -> Self {
        self.fqcn = Some(fqcn.to_owned());
        self
    }
}
