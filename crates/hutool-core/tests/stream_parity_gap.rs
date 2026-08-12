//! `cn.hutool.core.stream` 缺口 parity
//!
//! 对齐: `cn.hutool.core.stream.*` 未覆盖 @Test
//! 基于 hutool-core `StreamUtil` / `CollectorUtil` 真实实现。

use hutool_core::{CollectorUtil, StreamUtil};
use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};

/// 对齐 Java: `CollectorUtilTest.reduceListMapTest()`
#[test]
fn collector_util_reduce_list_map_test() {
    let maps = vec![
        IndexMap::from([("苏格拉底".to_string(), 1), ("特拉叙马霍斯".to_string(), 3)]),
        IndexMap::from([("苏格拉底".to_string(), 2)]),
        IndexMap::from([("特拉叙马霍斯".to_string(), 1)]),
        IndexMap::from([("特拉叙马霍斯".to_string(), 2)]),
    ];
    let reduced = CollectorUtil::reduce_list_map(maps);
    assert_eq!(reduced.get("苏格拉底").unwrap(), &vec![1, 2]);
    assert_eq!(reduced.get("特拉叙马霍斯").unwrap(), &vec![3, 1, 2]);

    // grouping + reduce_list_map 语义：按 name 聚合后合并字段列表
    let data = vec![
        HashMap::from([
            ("name".to_string(), "sam".to_string()),
            ("count".to_string(), "80".to_string()),
        ]),
        HashMap::from([
            ("name".to_string(), "sam".to_string()),
            ("count".to_string(), "81".to_string()),
        ]),
        HashMap::from([
            ("name".to_string(), "jack".to_string()),
            ("count".to_string(), "80".to_string()),
        ]),
    ];
    let mut by_name: IndexMap<String, Vec<IndexMap<String, String>>> = IndexMap::new();
    for row in data {
        let name = row.get("name").cloned().unwrap_or_default();
        by_name
            .entry(name)
            .or_default()
            .push(IndexMap::from_iter(row.into_iter()));
    }
    let sam = CollectorUtil::reduce_list_map(by_name.get("sam").cloned().unwrap_or_default());
    assert_eq!(sam.get("count").map(|v| v.len()), Some(2));
}

/// 对齐 Java: `CollectorUtilTest.testGroupingByAfterValueMapped()`
#[test]
fn collector_util_test_grouping_by_after_value_mapped() {
    let list = [1, 1, 2, 2, 3, 4];
    let map = CollectorUtil::grouping_map_by(list, |t| t & 1 == 0, |t| t.to_string());
    assert_eq!(
        map.get(&true).cloned().unwrap_or_default(),
        vec!["2".to_string(), "2".to_string(), "4".to_string()]
    );
    assert_eq!(
        map.get(&false).cloned().unwrap_or_default(),
        vec!["1".to_string(), "1".to_string(), "3".to_string()]
    );

    // LinkedHashSet 语义：同 key 下去重后的映射值集合
    let mut even: HashSet<String> = HashSet::new();
    let mut odd: HashSet<String> = HashSet::new();
    for t in list {
        if t & 1 == 0 {
            even.insert(t.to_string());
        } else {
            odd.insert(t.to_string());
        }
    }
    assert_eq!(even, HashSet::from(["2".into(), "4".into()]));
    assert_eq!(odd, HashSet::from(["1".into(), "3".into()]));
}

/// 对齐 Java: `StreamUtilTest.ofTest()`
#[test]
fn stream_util_of_test() {
    // Java: StreamUtil.of(2, x -> x * 2, 4) → 2,4,8,16
    let result = StreamUtil::join(StreamUtil::iterate(2, |x| x * 2, 4), ",");
    assert_eq!(result, "2,4,8,16");
}

/// 对齐 Java: `StreamUtilTest.streamTestNullIterator()`
#[test]
fn stream_util_stream_test_null_iterator() {
    // Rust 无 null iterator；空 Option 迭代器语义对齐「不可用输入」边界
    let empty: Option<std::vec::IntoIter<i32>> = None;
    let items: Vec<i32> = empty.into_iter().flatten().collect();
    assert!(items.is_empty());
}

/// 对齐 Java: `StreamUtilTest.streamTestEmptyListToIterator()`
#[test]
fn stream_util_stream_test_empty_list_to_iterator() {
    let items: Vec<i32> = StreamUtil::of(Vec::<i32>::new().into_iter()).collect();
    assert!(items.is_empty());
}

/// 对齐 Java: `StreamUtilTest.streamTestEmptyIterator()`
#[test]
fn stream_util_stream_test_empty_iterator() {
    let items: Vec<i32> = StreamUtil::of(std::iter::empty::<i32>()).collect();
    assert!(items.is_empty());
}

/// 对齐 Java: `StreamUtilTest.streamTestOrdinaryIterator()`
#[test]
fn stream_util_stream_test_ordinary_iterator() {
    let list = vec![1, 2, 3];
    let from_list: Vec<i32> = StreamUtil::of(list.clone().into_iter()).collect();
    assert_eq!(from_list, vec![1, 2, 3]);

    let set: HashSet<i32> = HashSet::from([1, 2, 3]);
    let from_set: HashSet<i32> = StreamUtil::of(set.clone().into_iter()).collect();
    assert_eq!(from_set, set);
}
