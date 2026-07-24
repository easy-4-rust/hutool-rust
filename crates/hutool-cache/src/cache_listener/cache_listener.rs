//! Callback invoked after an entry leaves a cache.
//!
//! Hutool 对齐: `cn.hutool.cache.CacheListener`

use std::sync::Arc;

/// Removal callback invoked after an entry leaves a cache.
///
/// Implementors may be installed via [`AbstractCache::set_listener`] and are called
/// once per evicted key/value pair (including explicit removes, expirations, and
/// capacity-based evictions).
pub trait CacheListener<K, V>: Send + Sync {
    /// Receives the removed key and value.
    fn on_remove(&self, key: &K, value: &V);
}

impl<K, V, F> CacheListener<K, V> for F
where
    F: Fn(&K, &V) + Send + Sync,
{
    fn on_remove(&self, key: &K, value: &V) {
        self(key, value);
    }
}
