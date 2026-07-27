//! 对齐: `cn.hutool.core.text.StrUtil` 扩展方法
//! 来源: hutool-core/src/main/java/cn/hutool/core/text/StrUtil.java
//!
//! 中文说明: 字符串扩展方法，定义 `StrExt` trait，提供
//! Rust idiomatic 的链式调用方式（`"hello".is_blank()` 等）。
//! 真正实现委托给 `crate::text::str_util` 中对齐 `StrUtil` 的关联函数。

use super::str_util::{is_blank, lower_first, remove_all, upper_first};

/// 字符串切片扩展方法集合。
///
/// Rust idiomatic 链式调用入口。底层调用 [`crate::text::str_util`]
/// 中的自由函数。
pub trait StrExt {
    /// 字符串为空或仅含空白。
    fn is_blank(&self) -> bool;

    /// 字符串包含至少一个非空白字符。
    fn is_not_blank(&self) -> bool;

    /// 借用版去除首尾空白。
    fn trimmed(&self) -> &str;

    /// 移除所有 `needle` 子串。
    fn without(&self, needle: &str) -> String;

    /// 首字符大写。
    fn upper_first(&self) -> String;

    /// 首字符小写。
    fn lower_first(&self) -> String;
}

impl StrExt for str {
    #[inline]
    fn is_blank(&self) -> bool {
        is_blank(self)
    }

    #[inline]
    fn is_not_blank(&self) -> bool {
        !is_blank(self)
    }

    #[inline]
    fn trimmed(&self) -> &str {
        self.trim()
    }

    fn without(&self, needle: &str) -> String {
        remove_all(self, needle)
    }

    fn upper_first(&self) -> String {
        upper_first(self)
    }

    fn lower_first(&self) -> String {
        lower_first(self)
    }
}
