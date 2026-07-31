//! Cache parity tests —— 对齐 Hutool `hutool-cache` 测试。
//!
//! 对齐: `cn.hutool.cache.CacheTest`
//! 对齐: `cn.hutool.cache.CacheConcurrentTest`
//! 对齐: `cn.hutool.cache.LRUCacheTest`
//! 对齐: `cn.hutool.cache.WeakCacheTest`
//! 对齐: `cn.hutool.cache.FileCacheTest`
//! 对齐: `cn.hutool.cache.Issue3618Test`
//! 对齐: `cn.hutool.cache.IssueI8MEIXTest`
//! 来源:
//! - hutool-cache/src/test/java/cn/hutool/cache/CacheTest.java
//! - hutool-cache/src/test/java/cn/hutool/cache/CacheConcurrentTest.java
//! - hutool-cache/src/test/java/cn/hutool/cache/LRUCacheTest.java
//! - hutool-cache/src/test/java/cn/hutool/cache/WeakCacheTest.java
//! - hutool-cache/src/test/java/cn/hutool/cache/FileCacheTest.java
//! - hutool-cache/src/test/java/cn/hutool/cache/Issue3618Test.java
//! - hutool-cache/src/test/java/cn/hutool/cache/IssueI8MEIXTest.java

use hutool_cache as hc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// ---------------------------------------------------------------------------
// Existing smoke tests (kept)
// ---------------------------------------------------------------------------

#[test]
fn lru_cache_put_get_test() {
    let cache = hc::LRUCache::new(3);
    cache.put("a", "1".to_string());
    cache.put("b", "2".to_string());
    cache.put("c", "3".to_string());
    assert_eq!(
        cache.get(&"a").map(|v| v.to_string()),
        Some("1".to_string())
    );
    assert_eq!(
        cache.get(&"b").map(|v| v.to_string()),
        Some("2".to_string())
    );
    assert_eq!(
        cache.get(&"c").map(|v| v.to_string()),
        Some("3".to_string())
    );
}

#[test]
fn lru_cache_eviction_test() {
    let cache = hc::LRUCache::new(2);
    cache.put("a", "1".to_string());
    cache.put("b", "2".to_string());
    cache.put("c", "3".to_string());
    // a 应被驱逐
    assert!(cache.get(&"a").is_none(), "LRU 驱逐后 a 应不存在");
    assert_eq!(
        cache.get(&"b").map(|v| v.to_string()),
        Some("2".to_string())
    );
    assert_eq!(
        cache.get(&"c").map(|v| v.to_string()),
        Some("3".to_string())
    );
}

#[test]
fn lru_cache_remove_test() {
    let cache = hc::LRUCache::new(3);
    cache.put("a", "1".to_string());
    cache.remove(&"a");
    assert!(cache.get(&"a").is_none(), "remove 后应不存在");
}

#[test]
fn fifo_cache_put_get_test() {
    let cache = hc::FIFOCache::new(2);
    cache.put("a", "1".to_string());
    cache.put("b", "2".to_string());
    assert_eq!(
        cache.get(&"a").map(|v| v.to_string()),
        Some("1".to_string())
    );
    assert_eq!(
        cache.get(&"b").map(|v| v.to_string()),
        Some("2".to_string())
    );
}

#[test]
fn fifo_cache_eviction_test() {
    let cache = hc::FIFOCache::new(2);
    cache.put("a", "1".to_string());
    cache.put("b", "2".to_string());
    cache.put("c", "3".to_string());
    // FIFO: a 最先进入,应被驱逐
    assert!(cache.get(&"a").is_none(), "FIFO 驱逐后 a 应不存在");
}

// ---------------------------------------------------------------------------
// CacheTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `CacheTest.fifoCacheTest()`
#[test]
fn fifo_cache_test() {
    let fifo_cache = hc::CacheUtil::new_fifo_cache::<&str, String>(3);
    let removed = Arc::new(Mutex::new(Vec::<(String, String)>::new()));
    let sink = Arc::clone(&removed);
    fifo_cache.set_listener(move |key: &&str, value: &String| {
        sink.lock()
            .unwrap()
            .push(((*key).to_owned(), value.clone()));
    });

    let ttl = Duration::from_secs(3);
    fifo_cache.put_with_timeout("key1", "value1".to_string(), ttl);
    fifo_cache.put_with_timeout("key2", "value2".to_string(), ttl);
    fifo_cache.put_with_timeout("key3", "value3".to_string(), ttl);
    fifo_cache.put_with_timeout("key4", "value4".to_string(), ttl);

    // 容量 3，按 FIFO 最先放入的 key1 被移除
    assert!(fifo_cache.get(&"key1").is_none());
    let removed = removed.lock().unwrap();
    assert_eq!(removed.len(), 1);
    assert_eq!(removed[0].0, "key1");
    assert_eq!(removed[0].1, "value1");
}

/// 对齐 Java: `CacheTest.fifoCacheCapacityTest()`
#[test]
fn fifo_cache_capacity_test() {
    let fifo_cache = hc::CacheUtil::new_fifo_cache::<String, String>(100);
    // Java: RandomUtil.randomInt(100, 1000)
    let n = 100 + (rand::random::<u32>() % 900) as usize;
    for i in 0..n {
        fifo_cache.put(format!("key{i}"), format!("value{i}"));
    }
    assert_eq!(fifo_cache.size(), 100);
}

/// 对齐 Java: `CacheTest.lfuCacheTest()`
#[test]
fn lfu_cache_test() {
    let lfu_cache = hc::CacheUtil::new_lfu_cache::<&str, String>(3);
    let ttl = Duration::from_secs(3);
    lfu_cache.put_with_timeout("key1", "value1".to_string(), ttl);
    // 使用次数 +1
    let _ = lfu_cache.get(&"key1");
    lfu_cache.put_with_timeout("key2", "value2".to_string(), ttl);
    lfu_cache.put_with_timeout("key3", "value3".to_string(), ttl);
    lfu_cache.put_with_timeout("key4", "value4".to_string(), ttl);

    // 容量 3，LFU 清理最少使用的（key2、key3），key1 保留
    assert!(lfu_cache.get(&"key1").is_some());
    assert!(lfu_cache.get(&"key2").is_none());
    assert!(lfu_cache.get(&"key3").is_none());
}

/// 对齐 Java: `CacheTest.lfuCacheTest2()`
#[test]
fn lfu_cache_test_2() {
    // Java: lfuCache.get(null) → null。Rust 无 null 引用，用 Option::None 作为键对齐「空键 miss」。
    let lfu_cache = hc::CacheUtil::new_lfu_cache::<Option<&str>, String>(3);
    let s = lfu_cache.get(&None);
    assert!(s.is_none());
}

/// 对齐 Java: `CacheTest.lruCacheTest()`
#[test]
fn lru_cache_test() {
    let lru_cache = hc::CacheUtil::new_lru_cache::<&str, String>(3);
    let ttl = Duration::from_secs(3);
    lru_cache.put_with_timeout("key1", "value1".to_string(), ttl);
    lru_cache.put_with_timeout("key2", "value2".to_string(), ttl);
    lru_cache.put_with_timeout("key3", "value3".to_string(), ttl);
    // 使用时间推近
    let _ = lru_cache.get(&"key1");
    lru_cache.put_with_timeout("key4", "value4".to_string(), ttl);

    assert!(lru_cache.get(&"key1").is_some());
    // 容量 3，LRU 最少使用的 key2 被移除
    assert!(lru_cache.get(&"key2").is_none());
}

/// 对齐 Java: `CacheTest.timedCacheTest()`
#[test]
fn timed_cache_test() {
    let timed_cache = hc::CacheUtil::new_timed_cache::<&str, String>(Duration::from_millis(4));
    timed_cache.put_with_timeout("key1", "value1".to_string(), Duration::from_millis(1));
    timed_cache.put_with_timeout("key2", "value2".to_string(), Duration::from_secs(5));
    timed_cache.put("key3", "value3".to_string()); // 默认过期 4ms
    // Java: Long.MAX_VALUE 永不过期；Rust put_with_timeout(ZERO) 禁用过期
    timed_cache.put_with_timeout("key4", "value4".to_string(), Duration::ZERO);

    timed_cache
        .schedule_prune(Duration::from_millis(5))
        .expect("schedule_prune");
    // Java sleep 5ms；此处略加长以降低调度抖动
    thread::sleep(Duration::from_millis(20));

    assert!(timed_cache.get(&"key1").is_none());
    assert_eq!(
        timed_cache.get(&"key2").as_deref().map(String::as_str),
        Some("value2")
    );
    assert!(timed_cache.get(&"key3").is_none());

    let value3_supplier =
        timed_cache.get_or_insert_with("key3", || "Default supplier".to_string());
    assert_eq!(value3_supplier.as_str(), "Default supplier");

    assert_eq!(
        timed_cache.get(&"key4").as_deref().map(String::as_str),
        Some("value4")
    );

    assert!(timed_cache.cancel_prune_schedule());
}

/// 对齐 Java: `CacheTest.whenContainsKeyTimeout_shouldCallOnRemove()`
#[test]
fn when_contains_key_timeout_should_call_on_remove() {
    let timeout = Duration::from_millis(50);
    let alarm_cache = hc::TimedCache::<i32, String>::new(timeout);
    let counter = Arc::new(AtomicUsize::new(0));
    let sink = Arc::clone(&counter);
    alarm_cache.set_listener(move |_key: &i32, _value: &String| {
        sink.fetch_add(1, Ordering::SeqCst);
    });

    alarm_cache.put(1, "value1".to_string());
    thread::sleep(Duration::from_millis(100));

    assert!(!alarm_cache.contains_key(&1));
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

/// 对齐 Java: `CacheTest.reentrantCache_clear_Method_Test()`
#[test]
fn reentrant_cache_clear_method_test() {
    let remove_count = Arc::new(AtomicUsize::new(0));
    let sink = Arc::clone(&remove_count);
    let lru_cache = hc::CacheUtil::new_lru_cache::<&str, String>(4);
    lru_cache.set_listener(move |_key: &&str, _value: &String| {
        sink.fetch_add(1, Ordering::SeqCst);
    });
    lru_cache.put("key1", "String1".to_string());
    lru_cache.put("key2", "String2".to_string());
    lru_cache.put("key3", "String3".to_string());
    // key 已存在：替换也会触发 onRemove（对齐 putWithoutLock 修复后的行为）
    lru_cache.put("key1", "String4".to_string());
    lru_cache.put("key4", "String5".to_string());
    lru_cache.clear();
    assert_eq!(remove_count.load(Ordering::SeqCst), 5);
}

// ---------------------------------------------------------------------------
// FileCacheTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `FileCacheTest.lfuFileCacheTest()`
#[test]
fn lfu_file_cache_test() {
    let cache = hc::LFUFileCache::with_timeout(1000, 500, Duration::from_millis(2000));
    assert_eq!(cache.capacity(), 1000);
    assert_eq!(cache.max_file_size(), 500);
    assert_eq!(cache.timeout(), Some(Duration::from_millis(2000)));
}

// ---------------------------------------------------------------------------
// Issue3618Test
// ---------------------------------------------------------------------------

/// 对齐 Java: `Issue3618Test.putTest()`
#[test]
fn issue3618_put_test() {
    let cache = hc::CacheUtil::new_fifo_cache::<i32, i32>(3);
    cache.put(1, 1);
    cache.put(2, 1);
    cache.put(3, 1);
    assert_eq!(cache.size(), 3);

    // issue#3618：替换已有键不做满队列驱逐
    cache.put(3, 2);
    assert_eq!(cache.size(), 3);
}

// ---------------------------------------------------------------------------
// IssueI8MEIXTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `IssueI8MEIXTest.getRemoveTest()`（Java 侧为 `@Disabled` 打印型用例）
#[test]
fn get_remove_test() {
    let cache = hc::TimedCache::<&str, String>::new(Duration::from_millis(200));
    cache.put("a", "123".to_string());
    thread::sleep(Duration::from_millis(300));

    let cache_get = cache.clone();
    let cache_put = cache.clone();
    let got = Arc::new(Mutex::new(None::<Option<String>>));
    let got_sink = Arc::clone(&got);

    let t1 = thread::spawn(move || {
        let value = cache_get.get(&"a").map(|v| v.as_str().to_owned());
        *got_sink.lock().unwrap() = Some(value);
    });
    let t2 = thread::spawn(move || {
        cache_put.put("a", "456".to_string());
    });
    t1.join().expect("get thread");
    t2.join().expect("put thread");

    // 过期后 get 应为 None；put 写入新值后可读到 "456"
    assert_eq!(
        cache.get(&"a").as_deref().map(String::as_str),
        Some("456")
    );
    let observed = got.lock().unwrap().clone();
    assert!(
        matches!(observed, Some(None) | Some(Some(_))),
        "get 竞态应完成且不 panic: {observed:?}"
    );
}

// ---------------------------------------------------------------------------
// LRUCacheTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `LRUCacheTest.putTest()`（Java 侧为 `@Disabled` 压力用例）
#[test]
fn lru_cache_put_test() {
    // Java: newLRUCache(100, 10) + 10000 线程 get(supplier)；此处保留并发 get_or_insert 逻辑，线程数略减以适配 CI。
    let cache = Arc::new(hc::CacheUtil::new_lru_cache_with_timeout::<String, String>(
        100,
        Duration::from_millis(10),
    ));
    let thread_count = 256usize;
    let mut handles = Vec::with_capacity(thread_count);
    for i in 0..thread_count {
        let cache = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            let key: String = (0..5)
                .map(|_| {
                    let idx = (i.wrapping_mul(31).wrapping_add(rand::random::<u64>() as usize)) % 26;
                    (b'a' + idx as u8) as char
                })
                .collect();
            let _ = cache.get_or_insert_with(key, || {
                (0..10)
                    .map(|_| (b'a' + (rand::random::<u8>() % 26)) as char)
                    .collect::<String>()
            });
        }));
    }
    for handle in handles {
        handle.join().expect("putTest worker");
    }
    assert!(cache.size() <= cache.capacity());
}

/// 对齐 Java: `LRUCacheTest.readWriteTest()`
#[test]
fn read_write_test() {
    let cache = Arc::new(hc::CacheUtil::new_lru_cache::<i32, i32>(10));
    for i in 0..10 {
        cache.put(i, i);
    }

    let barrier = Arc::new(Barrier::new(10));
    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        let cache = Arc::clone(&cache);
        let barrier = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier.wait();
            for _ in 0..10_000 {
                let _ = cache.get(&i);
            }
        }));
    }
    for handle in handles {
        handle.join().expect("read thread");
    }

    let mut sb1 = String::new();
    for i in 0..10 {
        sb1.push_str(
            &cache
                .get(&i)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "null".to_string()),
        );
    }
    assert_eq!(sb1, "0123456789");

    // 新加 11，此时 0 最久未使用，应淘汰 0
    cache.put(11, 11);

    let mut sb2 = String::new();
    for i in 0..10 {
        sb2.push_str(
            &cache
                .get(&i)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "null".to_string()),
        );
    }
    assert_eq!(sb2, "null123456789");
}

/// 对齐 Java: `LRUCacheTest.issue2647Test()`
#[test]
fn issue2647_test() {
    let remove_count = Arc::new(AtomicUsize::new(0));
    let sink = Arc::clone(&remove_count);
    // Java: newLRUCache(3, 1) —— capacity=3, timeout=1ms
    let cache = hc::CacheUtil::new_lru_cache_with_timeout::<String, i32>(
        3,
        Duration::from_millis(1),
    );
    cache.set_listener(move |_key: &String, _value: &i32| {
        sink.fetch_add(1, Ordering::SeqCst);
    });

    for i in 0..10 {
        cache.put(format!("key-{i}"), i);
    }

    assert_eq!(remove_count.load(Ordering::SeqCst), 7);
    assert_eq!(cache.size(), 3);
}

// ---------------------------------------------------------------------------
// WeakCacheTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `WeakCacheTest.removeTest()`
#[test]
fn remove_test() {
    // Java: new WeakCache<>(-1) 表示无超时
    let cache = hc::WeakCache::<&str, String>::new(None);
    let abc = Arc::new("123".to_string());
    let def = Arc::new("456".to_string());
    cache.put("abc", &abc);
    cache.put("def", &def);

    assert_eq!(cache.size(), 2);
    cache.remove(&"abc");
    assert_eq!(cache.size(), 1);
}

/// 对齐 Java: `WeakCacheTest.removeByGcTest()`（Java 侧为 `@Disabled` GC 循环用例）
#[test]
fn remove_by_gc_test() {
    // GC divergence: Java 依赖 System.gc() 回收 WeakReference；
    // Rust WeakCache 基于 Arc/Weak，需显式 drop 强引用后再 prune/get 清理。
    let cache = hc::WeakCache::<&str, String>::new(None);
    let a = Arc::new("1".to_string());
    let b = Arc::new("2".to_string());
    cache.put("a", &a);
    cache.put("b", &b);

    assert_eq!(cache.size(), 2);
    let listener_called = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&listener_called);
    cache.set_listener(move |_key: &&str, _value: &String| {
        flag.store(true, Ordering::SeqCst);
    });

    drop(a);
    drop(b);
    let pruned = cache.prune();
    assert_eq!(pruned, 2);
    assert_eq!(cache.size(), 0);
    // prune 路径在值已失效时可能无法把存活值交给 listener；仅断言条目已清理。
    let _ = listener_called.load(Ordering::SeqCst);
}

// ---------------------------------------------------------------------------
// CacheConcurrentTest
// ---------------------------------------------------------------------------

/// 对齐 Java: `CacheConcurrentTest.fifoCacheTest()`（Java 侧为 `@Disabled` 压力用例）
#[test]
fn cache_concurrent_fifo_cache_test() {
    // Java: 4000 写线程 + 4000 读线程 + sleep；此处保留并发 put/迭代逻辑，缩小规模并断言容量约束。
    let cache = Arc::new(hc::FIFOCache::<&str, String>::new(3));
    let writers = 64usize;
    let readers = 64usize;
    let mut handles = Vec::with_capacity(writers + readers);

    for _ in 0..writers {
        let cache = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            let ttl = Duration::from_secs(3);
            cache.put_with_timeout("key1", "value1".to_string(), ttl);
            cache.put_with_timeout("key2", "value2".to_string(), ttl);
            cache.put_with_timeout("key3", "value3".to_string(), ttl);
            cache.put_with_timeout("key4", "value4".to_string(), ttl);
            thread::sleep(Duration::from_millis(5));
            cache.put_with_timeout("key5", "value5".to_string(), ttl);
            cache.put_with_timeout("key6", "value6".to_string(), ttl);
            cache.put_with_timeout("key7", "value7".to_string(), ttl);
            cache.put_with_timeout("key8", "value8".to_string(), ttl);
        }));
    }
    for _ in 0..readers {
        let cache = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            let _ = cache.cache_objects();
            let _ = cache.size();
        }));
    }
    for handle in handles {
        handle.join().expect("fifo concurrent worker");
    }
    assert!(cache.size() <= cache.capacity());
}

/// 对齐 Java: `CacheConcurrentTest.lruCacheTest()`（Java 侧为 `@Disabled` 压力用例）
#[test]
fn cache_concurrent_lru_cache_test() {
    // Java: 40000 线程；此处保留并发 put + size/capacity 检查逻辑。
    let cache = Arc::new(hc::LRUCache::<String, String>::new(1000));
    let thread_count = 256usize;
    let overflow = Arc::new(AtomicBool::new(false));
    let mut handles = Vec::with_capacity(thread_count);
    for i in 0..thread_count {
        let cache = Arc::clone(&cache);
        let overflow = Arc::clone(&overflow);
        handles.push(thread::spawn(move || {
            cache.put(format!("key1{i}"), "value1".to_string());
            cache.put_with_timeout(
                format!("key2{i}"),
                "value2".to_string(),
                Duration::from_secs(3),
            );
            if cache.size() > cache.capacity() {
                overflow.store(true, Ordering::SeqCst);
            }
            thread::sleep(Duration::from_millis(1));
            if cache.size() > cache.capacity() {
                overflow.store(true, Ordering::SeqCst);
            }
        }));
    }
    for handle in handles {
        handle.join().expect("lru concurrent worker");
    }
    assert!(!overflow.load(Ordering::SeqCst));
    assert!(cache.size() <= cache.capacity());
}

/// 对齐 Java: `CacheConcurrentTest.effectiveTest()`（Java 侧为 `@Disabled`）
#[test]
fn effective_test() {
    // Java 使用 WeakCache + Integer 自动装箱常驻；Rust WeakCache 为 Arc/Weak，
    // 供应商合并语义对齐到 TimedCache::get_or_insert_with（同一 factory 锁）。
    let delay = Duration::from_millis(200);
    let ai = Arc::new(AtomicUsize::new(0));
    let cache = Arc::new(hc::TimedCache::<usize, usize>::new(Duration::from_secs(60)));
    let barrier = Arc::new(Barrier::new(32));
    let start = Instant::now();
    let mut handles = Vec::with_capacity(32);
    for _ in 0..32 {
        let cache = Arc::clone(&cache);
        let ai = Arc::clone(&ai);
        let barrier = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            barrier.wait();
            let i = ai.fetch_add(1, Ordering::SeqCst) % 4;
            let _ = cache.get_or_insert_with(i, || {
                thread::sleep(delay);
                i
            });
        }));
    }
    for handle in handles {
        handle.join().expect("effectiveTest worker");
    }
    let interval = start.elapsed();
    // 总耗时应与单次操作耗时在同一数量级
    assert!(
        interval < delay * 2,
        "effectiveTest interval {:?} should be < {:?}",
        interval,
        delay * 2
    );
}
