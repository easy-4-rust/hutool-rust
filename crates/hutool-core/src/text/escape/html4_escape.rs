//! 对齐: `cn.hutool.core.text.escape.Html4Escape`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/escape/Html4Escape.java
//!
//! 委托 `EscapeUtil::escape_html4`，避免与 util 层实现分叉。

use crate::Result;
use crate::escape_util::EscapeUtil;

/// 对齐 Java: `Html4Escape#`
#[derive(Debug, Clone, Copy, Default)]
pub struct Html4Escape;

impl Html4Escape {
    /// 对齐 Java: `Html4Escape()` — 构造转义器实例。
    pub fn new() -> Self {
        Self
    }

    /// 对齐 Java: `Html4Escape` / `StrReplacer.replace` 语义 — HTML4 转义。
    ///
    /// 来源: hutool-core/.../escape/Html4Escape.java（内部 LookupReplacer 链）
    pub fn escape(text: &str) -> Result<String> {
        Ok(EscapeUtil::escape_html4(text))
    }

    /// 实例方法形式，行为同 [`Self::escape`]。
    pub fn replace(&self, text: &str) -> Result<String> {
        Self::escape(text)
    }
}
