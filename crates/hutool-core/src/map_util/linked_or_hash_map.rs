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

/// `newHashMap(isLinked)` 的 Rust 表达：有序用 IndexMap，无序用 HashMap。
#[derive(Debug, Clone)]
pub enum LinkedOrHashMap<K, V> {
    /// 对齐 `LinkedHashMap`
    Linked(IndexMap<K, V>),
    /// 对齐 `HashMap`
    Hash(HashMap<K, V>),
}

impl<K: Eq + Hash, V> LinkedOrHashMap<K, V> {
    /// 插入键值。
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        match self {
            Self::Linked(m) => m.insert(k, v),
            Self::Hash(m) => m.insert(k, v),
        }
    }

    /// 获取值引用。
    pub fn get(&self, k: &K) -> Option<&V> {
        match self {
            Self::Linked(m) => m.get(k),
            Self::Hash(m) => m.get(k),
        }
    }

    /// 条目数。
    pub fn len(&self) -> usize {
        match self {
            Self::Linked(m) => m.len(),
            Self::Hash(m) => m.len(),
        }
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

use super::{DEFAULT_INITIAL_CAPACITY, DEFAULT_LOAD_FACTOR, simple_to_camel_case};
