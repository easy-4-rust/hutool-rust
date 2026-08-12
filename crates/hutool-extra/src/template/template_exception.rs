//! 模板引擎配置 facade。
//!
//! 1:1 对齐 hutool 的 `cn.hutool.extra.template.*` 包（不含各 template engine 适配）。
//!
//! - 原 Java 包：`cn.hutool.extra.template`
//! - 本文件覆盖：`TemplateConfig`、`ResourceMode`、`TemplateException`、`Template` interface、
//!   `AbstractTemplate` 抽象类、`TemplateUtil` 静态门面、`TemplateEngine` 接口
//! - 各 engine 适配（Beetl / Enjoy / Freemarker / Jetbrick / Rythm / Thymeleaf / Velocity / Wit）
//!   在各自的 `engine/<name>.rs` 子模块；本文件只提供配置 + facade。
//! - 迁移状态：✅ 已实现（Phase 1.4 工作）

use std::fmt;

use thiserror::Error;

/// 模板异常，对齐 `cn.hutool.extra.template.TemplateException`。
///
/// Java 继承 `RuntimeException`；Rust 用 `thiserror::Error` enum 实现。
#[derive(Debug, Error)]
pub enum TemplateException {
    /// 对齐 `TemplateException(String message)`
    #[error("{0}")]
    Message(String),

    /// 对齐 `TemplateException(Throwable e)`（cause 链）
    #[error("{message}")]
    WithCause {
        /// 错误消息文本
        message: String,
        /// 底层错误来源
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// 对齐 `TemplateException(String messageTemplate, Object... params)`（带格式化的 message）
    #[error("{message}")]
    Formatted {
        /// 格式化后的消息文本
        message: String,
    },

    /// 对齐 `TemplateException(String message, Throwable throwable)`
    #[error("{message}")]
    WithThrowable {
        /// 错误消息文本
        message: String,
        /// 底层错误来源
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    /// 对齐 `TemplateException(Throwable throwable, String messageTemplate, Object... params)`
    #[error("{message}")]
    FormattedWithCause {
        /// 格式化后的消息文本
        message: String,
        /// 底层错误来源
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl TemplateException {
    /// 格式化 message，参考 `StrUtil.format`（Phase 1.4 完成后用 `hutool-core::StrUtil.format`）。
    fn format_message(template: &str, params: &[&dyn fmt::Display]) -> String {
        let mut out = String::new();
        let mut param_idx = 0;
        let mut chars = template.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '{' && chars.peek() == Some(&'}') {
                chars.next();
                if param_idx < params.len() {
                    out.push_str(&format!("{}", params[param_idx]));
                    param_idx += 1;
                }
            } else {
                out.push(c);
            }
        }
        out
    }

    /// 对齐 `TemplateException(Throwable e)`
    pub fn from_error<E>(e: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        TemplateException::WithCause {
            message: e.to_string(),
            source: Box::new(e),
        }
    }

    /// 对齐 `TemplateException(String message)`
    pub fn new<S: Into<String>>(message: S) -> Self {
        TemplateException::Message(message.into())
    }

    /// 对齐 `TemplateException(String messageTemplate, Object... params)`
    pub fn formatted<S: AsRef<str>>(template: S, params: &[&dyn fmt::Display]) -> Self {
        TemplateException::Formatted {
            message: Self::format_message(template.as_ref(), params),
        }
    }

    /// 对齐 `TemplateException(String message, Throwable throwable)`
    pub fn with_throwable<S: Into<String>, E>(message: S, throwable: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        TemplateException::WithThrowable {
            message: message.into(),
            source: Box::new(throwable),
        }
    }

    /// 对齐 `TemplateException(Throwable throwable, String messageTemplate, Object... params)`
    pub fn formatted_with_cause<E>(
        throwable: E,
        template: &str,
        params: &[&dyn fmt::Display],
    ) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        TemplateException::FormattedWithCause {
            message: Self::format_message(template, params),
            source: Box::new(throwable),
        }
    }

    /// 返回错误 message（对齐 Java `getMessage()`）
    pub fn message(&self) -> &str {
        match self {
            TemplateException::Message(m)
            | TemplateException::Formatted { message: m }
            | TemplateException::WithCause { message: m, .. }
            | TemplateException::WithThrowable { message: m, .. }
            | TemplateException::FormattedWithCause { message: m, .. } => m,
        }
    }
}
