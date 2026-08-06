//! 对齐: `cn.hutool.cron.CronException`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/CronException.java
//! 中文说明: Cron 模块统一异常类。Java 侧为 `RuntimeException` 子类，
//! Rust 侧以 thiserror 结构体承载 5 个构造器语义（消息 + 可选根因）。

use hutool_core::format_template;
use std::fmt::Display;

/// 对齐: `cn.hutool.cron.CronException`
/// 中文说明: Cron 模块异常，支持模板消息与根因包装。
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct CronException {
    /// 错误消息。
    pub message: String,
    /// 可选原因。
    #[source]
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl CronException {
    /// 对齐 Java: `CronException(String)` — 仅消息。
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// 对齐 Java: `CronException(String, Object...)` — 模板消息。
    pub fn with_template(template: &str, params: &[&dyn Display]) -> Self {
        Self::new(format_template(template, params))
    }

    /// 对齐 Java: `CronException(Throwable)` — 包装根因，消息取根因消息。
    pub fn from_cause(cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            message: cause.to_string(),
            source: Some(Box::new(cause)),
        }
    }

    /// 对齐 Java: `CronException(String, Throwable)` —
    /// 消息 + 根因（Java 的 `enableSuppression`/`writableStackTrace` 为 JVM 特性，Rust 无对应）。
    pub fn with_cause(
        message: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(cause)),
        }
    }

    /// 对齐 Java: `CronException(Throwable, String, Object...)` —
    /// 模板消息 + 根因。
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cron_exception_constructors_match_java() {
        // CronException(String)
        let plain = CronException::new("boom");
        assert_eq!(plain.to_string(), "boom");
        assert!(plain.source.is_none());

        // CronException(String, Object...)
        let templated = CronException::with_template("id [{}] exists", &[&"job-1"]);
        assert_eq!(templated.to_string(), "id [job-1] exists");

        // CronException(Throwable)
        let cause = std::io::Error::other("io-caused");
        let wrapped = CronException::from_cause(cause);
        assert!(wrapped.to_string().contains("io-caused"));
        assert!(wrapped.source.is_some());

        // CronException(String, Throwable)
        let cause2 = std::io::Error::other("inner");
        let with_cause = CronException::with_cause("outer failed", cause2);
        assert_eq!(with_cause.to_string(), "outer failed");
        assert!(with_cause.source.is_some());

        // CronException(Throwable, String, Object...)
        let cause3 = std::io::Error::other("inner2");
        let templated_with_cause =
            CronException::with_cause_template(cause3, "task {} failed", &[&"t1"]);
        assert_eq!(templated_with_cause.to_string(), "task t1 failed");
        assert!(templated_with_cause.source.is_some());

        // 错误链可经 std::error::Error 遍历
        let chain: Box<dyn std::error::Error> = Box::new(templated_with_cause);
        assert!(chain.source().is_some());
    }
}
