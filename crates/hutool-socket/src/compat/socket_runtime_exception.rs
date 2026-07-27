//! `SocketRuntimeException` 的 Rust 对齐实现。
//!
//! Java 来源：`cn.hutool.socket.SocketRuntimeException`
//! 作用：保持 Hutool 风格的运行时异常入口，并补充 Rust 侧 `io::Error`
//! 与 `SocketError` 的桥接能力。

use std::fmt;
use std::io;

use crate::SocketError;

/// Socket 异常，保留 Hutool Java 的命名形态以降低迁移成本。
#[derive(Debug)]
pub struct SocketRuntimeException {
    message: String,
    source: Option<io::Error>,
}

impl SocketRuntimeException {
    /// 使用纯文本消息创建异常。
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// 使用消息和底层 I/O 异常创建异常。
    #[must_use]
    pub fn with_source(message: impl Into<String>, source: io::Error) -> Self {
        Self {
            message: message.into(),
            source: Some(source),
        }
    }

    /// 按 Hutool `StrUtil.format` 风格顺序替换 `{}` 占位符。
    #[must_use]
    pub fn formatted(template: &str, values: &[&dyn fmt::Display]) -> Self {
        let mut message = String::with_capacity(template.len());
        let mut rest = template;
        for value in values {
            if let Some(index) = rest.find("{}") {
                message.push_str(&rest[..index]);
                message.push_str(&value.to_string());
                rest = &rest[index + 2..];
            } else {
                break;
            }
        }
        message.push_str(rest);
        Self::new(message)
    }
}

impl fmt::Display for SocketRuntimeException {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for SocketRuntimeException {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| source as &(dyn std::error::Error + 'static))
    }
}

impl From<io::Error> for SocketRuntimeException {
    fn from(error: io::Error) -> Self {
        Self::with_source(error.to_string(), error)
    }
}

impl From<SocketError> for SocketRuntimeException {
    fn from(error: SocketError) -> Self {
        Self::new(error.to_string())
    }
}
