//! 对齐: `cn.hutool.core.lang.ConsistentHash`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/ConsistentHash.java
//!
//! 一致性哈希环：`BTreeMap` 模拟 Java `TreeMap`，默认 FNV32 哈希。

use crate::hash_util::HashUtil;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use super::hash32_fn::Hash32Fn;

/// 对齐 Java: `cn.hutool.core.lang.ConsistentHash<T>`
pub struct ConsistentHash<T> {
    number_of_replicas: usize,
    hash_fn: Hash32Fn,
    circle: BTreeMap<i32, T>,
}

impl<T: Clone + Display> ConsistentHash<T> {
    /// 对齐 Java: `ConsistentHash(int, Collection)` — 默认 FNV 哈希。
    pub fn new(number_of_replicas: usize, nodes: impl IntoIterator<Item = T>) -> Self {
        let mut ring = Self {
            number_of_replicas,
            hash_fn: Box::new(|s| HashUtil::fnv_hash(s)),
            circle: BTreeMap::new(),
        };
        for node in nodes {
            ring.add(node);
        }
        ring
    }

    /// 对齐 Java: 自定义 `Hash32` 构造。
    pub fn with_hash_fn(
        hash_fn: Hash32Fn,
        number_of_replicas: usize,
        nodes: impl IntoIterator<Item = T>,
    ) -> Self {
        let mut ring = Self {
            number_of_replicas,
            hash_fn,
            circle: BTreeMap::new(),
        };
        for node in nodes {
            ring.add(node);
        }
        ring
    }

    /// 对齐 Java: `add(T)` — 为节点写入 `numberOfReplicas` 个虚拟节点。
    pub fn add(&mut self, node: T) {
        let key = node.to_string();
        for i in 0..self.number_of_replicas {
            let h = (self.hash_fn)(&format!("{key}{i}"));
            self.circle.insert(h, node.clone());
        }
    }

    /// 对齐 Java: `remove(T)`
    pub fn remove(&mut self, node: &T) {
        let key = node.to_string();
        for i in 0..self.number_of_replicas {
            let h = (self.hash_fn)(&format!("{key}{i}"));
            self.circle.remove(&h);
        }
    }

    /// 对齐 Java: `get(Object)` — 顺时针最近虚拟节点。
    pub fn get(&self, key: &str) -> Option<T> {
        if self.circle.is_empty() {
            return None;
        }
        let hash = (self.hash_fn)(key);
        if let Some(v) = self.circle.get(&hash) {
            return Some(v.clone());
        }
        // tailMap：第一个 >= hash；否则绕回 firstKey
        if let Some((_, v)) = self.circle.range(hash..).next() {
            return Some(v.clone());
        }
        self.circle.values().next().cloned()
    }

    /// 环上虚拟节点数量（测试辅助）。
    #[must_use]
    pub fn virtual_size(&self) -> usize {
        self.circle.len()
    }
}
