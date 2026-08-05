//! `AIException` 对象。
//! 对齐 Java 来源: `cn.hutool.ai.AIException`
//!
//! 异常处理类，Rust 侧使用 `thiserror` 提供与 Java 等价的多种构造方式。

use std::fmt;
use thiserror::Error;

/// AI 模块统一异常类型。
///
/// 对齐 Java 来源: `cn.hutool.ai.AIException`
#[derive(Debug, Error)]
pub enum AIException {
    /// 包装底层异常，携带 message。
    #[error("{message}")]
    Wrapped {
        /// 渲染后的消息。
        message: String,
        /// 原始异常来源。
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    /// 仅携带消息。
    #[error("{0}")]
    Message(String),
}

impl AIException {
    /// 包装任意错误。
    pub fn from_error(error: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Wrapped {
            message: error.to_string(),
            source: Some(Box::new(error)),
        }
    }

    /// 使用格式化模板 + 参数构造异常。
    #[must_use]
    pub fn formatted(template: &str, params: &[&dyn fmt::Display]) -> Self {
        Self::Message(format_template(template, params))
    }

    /// 携带原因异常的模板格式化构造。
    pub fn formatted_with(
        template: &str,
        params: &[&dyn fmt::Display],
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        let message = format_template(template, params);
        let wrapped = cause.to_string();
        Self::Message(format!("{message}: {wrapped}"))
    }
}

impl From<std::io::Error> for AIException {
    fn from(value: std::io::Error) -> Self {
        Self::from_error(value)
    }
}

impl From<serde_json::Error> for AIException {
    fn from(value: serde_json::Error) -> Self {
        Self::from_error(value)
    }
}

impl From<url::ParseError> for AIException {
    fn from(value: url::ParseError) -> Self {
        Self::from_error(value)
    }
}

impl From<hutool_http::HttpError> for AIException {
    fn from(value: hutool_http::HttpError) -> Self {
        Self::from_error(value)
    }
}

/// 简易 `{0}` 风格的占位符替换，兼容 Java `StrUtil.format` 的常见用法。
fn format_template(template: &str, params: &[&dyn fmt::Display]) -> String {
    let mut output = String::with_capacity(template.len());
    let mut chars = template.chars().peekable();
    let mut index = 0usize;
    while let Some(c) = chars.next() {
        if c == '{' && chars.peek() == Some(&'}') {
            chars.next();
            if let Some(value) = params.get(index) {
                output.push_str(&value.to_string());
            } else {
                output.push_str("{}");
            }
            index += 1;
        } else {
            output.push(c);
        }
    }
    output
}

impl From<&str> for AIException {
    fn from(value: &str) -> Self {
        Self::Message(value.into())
    }
}

impl From<String> for AIException {
    fn from(value: String) -> Self {
        Self::Message(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructors_render_messages_and_preserve_cause() {
        let message = AIException::Message("boom".into());
        assert_eq!(message.to_string(), "boom");

        let formatted = AIException::formatted("hello {}", &[&"world"]);
        assert_eq!(formatted.to_string(), "hello world");

        let missing = AIException::formatted("hello {}", &[]);
        assert_eq!(missing.to_string(), "hello {}");

        let io = std::io::Error::new(std::io::ErrorKind::Other, "io-error");
        let wrapped = AIException::from_error(io);
        assert!(wrapped.to_string().contains("io-error"));

        let cause = std::io::Error::new(std::io::ErrorKind::Other, "io");
        let combined = AIException::formatted_with("fail {}", &[&"retry"], cause);
        assert!(combined.to_string().contains("fail retry"));
        assert!(combined.to_string().contains("io"));
    }

    #[test]
    fn from_conversions_preserve_message_and_source() {
        let io = std::io::Error::new(std::io::ErrorKind::NotFound, "io-err");
        let from_io: AIException = io.into();
        assert!(from_io.to_string().contains("io-err"));

        let from_json: AIException = serde_json::from_str::<serde_json::Value>("{")
            .unwrap_err()
            .into();
        assert!(!from_json.to_string().is_empty());

        let from_url: AIException = url::Url::parse("not a url").unwrap_err().into();
        assert!(!from_url.to_string().is_empty());

        let from_str: AIException = "direct".into();
        assert_eq!(from_str.to_string(), "direct");
        let from_string: AIException = String::from("owned").into();
        assert_eq!(from_string.to_string(), "owned");
    }
}
