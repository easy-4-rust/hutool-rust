//! 对齐: `cn.hutool.core.map.TreeEntry` / `ForestMap` / `LinkedForestMap`
//! 来源: hutool-core/.../LinkedForestMap.java（简化可运行实现）

use std::collections::HashMap;
use std::hash::Hash;

/// 对齐 Java 接口: `cn.hutool.core.map.TreeEntry`
#[derive(Debug, Clone)]
pub struct TreeEntry<K, V> {
    key: K,
    value: V,
    parent: Option<K>,
    children: Vec<K>,
}

impl<K: Clone + Eq + Hash, V: Clone> TreeEntry<K, V> {
    /// 新建节点。
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            parent: None,
            children: Vec::new(),
        }
    }

    /// 对齐 Java: `getKey`
    pub fn key(&self) -> &K {
        &self.key
    }

    /// 对齐 Java: `getValue`
    pub fn value(&self) -> &V {
        &self.value
    }

    /// 对齐 Java: `setValue`
    pub fn set_value(&mut self, value: V) -> V {
        std::mem::replace(&mut self.value, value)
    }

    /// 对齐 Java: `getDeclaredParent` / `hasParent`
    pub fn parent_key(&self) -> Option<&K> {
        self.parent.as_ref()
    }

    /// 对齐 Java: `hasParent`
    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    /// 子节点键。
    pub fn children(&self) -> &[K] {
        &self.children
    }
}
