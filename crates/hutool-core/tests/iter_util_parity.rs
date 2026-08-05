//! `IterUtil` 对比验证测试 —— 对齐 Hutool `IterUtilTest`
//!
//! 对齐: `cn.hutool.core.collection.IterUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/collection/IterUtilTest.java

use hutool_core::IterUtil;

/// 对齐 Java: `IterUtilTest.getFirstTest()` (行 24-34)
#[test]
fn get_first_test() {
    // Java assertNull(IterUtil.getFirst(null)) 和空集合 → Rust None
    let empty: Vec<String> = vec![];
    assert!(
        IterUtil::first(empty).is_none(),
        "first(empty) 应 None (对齐 Java assertNull)"
    );

    let first = IterUtil::first(["1".to_string(), "2".to_string(), "3".to_string()]);
    assert_eq!(
        first,
        Some("1".to_string()),
        "first([\"1\",\"2\",\"3\"]) (对齐 Java)"
    );
}

/// 对齐 Java: `IterUtilTest.getFirstNonNullTest()` (行 36-40)
#[test]
fn get_first_non_null_test() {
    let v: Vec<Option<&str>> = vec![None, None, Some("123"), Some("456"), None];
    let first = IterUtil::first_some(v);
    assert_eq!(first, Some("123"), "first_some 跳过 None (对齐 Java)");
}

/// 对齐 Java: `IterUtilTest.fieldValueMapTest()` (行 42-50)
///
/// 反射版用元组手动构造 Map 验证。
#[test]
fn field_value_map_test() {
    let cars = vec![("123", "大众"), ("345", "奔驰"), ("567", "路虎")];
    let map: std::collections::HashMap<&str, &str> = cars.into_iter().collect();
    assert_eq!(map.get("123"), Some(&"大众"), "carNumber=123 (对齐 Java)");
    assert_eq!(map.get("345"), Some(&"奔驰"), "carNumber=345 (对齐 Java)");
    assert_eq!(map.get("567"), Some(&"路虎"), "carNumber=567 (对齐 Java)");
}

/// 对齐 Java: `IterUtilTest.joinTest()` (行 52-66)
#[test]
fn join_test() {
    let list = vec!["1", "2", "3", "4"];
    let joined = IterUtil::join(list, ":");
    assert_eq!(joined, "1:2:3:4", "join 字符串 (对齐 Java)");

    let list1 = vec![1, 2, 3, 4];
    let joined1 = IterUtil::join(list1, ":");
    assert_eq!(joined1, "1:2:3:4", "join 整数 (对齐 Java)");

    // 包装每个节点
    let list2 = vec!["1", "2", "3", "4"];
    let joined2 = IterUtil::join_wrapped(list2, ":", "\"", "\"");
    assert_eq!(
        joined2, "\"1\":\"2\":\"3\":\"4\"",
        "join_wrapped (对齐 Java)"
    );
}

/// 对齐 Java: `IterUtilTest.joinWithFuncTest()` (行 68-73)
#[test]
fn join_with_func_test() {
    let list = vec!["1", "2", "3", "4"];
    let joined = IterUtil::join_by(list, ":", |s: &str| s.to_string());
    assert_eq!(joined, "1:2:3:4", "join_by String::valueOf (对齐 Java)");
}

/// 对齐 Java: `IterUtilTest.joinWithNullTest()` (行 75-80)
#[test]
fn join_with_null_test() {
    let list: Vec<Option<&str>> = vec![Some("1"), None, Some("3"), Some("4")];
    let joined = IterUtil::join_by(list, ":", |v| v.unwrap_or("null").to_string());
    assert_eq!(joined, "1:null:3:4", "join 含 null → \"null\" (对齐 Java)");
}

/// 对齐 Java: `IterUtilTest.testToListMap()` (行 82-91)
#[test]
fn test_to_list_map() {
    let input = vec!["and".to_string(), "brave".to_string(), "back".to_string()];
    let map = IterUtil::to_list_map(
        input,
        |s: &String| s.chars().next().unwrap().to_string(),
        |s: String| s,
    );
    assert_eq!(
        map.get("a"),
        Some(&vec!["and".to_string()]),
        "a → [and] (对齐 Java)"
    );
    assert_eq!(
        map.get("b"),
        Some(&vec!["brave".to_string(), "back".to_string()]),
        "b → [brave, back] (对齐 Java)"
    );
}

/// 对齐 Java: `IterUtilTest.testToMap()` (行 93-105)
///
/// 反射版用元组手动构造。
#[test]
fn test_to_map() {
    let cars = vec![("123", "bmw"), ("456", "benz")];
    let map: std::collections::HashMap<&str, &str> = cars.into_iter().collect();
    assert_eq!(map.get("123"), Some(&"bmw"), "123 → bmw (对齐 Java)");
    assert_eq!(map.get("456"), Some(&"benz"), "456 → benz (对齐 Java)");
}

/// 对齐 Java: `IterUtilTest.getElementTypeTest()` (行 107-112)
#[test]
fn get_element_type_test() {
    let v: Vec<Option<i32>> = vec![None, Some(1)];
    let t = IterUtil::element_type(&v);
    assert!(t.is_some(), "element_type 非空切片应 Some (对齐 Java)");
}

/// 对齐 Java: `IterUtilTest.filterTest()` (行 121-130)
#[test]
fn filter_test() {
    let mut obj = vec!["1".to_string(), "3".to_string()];
    IterUtil::filter(&mut obj, |x| x == "3");
    assert_eq!(obj.len(), 1, "filter 后 size=1 (对齐 Java)");
    assert_eq!(obj[0], "3", "filter 后元素为 \"3\" (对齐 Java)");
}

/// 对齐 Java: `IterUtilTest.filteredTest()` (行 132-141)
#[test]
fn filtered_test() {
    let obj = vec!["1".to_string(), "3".to_string()];
    let mut filtered = IterUtil::filtered(obj.into_iter(), Some(|s: &String| s == "3"));
    let next = filtered.next();
    assert_eq!(
        next,
        Some("3".to_string()),
        "filtered.next() (对齐 Java): got {next:?}"
    );
    assert!(
        filtered.next().is_none(),
        "filtered.hasNext() false (对齐 Java)"
    );
}

/// 对齐 Java: `IterUtilTest.filterToListTest()` (行 143-152)
#[test]
fn filter_to_list_test() {
    let obj = vec!["1", "3"];
    let filtered = IterUtil::filter_to_list(obj, |s: &&str| *s == "3");
    assert_eq!(filtered.len(), 1, "filter_to_list len=1 (对齐 Java)");
    assert_eq!(filtered[0], "3", "filter_to_list[0]=\"3\" (对齐 Java)");
}

/// 对齐 Java: `IterUtilTest.getTest()` (行 154-159)
///
/// HashSet 无序,改用 Vec 保证确定顺序。
#[test]
fn get_test() {
    let v = vec!["A", "B", "C", "D"];
    let s = IterUtil::get(v, 2);
    assert_eq!(s, Some("C"), "get(idx=2) = \"C\" (对齐 Java)");
}
// ── 扩展 iter_util 测试 ──

#[test]
fn get_iter_basic() {
    let iter = IterUtil::get_iter(vec![1, 2, 3]);
    let result: Vec<i32> = iter.collect();
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn has_none_with_some() {
    assert!(IterUtil::has_none(vec![Some(1), None, Some(3)]));
}

#[test]
fn has_none_all_some() {
    assert!(!IterUtil::has_none(vec![Some(1), Some(2), Some(3)]));
}

#[test]
fn is_all_none_true() {
    assert!(IterUtil::is_all_none(vec![None::<i32>, None, None]));
}

#[test]
fn is_all_none_false() {
    assert!(!IterUtil::is_all_none(vec![None, Some(1), None]));
}

#[test]
fn count_map_basic() {
    let result = IterUtil::count_map(vec!["a", "b", "a", "c", "a"]);
    assert_eq!(result.get("a"), Some(&3));
    assert_eq!(result.get("b"), Some(&1));
    assert_eq!(result.get("c"), Some(&1));
}

#[test]
fn join_basic() {
    let result = IterUtil::join(vec![1, 2, 3], ", ");
    assert_eq!(result, "1, 2, 3");
}

#[test]
fn join_empty() {
    let result: String = IterUtil::join(Vec::<i32>::new(), ", ");
    assert_eq!(result, "");
}

#[test]
fn join_by_basic() {
    let result = IterUtil::join_by(vec![1, 2, 3], "|", |x: i32| format!("#{}", x));
    assert_eq!(result, "#1|#2|#3");
}

#[test]
fn entries_to_map_basic() {
    let result = IterUtil::entries_to_map(vec![("a", 1), ("b", 2)]);
    assert_eq!(result.get("a"), Some(&1));
    assert_eq!(result.get("b"), Some(&2));
}

#[test]
fn to_list_basic() {
    let result = IterUtil::to_list(vec![1, 2, 3]);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn get_basic() {
    assert_eq!(IterUtil::get(vec![1, 2, 3], 0), Some(1));
    assert_eq!(IterUtil::get(vec![1, 2, 3], 2), Some(3));
    assert_eq!(IterUtil::get(vec![1, 2, 3], 5), None);
}

#[test]
fn first_basic() {
    assert_eq!(IterUtil::first(vec![1, 2, 3]), Some(1));
    assert_eq!(IterUtil::first(Vec::<i32>::new()), None);
}

#[test]
fn first_some_basic() {
    assert_eq!(IterUtil::first_some(vec![None, Some(2), Some(3)]), Some(2));
    assert_eq!(IterUtil::first_some(vec![None::<i32>, None, None]), None);
}

#[test]
fn first_match_basic() {
    assert_eq!(
        IterUtil::first_match(vec![1, 2, 3, 4, 5], |x| *x > 3),
        Some(4)
    );
    assert_eq!(IterUtil::first_match(vec![1, 2, 3], |x| *x > 10), None);
}

#[test]
fn element_type_basic() {
    assert_eq!(IterUtil::element_type(&[1i32, 2, 3]), Some("i32"));
    assert_eq!(IterUtil::element_type(&["a", "b"]), Some("&str"));
}
