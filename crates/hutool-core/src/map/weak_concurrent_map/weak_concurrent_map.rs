//! 对齐: JVM 弱/软引用并发 Map
//!
//! Rust 无 GC 弱引用语义；提供 `HashMap` 包装占位，语义记为 planned。

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use crate::{CoreError, Result};

/// 对齐 Java: `WeakConcurrentMap` —— 无弱引用语义的并发 HashMap 包装。
#[derive(Debug, Clone, Default)]
pub struct WeakConcurrentMap<K, V> {
    inner: Arc<Mutex<HashMap<K, V>>>,
}

impl<K: Eq + Hash, V> WeakConcurrentMap<K, V> {
    /// 构造。
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 放入（非弱引用）。
    pub fn put(&self, key: K, value: V) -> Option<V> {
        self.inner.lock().expect("lock").insert(key, value)
    }

    /// 取值。
    pub fn get(&self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        self.inner.lock().expect("lock").get(key).cloned()
    }

    /// 声明：弱引用 GC 语义不可移植。
    pub fn weak_semantics_status() -> Result<()> {
        Err(CoreError::PendingEngine(
            "JVM WeakHashMap / SoftReference GC semantics",
        ))
    }
}
