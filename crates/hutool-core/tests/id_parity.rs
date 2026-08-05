//! id module parity tests
//! 对齐: `cn.hutool.core.util.IdUtilTest`

use hutool_core::IdUtil;

// ── IdUtil ──

#[test]
fn id_util_simple_uuid() {
    let id = IdUtil::simple_uuid();
    assert!(!id.is_empty());
    assert_eq!(id.len(), 32);
}

#[test]
fn id_util_uuid() {
    let id = IdUtil::uuid();
    assert!(!id.is_empty());
    // UUID format: 8-4-4-4-12
    assert_eq!(id.len(), 36);
    assert_eq!(id.chars().filter(|c| *c == '-').count(), 4);
}

#[test]
fn id_util_is_valid() {
    assert!(IdUtil::is_valid("550e8400-e29b-41d4-a716-446655440000"));
    assert!(!IdUtil::is_valid("not-a-uuid"));
    assert!(!IdUtil::is_valid(""));
}

#[test]
fn id_util_uuid_uniqueness() {
    let id1 = IdUtil::uuid();
    let id2 = IdUtil::uuid();
    assert_ne!(id1, id2);
}

#[test]
fn id_util_simple_uuid_uniqueness() {
    let id1 = IdUtil::simple_uuid();
    let id2 = IdUtil::simple_uuid();
    assert_ne!(id1, id2);
}

// ── 对齐 Hutool IdUtilTest ──

/// 对齐 Java: `IdUtilTest.randomUUIDTest()`
#[test]
fn random_uuid_test() {
    let simple_uuid = IdUtil::simple_uuid();
    assert_eq!(32, simple_uuid.len());
    let random_uuid = IdUtil::uuid();
    assert_eq!(36, random_uuid.len());
}

/// 对齐 Java: `IdUtilTest.fastUUIDTest()`
#[test]
fn fast_uuid_test() {
    // Rust 无 uuid v4 对齐 Java fastUUID 长度语义
    let simple_uuid = IdUtil::simple_uuid();
    assert_eq!(32, simple_uuid.len());
    let random_uuid = IdUtil::uuid();
    assert_eq!(36, random_uuid.len());
}

// ── Hutool TEST parity gap wave ──
// ── Hutool IdUtilTest remaining gaps ──

/// 对齐 Java: `IdUtilTest.benchTest()` — Java `@Disabled`；缩小规模做唯一性 smoke。
#[test]
fn bench_test() {
    let mut seen = std::collections::HashSet::new();
    for _ in 0..1_000 {
        let id = IdUtil::simple_uuid();
        assert_eq!(32, id.len());
        assert!(seen.insert(id));
    }
}

/// 对齐 Java: `IdUtilTest.objectIdTest()`
#[test]
fn object_id_test() {
    let id = IdUtil::object_id();
    assert_eq!(id.len(), 24);
    assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
}

/// 对齐 Java: `IdUtilTest.getSnowflakeTest()`
#[test]
fn get_snowflake_test() {
    let snowflake = IdUtil::get_snowflake(1, 1);
    let id = snowflake.next_id();
    assert!(id > 0);
}

/// 对齐 Java: `IdUtilTest.snowflakeBenchTest()` — Java `@Disabled`；并发唯一性 smoke。
#[test]
fn snowflake_bench_test() {
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};
    use std::thread;

    let snowflake = IdUtil::get_snowflake(1, 1);
    let seen = Arc::new(Mutex::new(HashSet::new()));
    let thread_count = 10;
    let id_count_per_thread = 100;

    let mut handles = Vec::new();
    for _ in 0..thread_count {
        let snowflake = Arc::clone(&snowflake);
        let seen = Arc::clone(&seen);
        handles.push(thread::spawn(move || {
            for _ in 0..id_count_per_thread {
                let id = snowflake.next_id();
                let mut guard = seen.lock().expect("lock");
                assert!(guard.insert(id));
            }
        }));
    }
    for handle in handles {
        handle.join().expect("join");
    }
    assert_eq!(
        thread_count * id_count_per_thread,
        seen.lock().expect("lock").len()
    );
}

/// 对齐 Java: `IdUtilTest.snowflakeBenchTest2()` — Java `@Disabled`；每次 getSnowflake 并发 smoke。
#[test]
fn snowflake_bench_test_2() {
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};
    use std::thread;

    let seen = Arc::new(Mutex::new(HashSet::new()));
    let thread_count = 10;
    let id_count_per_thread = 100;

    let mut handles = Vec::new();
    for _ in 0..thread_count {
        let seen = Arc::clone(&seen);
        handles.push(thread::spawn(move || {
            for _ in 0..id_count_per_thread {
                let id = IdUtil::get_snowflake(1, 1).next_id();
                let mut guard = seen.lock().expect("lock");
                assert!(guard.insert(id));
            }
        }));
    }
    for handle in handles {
        handle.join().expect("join");
    }
    assert_eq!(
        thread_count * id_count_per_thread,
        seen.lock().expect("lock").len()
    );
}

/// 对齐 Java: `IdUtilTest.getDataCenterIdTest()`
#[test]
fn get_data_center_id_test() {
    let data_center_id = IdUtil::get_data_center_id(i64::MAX);
    assert!(data_center_id >= 0);
}
