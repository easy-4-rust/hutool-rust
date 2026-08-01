//! `cn.hutool.core.thread` 子包对比验证测试
//! 对齐: `cn.hutool.core.thread.ThreadUtilTest` (3 个 @Test)
//! 来源: hutool-core/src/test/java/cn/hutool/core/thread/ThreadUtilTest.java

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use hutool_core::thread::semaphore_runnable::{Semaphore, SemaphoreRunnable};
use hutool_core::thread::concurrency_tester::ConcurrencyTester;
use hutool_core::thread::executor_builder::ExecutorBuilder;
use hutool_core::thread::lock::segment_lock::SegmentLock;
use hutool_core::thread::recyclable_batch_thread_pool_executor::RecyclableBatchThreadPoolExecutor;
use hutool_core::thread::reject_policy::RejectPolicy;
use hutool_core::thread::sync_finisher::SyncFinisher;

/// 对齐 Java: `ThreadUtilTest.newExecutorTest()`
#[test]
fn new_executor_test() {
    let executor = ExecutorBuilder::create()
        .set_core_pool_size(5)
        .set_max_pool_size(5)
        .build();
    executor.shutdown();
}

/// 对齐 Java: `ThreadUtilTest.executeTest()`
#[test]
fn execute_test() {
    let done = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&done);
    let executor = ExecutorBuilder::create().set_core_pool_size(1).build();
    executor.execute(move || flag.store(true, Ordering::SeqCst));
    executor.shutdown();
    thread::sleep(Duration::from_millis(50));
    assert!(done.load(Ordering::SeqCst));
}

/// 对齐 Java: `ThreadUtilTest.safeSleepTest()`
#[test]
fn safe_sleep_test() {
    let start = std::time::Instant::now();
    assert!(hutool_core::thread::thread_util::ThreadUtil::safe_sleep(10));
    assert!(start.elapsed() >= Duration::from_millis(5));
}

/// 对齐 Java: `ThreadUtil` / `GlobalThreadPool` / `LockUtil` / `ThreadFactoryBuilder` 综合冒烟
#[test]
fn thread_util_global_pool_and_lock_util_smoke() {
    use hutool_core::thread::global_thread_pool::GlobalThreadPool;
    use hutool_core::thread::lock::lock_util::LockUtil;
    use hutool_core::thread::thread_factory_builder::ThreadFactoryBuilder;
    use hutool_core::thread::thread_util::ThreadUtil;

    GlobalThreadPool::init();
    let done = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&done);
    GlobalThreadPool::execute(move || flag.store(true, Ordering::SeqCst));
    thread::sleep(Duration::from_millis(30));
    assert!(done.load(Ordering::SeqCst));

    let factory = ThreadFactoryBuilder::create()
        .set_name_prefix("parity-")
        .set_daemon(false)
        .build();
    let h = factory.new_thread(|| {});
    h.join().unwrap();

    let no = LockUtil::get_no_lock();
    no.lock();
    assert!(no.try_lock());
    no.unlock();

    let seg = LockUtil::create_segment_lock(4);
    let _ = seg.get(&"k");

    let latch = ThreadUtil::new_count_down_latch(1);
    latch.count_down();
    latch.await_zero();

    let mut ct = ConcurrencyTester::new(2);
    ct.test(|| {});
    ct.reset();
    ct.close();
}


/// 对齐 Java: `ExecutorBuilderTest` (ExecutorBuilder 创建线程池)
#[test]
fn executor_builder_test() {
    let executor = ExecutorBuilder::create()
        .set_core_pool_size(2)
        .set_max_pool_size(4)
        .build();
    executor.execute(|| {});
    executor.shutdown();
}

/// 对齐 Java: `ConcurrencyTesterTest.concurrencyTesterTest()`
#[test]
fn concurrency_tester_test() {
    let mut ct = ConcurrencyTester::new(3);
    ct.test(|| thread::sleep(Duration::from_millis(1)));
    let _ = ct.get_interval();
}

/// 对齐 Java: `SyncFinisherTest` (多线程同步完成器)
#[test]
fn sync_finisher_test() {
    let done = Arc::new(AtomicUsize::new(0));
    let counter = Arc::clone(&done);
    let sf = SyncFinisher::new(4);
    for _ in 0..2 {
        let c = Arc::clone(&counter);
        sf.add_worker(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });
    }
    sf.start();
    sf.close();
    assert_eq!(done.load(Ordering::SeqCst), 2);
}

/// 对齐 Java: `cn.hutool.core.thread.SemaphoreRunnable`
/// 验证 acquire/release 与并发上限(permits=2)。
#[test]
fn semaphore_runnable_concurrency_limit() {
    let permits = 2usize;
    let sem = Arc::new(Semaphore::new(permits));
    let in_critical = Arc::new(AtomicUsize::new(0));
    let max_seen = Arc::new(AtomicUsize::new(0));
    let done = Arc::new(AtomicUsize::new(0));

    let mut handles = Vec::new();
    for _ in 0..6 {
        let sem = Arc::clone(&sem);
        let in_critical = Arc::clone(&in_critical);
        let max_seen = Arc::clone(&max_seen);
        let done = Arc::clone(&done);
        handles.push(thread::spawn(move || {
            let mut task = SemaphoreRunnable::with_semaphore(
                move || {
                    let n = in_critical.fetch_add(1, Ordering::SeqCst) + 1;
                    max_seen.fetch_max(n, Ordering::SeqCst);
                    thread::sleep(Duration::from_millis(20));
                    in_critical.fetch_sub(1, Ordering::SeqCst);
                    done.fetch_add(1, Ordering::SeqCst);
                },
                sem,
            );
            task.run();
        }));
    }

    for h in handles {
        h.join().expect("join");
    }

    assert_eq!(done.load(Ordering::SeqCst), 6);
    assert!(max_seen.load(Ordering::SeqCst) <= permits);
    assert_eq!(sem.available_permits(), permits);
}
// ── thread 缺口补齐（方法级对齐注释）──

/// 对齐 Java: `ConcurrencyTesterTest.multiTest()`

/// 对齐 Java: `ConcurrencyTesterTest.multiTest()`
#[test]
fn concurrency_tester_multi_test() {
    let mut ct = ConcurrencyTester::new(5);
    for _ in 0..3 {
        ct.test(|| {
            thread::sleep(Duration::from_millis(5));
        });
    }
    assert!(ct.get_interval() > 0);
}

/// 对齐 Java: `ExecutorBuilderTest.CallerRunsPolicyTest()`
#[test]
fn executor_builder_caller_runs_policy_test() {
    let executor = ExecutorBuilder::create()
        .set_core_pool_size(1)
        .set_max_pool_size(1)
        .set_handler(RejectPolicy::BLOCK)
        .build();
    executor.execute(|| {});
    executor.execute(|| {});
    executor.shutdown();
    executor.execute(|| {}); // CallerRuns / Block after shutdown
}

/// 对齐 Java: `RecyclableBatchThreadPoolExecutorTest.test()`
#[test]
fn recyclable_batch_thread_pool_executor_test() {
    let pool = RecyclableBatchThreadPoolExecutor::new(4);
    let out = pool.process(vec![1, 2, 3, 4, 5], |x| x * 2);
    assert_eq!(out, vec![2, 4, 6, 8, 10]);
}
/// 对齐 Java: `RecyclableBatchThreadPoolExecutorTest.test2()`
#[test]
fn recyclable_batch_thread_pool_executor_test_2() {
    let pool = RecyclableBatchThreadPoolExecutor::new(2);
    let out = pool.process(vec!["a", "b", "c"], |s| s.len());
    assert_eq!(out, vec![1, 1, 1]);
}

/// 对齐 Java: `SegmentLockTest.testSize()`
#[test]
fn segment_lock_test_size() {
    let strong = SegmentLock::lock(4);
    let weak = SegmentLock::lazy_weak_lock(4);
    let sem = SegmentLock::semaphore(4, 2);
    let rw = SegmentLock::read_write_lock(4);
    assert_eq!(strong.size(), 4);
    assert_eq!(weak.size(), 4);
    assert_eq!(sem.size(), 4);
    assert_eq!(rw.size(), 4);
}
/// 对齐 Java: `SegmentLockTest.testGetWithSameKey()`
#[test]
fn segment_lock_test_get_with_same_key() {
    let strong = SegmentLock::lock(4);
    let k1 = "testKey".to_string();
    let k2 = "testKey".to_string();
    assert!(std::ptr::eq(strong.get(&k1), strong.get(&k2)));
}
/// 对齐 Java: `SegmentLockTest.testGetAt()`
#[test]
fn segment_lock_test_get_at() {
    let strong = SegmentLock::lock(4);
    for i in 0..4 {
        let _ = strong.get_at(i);
    }
    let r = std::panic::catch_unwind(|| {
        let s = SegmentLock::lock(4);
        let _ = s.get_at(4);
    });
    assert!(r.is_err());
}
/// 对齐 Java: `SegmentLockTest.testBulkGet()`
#[test]
fn segment_lock_test_bulk_get() {
    let strong = SegmentLock::lock(4);
    let keys = ["key1", "key2", "key3"];
    let locks = strong.bulk_get(&keys);
    assert_eq!(locks.len(), 3);
    let mut prev = -1isize;
    for l in locks {
        let idx = strong.find_index_by_ptr(l as *const _).unwrap() as isize;
        assert!(idx >= prev);
        prev = idx;
    }
}
/// 对齐 Java: `SegmentLockTest.testLockConcurrency()`
#[test]
fn segment_lock_test_lock_concurrency() {
    let strong = Arc::new(SegmentLock::lock(4));
    let mut hs = vec![];
    for i in 0..8 {
        let s = Arc::clone(&strong);
        hs.push(thread::spawn(move || {
            let key = format!("key{i}");
            let lock = s.get(&key);
            let _g = lock.lock();
            thread::sleep(Duration::from_millis(20));
        }));
    }
    for h in hs {
        h.join().unwrap();
    }
}
/// 对齐 Java: `SegmentLockTest.testSemaphore()`
#[test]
fn segment_lock_test_semaphore() {
    let seg = SegmentLock::semaphore(4, 2);
    let sem = seg.get(&"testKey");
    assert_eq!(sem.available_permits(), 2);
    sem.acquire();
    sem.acquire();
    assert_eq!(sem.available_permits(), 0);
    sem.release();
    assert_eq!(sem.available_permits(), 1);
}
/// 对齐 Java: `SegmentLockTest.testReadWriteLock()`
#[test]
fn segment_lock_test_read_write_lock() {
    let seg = SegmentLock::read_write_lock(4);
    let rw = seg.get(&"testKey");
    {
        let _r1 = rw.read();
        assert!(rw.try_read().is_some());
    }
    let acquired = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&acquired);
    let rw2 = Arc::clone(rw);
    let _w = rw.write();
    let h = thread::spawn(move || {
        flag.store(rw2.try_read().is_some(), Ordering::SeqCst);
    });
    thread::sleep(Duration::from_millis(30));
    assert!(!acquired.load(Ordering::SeqCst));
    drop(_w);
    h.join().unwrap();
}
/// 对齐 Java: `SegmentLockTest.testWeakReferenceCleanup()`
#[test]
fn segment_lock_test_weak_reference_cleanup() {
    let weak = SegmentLock::lazy_weak_lock(1024);
    let a = weak.get(&"testKey");
    let b = weak.get(&"testKey");
    assert!(std::ptr::eq(a, b));
}
/// 对齐 Java: `SegmentLockTest.testInvalidSegmentCount()`
#[test]
fn segment_lock_test_invalid_segment_count() {
    assert!(std::panic::catch_unwind(|| SegmentLock::lock(0)).is_err());
}
/// 对齐 Java: `SegmentLockTest.testHashDistribution()`
#[test]
fn segment_lock_test_hash_distribution() {
    let lock = SegmentLock::lock(4);
    let mut counts = [0usize; 4];
    for i in 0..100 {
        let key = format!("key{i}");
        let l = lock.get(&key);
        let idx = lock.find_index_by_ptr(l as *const _).unwrap();
        counts[idx] += 1;
    }
    assert!(counts.iter().all(|&c| c > 0));
}

/// 对齐 Java: `SyncFinisherTest.executeExceptionTest()`
#[test]
fn sync_finisher_execute_exception_test() {
    let has = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&has);
    let sf = SyncFinisher::new(10);
    sf.add_worker(|| panic!("For input string: \"XYZ\""));
    sf.set_exception_handler(move |_t, e| {
        flag.store(true, Ordering::SeqCst);
        let msg = e
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| e.downcast_ref::<String>().cloned())
            .unwrap_or_default();
        assert!(msg.contains("XYZ"));
    });
    sf.start();
    sf.close();
    thread::sleep(Duration::from_millis(50));
    assert!(has.load(Ordering::SeqCst));
}
/// 对齐 Java: `SyncFinisherTest.executeExceptionTest2()`
#[test]
fn sync_finisher_execute_exception_test_2() {
    let sf = SyncFinisher::new(10);
    sf.add_worker(|| panic!("XYZ"));
    sf.start();
    sf.close();
}
