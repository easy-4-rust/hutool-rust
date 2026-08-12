//! collection_types module parity tests
//! 对齐: `cn.hutool.core.collection.UniqueKeySetTest` / `RingIndexUtilTest`
//! 来源: hutool-core collection types tests

use hutool_core::{
    BoundedPriorityQueue, ConcurrentHashSet, UniqueKeySet, ring_next_for_len, ring_next_index,
};
use std::sync::atomic::AtomicUsize;

// ── ring_next_index ──

#[test]
fn ring_next_index_basic() {
    let idx = AtomicUsize::new(0);
    // ring_next_index wraps: (0+1)%3=1, (1+1)%3=2, (2+1)%3=0
    assert_eq!(ring_next_index(3, &idx).unwrap(), 1);
    assert_eq!(ring_next_index(3, &idx).unwrap(), 2);
    assert_eq!(ring_next_index(3, &idx).unwrap(), 0);
}

// ── ring_next_for_len ──

#[test]
fn ring_next_for_len_basic() {
    let items = vec!["a", "b", "c"];
    let idx = AtomicUsize::new(0);
    assert_eq!(ring_next_for_len(&items, &idx).unwrap(), 1);
    assert_eq!(ring_next_for_len(&items, &idx).unwrap(), 2);
    assert_eq!(ring_next_for_len(&items, &idx).unwrap(), 0);
}

// ── BoundedPriorityQueue ──

#[test]
fn bounded_priority_queue_offer_and_pop() {
    let mut pq = BoundedPriorityQueue::new(3).unwrap();
    assert!(pq.offer(3));
    assert!(pq.offer(1));
    assert!(pq.offer(2));
    assert_eq!(pq.len(), 3);
    assert!(!pq.is_empty());
    assert_eq!(pq.pop_worst(), Some(3));
    assert_eq!(pq.pop_worst(), Some(2));
    assert_eq!(pq.pop_worst(), Some(1));
}

#[test]
fn bounded_priority_queue_peek_worst() {
    let mut pq = BoundedPriorityQueue::new(3).unwrap();
    pq.offer(5);
    pq.offer(1);
    pq.offer(3);
    assert_eq!(pq.peek_worst(), Some(&5));
}

#[test]
fn bounded_priority_queue_to_sorted_vec() {
    let mut pq = BoundedPriorityQueue::new(3).unwrap();
    pq.offer(3);
    pq.offer(1);
    pq.offer(2);
    let sorted = pq.to_sorted_vec();
    assert_eq!(sorted, vec![1, 2, 3]);
}

#[test]
fn bounded_priority_queue_clear() {
    let mut pq = BoundedPriorityQueue::new(3).unwrap();
    pq.offer(1);
    pq.offer(2);
    pq.clear();
    assert!(pq.is_empty());
}

// ── ConcurrentHashSet ──

#[test]
fn concurrent_hash_set_insert_contains() {
    let set = ConcurrentHashSet::new();
    assert!(set.insert("a"));
    assert!(set.insert("b"));
    assert!(!set.insert("a"));
    assert!(set.contains(&"a"));
    assert!(set.contains(&"b"));
    assert!(!set.contains(&"c"));
    assert_eq!(set.len(), 2);
}

#[test]
fn concurrent_hash_set_remove() {
    let set = ConcurrentHashSet::new();
    set.insert("a");
    set.insert("b");
    assert!(set.remove(&"a"));
    assert!(!set.contains(&"a"));
    assert_eq!(set.len(), 1);
}

#[test]
fn concurrent_hash_set_snapshot() {
    let set = ConcurrentHashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    let mut snapshot = set.snapshot();
    snapshot.sort();
    assert_eq!(snapshot, vec![1, 2, 3]);
}

// ── UniqueKeySet ──

#[test]
fn unique_key_set_insert() {
    let mut set = UniqueKeySet::new(|s: &String| s.len());
    assert!(set.insert("hello".to_string()));
    // "world" has same key (len=5) as "hello", so insert returns false
    assert!(!set.insert("world".to_string()));
    // "hi" has key len=2, which is different
    assert!(set.insert("hi".to_string()));
    assert_eq!(set.len(), 2);
}

#[test]
fn unique_key_set_contains_value() {
    let mut set = UniqueKeySet::new(|s: &String| s.len());
    set.insert("hello".to_string());
    assert!(set.contains_value(&"hello".to_string()));
    // "world" has same key but different value
    assert!(set.contains_value(&"world".to_string())); // same key as "hello"
}

#[test]
fn unique_key_set_insert_if_absent() {
    let mut set = UniqueKeySet::new(|s: &String| s.len());
    assert!(set.insert_if_absent("hello".to_string()));
    // same key (len=5) already exists
    assert!(!set.insert_if_absent("world".to_string()));
    assert_eq!(set.len(), 1);
}

/// 对齐 Java: `UniqueKeySetTest.addTest()`
#[test]
fn add_test() {
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct UniqueTestBean {
        id: String,
        name: String,
        address: String,
    }
    let mut set = UniqueKeySet::new(|b: &UniqueTestBean| b.id.clone());
    set.insert(UniqueTestBean {
        id: "id1".into(),
        name: "张三".into(),
        address: "地球".into(),
    });
    set.insert(UniqueTestBean {
        id: "id2".into(),
        name: "李四".into(),
        address: "火星".into(),
    });
    set.insert(UniqueTestBean {
        id: "id2".into(),
        name: "王五".into(),
        address: "木星".into(),
    });
    assert_eq!(set.len(), 2);
}

/// 对齐 Java: `RingIndexUtilTest.ringNextIntByObjTest()`
#[test]
fn ring_next_int_by_obj_test() {
    let str_list = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let idx = AtomicUsize::new(0);
    for _ in 0..str_list.len() {
        let index = ring_next_for_len(&str_list, &idx).unwrap();
        assert!(str_list.get(index).is_some());
    }
}
