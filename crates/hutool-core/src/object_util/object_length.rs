//! 对齐: `cn.hutool.core.util.ObjectUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ObjectUtil.java
//!
//! Rust 版本提供对象操作的 idiomatic 实现。

use std::any::Any;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

use super::object_util::ObjectUtil;

/// 对齐 Java `ObjectUtil.length` 支持的长度探测目标。
pub trait ObjectLength {
    /// 返回对象长度。
    fn object_length(&self) -> i32;
}

impl ObjectLength for str {
    fn object_length(&self) -> i32 {
        i32_from_usize(self.len())
    }
}

impl ObjectLength for String {
    fn object_length(&self) -> i32 {
        i32_from_usize(self.len())
    }
}

impl<T> ObjectLength for [T] {
    fn object_length(&self) -> i32 {
        i32_from_usize(self.len())
    }
}

impl<T> ObjectLength for Vec<T> {
    fn object_length(&self) -> i32 {
        i32_from_usize(self.len())
    }
}

impl<K, V, S> ObjectLength for HashMap<K, V, S> {
    fn object_length(&self) -> i32 {
        i32_from_usize(self.len())
    }
}

impl<K: Ord, V> ObjectLength for BTreeMap<K, V> {
    fn object_length(&self) -> i32 {
        i32_from_usize(self.len())
    }
}

fn i32_from_usize(value: usize) -> i32 {
    i32::try_from(value).unwrap_or(i32::MAX)
}
