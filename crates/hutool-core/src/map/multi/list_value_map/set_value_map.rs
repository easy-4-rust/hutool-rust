//! 对齐: `cn.hutool.core.map.multi.AbsCollValueMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/multi/AbsCollValueMap.java

use std::collections::HashMap;
use std::hash::Hash;

/// 对齐 Java: `SetValueMap`
#[derive(Debug, Clone, Default)]
pub struct SetValueMap<K, V> {
    inner: HashMap<K, Vec<V>>,
}

impl<K: Eq + Hash, V: PartialEq> SetValueMap<K, V> {
    /// 默认构造。
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// 放入（去重）。
    pub fn put_value(&mut self, key: K, value: V) {
        let list = self.inner.entry(key).or_default();
        if !list.contains(&value) {
            list.push(value);
        }
    }

    /// 获取。
    pub fn get(&self, key: &K) -> Option<&Vec<V>> {
        self.inner.get(key)
    }

    /// 移除单个值。
    pub fn remove_value(&mut self, key: &K, value: &V) -> bool {
        if let Some(list) = self.inner.get_mut(key) {
            let before = list.len();
            list.retain(|v| v != value);
            return list.len() != before;
        }
        false
    }

    /// 键数。
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// 是否为空。
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
