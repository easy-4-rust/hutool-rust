//! 对齐: `cn.hutool.core.map.MapUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/MapUtil.java
//!
//! Rust 版本提供 Map 操作的 idiomatic 实现。保留既有 `&HashMap` API，
//! 并补充 Option / IndexMap / BTreeMap / 可变就地编辑等 Hutool 同名能力。

use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::hash::Hash;

use indexmap::IndexMap;

use crate::{CoreError, Result};

use super::map_util::MapUtil;

/// `MapUtil.empty(Class)` 所支持的 Map 种类 —— 对齐 Java `Class<?>` 分支。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmptyMapKind {
    /// `java.util.Map`
    Map,
    /// `java.util.SortedMap`
    SortedMap,
    /// `java.util.NavigableMap`
    NavigableMap,
    /// `java.util.TreeMap`（Hutool 不支持 empty）
    TreeMap,
}

use super::{DEFAULT_INITIAL_CAPACITY, DEFAULT_LOAD_FACTOR, simple_to_camel_case};
