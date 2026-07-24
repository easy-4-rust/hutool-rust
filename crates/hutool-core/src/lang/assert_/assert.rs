//! 对齐: `cn.hutool.core.lang.Assert`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Assert.java
//!
//! Rust 版本以 [`Result`] + [`AssertError`] 表达 Java 的断言失败抛出；
//! 成功时返回被检查值，便于链式调用。

use std::collections::HashMap;
use std::fmt::{Display, Write};

use crate::string::{format_template, is_blank};

use super::assert_error::AssertError;
use super::assert_result::AssertResult;

/// 对齐 Java 类: `cn.hutool.core.lang.Assert`
#[derive(Debug, Clone, Copy, Default)]
pub struct Assert;

impl Assert {
    fn fmt_msg(template: &str, params: &[&dyn Display]) -> String {
        format_template(template, params)
    }

    // ── isTrue / isFalse ──

    /// 对齐 Java: `Assert.isTrue(boolean, Supplier)`
    pub fn is_true_or_else<E>(expression: bool, supplier: impl FnOnce() -> E) -> AssertResult<()>
    where
        E: Into<AssertError>,
    {
        if expression {
            Ok(())
        } else {
            Err(supplier().into())
        }
    }

    /// 对齐 Java: `Assert.isTrue(boolean, String, Object...)`
    pub fn is_true_msg(expression: bool, template: &str, params: &[&dyn Display]) -> AssertResult<()> {
        Self::is_true_or_else(expression, || AssertError::argument(Self::fmt_msg(template, params)))
    }

    /// 对齐 Java: `Assert.isTrue(boolean)`
    pub fn is_true(expression: bool) -> AssertResult<()> {
        Self::is_true_msg(
            expression,
            "[Assertion failed] - this expression must be true",
            &[],
        )
    }

    /// 对齐 Java: `Assert.isFalse(boolean, Supplier)`
    pub fn is_false_or_else<E>(expression: bool, supplier: impl FnOnce() -> E) -> AssertResult<()>
    where
        E: Into<AssertError>,
    {
        if !expression {
            Ok(())
        } else {
            Err(supplier().into())
        }
    }

    /// 对齐 Java: `Assert.isFalse(boolean, String, Object...)`
    pub fn is_false_msg(expression: bool, template: &str, params: &[&dyn Display]) -> AssertResult<()> {
        Self::is_false_or_else(expression, || AssertError::argument(Self::fmt_msg(template, params)))
    }

    /// 对齐 Java: `Assert.isFalse(boolean)`
    pub fn is_false(expression: bool) -> AssertResult<()> {
        Self::is_false_msg(
            expression,
            "[Assertion failed] - this expression must be false",
            &[],
        )
    }

    // ── null ──

    /// 对齐 Java: `Assert.isNull(Object, Supplier)` — Rust 用 `Option` 表达可空。
    pub fn is_null_or_else<T, E>(object: Option<T>, supplier: impl FnOnce() -> E) -> AssertResult<()>
    where
        E: Into<AssertError>,
    {
        if object.is_none() {
            Ok(())
        } else {
            Err(supplier().into())
        }
    }

    /// 对齐 Java: `Assert.isNull(Object, String, Object...)`
    pub fn is_null_msg<T>(object: Option<T>, template: &str, params: &[&dyn Display]) -> AssertResult<()> {
        Self::is_null_or_else(object, || AssertError::argument(Self::fmt_msg(template, params)))
    }

    /// 对齐 Java: `Assert.isNull(Object)`
    pub fn is_null<T>(object: Option<T>) -> AssertResult<()> {
        Self::is_null_msg(
            object,
            "[Assertion failed] - the object argument must be null",
            &[],
        )
    }

    /// 对齐 Java: `Assert.notNull(T, Supplier)`
    pub fn not_null_or_else<T, E>(object: Option<T>, supplier: impl FnOnce() -> E) -> AssertResult<T>
    where
        E: Into<AssertError>,
    {
        object.ok_or_else(|| supplier().into())
    }

    /// 对齐 Java: `Assert.notNull(T, String, Object...)`
    pub fn not_null_msg<T>(object: Option<T>, template: &str, params: &[&dyn Display]) -> AssertResult<T> {
        Self::not_null_or_else(object, || AssertError::argument(Self::fmt_msg(template, params)))
    }

    /// 对齐 Java: `Assert.notNull(T)`
    pub fn not_null<T>(object: Option<T>) -> AssertResult<T> {
        Self::not_null_msg(
            object,
            "[Assertion failed] - this argument is required; it must not be null",
            &[],
        )
    }

    // ── 字符串 empty / blank / contain ──

    /// 对齐 Java: `Assert.notEmpty(CharSequence, Supplier)`
    pub fn not_empty_str_or_else<'a, E>(
        text: Option<&'a str>,
        supplier: impl FnOnce() -> E,
    ) -> AssertResult<&'a str>
    where
        E: Into<AssertError>,
    {
        match text {
            Some(s) if !s.is_empty() => Ok(s),
            _ => Err(supplier().into()),
        }
    }

    /// 对齐 Java: `Assert.notEmpty(CharSequence, String, Object...)`
    pub fn not_empty_str_msg<'a>(
        text: Option<&'a str>,
        template: &str,
        params: &[&dyn Display],
    ) -> AssertResult<&'a str> {
        Self::not_empty_str_or_else(text, || AssertError::argument(Self::fmt_msg(template, params)))
    }

    /// 对齐 Java: `Assert.notEmpty(CharSequence)`
    pub fn not_empty_str<'a>(text: Option<&'a str>) -> AssertResult<&'a str> {
        Self::not_empty_str_msg(
            text,
            "[Assertion failed] - this String argument must have length; it must not be null or empty",
            &[],
        )
    }

    /// 对齐 Java: `Assert.notBlank(CharSequence, Supplier)`
    pub fn not_blank_or_else<'a, E>(
        text: Option<&'a str>,
        supplier: impl FnOnce() -> E,
    ) -> AssertResult<&'a str>
    where
        E: Into<AssertError>,
    {
        match text {
            Some(s) if !is_blank(s) => Ok(s),
            _ => Err(supplier().into()),
        }
    }

    /// 对齐 Java: `Assert.notBlank(CharSequence, String, Object...)`
    pub fn not_blank_msg<'a>(
        text: Option<&'a str>,
        template: &str,
        params: &[&dyn Display],
    ) -> AssertResult<&'a str> {
        Self::not_blank_or_else(text, || AssertError::argument(Self::fmt_msg(template, params)))
    }

    /// 对齐 Java: `Assert.notBlank(CharSequence)`
    pub fn not_blank<'a>(text: Option<&'a str>) -> AssertResult<&'a str> {
        Self::not_blank_msg(
            text,
            "[Assertion failed] - this String argument must have text; it must not be null, empty, or blank",
            &[],
        )
    }

    /// 对齐 Java: `Assert.notContain(CharSequence, CharSequence, Supplier)`
    pub fn not_contain_or_else<'a, E>(
        text_to_search: &str,
        substring: &'a str,
        supplier: impl FnOnce() -> E,
    ) -> AssertResult<&'a str>
    where
        E: Into<AssertError>,
    {
        if text_to_search.contains(substring) {
            Err(supplier().into())
        } else {
            Ok(substring)
        }
    }

    /// 对齐 Java: `Assert.notContain(String, String, String, Object...)`
    pub fn not_contain_msg<'a>(
        text_to_search: &str,
        substring: &'a str,
        template: &str,
        params: &[&dyn Display],
    ) -> AssertResult<&'a str> {
        Self::not_contain_or_else(text_to_search, substring, || {
            AssertError::argument(Self::fmt_msg(template, params))
        })
    }

    /// 对齐 Java: `Assert.notContain(String, String)`
    pub fn not_contain<'a>(text_to_search: &str, substring: &'a str) -> AssertResult<&'a str> {
        let params: [&dyn Display; 1] = [&substring];
        Self::not_contain_msg(
            text_to_search,
            substring,
            "[Assertion failed] - this String argument must not contain the substring [{}]",
            &params,
        )
    }

    // ── 集合 / 数组 / Map ──

    /// 对齐 Java: `Assert.notEmpty(T[])` / `Collection` / `Map`
    pub fn not_empty_slice_or_else<'a, T, E>(
        array: Option<&'a [T]>,
        supplier: impl FnOnce() -> E,
    ) -> AssertResult<&'a [T]>
    where
        E: Into<AssertError>,
    {
        match array {
            Some(a) if !a.is_empty() => Ok(a),
            _ => Err(supplier().into()),
        }
    }

    /// 对齐 Java: `Assert.notEmpty(T[], String, Object...)`
    pub fn not_empty_slice_msg<'a, T>(
        array: Option<&'a [T]>,
        template: &str,
        params: &[&dyn Display],
    ) -> AssertResult<&'a [T]> {
        Self::not_empty_slice_or_else(array, || AssertError::argument(Self::fmt_msg(template, params)))
    }

    /// 对齐 Java: `Assert.notEmpty(T[])`
    pub fn not_empty_slice<'a, T>(array: Option<&'a [T]>) -> AssertResult<&'a [T]> {
        Self::not_empty_slice_msg(
            array,
            "[Assertion failed] - this array must not be empty: it must contain at least 1 element",
            &[],
        )
    }

    /// 对齐 Java: `Assert.noNullElements(T[], Supplier)` — Rust 用 `Option` 元素表达可空。
    pub fn no_null_elements_or_else<'a, T, E>(
        array: Option<&'a [Option<T>]>,
        supplier: impl FnOnce() -> E,
    ) -> AssertResult<&'a [Option<T>]>
    where
        E: Into<AssertError>,
    {
        let Some(a) = array else {
            return Err(supplier().into());
        };
        if a.iter().any(|e| e.is_none()) {
            Err(supplier().into())
        } else {
            Ok(a)
        }
    }

    /// 对齐 Java: `Assert.noNullElements(T[], String, Object...)`
    pub fn no_null_elements_msg<'a, T>(
        array: Option<&'a [Option<T>]>,
        template: &str,
        params: &[&dyn Display],
    ) -> AssertResult<&'a [Option<T>]> {
        Self::no_null_elements_or_else(array, || AssertError::argument(Self::fmt_msg(template, params)))
    }

    /// 对齐 Java: `Assert.noNullElements(T[])`
    pub fn no_null_elements<'a, T>(array: Option<&'a [Option<T>]>) -> AssertResult<&'a [Option<T>]> {
        Self::no_null_elements_msg(
            array,
            "[Assertion failed] - this array must not contain any null elements",
            &[],
        )
    }

    /// 对齐 Java: `Assert.empty(Collection, Supplier)`
    pub fn empty_slice_or_else<'a, T, E>(
        collection: Option<&'a [T]>,
        supplier: impl FnOnce() -> E,
    ) -> AssertResult<()>
    where
        E: Into<AssertError>,
    {
        if collection.map_or(true, |c| c.is_empty()) {
            Ok(())
        } else {
            Err(supplier().into())
        }
    }

    /// 对齐 Java: `Assert.empty(Collection, String, Object...)`
    pub fn empty_slice_msg<T>(
        collection: Option<&[T]>,
        template: &str,
        params: &[&dyn Display],
    ) -> AssertResult<()> {
        Self::empty_slice_or_else(collection, || AssertError::argument(Self::fmt_msg(template, params)))
    }

    /// 对齐 Java: `Assert.empty(Collection)`
    pub fn empty_slice<T>(collection: Option<&[T]>) -> AssertResult<()> {
        Self::empty_slice_msg(
            collection,
            "[Assertion failed] - this collection must be empty",
            &[],
        )
    }

    /// 对齐 Java: `Assert.notEmpty(Map)`
    pub fn not_empty_map_or_else<'a, K, V, E>(
        map: Option<&'a HashMap<K, V>>,
        supplier: impl FnOnce() -> E,
    ) -> AssertResult<&'a HashMap<K, V>>
    where
        E: Into<AssertError>,
    {
        match map {
            Some(m) if !m.is_empty() => Ok(m),
            _ => Err(supplier().into()),
        }
    }

    /// 对齐 Java: `Assert.notEmpty(Map, String, Object...)`
    pub fn not_empty_map_msg<'a, K, V>(
        map: Option<&'a HashMap<K, V>>,
        template: &str,
        params: &[&dyn Display],
    ) -> AssertResult<&'a HashMap<K, V>> {
        Self::not_empty_map_or_else(map, || AssertError::argument(Self::fmt_msg(template, params)))
    }

    /// 对齐 Java: `Assert.notEmpty(Map)`
    pub fn not_empty_map<'a, K, V>(map: Option<&'a HashMap<K, V>>) -> AssertResult<&'a HashMap<K, V>> {
        Self::not_empty_map_msg(
            map,
            "[Assertion failed] - this map must not be empty; it must contain at least one entry",
            &[],
        )
    }

    // ── state / index / between / equals ──

    /// 对齐 Java: `Assert.state(boolean, Supplier<String>)`
    pub fn state_or_else(expression: bool, error_msg: impl FnOnce() -> String) -> AssertResult<()> {
        if expression {
            Ok(())
        } else {
            Err(AssertError::state(error_msg()))
        }
    }

    /// 对齐 Java: `Assert.state(boolean, String, Object...)`
    pub fn state_msg(expression: bool, template: &str, params: &[&dyn Display]) -> AssertResult<()> {
        Self::state_or_else(expression, || Self::fmt_msg(template, params))
    }

    /// 对齐 Java: `Assert.state(boolean)`
    pub fn state(expression: bool) -> AssertResult<()> {
        Self::state_msg(
            expression,
            "[Assertion failed] - this state invariant must be true",
            &[],
        )
    }

    /// 对齐 Java: `Assert.checkIndex(int, int)`
    pub fn check_index(index: usize, size: usize) -> AssertResult<usize> {
        Self::check_index_msg(index, size, "[Assertion failed]", &[])
    }

    /// 对齐 Java: `Assert.checkIndex(int, int, String, Object...)`
    pub fn check_index_msg(
        index: usize,
        size: usize,
        template: &str,
        params: &[&dyn Display],
    ) -> AssertResult<usize> {
        if index < size {
            Ok(index)
        } else {
            let mut msg = Self::fmt_msg(template, params);
            if msg == "[Assertion failed]" {
                msg.clear();
                let _ = write!(
                    &mut msg,
                    "[Assertion failed] - Index: {index}, Size: {size}"
                );
            }
            Err(AssertError::argument(msg))
        }
    }

    /// 对齐 Java: `Assert.checkBetween(int/long/double, min, max)`
    pub fn check_between_i64(value: i64, min: i64, max: i64) -> AssertResult<i64> {
        Self::check_between_i64_msg(value, min, max, "The value must be between {} and {}.", &[&min, &max])
    }

    /// 对齐 Java: `Assert.checkBetween(..., String, Object...)`
    pub fn check_between_i64_msg(
        value: i64,
        min: i64,
        max: i64,
        template: &str,
        params: &[&dyn Display],
    ) -> AssertResult<i64> {
        if (min..=max).contains(&value) {
            Ok(value)
        } else {
            Err(AssertError::argument(Self::fmt_msg(template, params)))
        }
    }

    /// 对齐 Java: `Assert.checkBetween(int, int, int, Supplier)`
    pub fn check_between_i64_or_else<E>(
        value: i64,
        min: i64,
        max: i64,
        supplier: impl FnOnce() -> E,
    ) -> AssertResult<i64>
    where
        E: Into<AssertError>,
    {
        if (min..=max).contains(&value) {
            Ok(value)
        } else {
            Err(supplier().into())
        }
    }

    /// 对齐 Java: `Assert.checkBetween(double, double, double)`
    pub fn check_between_f64(value: f64, min: f64, max: f64) -> AssertResult<f64> {
        Self::check_between_f64_msg(value, min, max, "The value must be between {} and {}.", &[&min, &max])
    }

    /// 对齐 Java: `Assert.checkBetween(double, ..., String, Object...)`
    pub fn check_between_f64_msg(
        value: f64,
        min: f64,
        max: f64,
        template: &str,
        params: &[&dyn Display],
    ) -> AssertResult<f64> {
        if value >= min && value <= max {
            Ok(value)
        } else {
            Err(AssertError::argument(Self::fmt_msg(template, params)))
        }
    }

    /// 对齐 Java: `Assert.checkBetween(Number, Number, Number)` — f64 门面。
    pub fn check_between_number(value: f64, min: f64, max: f64) -> AssertResult<f64> {
        Self::check_between_f64(value, min, max)
    }

    /// 对齐 Java: `Assert.equals(Object, Object)`
    pub fn equals<T: PartialEq>(obj1: &T, obj2: &T) -> AssertResult<()> {
        Self::equals_msg(obj1, obj2, "[Assertion failed] - objects must be equal", &[])
    }

    /// 对齐 Java: `Assert.equals(Object, Object, String, Object...)`
    pub fn equals_msg<T: PartialEq>(
        obj1: &T,
        obj2: &T,
        template: &str,
        params: &[&dyn Display],
    ) -> AssertResult<()> {
        if obj1 == obj2 {
            Ok(())
        } else {
            Err(AssertError::argument(Self::fmt_msg(template, params)))
        }
    }

    /// 对齐 Java: `Assert.equals(Object, Object, Supplier)`
    pub fn equals_or_else<T: PartialEq, E>(
        obj1: &T,
        obj2: &T,
        supplier: impl FnOnce() -> E,
    ) -> AssertResult<()>
    where
        E: Into<AssertError>,
    {
        if obj1 == obj2 {
            Ok(())
        } else {
            Err(supplier().into())
        }
    }

    /// 对齐 Java: `Assert.notEquals(Object, Object)`
    pub fn not_equals<T: PartialEq>(obj1: &T, obj2: &T) -> AssertResult<()> {
        Self::not_equals_msg(obj1, obj2, "[Assertion failed] - objects must not be equal", &[])
    }

    /// 对齐 Java: `Assert.notEquals(Object, Object, String, Object...)`
    pub fn not_equals_msg<T: PartialEq>(
        obj1: &T,
        obj2: &T,
        template: &str,
        params: &[&dyn Display],
    ) -> AssertResult<()> {
        if obj1 != obj2 {
            Ok(())
        } else {
            Err(AssertError::argument(Self::fmt_msg(template, params)))
        }
    }

    /// 对齐 Java: `Assert.notEquals(Object, Object, Supplier)`
    pub fn not_equals_or_else<T: PartialEq, E>(
        obj1: &T,
        obj2: &T,
        supplier: impl FnOnce() -> E,
    ) -> AssertResult<()>
    where
        E: Into<AssertError>,
    {
        if obj1 != obj2 {
            Ok(())
        } else {
            Err(supplier().into())
        }
    }
}
