//! 对齐: `cn.hutool.core.util.ObjectUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ObjectUtil.java
//!
//! Rust 版本提供对象操作的 idiomatic 实现。

use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

use super::char_sequence::CharSequence;
use super::char_sequence_element::CharSequenceElement;
use super::object_contains::ObjectContains;
use super::object_length::ObjectLength;

/// 对齐 Java: `cn.hutool.core.util.ObjectUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ObjectUtil;

impl ObjectUtil {
    // ── 空值判断 ──

    /// 对齐 Java: `ObjectUtil.isNull(Object)`
    pub fn is_null<T>(value: Option<&T>) -> bool {
        value.is_none()
    }

    /// 对齐 Java: `ObjectUtil.isNotNull(Object)`
    pub fn is_not_null<T>(value: Option<&T>) -> bool {
        value.is_some()
    }

    // ── 默认值 ──

    /// 对齐 Java: `ObjectUtil.defaultIfNull(Object, Object)`
    pub fn default_if_null<T: Clone>(value: Option<&T>, default: &T) -> T {
        value.cloned().unwrap_or_else(|| default.clone())
    }

    // ── 相等判断 ──

    /// 对齐 Java: `ObjectUtil.equal(Object, Object)`
    pub fn equal<T: PartialEq>(a: Option<&T>, b: Option<&T>) -> bool {
        match (a, b) {
            (Some(a), Some(b)) => a == b,
            (None, None) => true,
            _ => false,
        }
    }

    /// 对齐 Java: `ObjectUtil.notEqual(Object, Object)`
    pub fn not_equal<T: PartialEq>(a: Option<&T>, b: Option<&T>) -> bool {
        !Self::equal(a, b)
    }

    // ── 比较操作 ──

    /// 对齐 Java: `ObjectUtil.compare(Object, Object)`
    pub fn compare<T: Ord>(a: Option<&T>, b: Option<&T>) -> i32 {
        match (a, b) {
            (Some(a), Some(b)) => a.cmp(b) as i32,
            (None, Some(_)) => -1,
            (Some(_), None) => 1,
            (None, None) => 0,
        }
    }

    // ── 类型判断 ──

    /// 对齐 Java: `ObjectUtil.isBasicType(Object)`
    pub fn is_basic_type(value: &dyn Any) -> bool {
        value.is::<i32>()
            || value.is::<i64>()
            || value.is::<f32>()
            || value.is::<f64>()
            || value.is::<bool>()
            || value.is::<char>()
            || value.is::<i8>()
            || value.is::<i16>()
            || value.is::<u8>()
            || value.is::<u16>()
            || value.is::<u32>()
            || value.is::<u64>()
    }

    // ── 克隆操作 ──

    /// 对齐 Java: `ObjectUtil.clone(Object)`
    pub fn clone_if_some<T: Clone>(value: Option<&T>) -> Option<T> {
        value.cloned()
    }

    // ── 序列化辅助 ──

    /// 对齐 Java: `ObjectUtil.toString(Object)`
    pub fn to_string<T: std::fmt::Display>(value: Option<&T>) -> String {
        value.map_or_else(|| "null".to_string(), |v| v.to_string())
    }

    // ── 集合判断 ──

    /// 对齐 Java: `ObjectUtil.isEmpty(Object)`
    pub fn is_empty_str(value: Option<&str>) -> bool {
        value.map_or(true, |s| s.is_empty())
    }

    /// 对齐 Java: `ObjectUtil.isNotEmpty(Object)`
    pub fn is_not_empty_str(value: Option<&str>) -> bool {
        !Self::is_empty_str(value)
    }

    // ── 长度 / 包含 ──

    /// 对齐 Java: `ObjectUtil.length(Object)` — null 返回 0。
    pub fn length<T: ObjectLength + ?Sized>(obj: Option<&T>) -> i32 {
        obj.map(ObjectLength::object_length).unwrap_or(0)
    }

    /// 对齐 Java: `ObjectUtil.length(Object)` — Iterator/Enumeration；副作用：消耗迭代器。
    pub fn length_iter<I: Iterator>(iter: I) -> i32 {
        i32_from_usize(iter.count())
    }

    /// 对齐 Java: `ObjectUtil.length(Object)` — 不支持的类型返回 -1。
    pub fn length_unsupported<T: ?Sized>(_obj: &T) -> i32 {
        -1
    }

    /// 对齐 Java: `ObjectUtil.contains(Object, Object)` — null obj 返回 false。
    pub fn contains<O, E>(obj: Option<&O>, element: Option<&E>) -> bool
    where
        O: ObjectContains<E> + ?Sized,
        E: ?Sized,
    {
        match (obj, element) {
            (Some(obj), Some(element)) => obj.object_contains(element),
            _ => false,
        }
    }

    /// 对齐 Java: CharSequence 分支 — element 为 CharSequence 时使用 `toString()` 文本匹配。
    pub fn contains_text<T, E>(obj: Option<&T>, element: &E) -> bool
    where
        T: CharSequence + ?Sized,
        E: CharSequenceElement,
    {
        match (obj.map(CharSequence::as_text), element.element_text()) {
            (Some(obj_text), Some(element_text)) => obj_text.contains(element_text),
            _ => false,
        }
    }

    /// 对齐 Java: CharSequence 分支 — element 非 CharSequence 时返回 false。
    pub fn contains_text_with_non_char_sequence<T, E>(obj: Option<&T>, _element: &E) -> bool
    where
        T: CharSequence + ?Sized,
        E: ?Sized,
    {
        let _ = obj;
        false
    }

    /// 对齐 Java: `ObjectUtil.contains(Object, Object)` — Iterator/Enumeration。
    pub fn contains_iter<I, T>(mut obj: I, element: Option<&T>) -> bool
    where
        I: Iterator<Item = T>,
        T: PartialEq,
    {
        match element {
            Some(element) => obj.any(|item| Self::equal(Some(&item), Some(element))),
            None => false,
        }
    }

    /// 对齐 Java: `ObjectUtil.equals`（`equal` 别名）
    pub fn equals<T: PartialEq>(a: Option<&T>, b: Option<&T>) -> bool {
        Self::equal(a, b)
    }

    /// 对齐 Java: `ObjectUtil.isEmpty(Object)` — 通用空判断。
    pub fn is_empty<T: ObjectLength + ?Sized>(obj: Option<&T>) -> bool {
        obj.map_or(true, |v| v.object_length() == 0)
    }

    /// 对齐 Java: `ObjectUtil.isNotEmpty(Object)`
    pub fn is_not_empty<T: ObjectLength + ?Sized>(obj: Option<&T>) -> bool {
        !Self::is_empty(obj)
    }

    /// 对齐 Java: `ObjectUtil.defaultIfEmpty(String, String)`
    pub fn default_if_empty<'a>(value: &'a str, default: &'a str) -> &'a str {
        if value.is_empty() { default } else { value }
    }

    /// 对齐 Java: `ObjectUtil.defaultIfBlank(String, String)`
    pub fn default_if_blank<'a>(value: &'a str, default: &'a str) -> &'a str {
        if value.trim().is_empty() {
            default
        } else {
            value
        }
    }

    /// 对齐 Java: `ObjectUtil.cloneIfPossible`
    pub fn clone_if_possible<T: Clone>(value: &T) -> T {
        value.clone()
    }

    /// 对齐 Java: `ObjectUtil.hasNull(Object...)`
    pub fn has_null(values: &[bool]) -> bool {
        // values[i]==true 表示该槽位为 null
        values.iter().any(|&is_null| is_null)
    }

    /// 对齐 Java: `ObjectUtil.hasEmpty` — 空字符串检测。
    pub fn has_empty(values: &[&str]) -> bool {
        values.iter().any(|s| s.is_empty())
    }

    /// 对齐 Java: `ObjectUtil.isAllEmpty`
    pub fn is_all_empty(values: &[&str]) -> bool {
        values.iter().all(|s| s.is_empty())
    }

    /// 对齐 Java: `ObjectUtil.isAllNotEmpty`
    pub fn is_all_not_empty(values: &[&str]) -> bool {
        values.iter().all(|s| !s.is_empty())
    }

    /// 对齐 Java: `ObjectUtil.emptyCount`
    pub fn empty_count(values: &[&str]) -> i32 {
        values.iter().filter(|s| s.is_empty()).count() as i32
    }

    /// 对齐 Java: `ObjectUtil.isValidIfNumber` — 非数字对象视为 valid。
    pub fn is_valid_if_number(value: &dyn Any) -> bool {
        if let Some(n) = value.downcast_ref::<f64>() {
            return n.is_finite();
        }
        if let Some(n) = value.downcast_ref::<f32>() {
            return n.is_finite();
        }
        true
    }

    /// 对齐 Java: `ObjectUtil.compare(..., boolean nullGreater)`
    pub fn compare_null_greater<T: Ord>(a: Option<&T>, b: Option<&T>, null_greater: bool) -> i32 {
        match (a, b) {
            (Some(a), Some(b)) => a.cmp(b) as i32,
            (None, Some(_)) => {
                if null_greater {
                    1
                } else {
                    -1
                }
            }
            (Some(_), None) => {
                if null_greater {
                    -1
                } else {
                    1
                }
            }
            (None, None) => 0,
        }
    }

    /// 对齐 Java: `ObjectUtil.apply` — source 非空时应用 handler。
    pub fn apply<T, R, F>(source: Option<T>, handler: F) -> Option<R>
    where
        F: FnOnce(T) -> R,
    {
        source.map(handler)
    }

    /// 对齐 Java: `ObjectUtil.accept` — source 非空时消费。
    pub fn accept<T, F>(source: Option<T>, consumer: F)
    where
        F: FnOnce(T),
    {
        if let Some(value) = source {
            consumer(value);
        }
    }

    /// 对齐 Java: `ObjectUtil.clone` — Clone trait 路径。
    pub fn clone_value<T: Clone>(value: &T) -> T {
        value.clone()
    }
}

fn i32_from_usize(value: usize) -> i32 {
    i32::try_from(value).unwrap_or(i32::MAX)
}
