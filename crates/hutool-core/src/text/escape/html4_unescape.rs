//! 对齐: `cn.hutool.core.text.escape.Html4Unescape`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/escape/Html4Unescape.java
//!
//! 委托 `EscapeUtil::unescape_html4`。

use crate::Result;
use crate::escape_util::EscapeUtil;

/// 对齐 Java: `Html4Unescape#`
#[derive(Debug, Clone, Copy, Default)]
pub struct Html4Unescape;

impl Html4Unescape {
    /// 对齐 Java: `Html4Unescape()` — 构造反转义器。
    pub fn new() -> Self {
        Self
    }

    /// 对齐 Java: HTML4 反转义（含数值实体由 NumericEntityUnescaper 覆盖的部分走 EscapeUtil）。
    pub fn unescape(text: &str) -> Result<String> {
        Ok(EscapeUtil::unescape_html4(text))
    }

    /// 实例方法形式，行为同 [`Self::unescape`]。
    pub fn replace(&self, text: &str) -> Result<String> {
        Self::unescape(text)
    }
}
