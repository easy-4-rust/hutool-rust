//! 对齐: `cn.hutool.core.text.escape.XmlUnescape`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/escape/XmlUnescape.java
//!
//! 委托 `EscapeUtil::unescape_xml`。

use crate::Result;
use crate::escape_util::EscapeUtil;

/// 对齐 Java: `XmlUnescape#`
#[derive(Debug, Clone, Copy, Default)]
pub struct XmlUnescape;

impl XmlUnescape {
    /// 对齐 Java: `XmlUnescape()` — 构造 XML 反转义器。
    pub fn new() -> Self {
        Self
    }

    /// 对齐 Java: XML 实体反转义。
    pub fn unescape(text: &str) -> Result<String> {
        Ok(EscapeUtil::unescape_xml(text))
    }

    /// 实例方法形式，行为同 [`Self::unescape`]。
    pub fn replace(&self, text: &str) -> Result<String> {
        Self::unescape(text)
    }
}
