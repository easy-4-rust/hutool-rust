//! Callback invoked after an entry leaves a cache.
//!
//! Hutool 对齐: `cn.hutool.cache.CacheListener`

use std::sync::Arc;

use super::cache_listener::CacheListener;

/// Type-erased shared reference to a [`CacheListener`].
pub type SharedListener<K, V> = Arc<dyn CacheListener<K, V>>;
