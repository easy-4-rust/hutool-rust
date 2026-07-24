//! 对齐: `cn.hutool.core.map.multi.AbsCollValueMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/multi/AbsCollValueMap.java

use std::collections::HashMap;
use std::hash::Hash;

use super::coll_value_map_ops::CollValueMapOps;

/// 对齐 Java: `ListValueMap`
#[derive(Debug, Clone, Default)]
pub struct ListValueMap<K, V> {
    inner: HashMap<K, Vec<V>>,
}

impl<K: Eq + Hash, V> ListValueMap<K, V> {
    /// 默认构造。
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// 带容量。
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: HashMap::with_capacity(capacity),
        }
    }

    /// 对齐 Java: `putValue`
    pub fn put_value(&mut self, key: K, value: V) {
        self.inner.entry(key).or_default().push(value);
    }

    /// 对齐 Java: `putAllValues`
    pub fn put_all_values(&mut self, other: HashMap<K, Vec<V>>)
    where
        K: Clone,
    {
        for (k, values) in other {
            for v in values {
                self.put_value(k.clone(), v);
            }
        }
    }

    /// 获取值列表。
    pub fn get(&self, key: &K) -> Option<&Vec<V>> {
        self.inner.get(key)
    }

    /// 对齐 Java: `getValues` —— 切片视图。
    pub fn get_values(&self, key: &K) -> Option<&[V]> {
        self.inner.get(key).map(|v| v.as_slice())
    }

    /// 对齐 Java: `removeValue`
    pub fn remove_value(&mut self, key: &K, value: &V) -> bool
    where
        V: PartialEq,
    {
        if let Some(list) = self.inner.get_mut(key) {
            let before = list.len();
            list.retain(|v| v != value);
            return list.len() != before;
        }
        false
    }

    /// 对齐 Java: `removeValues`
    pub fn remove_values(&mut self, key: &K, values: &[V]) -> bool
    where
        V: PartialEq,
    {
        if let Some(list) = self.inner.get_mut(key) {
            let before = list.len();
            list.retain(|v| !values.contains(v));
            return list.len() != before;
        }
        false
    }

    /// 清空某键。
    pub fn remove(&mut self, key: &K) -> Option<Vec<V>> {
        self.inner.remove(key)
    }

    /// 是否包含键。
    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    /// 清空。
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// 条目数（键数）。
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// 内层。
    pub fn raw(&self) -> &HashMap<K, Vec<V>> {
        &self.inner
    }
}

impl<K: Eq + Hash, V> CollValueMapOps<K, V> for ListValueMap<K, V> {
    /// 对齐 Java: `putValue`
    fn put_value(&mut self, key: K, value: V) {
        ListValueMap::put_value(self, key, value);
    }

    /// 对齐 Java: `getValues`
    fn get_values(&self, key: &K) -> Option<&[V]> {
        ListValueMap::get_values(self, key)
    }

    /// 对齐 Java: `removeValue`
    fn remove_value(&mut self, key: &K, value: &V) -> bool
    where
        V: PartialEq,
    {
        ListValueMap::remove_value(self, key, value)
    }
}
