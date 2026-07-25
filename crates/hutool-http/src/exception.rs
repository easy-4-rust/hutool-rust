//! 对齐: `cn.hutool.http.HttpException`
//! 来源: hutool-http/src/main/java/cn/hutool/http/HttpException.java
//! 中文说明: HTTP应用异常类型，用于API对齐时抛出带消息的异常

use std::fmt;

/// Application-facing HTTP exception matching Hutool's constructor surface.
///
/// Transport errors still use [`crate::HttpError`]; this type exists for API
/// parity when callers want an owned message exception.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpException {
    message: String,
}

impl HttpException {
    /// Creates an exception from another error's display text.
    ///
    /// Java: `HttpException(Throwable e)`
    #[must_use]
    pub fn from_error(error: impl fmt::Display) -> Self {
        Self {
            message: error.to_string(),
        }
    }

    /// Creates an exception with a fixed message.
    ///
    /// Java: `HttpException(String message)`
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Formats a message template by replacing each `{}` with the next param.
    ///
    /// Java: `HttpException(String messageTemplate, Object... params)`
    #[must_use]
    pub fn with_params(template: &str, params: &[&str]) -> Self {
        let mut message = template.to_string();
        for param in params {
            if let Some(idx) = message.find("{}") {
                message.replace_range(idx..idx + 2, param);
            }
        }
        Self { message }
    }

    /// Creates an exception with message and cause display text.
    ///
    /// Java: `HttpException(String message, Throwable throwable)`
    #[must_use]
    pub fn with_cause(message: impl Into<String>, cause: impl fmt::Display) -> Self {
        Self {
            message: format!("{}: {}", message.into(), cause),
        }
    }

    /// Java: `HttpException(String message, Throwable, boolean, boolean)` —
    /// suppression flags are ignored in Rust.
    #[must_use]
    pub fn with_cause_flags(
        message: impl Into<String>,
        cause: impl fmt::Display,
        _enable_suppression: bool,
        _writable_stack_trace: bool,
    ) -> Self {
        Self::with_cause(message, cause)
    }

    /// Java: `HttpException(Throwable throwable, String messageTemplate, Object... params)`
    #[must_use]
    pub fn from_cause_with_params(
        cause: impl fmt::Display,
        template: &str,
        params: &[&str],
    ) -> Self {
        let mut base = Self::with_params(template, params);
        base.message = format!("{}: {}", base.message, cause);
        base
    }

    /// Returns the exception message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for HttpException {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for HttpException {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_match_hutool_message_shapes() {
        assert_eq!(HttpException::new("boom").message(), "boom");
        assert_eq!(
            HttpException::with_params("a={} b={}", &["1", "2"]).message(),
            "a=1 b=2"
        );
        assert!(HttpException::from_error("io").to_string().contains("io"));
        assert!(HttpException::with_cause("wrap", "root")
            .message()
            .contains("root"));
        assert!(HttpException::with_cause_flags("wrap", "root", true, false)
            .message()
            .contains("wrap"));
        assert!(HttpException::from_cause_with_params("root", "x={}", &["1"])
            .message()
            .contains("x=1"));
    }
}
