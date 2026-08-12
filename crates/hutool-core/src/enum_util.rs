//! 对齐: `cn.hutool.core.util.EnumUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/EnumUtil.java
//!
//! Rust 版本通过显式枚举切片与闭包替代 Java 反射。

use std::collections::HashMap;

/// 对齐 Java: `cn.hutool.core.util.EnumUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct EnumUtil;

impl EnumUtil {
    // ── 枚举名称 ──

    /// 对齐 Java: `EnumUtil.getName(Enum)`
    pub fn name_of<E: std::fmt::Debug>(value: &E) -> String {
        format!("{:?}", value)
    }

    // ── 枚举查找 ──

    /// 对齐 Java: `EnumUtil.fromString(Class, String)`
    pub fn from_names<E: std::fmt::Debug + Clone>(variants: &[E], name: &str) -> Option<E> {
        variants
            .iter()
            .find(|v| format!("{:?}", v) == name)
            .cloned()
    }

    /// 对齐 Java: `EnumUtil.getBy(Class, Predicate)` — Rust 使用闭包
    pub fn get_by<E, F>(variants: &[E], predicate: F) -> Option<E>
    where
        E: Clone,
        F: Fn(&E) -> bool,
    {
        variants.iter().find(|v| predicate(v)).cloned()
    }

    /// 对齐 Java: `EnumUtil.getBy(Class, Func1, value)`
    pub fn get_by_field<E, C, F>(variants: &[E], field: F, value: &C) -> Option<E>
    where
        E: Clone,
        C: PartialEq,
        F: Fn(&E) -> C,
    {
        variants.iter().find(|v| field(v) == *value).cloned()
    }

    /// 对齐 Java: `EnumUtil.getFieldBy(E, Func1, Enum::ordinal, value)`
    pub fn get_field_by<E, C, R, Field, Match>(
        variants: &[E],
        field: Field,
        matcher: Match,
        value: &C,
    ) -> Option<R>
    where
        E: Clone,
        C: PartialEq,
        Field: Fn(&E) -> R,
        Match: Fn(&E) -> C,
    {
        variants.iter().find(|v| matcher(v) == *value).map(field)
    }

    /// 对齐 Java: `EnumUtil.likeValueOf(Class, Object)`
    pub fn like_value_of<E, F>(variants: &[E], value: &str, fields: &[F]) -> Option<E>
    where
        E: Clone + PartialEq + std::fmt::Debug,
        F: Fn(&E) -> String,
    {
        let trimmed = value.trim();
        for variant in variants {
            if format!("{:?}", variant) == trimmed {
                return Some(variant.clone());
            }
            for field in fields {
                if field(variant) == trimmed {
                    return Some(variant.clone());
                }
            }
        }
        None
    }

    // ── 枚举列表 ──

    /// 对齐 Java: `EnumUtil.getNames(Class)`
    pub fn names<E: std::fmt::Debug>(variants: &[E]) -> Vec<String> {
        variants.iter().map(|v| format!("{:?}", v)).collect()
    }

    /// 对齐 Java: `EnumUtil.getFieldValues(Class, fieldName)` — Rust 传入字段提取闭包
    pub fn get_field_values<E, F>(variants: &[E], field: F) -> Vec<String>
    where
        F: Fn(&E) -> String,
    {
        variants.iter().map(field).collect()
    }

    /// 对齐 Java: `EnumUtil.getFieldValues` 递归初始化安全版本（fix issue#IDQYJK）
    pub fn get_field_values_recursive<E, F>(variants: &[E], field: F) -> Vec<String>
    where
        F: Fn(&E) -> String,
    {
        Self::get_field_values(variants, field)
    }

    /// 对齐 Java: `EnumUtil.getFieldNames(Class)` — Rust 传入字段名列表
    pub fn get_field_names(field_names: &[&str]) -> Vec<String> {
        field_names.iter().map(|name| (*name).to_string()).collect()
    }

    /// 对齐 Java: `EnumUtil.getValues(Class)`
    pub fn count<E>(variants: &[E]) -> usize {
        variants.len()
    }

    // ── 枚举映射 ──

    /// 对齐 Java: `EnumUtil.getEnumMap(Class)`
    pub fn name_map<E: std::fmt::Debug + Clone>(variants: &[E]) -> HashMap<String, E> {
        variants
            .iter()
            .map(|v| (format!("{:?}", v), v.clone()))
            .collect()
    }

    /// 对齐 Java: `EnumUtil.getNameFieldMap(Class, fieldName)`
    pub fn get_name_field_map<E, F>(variants: &[E], field: F) -> HashMap<String, String>
    where
        E: std::fmt::Debug,
        F: Fn(&E) -> String,
    {
        variants
            .iter()
            .map(|v| (format!("{:?}", v), field(v)))
            .collect()
    }

    // ── 枚举验证 ──

    /// 对齐 Java: `EnumUtil.contains(Class, String)`
    pub fn contains_name<E: std::fmt::Debug>(variants: &[E], name: &str) -> bool {
        variants.iter().any(|v| format!("{:?}", v) == name)
    }
}
