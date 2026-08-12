//! 对齐: `cn.hutool.core.lang.SimpleCache`

#![allow(dead_code)] // 对齐 Java SimpleCache，暂未接线，预留

use parking_lot::RwLock;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;

/// 对齐 Java: `SimpleCache<K,V>`
#[derive(Clone, Default)]
pub struct SimpleCache<K, V> {
    inner: Arc<RwLock<HashMap<K, V>>>,
}

impl<K: Eq + Hash + Clone, V: Clone> SimpleCache<K, V> {
    /// 创建空缓存
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 对齐 `put`
    pub fn put(&self, key: K, value: V) -> Option<V> {
        self.inner.write().insert(key, value)
    }

    /// 对齐 `get(K)`
    pub fn get(&self, key: &K) -> Option<V> {
        self.inner.read().get(key).cloned()
    }

    /// 对齐 `get(K, Supplier)`
    pub fn get_or_put<F>(&self, key: K, supplier: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(v) = self.get(&key) {
            return v;
        }
        let mut w = self.inner.write();
        if let Some(v) = w.get(&key) {
            return v.clone();
        }
        let v = supplier();
        w.insert(key, v.clone());
        v
    }

    /// 对齐 `remove`
    pub fn remove(&self, key: &K) -> Option<V> {
        self.inner.write().remove(key)
    }

    /// 对齐 `clear`
    pub fn clear(&self) {
        self.inner.write().clear();
    }

    /// 对齐 `get(K, Predicate, Supplier)` — 值不满足谓词时重新加载。
    pub fn get_if_valid_or_put<F, P>(&self, key: K, valid: P, supplier: F) -> V
    where
        F: FnOnce() -> V,
        P: Fn(&V) -> bool,
    {
        if let Some(v) = self.get(&key) {
            if valid(&v) {
                return v;
            }
            self.remove(&key);
        }
        self.get_or_put(key, supplier)
    }

    /// 当前条目数（测试辅助）。
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.read().len()
    }

    /// 是否为空。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod simple_cache_idiomatic_parity {
    use super::*;

    /// 对齐 Java SimpleCache put/get/remove/clear 可执行证据。
    #[test]
    fn simple_cache_put_get_or_put_and_clear() {
        let c = SimpleCache::new();
        assert!(c.put("k", 1).is_none());
        assert_eq!(c.get(&"k"), Some(1));
        assert_eq!(c.get_or_put("k", || 9), 1);
        assert_eq!(c.get_or_put("m", || 2), 2);
        assert_eq!(c.get_if_valid_or_put("m", |v| *v > 0, || 3), 2);
        assert_eq!(c.remove(&"k"), Some(1));
        c.clear();
        assert!(c.is_empty());
    }
}
