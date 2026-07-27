//! Concurrent in-process caching aligned with Hutool's cache module.

#![forbid(unsafe_code)]

use moka::sync::{Cache as MokaCache, CacheBuilder};
use std::{hash::Hash, sync::Arc, time::Duration};

mod compat;
mod global_prune_timer;
mod file_cache;
mod cache_util;

pub use compat::{
    CacheListener, ReentrantCache, FIFOCache,
    LFUCache, LRUCache, NoCache, ScheduledTimedCache, StampedCache,
    TimedCache, WeakCache,
};
pub use global_prune_timer::{GlobalPruneTimer, PruneHandle};
pub use file_cache::{AbstractFileCache, LFUFileCache, LRUFileCache};
pub use cache_util::CacheUtil;

/// Hutool-aligned implementation namespace.
pub mod r#impl {
    pub use crate::{
        FIFOCache, LFUCache, LRUCache, NoCache, ReentrantCache,
        TimedCache, WeakCache,
    };
}

/// Hutool-aligned file-cache namespace.
pub mod file {
    pub use crate::{AbstractFileCache, LFUFileCache, LRUFileCache};
}

/// Builder for a bounded, concurrent cache.
#[derive(Debug, Clone, Copy)]
pub struct CacheConfig {
    /// Maximum number of entries retained by the cache.
    pub max_capacity: u64,
    /// Optional time-to-live measured from insertion.
    pub time_to_live: Option<Duration>,
    /// Optional time-to-idle measured from the last access.
    pub time_to_idle: Option<Duration>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_capacity: 10_000,
            time_to_live: None,
            time_to_idle: None,
        }
    }
}

/// A cheap-to-clone handle to a bounded, thread-safe cache.
pub struct Cache<K, V> {
    inner: MokaCache<K, Arc<V>>,
}

impl<K, V> Clone for Cache<K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    /// Creates a cache from explicit production limits.
    #[must_use]
    pub fn new(config: CacheConfig) -> Self {
        let mut builder: CacheBuilder<K, Arc<V>, MokaCache<K, Arc<V>>> =
            MokaCache::builder().max_capacity(config.max_capacity);
        if let Some(ttl) = config.time_to_live {
            builder = builder.time_to_live(ttl);
        }
        if let Some(tti) = config.time_to_idle {
            builder = builder.time_to_idle(tti);
        }
        Self {
            inner: builder.build(),
        }
    }

    /// Inserts or replaces a cached value.
    pub fn insert(&self, key: K, value: V) {
        self.inner.insert(key, Arc::new(value));
    }

    /// Gets a shared value handle when the key is present.
    #[must_use]
    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        self.inner.get(key)
    }

    /// Invalidates one key.
    pub fn invalidate(&self, key: &K) {
        self.inner.invalidate(key);
    }

    /// Invalidates every entry.
    pub fn clear(&self) {
        self.inner.invalidate_all();
    }

    /// Returns the approximate number of entries.
    #[must_use]
    pub fn entry_count(&self) -> u64 {
        self.inner.entry_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_are_shared_and_invalidation_is_explicit() {
        let cache = Cache::new(CacheConfig {
            max_capacity: 10,
            ..CacheConfig::default()
        });
        cache.insert("answer", 42);
        assert_eq!(*cache.get(&"answer").unwrap(), 42);
        cache.invalidate(&"answer");
        assert!(cache.get(&"answer").is_none());
    }

    #[test]
    fn mature_cache_covers_clone_clear_count_ttl_tti_and_capacity() {
        let cache = Cache::new(CacheConfig {
            max_capacity: 2,
            time_to_live: Some(Duration::from_millis(8)),
            time_to_idle: Some(Duration::from_millis(5)),
        });
        let clone = cache.clone();
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.inner.run_pending_tasks();
        assert_eq!(cache.entry_count(), 2);
        assert_eq!(*clone.get(&"a").unwrap(), 1);
        std::thread::sleep(Duration::from_millis(10));
        assert!(cache.get(&"a").is_none());
        cache.clear();
        cache.inner.run_pending_tasks();
        assert_eq!(cache.entry_count(), 0);

        let bounded = Cache::new(CacheConfig {
            max_capacity: 1,
            ..CacheConfig::default()
        });
        bounded.insert("a", 1);
        bounded.insert("b", 2);
        bounded.inner.run_pending_tasks();
        assert!(bounded.entry_count() <= 1);
    }
}
