//! 对齐: `cn.hutool.core.text.StrJoiner`
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/StrJoiner.java
//!
//! 字符串拼接器(类似 Java `StringJoiner`,支持前缀/后缀/`null` 模式)。

use crate::Result;

use super::str_joiner::StrJoiner;

/// 对齐 Java: `StrJoiner#NullMode` 枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NullMode {
    /// 忽略 null
    #[default]
    Ignore,
    /// 视为空字符串
    ToEmpty,
    /// 输出字面量 `"null"`
    NullString,
}
