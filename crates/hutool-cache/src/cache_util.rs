//! CacheUtil — 对齐 cn.hutool.cache.CacheUtil。缓存工厂方法静态门面。

use std::hash::Hash;
use std::time::Duration;
use std::sync::Arc;
use crate::global_prune_timer::{GlobalPruneTimer, PruneHandle};
use crate::compat::{AbstractCache, CachePolicy, FIFOCache, LFUCache, LRUCache, WeakCache, NoCache, ReentrantCache, StampedCache, TimedCache, ScheduledTimedCache};

/// CacheUtil — 对齐 cn.hutool.cache.CacheUtil。缓存工厂方法静态门面。
///
/// 对齐 Java 类: `cn.hutool.cache.CacheUtil`
/// 来源: hutool-cache/src/main/java/cn/hutool/cache/CacheUtil.java
///
/// 提供各种缓存类型的工厂方法，包括 FIFO、LFU、LRU、Weak、NoCache 等。
pub struct CacheUtil;

impl CacheUtil {
    /// 创建无过期时间的 FIFO 缓存。
    ///
    /// 对齐 Java: `CacheUtil.newFifoCache(int)`
    pub fn new_fifo_cache<K, V>(capacity: usize) -> FIFOCache<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        FIFOCache::new(capacity)
    }

    /// 创建带过期时间的 FIFO 缓存。
    ///
    /// 对齐 Java: `CacheUtil.newFifoCache(int, long)`
    pub fn new_fifo_cache_with_timeout<K, V>(capacity: usize, timeout: Duration) -> FIFOCache<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        FIFOCache::with_timeout(capacity, timeout)
    }

    /// 创建无过期时间的 LFU 缓存。
    ///
    /// 对齐 Java: `CacheUtil.newLfuCache(int)`
    pub fn new_lfu_cache<K, V>(capacity: usize) -> LFUCache<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        LFUCache::new(capacity)
    }

    /// 创建带过期时间的 LFU 缓存。
    ///
    /// 对齐 Java: `CacheUtil.newLfuCache(int, long)`
    pub fn new_lfu_cache_with_timeout<K, V>(capacity: usize, timeout: Duration) -> LFUCache<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        LFUCache::with_timeout(capacity, timeout)
    }

    /// 创建无过期时间的 LRU 缓存。
    ///
    /// 对齐 Java: `CacheUtil.newLruCache(int)`
    pub fn new_lru_cache<K, V>(capacity: usize) -> LRUCache<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        LRUCache::new(capacity)
    }

    /// 创建带过期时间的 LRU 缓存。
    ///
    /// 对齐 Java: `CacheUtil.newLruCache(int, long)`
    pub fn new_lru_cache_with_timeout<K, V>(capacity: usize, timeout: Duration) -> LRUCache<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        LRUCache::with_timeout(capacity, timeout)
    }

    /// 创建非定时清理的 TimedCache。
    ///
    /// 对齐 Java: `CacheUtil.newTimedCache(long)`
    pub fn new_timed_cache<K, V>(timeout: Duration) -> TimedCache<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        TimedCache::new(timeout)
    }

    /// 创建并调度 TimedCache。
    ///
    /// 对齐 Java: `CacheUtil.newScheduledTimedCache(long, long)`
    pub fn new_scheduled_timed_cache<K, V>(
        timeout: Duration,
        delay: Duration,
    ) -> Result<ScheduledTimedCache<K, V>, &'static str>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        let cache = TimedCache::new(timeout);
        cache.schedule_prune(delay)?;
        Ok(ScheduledTimedCache { cache })
    }

    /// 创建弱引用缓存。
    ///
    /// 对齐 Java: `CacheUtil.newWeakCache(long)`
    pub fn new_weak_cache<K, V>(timeout: Option<Duration>) -> WeakCache<K, V>
    where
        K: Eq + Hash + Clone,
    {
        WeakCache::new(timeout)
    }

    /// 创建空操作缓存（不存储任何内容）。
    ///
    /// 对齐 Java: `CacheUtil.newNoCache()`
    pub const fn new_no_cache<K, V>() -> NoCache<K, V> {
        NoCache::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parking_lot::Mutex;
    use std::{
        sync::atomic::{AtomicUsize, Ordering},
        thread,
    };

    fn wait_for_expiry() {
        thread::sleep(Duration::from_millis(12));
    }

    #[test]
    fn fifo_lru_and_lfu_apply_deterministic_eviction() {
        let fifo = FIFOCache::new(2);
        fifo.put("a", 1);
        fifo.put("b", 2);
        assert_eq!(*fifo.get(&"a").unwrap(), 1);
        fifo.put("c", 3);
        assert!(!fifo.contains_key(&"a"));
        assert!(fifo.contains_key(&"b"));

        let lru = LRUCache::new(2);
        lru.put("a", 1);
        lru.put("b", 2);
        assert_eq!(*lru.get(&"a").unwrap(), 1);
        lru.put("c", 3);
        assert!(lru.contains_key(&"a"));
        assert!(!lru.contains_key(&"b"));

        let lfu = LFUCache::new(2);
        lfu.put("a", 1);
        lfu.put("b", 2);
        assert_eq!(*lfu.get(&"a").unwrap(), 1);
        assert_eq!(*lfu.get(&"a").unwrap(), 1);
        assert_eq!(*lfu.get(&"b").unwrap(), 2);
        lfu.put("c", 3);
        assert!(lfu.contains_key(&"a"));
        assert!(!lfu.contains_key(&"b"));

        assert!(format!("{fifo:?}{lru:?}{lfu:?}").contains("Cache"));
    }

    #[test]
    fn abstract_cache_covers_expiration_refresh_counters_and_views() {
        let cache = AbstractCache::new(0, Some(Duration::from_millis(8)), CachePolicy::Timed);
        assert_eq!(cache.capacity(), 0);
        assert_eq!(cache.timeout(), Some(Duration::from_millis(8)));
        assert!(!cache.is_full());
        assert!(cache.is_empty());

        cache.put("a", 1);
        cache.put_arc("b", Arc::new(2));
        let objects = cache.cache_objects();
        assert_eq!(objects.len(), 2);
        let object = objects.iter().find(|item| item.key() == &"a").unwrap();
        assert_eq!(*object.value(), 1);
        assert_eq!(object.ttl(), Some(Duration::from_millis(8)));
        assert!(object.created_at() <= object.last_access());
        assert!(object.expired_time().is_some());
        assert!(!object.is_expired());
        assert!(format!("{object:?}").contains("CacheObj"));
        assert_eq!(cache.key_set().len(), 2);
        assert_eq!(cache.values().len(), 2);
        assert_eq!(cache.size(), 2);

        assert_eq!(*cache.get_without_refresh(&"a").unwrap(), 1);
        assert_eq!(cache.get(&"missing"), None);
        assert_eq!(cache.hit_count(), 1);
        assert_eq!(cache.miss_count(), 1);
        wait_for_expiry();
        assert!(!cache.contains_key(&"a"));
        assert_eq!(cache.get(&"b"), None);
        assert!(cache.miss_count() >= 2);
        assert_eq!(cache.prune(), 0);
        assert!(cache.is_empty());
        assert!(format!("{cache:?}").contains("AbstractCache"));
    }

    #[test]
    fn factories_listeners_replacement_clear_and_unlimited_capacity_work() {
        let removed = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&removed);
        let cache = AbstractCache::new(1, None, CachePolicy::Fifo);
        cache.set_listener(move |key: &&str, value: &i32| {
            sink.lock().push(((*key).to_owned(), *value));
        });
        cache.put("a", 1);
        assert!(cache.is_full());
        cache.put("a", 2);
        cache.put("b", 3);
        assert_eq!(*cache.remove(&"b").unwrap(), 3);
        assert_eq!(cache.remove(&"missing"), None);
        cache.put("c", 4);
        cache.clear();
        assert!(removed.lock().len() >= 4);
        cache.clear_listener();
        cache.put("d", 5);
        cache.clear();

        let unlimited = FIFOCache::new(0);
        unlimited.put("a", 1);
        unlimited.put("b", 2);
        assert_eq!(unlimited.size(), 2);
        let _reentrant: ReentrantCache<&str, i32> = (*unlimited).clone();
        let _stamped: StampedCache<&str, i32> = (*unlimited).clone();
    }

    #[test]
    fn get_or_insert_and_per_entry_timeout_cover_hit_and_miss_paths() {
        fn one() -> i32 {
            1
        }
        fn three() -> i32 {
            3
        }

        let cache = LRUCache::new(4);
        assert_eq!(*cache.get_or_insert_with("a", one as fn() -> i32), 1);
        assert_eq!(*cache.get_or_insert_with("a", one as fn() -> i32), 1);
        cache.put_with_timeout("short", 2, Duration::from_millis(5));
        assert_eq!(
            *cache.get_or_insert_with_timeout("custom", None, three as fn() -> i32),
            3
        );

        let shared = Arc::new(LRUCache::new(4));
        let calls = Arc::new(AtomicUsize::new(0));
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let shared = Arc::clone(&shared);
                let calls = Arc::clone(&calls);
                thread::spawn(move || {
                    shared.get_or_insert_with("once", || {
                        calls.fetch_add(1, Ordering::Relaxed);
                        7
                    })
                })
            })
            .collect();
        for handle in handles {
            assert_eq!(*handle.join().unwrap(), 7);
        }
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert_eq!(
            *cache.get_or_insert_with_timeout("custom", None, three as fn() -> i32),
            3
        );
        wait_for_expiry();
        assert_eq!(cache.get(&"short"), None);
        assert_eq!(*cache.get(&"custom").unwrap(), 3);
        let timeless = cache
            .cache_objects()
            .into_iter()
            .find(|item| item.key() == &"custom")
            .unwrap();
        assert_eq!(timeless.ttl(), None);
        assert_eq!(timeless.expired_time(), None);
    }

    #[test]
    fn timed_cache_schedules_replaces_and_cancels_workers() {
        let cache = TimedCache::new(Duration::from_millis(4));
        let clone = cache.clone();
        assert_eq!(clone.timeout(), cache.timeout());
        assert!(cache.schedule_prune(Duration::ZERO).is_err());
        assert!(!cache.cancel_prune_schedule());
        cache.put("a", 1);
        cache.schedule_prune(Duration::from_millis(2)).unwrap();
        cache.schedule_prune(Duration::from_millis(2)).unwrap();
        thread::sleep(Duration::from_millis(16));
        assert!(cache.is_empty());
        assert!(cache.cancel_prune_schedule());
        assert!(!cache.cancel_prune_schedule());
        assert!(format!("{cache:?}").contains("TimedCache"));

        let scheduled = CacheUtil::new_scheduled_timed_cache::<&str, i32>(
            Duration::from_millis(3),
            Duration::from_millis(2),
        )
        .unwrap();
        assert!(format!("{scheduled:?}").contains("ScheduledTimedCache"));
        assert!(
            CacheUtil::new_scheduled_timed_cache::<&str, i32>(
                Duration::from_millis(3),
                Duration::ZERO
            )
            .is_err()
        );
    }

    #[test]
    fn explicit_prune_handle_runs_and_stops() {
        let calls = Arc::new(AtomicUsize::new(0));
        let sink = Arc::clone(&calls);
        let handle = GlobalPruneTimer::schedule(
            move || {
                sink.fetch_add(1, Ordering::Relaxed);
            },
            Duration::from_millis(2),
        );
        assert!(format!("{handle:?}").contains("PruneHandle"));
        thread::sleep(Duration::from_millis(8));
        drop(handle);
        assert!(calls.load(Ordering::Relaxed) > 0);
        let zero_delay = GlobalPruneTimer::schedule(|| {}, Duration::ZERO);
        thread::sleep(Duration::from_millis(3));
        drop(zero_delay);
        drop(PruneHandle {
            stop: None,
            worker: None,
        });
        GlobalPruneTimer::create();
        GlobalPruneTimer::shutdown();
        GlobalPruneTimer::shutdown_now();
    }

    #[test]
    fn weak_cache_observes_arc_lifetime_timeout_listener_and_prune() {
        let removed = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&removed);
        let cache = WeakCache::new(Some(Duration::from_millis(5)));
        cache.set_listener(move |key: &&str, value: &String| {
            sink.lock().push(((**key).to_owned(), value.clone()));
        });
        assert_eq!(cache.capacity(), 0);
        assert_eq!(cache.timeout(), Some(Duration::from_millis(5)));
        assert!(cache.is_empty());
        let value = Arc::new(String::from("value"));
        cache.put("live", &value);
        assert!(cache.contains_key(&"live"));
        assert_eq!(cache.size(), 1);
        assert_eq!(cache.remove(&"live").unwrap().as_str(), "value");
        assert_eq!(removed.lock().len(), 1);

        cache.put("expired", &value);
        wait_for_expiry();
        assert_eq!(cache.get(&"expired"), None);
        assert_eq!(removed.lock().len(), 2);
        cache.put("dead", &value);
        drop(value);
        assert_eq!(cache.prune(), 1);
        assert_eq!(cache.remove(&"absent"), None);
        let live = Arc::new(String::from("clear"));
        cache.put("clear", &live);
        cache.clear();
        assert!(cache.is_empty());

        let without_listener = WeakCache::new(None);
        without_listener.put("value", &live);
        assert_eq!(without_listener.remove(&"value").unwrap().as_str(), "clear");
    }

    #[test]
    fn no_cache_and_all_cache_util_constructors_are_usable() {
        let cache = NoCache::<&str, i32>::new();
        cache.put("a", 1);
        cache.put_with_timeout("b", 2, Duration::from_secs(1));
        assert_eq!(cache.get(&"a"), None);
        assert_eq!(*cache.get_or_insert_with("a", || 3), 3);
        assert!(!cache.contains_key(&"a"));
        assert_eq!(cache.values().count(), 0);
        assert_eq!(cache.cache_objects().count(), 0);
        cache.remove(&"a");
        cache.clear();
        assert_eq!(cache.prune(), 0);
        assert!(!cache.is_full());
        assert_eq!(cache.capacity(), 0);
        assert_eq!(cache.timeout(), None);
        assert_eq!(cache.size(), 0);
        assert!(cache.is_empty());

        let _: FIFOCache<&str, i32> = CacheUtil::new_fifo_cache(2);
        let _: FIFOCache<&str, i32> =
            CacheUtil::new_fifo_cache_with_timeout(2, Duration::from_secs(1));
        let _: LFUCache<&str, i32> = CacheUtil::new_lfu_cache(2);
        let _: LFUCache<&str, i32> =
            CacheUtil::new_lfu_cache_with_timeout(2, Duration::from_secs(1));
        let _: LRUCache<&str, i32> = CacheUtil::new_lru_cache(2);
        let _: LRUCache<&str, i32> =
            CacheUtil::new_lru_cache_with_timeout(2, Duration::from_secs(1));
        let _: TimedCache<&str, i32> = CacheUtil::new_timed_cache(Duration::from_secs(1));
        let _: WeakCache<&str, i32> = CacheUtil::new_weak_cache(None);
        let _: NoCache<&str, i32> = CacheUtil::new_no_cache();
    }
}
