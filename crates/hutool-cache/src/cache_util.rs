//! `CacheUtil`。
//!
//! 对齐 Java 类: `cn.hutool.cache.CacheUtil`
//! 来源: `hutool-cache/src/main/java/cn/hutool/cache/CacheUtil.java`
//!
//! 该对象提供 Hutool 风格的静态工厂方法，用于创建各类缓存实现。

use std::hash::Hash;
use std::time::Duration;

use crate::compat::{
    FIFOCache, LFUCache, LRUCache, NoCache, ScheduledTimedCache, TimedCache, WeakCache,
};

/// Hutool 缓存工厂方法门面。
///
/// 这里保留 Java `CacheUtil` 的“静态工具类”风格，统一创建 FIFO、LFU、LRU、
/// Timed、Weak 与 `NoCache` 等缓存实例。
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

    /// 创建不带调度器的定时缓存。
    ///
    /// 对齐 Java: `CacheUtil.newTimedCache(long)`
    pub fn new_timed_cache<K, V>(timeout: Duration) -> TimedCache<K, V>
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        TimedCache::new(timeout)
    }

    /// 创建并启动周期清理任务的定时缓存。
    ///
    /// Java 原型为 `CacheUtil.newTimedCache(long, long)`，Rust 侧保留更直观的方法名。
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
    use super::CacheUtil;
    use std::time::Duration;

    #[test]
    fn constructors_are_wired_to_expected_cache_types() {
        let _: super::FIFOCache<&str, i32> = CacheUtil::new_fifo_cache(2);
        let _: super::FIFOCache<&str, i32> =
            CacheUtil::new_fifo_cache_with_timeout(2, Duration::from_secs(1));
        let _: super::LFUCache<&str, i32> = CacheUtil::new_lfu_cache(2);
        let _: super::LFUCache<&str, i32> =
            CacheUtil::new_lfu_cache_with_timeout(2, Duration::from_secs(1));
        let _: super::LRUCache<&str, i32> = CacheUtil::new_lru_cache(2);
        let _: super::LRUCache<&str, i32> =
            CacheUtil::new_lru_cache_with_timeout(2, Duration::from_secs(1));
        let _: super::TimedCache<&str, i32> = CacheUtil::new_timed_cache(Duration::from_secs(1));
        let _: super::WeakCache<&str, i32> = CacheUtil::new_weak_cache(None);
        let _: super::NoCache<&str, i32> = CacheUtil::new_no_cache();
    }
}
