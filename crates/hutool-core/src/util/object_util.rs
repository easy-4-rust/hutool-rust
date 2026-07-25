//! 对齐: `cn.hutool.core.util.ObjectUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ObjectUtil.java
//!
//! Rust 版本按 idiomatic 风格对每个公开方法提供实现。
//! 序列化相关方法基于 `serde_json`，类型名称查询基于 `std::any::type_name`。

#![allow(
    dead_code,
    unused_variables,
    clippy::too_many_arguments,
    non_snake_case
)]

use std::any::{Any, type_name};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::hash::Hash;

use serde::{Serialize, de::DeserializeOwned};

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.util.ObjectUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ObjectUtil;

impl ObjectUtil {
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 空值判断
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// 对齐 Java: `ObjectUtil.isNull(Object)`
    /// 检查对象是否为 None（对应 Java null）。
    pub fn isNull<T>(obj: Option<&T>) -> bool {
        obj.is_none()
    }

    /// 对齐 Java: `ObjectUtil.isNotNull(Object)`
    /// 检查对象是否不为 None。
    pub fn isNotNull<T>(obj: Option<&T>) -> bool {
        obj.is_some()
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 空 / 非空判断（字符串/集合/映射）
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// 对齐 Java: `ObjectUtil.isEmpty(Object)`
    /// 对字符串: 检查是否为空字符串; 对 Option: 检查是否为 None。
    /// 通用路径使用 `IsEmptiable` trait; 字符串特化为 `Option<&str>`。
    pub fn isEmpty_str(obj: Option<&str>) -> bool {
        obj.map_or(true, |s| s.is_empty())
    }

    /// 对齐 Java: `ObjectUtil.isEmpty(Object)` — Option 版本。
    pub fn isEmpty<T>(obj: Option<&T>) -> bool {
        obj.is_none()
    }

    /// 对齐 Java: `ObjectUtil.isNotEmpty(Object)` — 字符串特化。
    pub fn isNotEmpty_str(obj: Option<&str>) -> bool {
        !Self::isEmpty_str(obj)
    }

    /// 对齐 Java: `ObjectUtil.isNotEmpty(Object)` — Option 版本。
    pub fn isNotEmpty<T>(obj: Option<&T>) -> bool {
        obj.is_some()
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 相等判断
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// 对齐 Java: `ObjectUtil.equals(Object, Object)`
    /// 使用 `PartialEq` trait 进行相等比较; 两 None 视为相等。
    pub fn equals<T: PartialEq>(a: Option<&T>, b: Option<&T>) -> bool {
        match (a, b) {
            (Some(a), Some(b)) => a == b,
            (None, None) => true,
            _ => false,
        }
    }

    /// 对齐 Java: `ObjectUtil.equal(Object, Object)` — 与 `equals` 相同。
    pub fn equal<T: PartialEq>(a: Option<&T>, b: Option<&T>) -> bool {
        Self::equals(a, b)
    }

    /// 对齐 Java: `ObjectUtil.notEqual(Object, Object)`
    /// 返回 `!equals(a, b)`。
    pub fn notEqual<T: PartialEq>(a: Option<&T>, b: Option<&T>) -> bool {
        !Self::equals(a, b)
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 默认值
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// 对齐 Java: `ObjectUtil.defaultIfNull(T, T)`
    /// 若 `val` 为 None 则返回 `default_val` 的克隆。
    pub fn defaultIfNull<T: Clone>(val: Option<&T>, default_val: &T) -> T {
        val.cloned().unwrap_or_else(|| default_val.clone())
    }

    /// 对齐 Java: `ObjectUtil.defaultIfNull(T, Supplier)`
    /// 通过函数提供默认值。
    pub fn defaultIfNull_2<T>(source: Option<T>, defaultValueSupplier: fn() -> T) -> T {
        source.unwrap_or_else(defaultValueSupplier)
    }

    /// 对齐 Java: `ObjectUtil.defaultIfNull(T, Function)`
    /// 通过映射函数从可空源产生默认值。
    pub fn defaultIfNull_3<T: Clone>(
        source: Option<&T>,
        defaultValueSupplier: fn(Option<&T>) -> T,
    ) -> T {
        match source {
            Some(v) => v.clone(),
            None => defaultValueSupplier(None),
        }
    }

    /// 对齐 Java: `ObjectUtil.defaultIfNull(Object, Supplier, T)`
    /// source 为 None 时使用 handle 生成值; handle 也失败则返回 defaultValue。
    pub fn defaultIfNull_4<T: Clone>(
        source: Option<&T>,
        handle: fn() -> Option<T>,
        defaultValue: &T,
    ) -> T {
        source
            .cloned()
            .unwrap_or_else(|| handle().unwrap_or_else(|| defaultValue.clone()))
    }

    /// 对齐 Java: `ObjectUtil.defaultIfNull(R, Function, T)`
    /// source 为 None 时使用 handle(source) 映射; 失败则返回 defaultValue。
    pub fn defaultIfNull_5<T: Clone>(
        source: Option<&T>,
        handle: fn(Option<&T>) -> Option<T>,
        defaultValue: &T,
    ) -> T {
        source
            .cloned()
            .unwrap_or_else(|| handle(None).unwrap_or_else(|| defaultValue.clone()))
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 空字符串 / 空白默认值
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// 对齐 Java: `ObjectUtil.defaultIfEmpty(String, Supplier, T)`
    /// 字符串为空时使用 handle 生成值; 失败则返回 defaultValue。
    pub fn defaultIfEmpty<T: Clone>(
        s: Option<&str>,
        handle: fn() -> Option<T>,
        defaultValue: T,
    ) -> T {
        match s {
            Some(s) if !s.is_empty() => handle().unwrap_or(defaultValue),
            _ => defaultValue,
        }
    }

    /// 对齐 Java: `ObjectUtil.defaultIfEmpty(String, Function, T)`
    /// 字符串为空时使用 handle 映射; 失败则返回 defaultValue。
    pub fn defaultIfEmpty_2<T: Clone>(
        s: Option<&str>,
        handle: fn(&str) -> Option<T>,
        defaultValue: T,
    ) -> T {
        match s {
            Some(s) if !s.is_empty() => handle(s).unwrap_or(defaultValue),
            _ => defaultValue,
        }
    }

    /// 对齐 Java: `ObjectUtil.defaultIfEmpty(T, T)`
    /// 对字符串: 为空则返回默认值; 对 Option: 为 None 则返回默认值。
    pub fn defaultIfEmpty_str<'a>(val: &'a str, default_val: &'a str) -> &'a str {
        if val.is_empty() {
            default_val
        } else {
            val
        }
    }

    /// 对齐 Java: `ObjectUtil.defaultIfEmpty(T, T)` — Option 版本。
    pub fn defaultIfEmpty_opt<T: Clone>(val: Option<&T>, default_val: &T) -> T {
        val.cloned().unwrap_or_else(|| default_val.clone())
    }

    /// 对齐 Java: `ObjectUtil.defaultIfEmpty(T, Supplier)` — 通过函数提供默认值。
    pub fn defaultIfEmpty_4<T: Clone>(val: Option<T>, defaultValueSupplier: fn() -> T) -> T {
        val.unwrap_or_else(defaultValueSupplier)
    }

    /// 对齐 Java: `ObjectUtil.defaultIfEmpty(T, Function)` — 通过映射函数提供默认值。
    pub fn defaultIfEmpty_5<T: Clone>(
        val: Option<&T>,
        defaultValueSupplier: fn(Option<&T>) -> T,
    ) -> T {
        match val {
            Some(v) => v.clone(),
            None => defaultValueSupplier(None),
        }
    }

    /// 对齐 Java: `ObjectUtil.defaultIfBlank(T, T)`
    /// 对字符串: 为空白（仅含空白字符）则返回默认值。
    pub fn defaultIfBlank<'a>(val: &'a str, default_val: &'a str) -> &'a str {
        if val.trim().is_empty() {
            default_val
        } else {
            val
        }
    }

    /// 对齐 Java: `ObjectUtil.defaultIfBlank(T, Supplier)`
    /// 为空白时通过函数提供默认值。
    pub fn defaultIfBlank_2<T: Clone>(val: Option<&str>, defaultValueSupplier: fn() -> T) -> T {
        match val {
            Some(s) if !s.trim().is_empty() => {
                // 无法直接将 &str 转为 T，此处需要调用方提供转换
                defaultValueSupplier()
            }
            _ => defaultValueSupplier(),
        }
    }

    /// 对齐 Java: `ObjectUtil.defaultIfBlank(T, Function)`
    /// 为空白时通过映射函数提供默认值。
    pub fn defaultIfBlank_3<T: Clone>(
        val: Option<&str>,
        defaultValueSupplier: fn(Option<&str>) -> T,
    ) -> T {
        match val {
            Some(s) if !s.trim().is_empty() => defaultValueSupplier(Some(s)),
            _ => defaultValueSupplier(None),
        }
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 克隆操作
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// 对齐 Java: `ObjectUtil.clone(T)`
    /// 使用 Clone trait 进行深拷贝。
    pub fn clone<T: Clone>(obj: &T) -> T {
        obj.clone()
    }

    /// 对齐 Java: `ObjectUtil.cloneIfPossible(T)`
    /// 尝试克隆; 对实现了 Clone 的类型直接克隆。
    pub fn cloneIfPossible<T: Clone>(obj: &T) -> T {
        obj.clone()
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 序列化（基于 serde_json）
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// 对齐 Java: `ObjectUtil.cloneByStream(T)`
    /// 通过序列化再反序列化实现深拷贝（字节流往返）。
    /// 类型须实现 `serde::Serialize` + `serde::de::DeserializeOwned`。
    pub fn cloneByStream<T: Serialize + DeserializeOwned>(obj: &T) -> Result<T> {
        let bytes = serde_json::to_vec(obj)
            .map_err(|e| CoreError::Codec(format!("cloneByStream serialize: {e}")))?;
        serde_json::from_slice(&bytes)
            .map_err(|e| CoreError::Codec(format!("cloneByStream deserialize: {e}")))
    }

    /// 对齐 Java: `ObjectUtil.serialize(T)`
    /// 将对象序列化为 JSON 字节流。
    /// 类型须实现 `serde::Serialize`。
    pub fn serialize<T: Serialize>(obj: &T) -> Result<Vec<u8>> {
        serde_json::to_vec(obj)
            .map_err(|e| CoreError::Codec(format!("serialize: {e}")))
    }

    /// 对齐 Java: `ObjectUtil.deserialize(byte[], Class<?>...)`
    /// 从 JSON 字节流反序列化为指定类型。
    /// 类型须实现 `serde::de::DeserializeOwned`。
    pub fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes)
            .map_err(|e| CoreError::Codec(format!("deserialize: {e}")))
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 类型判断
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// 对齐 Java: `ObjectUtil.isBasicType(Object)`
    /// 检查是否为基本类型（bool、char、数值类型）。
    pub fn isBasicType(value: &dyn Any) -> bool {
        value.is::<bool>()
            || value.is::<char>()
            || value.is::<i8>()
            || value.is::<i16>()
            || value.is::<i32>()
            || value.is::<i64>()
            || value.is::<i128>()
            || value.is::<isize>()
            || value.is::<u8>()
            || value.is::<u16>()
            || value.is::<u32>()
            || value.is::<u64>()
            || value.is::<u128>()
            || value.is::<usize>()
            || value.is::<f32>()
            || value.is::<f64>()
    }

    /// 对齐 Java: `ObjectUtil.isValidIfNumber(Object)`
    /// 若为浮点数则检查是否为有限值; 非数字类型视为 valid。
    pub fn isValidIfNumber(value: &dyn Any) -> bool {
        if let Some(n) = value.downcast_ref::<f64>() {
            return n.is_finite();
        }
        if let Some(n) = value.downcast_ref::<f32>() {
            return n.is_finite();
        }
        true
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 比较操作
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// 对齐 Java: `ObjectUtil.compare(T, T)`
    /// 使用 `Ord` trait 进行比较; None 视为小于 Some。
    pub fn compare<T: Ord>(c1: Option<&T>, c2: Option<&T>) -> i32 {
        match (c1, c2) {
            (Some(a), Some(b)) => a.cmp(b) as i32,
            (None, Some(_)) => -1,
            (Some(_), None) => 1,
            (None, None) => 0,
        }
    }

    /// 对齐 Java: `ObjectUtil.compare(T, T, boolean)`
    /// 使用 `Ord` trait 进行比较; `nullGreater` 控制 None 排序方向。
    pub fn compare_2<T: Ord>(c1: Option<&T>, c2: Option<&T>, nullGreater: bool) -> i32 {
        match (c1, c2) {
            (Some(a), Some(b)) => a.cmp(b) as i32,
            (None, Some(_)) => {
                if nullGreater {
                    1
                } else {
                    -1
                }
            }
            (Some(_), None) => {
                if nullGreater {
                    -1
                } else {
                    1
                }
            }
            (None, None) => 0,
        }
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 反射相关（基于 std::any::type_name）
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// 对齐 Java: `ObjectUtil.getTypeArgument(Object)`
    /// 返回对象的完整类型名称（Rust 中使用 `std::any::type_name` 近似 Java 反射获取泛型参数）。
    /// 返回的字符串格式为模块路径限定的完整类型名，如 `"alloc::string::String"`。
    pub fn getTypeArgument<T: ?Sized>(_obj: &T) -> &'static str {
        type_name::<T>()
    }

    /// 对齐 Java: `ObjectUtil.getTypeArgument(Object, int)`
    /// 返回对象的类型名称，并尝试按 `::` 分割取第 `index` 段。
    /// 若 `index` 越界则返回完整类型名称。
    pub fn getTypeArgument_2<T: ?Sized>(_obj: &T, index: usize) -> String {
        let full = type_name::<T>();
        full.split("::")
            .nth(index)
            .unwrap_or(full)
            .to_string()
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 转换 / 字符串
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// 对齐 Java: `ObjectUtil.toString(Object)`
    /// 使用 `Display` trait 转换为字符串; None 输出 `"null"`。
    pub fn toString<T: Display>(obj: Option<&T>) -> String {
        obj.map_or_else(|| "null".to_string(), |v| v.to_string())
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // apply / accept
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// 对齐 Java: `ObjectUtil.apply(T, Function)`
    /// source 非空时应用 handler 并返回结果。
    pub fn apply<T, R, F>(source: Option<T>, handler: F) -> Option<R>
    where
        F: FnOnce(T) -> R,
    {
        source.map(handler)
    }

    /// 对齐 Java: `ObjectUtil.accept(T, Consumer)`
    /// source 非空时消费。
    pub fn accept<T, F>(source: Option<T>, consumer: F)
    where
        F: FnOnce(T),
    {
        if let Some(value) = source {
            consumer(value);
        }
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 批量判断
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// 对齐 Java: `ObjectUtil.emptyCount(Object...)`
    /// 计算字符串切片中为空的元素个数。
    pub fn emptyCount(objs: &[Option<&str>]) -> i32 {
        objs.iter()
            .filter(|o| o.map_or(true, |s| s.is_empty()))
            .count() as i32
    }

    /// 对齐 Java: `ObjectUtil.hasNull(Object...)`
    /// 检查是否有任意元素为 None。
    pub fn hasNull<T>(objs: &[Option<&T>]) -> bool {
        objs.iter().any(|o| o.is_none())
    }

    /// 对齐 Java: `ObjectUtil.hasEmpty(Object...)`
    /// 检查是否有任意字符串元素为空。
    pub fn hasEmpty(objs: &[Option<&str>]) -> bool {
        objs.iter().any(|o| o.map_or(true, |s| s.is_empty()))
    }

    /// 对齐 Java: `ObjectUtil.isAllEmpty(Object...)`
    /// 检查是否所有字符串元素都为空。
    pub fn isAllEmpty(objs: &[Option<&str>]) -> bool {
        objs.iter().all(|o| o.map_or(true, |s| s.is_empty()))
    }

    /// 对齐 Java: `ObjectUtil.isAllNotEmpty(Object...)`
    /// 检查是否所有字符串元素都不为空。
    pub fn isAllNotEmpty(objs: &[Option<&str>]) -> bool {
        objs.iter().all(|o| o.map_or(false, |s| !s.is_empty()))
    }

    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
    // 长度 / 包含
    // ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

    /// 对齐 Java: `ObjectUtil.length(Object)` — 字符串长度; null 返回 0。
    pub fn length_str(obj: Option<&str>) -> i32 {
        obj.map_or(0, |s| i32_from_usize(s.len()))
    }

    /// 对齐 Java: `ObjectUtil.length(Object)` — String 长度。
    pub fn length_string(obj: Option<&String>) -> i32 {
        obj.map_or(0, |s| i32_from_usize(s.len()))
    }

    /// 对齐 Java: `ObjectUtil.length(Object)` — 切片长度。
    pub fn length_slice<T>(obj: Option<&[T]>) -> i32 {
        obj.map_or(0, |s| i32_from_usize(s.len()))
    }

    /// 对齐 Java: `ObjectUtil.length(Object)` — Vec 长度。
    pub fn length_vec<T>(obj: Option<&Vec<T>>) -> i32 {
        obj.map_or(0, |v| i32_from_usize(v.len()))
    }

    /// 对齐 Java: `ObjectUtil.length(Object)` — HashMap 大小。
    pub fn length_map<K: Eq + Hash, V>(obj: Option<&HashMap<K, V>>) -> i32 {
        obj.map_or(0, |m| i32_from_usize(m.len()))
    }

    /// 对齐 Java: `ObjectUtil.length(Object)` — BTreeMap 大小。
    pub fn length_btree_map<K: Ord, V>(obj: Option<&BTreeMap<K, V>>) -> i32 {
        obj.map_or(0, |m| i32_from_usize(m.len()))
    }

    /// 对齐 Java: `ObjectUtil.contains(Object, Object)` — 字符串包含检测。
    pub fn contains_str(obj: Option<&str>, element: Option<&str>) -> bool {
        match (obj, element) {
            (Some(obj), Some(element)) => obj.contains(element),
            _ => false,
        }
    }

    /// 对齐 Java: `ObjectUtil.contains(Object, Object)` — 切片包含检测。
    pub fn contains_slice<T: PartialEq>(obj: Option<&[T]>, element: Option<&T>) -> bool {
        match (obj, element) {
            (Some(obj), Some(element)) => obj.contains(element),
            _ => false,
        }
    }

    /// 对齐 Java: `ObjectUtil.contains(Object, Object)` — Vec 包含检测。
    pub fn contains_vec<T: PartialEq>(obj: Option<&Vec<T>>, element: Option<&T>) -> bool {
        match (obj, element) {
            (Some(obj), Some(element)) => obj.contains(element),
            _ => false,
        }
    }

    /// 对齐 Java: `ObjectUtil.contains(Object, Object)` — HashMap 值包含检测。
    pub fn contains_map_values<K, V: PartialEq>(
        obj: Option<&HashMap<K, V>>,
        element: Option<&V>,
    ) -> bool
    where
        K: Eq + Hash,
    {
        match (obj, element) {
            (Some(obj), Some(element)) => obj.values().any(|v| v == element),
            _ => false,
        }
    }
}

/// 内部辅助: usize 安全转 i32。
fn i32_from_usize(value: usize) -> i32 {
    i32::try_from(value).unwrap_or(i32::MAX)
}
