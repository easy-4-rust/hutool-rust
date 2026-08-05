//! 对齐: `cn.hutool.core.io.IORuntimeException`
//! 来源: hutool-core/src/main/java/cn/hutool/core/io/IORuntimeException.java

use std::error::Error as StdError;
use std::fmt;
use std::io;

/// 对齐 Java 异常类: `cn.hutool.core.io.IORuntimeException`
#[derive(Debug)]
pub struct IORuntimeException {
    /// 错误消息。
    pub message: String,
    /// 可选根因。
    pub source: Option<io::Error>,
}

impl IORuntimeException {
    /// 对齐 Java: `IORuntimeException(String)`
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// 对齐 Java: `IORuntimeException(Throwable)`
    pub fn from_io(err: io::Error) -> Self {
        Self {
            message: err.to_string(),
            source: Some(err),
        }
    }

    /// 对齐 Java: `IORuntimeException(String, Throwable)`
    pub fn with_cause(message: impl Into<String>, err: io::Error) -> Self {
        Self {
            message: message.into(),
            source: Some(err),
        }
    }

    /// 对齐 Java: `IORuntimeException(Throwable, String, Object...)` — 格式化消息。
    pub fn with_format(err: io::Error, message: impl Into<String>) -> Self {
        Self::with_cause(message, err)
    }

    /// 对齐 Java: `causeInstanceOf(Class)` — 根因类型名匹配（Rust 侧按 kind 近似）。
    pub fn cause_instance_of(&self, kind: io::ErrorKind) -> bool {
        self.source.as_ref().is_some_and(|e| e.kind() == kind)
    }
}

impl fmt::Display for IORuntimeException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IORuntimeException: {}", self.message)
    }
}

impl StdError for IORuntimeException {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|e| e as &(dyn StdError + 'static))
    }
}

impl From<io::Error> for IORuntimeException {
    fn from(value: io::Error) -> Self {
        Self::from_io(value)
    }
}
