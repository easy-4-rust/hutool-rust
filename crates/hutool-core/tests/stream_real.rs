//! stream module real functional tests
//! 对齐: hutool-core StreamUtilTest/CollectorUtilTest

use hutool_core::{CollectorUtil, SimpleCollector, StreamUtil};
use indexmap::IndexMap;

#[test]
fn stream_util_join() {
    let result = StreamUtil::join(vec![1, 2, 3].into_iter(), ", ");
    assert_eq!(result, "1, 2, 3");
}

#[test]
fn stream_util_join_empty() {
    let result = StreamUtil::join(Vec::<i32>::new().into_iter(), ",");
    assert_eq!(result, "");
}

#[test]
fn stream_util_join_single() {
    let result = StreamUtil::join(vec!["hello"], ", ");
    assert_eq!(result, "hello");
}

#[test]
fn stream_util_join_numbers() {
    let result = StreamUtil::join(vec![1, 2, 3, 4, 5], "-");
    assert_eq!(result, "1-2-3-4-5");
}

#[test]
fn stream_util_of() {
    let items: Vec<i32> = StreamUtil::of(vec![1, 2, 3]).collect();
    assert_eq!(items, vec![1, 2, 3]);
}

#[test]
fn stream_util_of_array() {
    let items: Vec<i32> = StreamUtil::of_array([10, 20, 30]).collect();
    assert_eq!(items, vec![10, 20, 30]);
}

#[test]
fn stream_util_iterate() {
    let items: Vec<i32> = StreamUtil::iterate(1, |x| x * 2, 5).collect();
    assert_eq!(items, vec![1, 2, 4, 8, 16]);
}

#[test]
fn stream_util_join_by() {
    let result = StreamUtil::join_by(vec![1, 2, 3], ", ", |x: i32| format!("num:{}", x));
    assert_eq!(result, "num:1, num:2, num:3");
}

#[test]
fn collector_util_joining() {
    let result = CollectorUtil::joining(vec![1, 2, 3], ", ");
    assert_eq!(result, "1, 2, 3");
}

#[test]
fn collector_util_joining_empty() {
    let result: String = CollectorUtil::joining(Vec::<i32>::new(), ",");
    assert_eq!(result, "");
}

#[test]
fn collector_util_joining_by() {
    let result = CollectorUtil::joining_by(vec![1, 2, 3], "|", |x: i32| format!("#{}", x));
    assert_eq!(result, "#1|#2|#3");
}

#[test]
fn collector_util_joining_wrapped() {
    let result = CollectorUtil::joining_wrapped(vec!["a", "b", "c"], ", ", "[", "]", |s: &str| {
        s.to_uppercase()
    });
    assert_eq!(result, "[A, B, C]");
}

#[test]
fn collector_util_grouping_by() {
    let result = CollectorUtil::grouping_by(vec![1, 2, 3, 4, 5, 6], |x: &i32| x % 2);
    let evens = result.get(&0).map(|v| v.len()).unwrap_or(0);
    let odds = result.get(&1).map(|v| v.len()).unwrap_or(0);
    assert_eq!(evens, 3);
    assert_eq!(odds, 3);
}

#[test]
fn collector_util_grouping_map_by() {
    let result =
        CollectorUtil::grouping_map_by(vec![1, 2, 3, 4, 5, 6], |x: &i32| x % 2, |x: i32| x * 10);
    let evens = result.get(&0).unwrap();
    assert!(evens.contains(&20));
    assert!(evens.contains(&40));
    assert!(evens.contains(&60));
}

#[test]
fn collector_util_grouping_by_nullable() {
    let result = CollectorUtil::grouping_by_nullable(
        vec![Some(1), None, Some(2), Some(3), None],
        |x: &i32| x % 2,
    );
    let total: usize = result.values().map(|v| v.len()).sum();
    assert_eq!(total, 5);
    assert_eq!(result.get(&None).map(|v| v.len()), Some(2));
}

#[test]
fn collector_util_to_map() {
    let result = CollectorUtil::to_map(
        vec![("a", 1), ("b", 2)],
        |(k, _): &(&str, i32)| k.to_string(),
        |(_, v): (&str, i32)| v,
        |old, _new| old,
    );
    assert_eq!(result.get("a"), Some(&1));
    assert_eq!(result.get("b"), Some(&2));
}

#[test]
fn collector_util_map_merger() {
    let mut left = IndexMap::new();
    left.insert("a", 1);
    let mut right = IndexMap::new();
    right.insert("a", 2);
    right.insert("b", 3);
    let result = CollectorUtil::map_merger(left, right, |_old: i32, new: i32| new);
    assert_eq!(result.get("a"), Some(&2));
    assert_eq!(result.get("b"), Some(&3));
}

#[test]
fn collector_util_reduce_list_map() {
    let mut m1 = IndexMap::new();
    m1.insert("a", 1);
    m1.insert("b", 2);
    let mut m2 = IndexMap::new();
    m2.insert("a", 10);
    m2.insert("c", 30);
    let result = CollectorUtil::reduce_list_map(vec![m1, m2]);
    assert_eq!(result.get("a").unwrap(), &vec![1, 10]);
    assert_eq!(result.get("b").unwrap(), &vec![2]);
    assert_eq!(result.get("c").unwrap(), &vec![30]);
}

#[test]
fn simple_collector_identity() {
    let collector = SimpleCollector::identity(
        Vec::<i32>::new,
        Vec::push,
        |mut left: Vec<i32>, right: Vec<i32>| {
            left.extend(right);
            left
        },
    );
    let result = collector.collect(vec![1, 2, 3]);
    assert_eq!(result, vec![1, 2, 3]);
}
