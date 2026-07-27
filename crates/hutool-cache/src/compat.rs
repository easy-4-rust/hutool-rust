use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak};
use std::time::{Duration, Instant};

use parking_lot::{Mutex, ReentrantMutex, RwLock};
use crate::global_prune_timer::{GlobalPruneTimer, PruneHandle};

/// Deterministic eviction policy used by Hutool-aligned caches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CachePolicy {
    /// First inserted, first removed.
    Fifo,
    /// Least frequently accessed, then oldest, first removed.
    Lfu,
    /// Least recently accessed first.
    Lru,
    /// Unbounded entries removed only by expiration.
    Timed,
}

/// Callback invoked after an entry leaves a cache.
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

struct Entry<V> {
    value: Arc<V>,
    ttl: Option<Duration>,
    created_at: Instant,
    last_access: Instant,
    created_seq: u64,
    last_access_seq: u64,
    accesses: u64,
}

impl<V> Entry<V> {
    fn is_expired_at(&self, now: Instant) -> bool {
        self.ttl
            .is_some_and(|ttl| now.saturating_duration_since(self.last_access) >= ttl)
    }
}

/// Snapshot of one cached object and its expiration/access metadata.
#[derive(Clone)]
pub struct CacheObj<K, V> {
    key: K,
    value: Arc<V>,
    ttl: Option<Duration>,
    created_at: Instant,
    last_access: Instant,
}

impl<K, V> CacheObj<K, V> {
    /// Returns the key.
    pub const fn key(&self) -> &K {
        &self.key
    }

    /// Returns a shared value handle.
    pub fn value(&self) -> Arc<V> {
        Arc::clone(&self.value)
    }

    /// Returns the entry time-to-idle, if configured.
    pub const fn ttl(&self) -> Option<Duration> {
        self.ttl
    }

    /// Returns the insertion instant.
    pub const fn created_at(&self) -> Instant {
        self.created_at
    }

    /// Returns the last refreshed access instant.
    pub const fn last_access(&self) -> Instant {
        self.last_access
    }

    /// Returns the calculated expiration instant.
    pub fn expired_time(&self) -> Option<Instant> {
        self.ttl.and_then(|ttl| self.last_access.checked_add(ttl))
    }

    /// Returns whether the snapshot is expired now.
    pub fn is_expired(&self) -> bool {
        self.ttl
            .is_some_and(|ttl| self.last_access.elapsed() >= ttl)
    }
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for CacheObj<K, V> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CacheObj")
            .field("key", &self.key)
            .field("value", &self.value)
            .field("ttl", &self.ttl)
            .finish_non_exhaustive()
    }
}

struct State<K, V> {
    entries: HashMap<K, Entry<V>>,
}

struct CacheInner<K, V> {
    capacity: usize,
    timeout: Option<Duration>,
    policy: CachePolicy,
    state: Mutex<State<K, V>>,
    listener: RwLock<Option<Arc<dyn CacheListener<K, V>>>>,
    factory_lock: ReentrantMutex<()>,
    sequence: AtomicU64,
    hits: AtomicU64,
    misses: AtomicU64,
}

/// Thread-safe Hutool-compatible cache engine with deterministic eviction.
pub struct AbstractCache<K, V> {
    inner: Arc<CacheInner<K, V>>,
}

impl<K, V> Clone for AbstractCache<K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<K, V> fmt::Debug for AbstractCache<K, V>
where
    K: Eq + Hash,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AbstractCache")
            .field("capacity", &self.inner.capacity)
            .field("timeout", &self.inner.timeout)
            .field("policy", &self.inner.policy)
            .field("size", &self.inner.state.lock().entries.len())
            .finish()
    }
}

impl<K, V> AbstractCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    /// Creates a cache. Capacity zero means unlimited.
    #[must_use]
    pub fn new(capacity: usize, timeout: Option<Duration>, policy: CachePolicy) -> Self {
        Self {
            inner: Arc::new(CacheInner {
                capacity,
                timeout: timeout.filter(|value| !value.is_zero()),
                policy,
                state: Mutex::new(State {
                    entries: HashMap::new(),
                }),
                listener: RwLock::new(None),
                factory_lock: ReentrantMutex::new(()),
                sequence: AtomicU64::new(0),
                hits: AtomicU64::new(0),
                misses: AtomicU64::new(0),
            }),
        }
    }

    fn next_sequence(&self) -> u64 {
        self.inner.sequence.fetch_add(1, Ordering::Relaxed)
    }

    fn notify(&self, removed: Vec<(K, Arc<V>)>) {
        let listener = self.inner.listener.read().clone();
        if let Some(listener) = listener {
            for (key, value) in removed {
                listener.on_remove(&key, value.as_ref());
            }
        }
    }

    fn victim_key(&self, state: &State<K, V>) -> Option<K> {
        state
            .entries
            .iter()
            .min_by_key(|(_, entry)| match self.inner.policy {
                CachePolicy::Fifo => (entry.created_seq, 0),
                CachePolicy::Lfu => (entry.accesses, entry.created_seq),
                CachePolicy::Lru | CachePolicy::Timed => (entry.last_access_seq, 0),
            })
            .map(|(key, _)| key.clone())
    }

    fn remove_expired_locked(state: &mut State<K, V>, now: Instant) -> Vec<(K, Arc<V>)> {
        let keys: Vec<_> = state
            .entries
            .iter()
            .filter(|(_, entry)| entry.is_expired_at(now))
            .map(|(key, _)| key.clone())
            .collect();
        keys.into_iter()
            .filter_map(|key| state.entries.remove(&key).map(|entry| (key, entry.value)))
            .collect()
    }

    /// Hutool `LFUCache.pruneCache`: subtract the minimum access count from every
    /// entry and remove those that reach zero so new inserts share a fair counter.
    fn prune_lfu_when_full_locked(state: &mut State<K, V>) -> Vec<(K, Arc<V>)> {
        let Some(min_access) = state.entries.values().map(|entry| entry.accesses).min() else {
            return Vec::new();
        };
        let keys: Vec<_> = state.entries.keys().cloned().collect();
        let mut removed = Vec::new();
        for key in keys {
            let Some(entry) = state.entries.get_mut(&key) else {
                continue;
            };
            entry.accesses = entry.accesses.saturating_sub(min_access);
            if entry.accesses == 0 {
                let entry = state
                    .entries
                    .remove(&key)
                    .expect("LFU victim key was present under exclusive lock");
                removed.push((key, entry.value));
            }
        }
        removed
    }

    /// Inserts using the default timeout.
    pub fn put(&self, key: K, value: V) {
        self.put_arc_with_timeout(key, Arc::new(value), self.inner.timeout);
    }

    /// Inserts using an entry-specific time-to-idle. Zero disables expiration.
    pub fn put_with_timeout(&self, key: K, value: V, timeout: Duration) {
        self.put_arc_with_timeout(
            key,
            Arc::new(value),
            Some(timeout).filter(|value| !value.is_zero()),
        );
    }

    /// Inserts a shared value using the default timeout.
    pub fn put_arc(&self, key: K, value: Arc<V>) {
        self.put_arc_with_timeout(key, value, self.inner.timeout);
    }

    fn put_arc_with_timeout(&self, key: K, value: Arc<V>, ttl: Option<Duration>) {
        let now = Instant::now();
        let sequence = self.next_sequence();
        let mut state = self.inner.state.lock();
        let mut removed = Self::remove_expired_locked(&mut state, now);
        if let Some(previous) = state.entries.remove(&key) {
            removed.push((key.clone(), previous.value));
        }
        // Hutool: when full, prune before insert. LFU subtracts min access and
        // removes every entry that reaches zero (not a single victim).
        if self.inner.capacity > 0 && state.entries.len() >= self.inner.capacity {
            match self.inner.policy {
                CachePolicy::Lfu => {
                    removed.extend(Self::prune_lfu_when_full_locked(&mut state));
                }
                _ => {
                    let victim = self
                        .victim_key(&state)
                        .expect("a cache at positive capacity has an eviction victim");
                    let entry = state
                        .entries
                        .remove(&victim)
                        .expect("the selected eviction victim is present");
                    removed.push((victim, entry.value));
                }
            }
        }
        state.entries.insert(
            key,
            Entry {
                value,
                ttl,
                created_at: now,
                last_access: now,
                created_seq: sequence,
                last_access_seq: sequence,
                accesses: 0,
            },
        );
        drop(state);
        self.notify(removed);
    }

    fn get_internal(&self, key: &K, refresh: bool) -> Option<Arc<V>> {
        let now = Instant::now();
        let sequence = self.next_sequence();
        let mut state = self.inner.state.lock();
        if state
            .entries
            .get(key)
            .is_some_and(|entry| entry.is_expired_at(now))
        {
            let entry = state.entries.remove(key).expect("entry existed above");
            self.inner.misses.fetch_add(1, Ordering::Relaxed);
            drop(state);
            self.notify(vec![(key.clone(), entry.value)]);
            return None;
        }
        let result = state.entries.get_mut(key).map(|entry| {
            entry.accesses = entry.accesses.saturating_add(1);
            if refresh {
                entry.last_access = now;
                entry.last_access_seq = sequence;
            }
            Arc::clone(&entry.value)
        });
        if result.is_some() {
            self.inner.hits.fetch_add(1, Ordering::Relaxed);
        } else {
            self.inner.misses.fetch_add(1, Ordering::Relaxed);
        }
        result
    }

    /// Gets and refreshes an entry's last-access time.
    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        self.get_internal(key, true)
    }

    /// Gets without refreshing expiration.
    pub fn get_without_refresh(&self, key: &K) -> Option<Arc<V>> {
        self.get_internal(key, false)
    }

    /// Gets or computes and stores a value with the default timeout.
    pub fn get_or_insert_with<F>(&self, key: K, factory: F) -> Arc<V>
    where
        F: FnOnce() -> V,
    {
        self.get_or_insert_with_timeout(key, self.inner.timeout, factory)
    }

    /// Gets or computes and stores a value with an explicit optional timeout.
    pub fn get_or_insert_with_timeout<F>(
        &self,
        key: K,
        timeout: Option<Duration>,
        factory: F,
    ) -> Arc<V>
    where
        F: FnOnce() -> V,
    {
        let _factory_guard = self.inner.factory_lock.lock();
        if let Some(value) = self.get(&key) {
            return value;
        }
        let value = Arc::new(factory());
        self.put_arc_with_timeout(key, Arc::clone(&value), timeout);
        value
    }

    /// Returns whether a live entry exists without changing hit/miss counters.
    ///
    /// # Panics
    ///
    /// Panics only if the internal map changes while its exclusive lock is held.
    pub fn contains_key(&self, key: &K) -> bool {
        let now = Instant::now();
        let mut state = self.inner.state.lock();
        if state
            .entries
            .get(key)
            .is_some_and(|entry| entry.is_expired_at(now))
        {
            let entry = state
                .entries
                .remove(key)
                .expect("the expired entry was observed while holding the cache lock");
            drop(state);
            self.notify(vec![(key.clone(), entry.value)]);
            false
        } else {
            state.entries.contains_key(key)
        }
    }

    /// Removes one entry and returns its value.
    pub fn remove(&self, key: &K) -> Option<Arc<V>> {
        let removed = self.inner.state.lock().entries.remove(key);
        removed.map(|entry| {
            let value = Arc::clone(&entry.value);
            self.notify(vec![(key.clone(), entry.value)]);
            value
        })
    }

    /// Removes expired entries and returns the number removed.
    pub fn prune(&self) -> usize {
        let mut state = self.inner.state.lock();
        let removed = Self::remove_expired_locked(&mut state, Instant::now());
        let count = removed.len();
        drop(state);
        self.notify(removed);
        count
    }

    /// Clears all entries.
    pub fn clear(&self) {
        let mut state = self.inner.state.lock();
        let removed = state
            .entries
            .drain()
            .map(|(key, entry)| (key, entry.value))
            .collect();
        drop(state);
        self.notify(removed);
    }

    /// Installs or replaces a removal listener.
    pub fn set_listener<L>(&self, listener: L) -> &Self
    where
        L: CacheListener<K, V> + 'static,
    {
        *self.inner.listener.write() = Some(Arc::new(listener));
        self
    }

    /// Removes the listener.
    pub fn clear_listener(&self) {
        *self.inner.listener.write() = None;
    }

    /// Returns the configured capacity; zero means unlimited.
    pub fn capacity(&self) -> usize {
        self.inner.capacity
    }

    /// Returns the default timeout.
    pub fn timeout(&self) -> Option<Duration> {
        self.inner.timeout
    }

    /// Returns whether a bounded cache has reached capacity.
    pub fn is_full(&self) -> bool {
        self.inner.capacity > 0 && self.size() >= self.inner.capacity
    }

    /// Returns the current entry count.
    pub fn size(&self) -> usize {
        self.inner.state.lock().entries.len()
    }

    /// Returns whether there are no entries.
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Returns the hit count.
    pub fn hit_count(&self) -> u64 {
        self.inner.hits.load(Ordering::Relaxed)
    }

    /// Returns the miss count.
    pub fn miss_count(&self) -> u64 {
        self.inner.misses.load(Ordering::Relaxed)
    }

    /// Returns a snapshot of keys.
    pub fn key_set(&self) -> HashSet<K> {
        self.inner.state.lock().entries.keys().cloned().collect()
    }

    /// Returns shared value snapshots.
    pub fn values(&self) -> Vec<Arc<V>> {
        self.inner
            .state
            .lock()
            .entries
            .values()
            .map(|entry| Arc::clone(&entry.value))
            .collect()
    }

    /// Returns cache-object snapshots.
    pub fn cache_objects(&self) -> Vec<CacheObj<K, V>> {
        self.inner
            .state
            .lock()
            .entries
            .iter()
            .map(|(key, entry)| CacheObj {
                key: key.clone(),
                value: Arc::clone(&entry.value),
                ttl: entry.ttl,
                created_at: entry.created_at,
                last_access: entry.last_access,
            })
            .collect()
    }
}

/// Hutool's lock implementation maps to the same safe Rust engine.
pub type ReentrantCache<K, V> = AbstractCache<K, V>;
/// Hutool's stamped-lock implementation maps to the same safe Rust engine.
pub type StampedCache<K, V> = AbstractCache<K, V>;

macro_rules! bounded_cache {
    ($name:ident, $policy:expr) => {
        #[doc = concat!(stringify!($name), " cache.")]
        #[derive(Clone)]
        pub struct $name<K, V>(AbstractCache<K, V>);

        impl<K, V> fmt::Debug for $name<K, V>
        where
            K: Eq + Hash,
        {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&self.0)
                    .finish()
            }
        }

        impl<K, V> $name<K, V>
        where
            K: Eq + Hash + Clone + Send + Sync + 'static,
            V: Send + Sync + 'static,
        {
            /// Creates a cache without expiration.
            #[must_use]
            pub fn new(capacity: usize) -> Self {
                Self(AbstractCache::new(capacity, None, $policy))
            }

            /// Creates a cache with default time-to-idle.
            #[must_use]
            pub fn with_timeout(capacity: usize, timeout: Duration) -> Self {
                Self(AbstractCache::new(capacity, Some(timeout), $policy))
            }
        }

        impl<K, V> std::ops::Deref for $name<K, V> {
            type Target = AbstractCache<K, V>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

bounded_cache!(FIFOCache, CachePolicy::Fifo);
bounded_cache!(LFUCache, CachePolicy::Lfu);
bounded_cache!(LRUCache, CachePolicy::Lru);

/// Unbounded expiring cache with optional scheduled pruning.
pub struct TimedCache<K, V> {
    cache: AbstractCache<K, V>,
    prune_handle: Arc<Mutex<Option<PruneHandle>>>,
}

impl<K, V> Clone for TimedCache<K, V> {
    fn clone(&self) -> Self {
        Self {
            cache: self.cache.clone(),
            prune_handle: Arc::clone(&self.prune_handle),
        }
    }
}

impl<K, V> fmt::Debug for TimedCache<K, V>
where
    K: Eq + Hash,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("TimedCache")
            .field(&self.cache)
            .finish()
    }
}

impl<K, V> TimedCache<K, V>
where
    K: Eq + Hash + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    /// Creates an unbounded timed cache.
    #[must_use]
    pub fn new(timeout: Duration) -> Self {
        Self {
            cache: AbstractCache::new(0, Some(timeout), CachePolicy::Timed),
            prune_handle: Arc::new(Mutex::new(None)),
        }
    }

    /// Starts per-cache scheduled pruning, replacing an existing schedule.
    pub fn schedule_prune(&self, delay: Duration) -> Result<(), &'static str> {
        if delay.is_zero() {
            return Err("prune delay must be greater than zero");
        }
        self.cancel_prune_schedule();
        let cache = self.cache.clone();
        *self.prune_handle.lock() = Some(GlobalPruneTimer::schedule(
            move || {
                cache.prune();
            },
            delay,
        ));
        Ok(())
    }

    /// Cancels scheduled pruning.
    pub fn cancel_prune_schedule(&self) -> bool {
        self.prune_handle.lock().take().is_some()
    }
}

impl<K, V> std::ops::Deref for TimedCache<K, V> {
    type Target = AbstractCache<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.cache
    }
}

struct WeakEntry<V> {
    value: Weak<V>,
    ttl: Option<Duration>,
    last_access: Instant,
}

/// Explicit weak-value cache using Rust `Arc`/`Weak` ownership.
pub struct WeakCache<K, V> {
    timeout: Option<Duration>,
    entries: Mutex<HashMap<K, WeakEntry<V>>>,
    listener: RwLock<Option<Arc<dyn CacheListener<K, V>>>>,
    /// Per-key locks for Hutool-aligned `get(key, supplier)` double-checked loading.
    key_locks: Mutex<HashMap<K, Arc<Mutex<()>>>>,
}

impl<K, V> WeakCache<K, V>
where
    K: Eq + Hash + Clone,
{
    /// Creates a weak-value cache.
    #[must_use]
    pub fn new(timeout: Option<Duration>) -> Self {
        Self {
            timeout: timeout.filter(|value| !value.is_zero()),
            entries: Mutex::new(HashMap::new()),
            listener: RwLock::new(None),
            key_locks: Mutex::new(HashMap::new()),
        }
    }

    /// Stores a weak reference to a shared value.
    pub fn put(&self, key: K, value: &Arc<V>) {
        self.entries.lock().insert(
            key,
            WeakEntry {
                value: Arc::downgrade(value),
                ttl: self.timeout,
                last_access: Instant::now(),
            },
        );
    }

    /// Gets a live value and refreshes its last access.
    pub fn get(&self, key: &K) -> Option<Arc<V>> {
        let now = Instant::now();
        let mut entries = self.entries.lock();
        let expired = entries.get(key).is_some_and(|entry| {
            entry.value.strong_count() == 0
                || entry
                    .ttl
                    .is_some_and(|ttl| now.saturating_duration_since(entry.last_access) >= ttl)
        });
        if expired {
            let value = entries.remove(key).and_then(|entry| entry.value.upgrade());
            drop(entries);
            if let (Some(listener), Some(value)) = (self.listener.read().clone(), value.as_ref()) {
                listener.on_remove(key, value.as_ref());
            }
            return None;
        }
        entries.get_mut(key).and_then(|entry| {
            entry.last_access = now;
            entry.value.upgrade()
        })
    }

    /// Hutool `Cache.get(key, supplier)`: load-or-compute under a per-key lock.
    ///
    /// The returned [`Arc`] must be retained by the caller; dropping every strong
    /// reference lets the weak entry evaporate (Rust ownership vs Java GC).
    pub fn get_or_insert_with<F>(&self, key: K, factory: F) -> Arc<V>
    where
        F: FnOnce() -> V,
        V: Send + Sync + 'static,
    {
        if let Some(value) = self.get(&key) {
            return value;
        }
        let lock = {
            let mut locks = self.key_locks.lock();
            Arc::clone(
                locks
                    .entry(key.clone())
                    .or_insert_with(|| Arc::new(Mutex::new(()))),
            )
        };
        let _guard = lock.lock();
        if let Some(value) = self.get(&key) {
            return value;
        }
        let value = Arc::new(factory());
        self.put(key, &value);
        value
    }

    /// Returns whether a live weak entry exists.
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Removes one weak entry and notifies the listener if the value is alive.
    pub fn remove(&self, key: &K) -> Option<Arc<V>> {
        let value = self.entries.lock().remove(key)?.value.upgrade();
        if let (Some(listener), Some(value)) = (self.listener.read().clone(), value.as_ref()) {
            listener.on_remove(key, value.as_ref());
        }
        value
    }

    /// Removes expired or dropped values.
    pub fn prune(&self) -> usize {
        let before = self.entries.lock().len();
        let keys: Vec<_> = self.entries.lock().keys().cloned().collect();
        for key in keys {
            let _ = self.get(&key);
        }
        before.saturating_sub(self.entries.lock().len())
    }

    /// Clears all weak entries.
    pub fn clear(&self) {
        let keys: Vec<_> = self.entries.lock().keys().cloned().collect();
        for key in keys {
            let _ = self.remove(&key);
        }
    }

    /// Installs a removal listener.
    pub fn set_listener<L>(&self, listener: L) -> &Self
    where
        L: CacheListener<K, V> + 'static,
    {
        *self.listener.write() = Some(Arc::new(listener));
        self
    }

    /// Returns the number of tracked weak entries.
    pub fn size(&self) -> usize {
        self.entries.lock().len()
    }

    /// Returns whether no weak entries are tracked.
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Weak caches are unbounded.
    pub const fn capacity(&self) -> usize {
        0
    }

    /// Returns the default timeout.
    pub const fn timeout(&self) -> Option<Duration> {
        self.timeout
    }
}

/// Cache implementation that deliberately stores nothing.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoCache<K, V>(PhantomData<fn(K) -> V>);

impl<K, V> NoCache<K, V> {
    /// Creates a no-op cache.
    pub const fn new() -> Self {
        Self(PhantomData)
    }

    /// No-op insert.
    pub fn put(&self, _key: K, _value: V) {}

    /// No-op insert with timeout.
    pub fn put_with_timeout(&self, _key: K, _value: V, _timeout: Duration) {}

    /// Always misses.
    pub const fn get(&self, _key: &K) -> Option<Arc<V>> {
        None
    }

    /// Produces but does not store a value.
    pub fn get_or_insert_with<F>(&self, _key: K, factory: F) -> Arc<V>
    where
        F: FnOnce() -> V,
    {
        Arc::new(factory())
    }

    /// Always returns false.
    pub const fn contains_key(&self, _key: &K) -> bool {
        false
    }

    /// Returns an empty iterator.
    pub fn values(&self) -> std::iter::Empty<Arc<V>> {
        std::iter::empty()
    }

    /// Returns an empty cache-object iterator.
    pub fn cache_objects(&self) -> std::iter::Empty<CacheObj<K, V>> {
        std::iter::empty()
    }

    /// Removes nothing.
    pub const fn remove(&self, _key: &K) {}

    /// Clears nothing.
    pub const fn clear(&self) {}

    /// Prunes nothing.
    pub const fn prune(&self) -> usize {
        0
    }

    /// A no-op cache is never full.
    pub const fn is_full(&self) -> bool {
        false
    }

    /// Capacity is zero.
    pub const fn capacity(&self) -> usize {
        0
    }

    /// Timeout is absent.
    pub const fn timeout(&self) -> Option<Duration> {
        None
    }

    /// Size is zero.
    pub const fn size(&self) -> usize {
        0
    }

    /// A no-op cache is always empty.
    pub const fn is_empty(&self) -> bool {
        true
    }
}

/// Timed cache bundled with a schedule guard when construction-time scheduling is desired.
pub struct ScheduledTimedCache<K, V> {
    /// Timed cache.
    pub cache: TimedCache<K, V>,
}

impl<K, V> fmt::Debug for ScheduledTimedCache<K, V>
where
    K: Eq + Hash,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ScheduledTimedCache")
            .field("cache", &self.cache)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cache_util::CacheUtil;
    use crate::global_prune_timer::{GlobalPruneTimer, PruneHandle};
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
        let _ = GlobalPruneTimer::shutdown_now();
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
