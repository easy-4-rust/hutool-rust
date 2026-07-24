//! 对齐: `cn.hutool.core.util.ObjectUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ObjectUtil.java
//!
//! Rust 版本提供对象操作的 idiomatic 实现。

use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

use super::char_sequence::CharSequence;

/// 对齐 Java CharSequence 元素经 `toString()` 后的文本；`None` 等价于 Java 返回 null。
pub trait CharSequenceElement {
    /// 返回元素文本；`None` 表示 `toString()` 为 null 或不可用。
    fn element_text(&self) -> Option<&str>;
}

impl CharSequenceElement for str {
    fn element_text(&self) -> Option<&str> {
        Some(self)
    }
}

impl CharSequenceElement for String {
    fn element_text(&self) -> Option<&str> {
        Some(self.as_str())
    }
}
