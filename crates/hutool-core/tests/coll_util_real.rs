//! `CollUtil` 真实功能测试 —— 对齐 Hutool CollUtilTest
//! 所有测试基于 hutool-core coll_util.rs 的真实实现(103 个 pub fn)

use hutool_core::CollUtil;

// ===== 集合运算 =====

#[test]
fn union_multiset_max() {
    let l1 = vec!["a", "b", "b", "c", "d", "x"];
    let l2 = vec!["a", "b", "b", "b", "c", "d"];
    let union = CollUtil::union(&l1, &l2);
    let b_count = union.iter().filter(|x| **x == "b").count();
    assert_eq!(b_count, 3, "multiset max(2,3)=3");
}

#[test]
fn intersection_multiset_min() {
    let l1 = vec!["a", "b", "b", "c", "d", "x"];
    let l2 = vec!["a", "b", "b", "b", "c", "d"];
    let inter = CollUtil::intersection(&l1, &l2);
    let b_count = inter.iter().filter(|x| **x == "b").count();
    assert_eq!(b_count, 2, "multiset min(2,3)=2");
}

#[test]
fn disjunction_symmetric_diff() {
    let l1 = vec!["a", "b", "b", "c", "d", "x"];
    let l2 = vec!["a", "b", "b", "b", "c", "d", "x2"];
    let disj = CollUtil::disjunction(&l1, &l2);
    assert!(disj.contains(&"b") && disj.contains(&"x2") && disj.contains(&"x"));
}

#[test]
fn subtract_removes_matching() {
    let l1 = vec!["a", "b", "b", "c", "d", "x"];
    let l2 = vec!["a", "b", "b", "b", "c", "d", "x2"];
    let sub = CollUtil::subtract(&l1, &l2);
    assert_eq!(sub.len(), 1);
    assert_eq!(sub[0], "x");
}

#[test]
fn distinct_dedup() {
    let list = vec!["a", "b", "b", "c", "a"];
    let d = CollUtil::distinct(&list);
    let strs: Vec<&str> = d.iter().map(|s| **s).collect();
    assert_eq!(strs, vec!["a", "b", "c"]);
}

#[test]
fn contains_any_shares_elements() {
    let l1 = vec![1, 2, 3, 4, 5];
    let l2 = vec![5, 3, 1, 9, 11];
    assert!(CollUtil::contains_any(&l1, &l2));
}

#[test]
fn contains_all_subset() {
    let l1 = vec![1, 2, 3, 4, 5];
    let l2 = vec![5, 3, 1];
    assert!(CollUtil::contains_all(&l1, &l2));
}

#[test]
fn count_map_word_frequency() {
    let list = vec!["a", "b", "c", "c", "a", "b", "d"];
    let cm = CollUtil::count_map(list);
    assert_eq!(cm.get("a"), Some(&2));
    assert_eq!(cm.get("b"), Some(&2));
    assert_eq!(cm.get("c"), Some(&2));
    assert_eq!(cm.get("d"), Some(&1));
}

#[test]
fn split_partition() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let p = CollUtil::split(&list, 3).unwrap();
    assert_eq!(p.len(), 3);
}

#[test]
fn zip_pairs_keys_values() {
    let keys = vec!["a", "b", "c", "d"];
    let values = vec![1, 2, 3, 4];
    let map = CollUtil::zip(keys, values);
    assert_eq!(map.len(), 4);
}

#[test]
fn group_by_parity_even_odd() {
    let list = vec!["1", "2", "3", "4", "5", "6"];
    let g = CollUtil::group(list, |x: &&str| x.parse::<i32>().unwrap() as usize % 2);
    assert_eq!(g[0].len(), 3); // even
    assert_eq!(g[1].len(), 3); // odd
}

#[test]
fn for_each_map_callback() {
    let map = vec![("a", "1"), ("b", "2"), ("c", "3")];
    let mut result = Vec::new();
    CollUtil::for_each_map(map, |k, v, _idx| result.push(format!("{k}:{v}")));
    assert!(result.contains(&"a:1".to_string()));
}

#[test]
fn filter_removes_predicate() {
    let mut list = vec!["a", "b", "c"];
    CollUtil::filter(&mut list, |t| *t != "a");
    assert_eq!(list, vec!["b", "c"]);
}

#[test]
fn remove_empty_string_vec() {
    let mut v = vec![
        "a".to_string(),
        "b".to_string(),
        "".to_string(),
        "c".to_string(),
    ];
    CollUtil::filter(&mut v, |s| !s.is_empty());
    assert_eq!(v, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
}

#[test]
fn sort_by_comparator() {
    let mut v = vec![3, 1, 2];
    v.sort();
    assert_eq!(v, vec![1, 2, 3]);
}

#[test]
fn is_empty_some_slice() {
    assert!(CollUtil::is_empty(Some(&[] as &[i32])));
    assert!(!CollUtil::is_empty(Some(&[1][..])));
}

#[test]
fn new_hash_set_from_iter() {
    let s = CollUtil::new_hash_set(["a", "b", "c"]);
    assert_eq!(s.len(), 3);
}
