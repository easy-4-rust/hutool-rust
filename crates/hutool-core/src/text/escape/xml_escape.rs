//! 对齐: `cn.hutool.core.text.escape.XmlEscape`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/escape/XmlEscape.java
//!
//! 委托 `EscapeUtil::escape_xml`。

use crate::Result;
use crate::escape_util::EscapeUtil;

/// 对齐 Java: `XmlEscape#`
#[derive(Debug, Clone, Copy, Default)]
pub struct XmlEscape;

impl XmlEscape {
    /// 对齐 Java: `XmlEscape()` — 构造 XML 转义器。
    pub fn new() -> Self {
        Self
    }

    /// 对齐 Java: XML 特殊字符转义。
    pub fn escape(text: &str) -> Result<String> {
        Ok(EscapeUtil::escape_xml(text))
    }

    /// 实例方法形式，行为同 [`Self::escape`]。
    pub fn replace(&self, text: &str) -> Result<String> {
        Self::escape(text)
    }
}
