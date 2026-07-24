//! Common [`Cache`] trait used across all Hutool-aligned cache engines.
//!
//! Hutool 对齐: `cn.hutool.cache.Cache`

use std::hash::Hash;

use crate::impl::cache_obj::CacheObj;

use super::cache::Cache;

/// Blanket helper that forwards the standard `Iterator` shape to a `Cache`.
pub trait CacheIntoIterator<K, V>: Cache<K, V> {
    /// Returns an iterator over the cached values.
    fn iter_values(&self) -> Box<dyn Iterator<Item = std::sync::Arc<V>> + '_>;
}

impl<K, V, T> CacheIntoIterator<K, V> for T
where
    K: Eq + Hash,
    T: Cache<K, V>,
{
    fn iter_values(&self) -> Box<dyn Iterator<Item = std::sync::Arc<V>> + '_> {
        Box::new(
            self.cache_obj_iter()
                .map(|object| object.value()),
        )
    }
}
