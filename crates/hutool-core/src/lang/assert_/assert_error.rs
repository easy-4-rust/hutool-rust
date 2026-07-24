//! 对齐: `cn.hutool.core.lang.Assert`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Assert.java
//!
//! Rust 版本以 [`Result`] + [`AssertError`] 表达 Java 的断言失败抛出；
//! 成功时返回被检查值，便于链式调用。

use std::collections::HashMap;
use std::fmt::{Display, Write};

use crate::string::{format_template, is_blank};

/// 对齐 Java: `IllegalArgumentException` / `IllegalStateException` 断言失败。
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{message}")]
pub struct AssertError {
    /// 错误消息。
    pub message: String,
    /// 是否对应 Java `IllegalStateException`（`state` 系列）。
    pub is_state: bool,
}

impl AssertError {
    /// 创建参数类断言错误。
    pub fn argument(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            is_state: false,
        }
    }

    /// 创建状态类断言错误。
    pub fn state(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            is_state: true,
        }
    }
}

impl From<String> for AssertError {
    fn from(message: String) -> Self {
        Self::argument(message)
    }
}

impl From<&str> for AssertError {
    fn from(message: &str) -> Self {
        Self::argument(message)
    }
}
