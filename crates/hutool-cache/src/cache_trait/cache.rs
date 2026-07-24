//! Common [`Cache`] trait used across all Hutool-aligned cache engines.
//!
//! Hutool 对齐: `cn.hutool.cache.Cache`

use std::hash::Hash;

use crate::impl::cache_obj::CacheObj;

/// Hutool-aligned cache trait.
///
/// All concrete caches — including [`AbstractCache`](crate::impl::AbstractCache)
/// and its subclasses — implement this trait so that consumers can write
/// polymorphic code that doesn't care which eviction policy is in use.
pub trait Cache<K, V>: Send + Sync {
    /// Returns the configured capacity; zero means unbounded.
    fn capacity(&self) -> usize;

    /// Returns the default time-to-idle; `None` means no default expiration.
    fn timeout(&self) -> Option<std::time::Duration>;

    /// Inserts a key/value pair using the default timeout.
    fn put(&self, key: K, value: V);

    /// Inserts a key/value pair using an explicit timeout. Zero disables expiration.
    fn put_with_timeout(&self, key: K, value: V, timeout: std::time::Duration);

    /// Gets a value by key, refreshing the last-access timestamp.
    fn get(&self, key: &K) -> Option<std::sync::Arc<V>>;

    /// Gets a value by key without refreshing the last-access timestamp.
    fn get_without_refresh(&self, key: &K) -> Option<std::sync::Arc<V>>;

    /// Gets-or-computes a value, then caches it.
    fn get_or_insert_with<F>(&self, key: K, factory: F) -> std::sync::Arc<V>
    where
        F: FnOnce() -> V;

    /// Returns whether a live entry exists for `key`.
    fn contains_key(&self, key: &K) -> bool;

    /// Returns a snapshot iterator over live [`CacheObj`] entries.
    fn cache_obj_iter(&self) -> Box<dyn Iterator<Item = CacheObj<K, V>> + '_>;

    /// Removes and returns expired entries. Returns the number removed.
    fn prune(&self) -> usize;

    /// Returns whether the cache is full (capacity-bounded only).
    fn is_full(&self) -> bool;

    /// Removes one entry by key.
    fn remove(&self, key: &K);

    /// Clears all entries.
    fn clear(&self);

    /// Returns the number of cached entries.
    fn size(&self) -> usize;

    /// Returns whether the cache is empty.
    fn is_empty(&self) -> bool;
}
