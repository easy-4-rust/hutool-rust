//! 对齐: `cn.hutool.core.util.ObjectUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ObjectUtil.java
//!
//! Rust 版本提供对象操作的 idiomatic 实现。

use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

/// 对齐 Java `CharSequence`：文本序列视图。
pub trait CharSequence {
    /// 返回 UTF-8 文本内容。
    fn as_text(&self) -> &str;
}

impl CharSequence for str {
    fn as_text(&self) -> &str {
        self
    }
}

impl CharSequence for String {
    fn as_text(&self) -> &str {
        self.as_str()
    }
}
