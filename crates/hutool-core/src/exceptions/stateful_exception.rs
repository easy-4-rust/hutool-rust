//! 对齐: `cn.hutool.core.exceptions.StatefulException`
//! 来源: hutool-core/src/main/java/cn/hutool/core/exceptions/StatefulException.java

use crate::string::format_template;
use std::fmt::Display;

/// 对齐 Java: `cn.hutool.core.exceptions.StatefulException`
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct StatefulException {
    /// 对齐 Java: `status` 状态码。
    pub status: i32,
    /// 错误消息。
    pub message: String,
    /// 可选原因。
    #[source]
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl StatefulException {
    /// 对齐 Java: `StatefulException()`
    pub fn empty() -> Self {
        Self {
            status: 0,
            message: String::new(),
            source: None,
        }
    }

    /// 对齐 Java: `StatefulException(String)`
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            status: 0,
            message: message.into(),
            source: None,
        }
    }

    /// 对齐 Java: `StatefulException(String, Object...)`
    pub fn with_template(template: &str, params: &[&dyn Display]) -> Self {
        Self::new(format_template(template, params))
    }

    /// 对齐 Java: `StatefulException(Throwable)`
    pub fn from_cause(cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            status: 0,
            message: cause.to_string(),
            source: Some(Box::new(cause)),
        }
    }

    /// 对齐 Java: `StatefulException(String, Throwable)`
    pub fn with_cause(
        message: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            status: 0,
            message: message.into(),
            source: Some(Box::new(cause)),
        }
    }

    /// 对齐 Java: `StatefulException(int, String)`
    pub fn with_status(status: i32, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
            source: None,
        }
    }

    /// 对齐 Java: `StatefulException(int, Throwable)`
    pub fn with_status_cause(
        status: i32,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            status,
            message: cause.to_string(),
            source: Some(Box::new(cause)),
        }
    }

    /// 对齐 Java: `StatefulException(int, String, Throwable)`
    pub fn with_status_message_cause(
        status: i32,
        message: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            status,
            message: message.into(),
            source: Some(Box::new(cause)),
        }
    }

    /// 对齐 Java: `getStatus()`
    pub fn get_status(&self) -> i32 {
        self.status
    }
}
