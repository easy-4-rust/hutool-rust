//! 统一实现 hutool-extra 6 个 Exception 类型。
//!
//! 每个 hutool-extra Exception 都是同样的 6 个构造器模式：
//!   - `(Throwable e)` → 包装底层错误
//!   - `(String message)` → 简单消息
//!   - `(String messageTemplate, Object... params)` → 模板格式化
//!   - `(String message, Throwable throwable)` → 消息 + 错误
//!   - `(String message, Throwable throwable, boolean, boolean)` → 完整控制
//!   - `(Throwable throwable, String messageTemplate, Object... params)` → 模板 + 错误
//!
//! 使用宏 `define_hutool_exception!` 批量实现，避免重复代码。

use thiserror::Error;

use super::mail_exception::MailException;
use super::qr_code_exception::QrCodeException;

/// 通用 hutool Exception 错误枚举，对齐 `cn.hutool.extra.X.XxxException` 的 6 构造器模式。
///
/// 各具体 Exception 类型（MailException、QrCodeException 等）都是这个 enum 的 re-export 别名。
#[derive(Debug, Error)]
pub enum HutoolException {
    /// 对齐 `(Throwable e)`
    #[error("{message}")]
    FromCause {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// 对齐 `(String message)`
    #[error("{0}")]
    Message(String),

    /// 对齐 `(String messageTemplate, Object... params)`
    #[error("{message}")]
    Formatted { message: String },

    /// 对齐 `(String message, Throwable throwable)`
    #[error("{message}")]
    WithThrowable {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// 对齐 `(String message, Throwable throwable, boolean, boolean)`
    /// Rust 没有 suppression/writableStackTrace 直接对应，使用 Box 包装
    #[error("{message}")]
    Full {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// 对齐 `(Throwable throwable, String messageTemplate, Object... params)`
    #[error("{message}")]
    FormattedWithCause {
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl HutoolException {
    /// 对齐 `XxxException(Throwable e)`
    pub fn from_cause<E>(e: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        HutoolException::FromCause {
            message: e.to_string(),
            source: Box::new(e),
        }
    }

    /// 对齐 `XxxException(String message)`
    pub fn message<S: Into<String>>(msg: S) -> Self {
        HutoolException::Message(msg.into())
    }

    /// 对齐 `XxxException(String messageTemplate, Object... params)`
    pub fn formatted<S: AsRef<str>>(template: S, params: &[&dyn std::fmt::Display]) -> Self {
        HutoolException::Formatted {
            message: format_message(template.as_ref(), params),
        }
    }

    /// 对齐 `XxxException(String message, Throwable throwable)`
    pub fn with_throwable<S: Into<String>, E>(message: S, throwable: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        HutoolException::WithThrowable {
            message: message.into(),
            source: Box::new(throwable),
        }
    }

    /// 对齐 `XxxException(String message, Throwable throwable, boolean, boolean)`
    pub fn full<S: Into<String>, E>(message: S, throwable: E, _suppression: bool, _writable: bool) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        HutoolException::Full {
            message: message.into(),
            source: Box::new(throwable),
        }
    }

    /// 对齐 `XxxException(Throwable throwable, String messageTemplate, Object... params)`
    pub fn formatted_with_cause<E>(
        throwable: E,
        template: &str,
        params: &[&dyn std::fmt::Display],
    ) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        HutoolException::FormattedWithCause {
            message: format_message(template, params),
            source: Box::new(throwable),
        }
    }

    /// 获取消息字符串（对齐 `Throwable.getMessage()`）
    pub fn get_message(&self) -> &str {
        match self {
            HutoolException::FromCause { message, .. }
            | HutoolException::Formatted { message }
            | HutoolException::WithThrowable { message, .. }
            | HutoolException::Full { message, .. }
            | HutoolException::FormattedWithCause { message, .. } => message,
            HutoolException::Message(m) => m,
        }
    }
}

fn format_message(template: &str, params: &[&dyn std::fmt::Display]) -> String {
    let mut out = String::new();
    let mut idx = 0;
    let mut chars = template.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '{' && chars.peek() == Some(&'}') {
            chars.next();
            if idx < params.len() {
                use std::fmt::Write;
                let _ = write!(out, "{}", params[idx]);
                idx += 1;
            }
        } else {
            out.push(c);
        }
    }
    out
}
