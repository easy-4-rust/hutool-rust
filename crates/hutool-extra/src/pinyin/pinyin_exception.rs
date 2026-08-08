//! Pinyin helpers aligned with Hutool `PinyinUtil`, backed by the `pinyin` crate.

use hutool_core::format_template;
use std::fmt::Display;

/// Error type matching Hutool `PinyinException` messaging.
///
/// Java: `cn.hutool.extra.pinyin.PinyinException`（`RuntimeException` 子类，
/// 6 个构造器）。Rust 侧以 thiserror 结构体承载（消息 + 可选根因）。
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct PinyinException {
    /// 错误消息。
    pub message: String,
    /// 可选原因。
    #[source]
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl PinyinException {
    /// Java: `new PinyinException(String message)`
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// Java: `new PinyinException(String messageTemplate, Object... params)` — 模板消息。
    #[must_use]
    pub fn with_template(template: &str, params: &[&dyn Display]) -> Self {
        Self::new(format_template(template, params))
    }

    /// Java: `new PinyinException(Throwable e)` — 包装根因，消息取根因消息。
    #[must_use]
    pub fn from_cause(cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            message: cause.to_string(),
            source: Some(Box::new(cause)),
        }
    }

    /// Java: `new PinyinException(String message, Throwable throwable)` —
    /// 消息 + 根因（Java 的 `enableSuppression`/`writableStackTrace` 为 JVM 特性，Rust 无对应）。
    #[must_use]
    pub fn with_cause(
        message: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(cause)),
        }
    }

    /// Java: `new PinyinException(Throwable throwable, String messageTemplate, Object... params)` —
    /// 模板消息 + 根因。
    #[must_use]
    pub fn with_cause_template(
        cause: impl std::error::Error + Send + Sync + 'static,
        template: &str,
        params: &[&dyn Display],
    ) -> Self {
        Self {
            message: format_template(template, params),
            source: Some(Box::new(cause)),
        }
    }

    /// Returns the error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinyin_exception_constructors_match_java() {
        // PinyinException(String)
        let plain = PinyinException::new("no engine");
        assert_eq!(plain.to_string(), "no engine");
        assert!(plain.source.is_none());

        // PinyinException(String, Object...)
        let templated = PinyinException::with_template("jar {} missing", &[&"tinypinyin"]);
        assert_eq!(templated.to_string(), "jar tinypinyin missing");

        // PinyinException(Throwable)
        let cause = std::io::Error::new(std::io::ErrorKind::NotFound, "lib absent");
        let wrapped = PinyinException::from_cause(cause);
        assert!(wrapped.to_string().contains("lib absent"));
        assert!(wrapped.source.is_some());

        // PinyinException(String, Throwable)
        let cause2 = std::io::Error::other("inner");
        let with_cause = PinyinException::with_cause("outer", cause2);
        assert_eq!(with_cause.to_string(), "outer");
        assert!(with_cause.source.is_some());

        // PinyinException(Throwable, String, Object...)
        let cause3 = std::io::Error::other("inner2");
        let templated_cause =
            PinyinException::with_cause_template(cause3, "engine {} failed", &[&"pinyin4j"]);
        assert_eq!(templated_cause.to_string(), "engine pinyin4j failed");
        assert!(templated_cause.source.is_some());

        assert_eq!(templated_cause.message(), "engine pinyin4j failed");
    }
}
