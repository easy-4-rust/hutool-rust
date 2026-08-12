//! `ListUtil` 对比验证测试 —— 对齐 Hutool `ListUtilTest`
//!
//! 对齐: `cn.hutool.core.collection.ListUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/collection/ListUtilTest.java

use hutool_core::ListUtil;

/// 对齐 Java: `ListUtilTest.splitTest()` (行 18-33)
#[test]
fn split_test() {
    let empty: Vec<i32> = vec![];
    let p = ListUtil::partition(&empty, 3).unwrap();
    assert_eq!(p.iter().count(), 0, "split(null, 3) = empty (对齐 Java)");

    let list = vec![1, 2, 3, 4];
    for (size, expected) in [
        (1, vec![vec![1], vec![2], vec![3], vec![4]]),
        (2, vec![vec![1, 2], vec![3, 4]]),
        (3, vec![vec![1, 2, 3], vec![4]]),
        (4, vec![vec![1, 2, 3, 4]]),
        (5, vec![vec![1, 2, 3, 4]]),
    ] {
        let p = ListUtil::partition(&list, size).unwrap();
        let collected: Vec<Vec<i32>> = p.iter().map(|s| s.to_vec()).collect();
        assert_eq!(collected, expected, "split size={size} (对齐 Java)");
    }
}

/// 对齐 Java: `ListUtilTest.splitAvgTest()` (行 59-75)
#[test]
fn split_avg_test() {
    let empty: Vec<i32> = vec![];
    let p = ListUtil::split_avg(&empty, 3).unwrap();
    assert_eq!(p.iter().count(), 0, "splitAvg(null, 3) = empty (对齐 Java)");

    let list = vec![1, 2, 3, 4];
    for (limit, expected) in [
        (1, vec![vec![1, 2, 3, 4]]),
        (2, vec![vec![1, 2], vec![3, 4]]),
        (3, vec![vec![1, 2], vec![3], vec![4]]),
        (4, vec![vec![1], vec![2], vec![3], vec![4]]),
    ] {
        let p = ListUtil::split_avg(&list, limit).unwrap();
        let collected: Vec<Vec<i32>> = p.iter().map(|s| s.to_vec()).collect();
        assert_eq!(collected, expected, "splitAvg limit={limit} (对齐 Java)");
    }

    let list2 = vec![1, 2, 3];
    let p = ListUtil::split_avg(&list2, 2).unwrap();
    let collected: Vec<Vec<i32>> = p.iter().map(|s| s.to_vec()).collect();
    assert_eq!(
        collected,
        vec![vec![1, 2], vec![3]],
        "splitAvg limit=2 size=3 (对齐 Java)"
    );
}

/// 对齐 Java: `ListUtilTest.splitAvgTest2()` (行 77-81)
#[test]
fn split_avg_test_2() {
    let list = vec![1, 2, 3];
    let p = ListUtil::split_avg(&list, 5).unwrap();
    let collected: Vec<Vec<i32>> = p.iter().map(|s| s.to_vec()).collect();
    assert_eq!(
        collected,
        vec![vec![1], vec![2], vec![3], vec![], vec![]],
        "splitAvg limit=5 size=3 (对齐 Java)"
    );
}

/// 对齐 Java: `ListUtilTest.splitAvgNotZero()` (行 83-89)
#[test]
fn split_avg_not_zero() {
    let list = vec![1, 2, 3, 4];
    let result = ListUtil::split_avg(&list, 0);
    assert!(
        result.is_err(),
        "splitAvg limit=0 应返回 Err (对齐 Java: 抛 IllegalArgumentException)"
    );
}

/// 对齐 Java: `ListUtilTest.indexOfAll()` (行 100-107)
#[test]
fn index_of_all_test() {
    let a = vec!["1", "2", "3", "4", "3", "2", "1"];
    let idx = ListUtil::index_of_all(&a, |x| *x == "2");
    assert_eq!(idx, vec![1, 5], "indexOfAll(\"2\") (对齐 Java)");

    let idx2 = ListUtil::index_of_all(&a, |x| *x == "1");
    assert_eq!(idx2, vec![0, 6], "indexOfAll(\"1\") (对齐 Java)");
}

/// 对齐 Java: `ListUtilTest.subTest()` (行 178-187)
#[test]
fn sub_test() {
    let of = vec![1, 2, 3, 4];
    let mut sub = ListUtil::sub(&of, 2, 4, 1).unwrap();
    sub.remove(0);
    assert_eq!(of.len(), 4, "原列表 size=4 (对齐 Java)");
    assert_eq!(sub.len(), 1, "子列表 size=1 (对齐 Java)");
}

/// 对齐 Java: `ListUtilTest.swapIndex()` (行 214-219)
#[test]
fn swap_index_test() {
    let mut list = vec![7, 2, 8, 9];
    let _ = ListUtil::swap_to(&mut list, &8, 1);
    assert_eq!(list[1], 8, "swapTo(8, 1) 后 list[1]=8 (对齐 Java)");
}

/// 对齐 Java: `ListUtilTest.swapElement()` (行 221-237)
#[test]
fn swap_element_test() {
    let mut list: Vec<std::collections::HashMap<&str, &str>> = vec![
        [("1", "张三")].into_iter().collect(),
        [("2", "李四")].into_iter().collect(),
        [("3", "王五")].into_iter().collect(),
    ];
    let map2 = list[1].clone();
    let map3 = list[2].clone();
    let swapped = ListUtil::swap_element(&mut list, &map2, &map3);
    assert!(swapped, "swap_element 应成功 (对齐 Java)");
    assert_eq!(
        list[2].get("2"),
        Some(&"李四"),
        "swapElement 后 list[2]=\"李四\" (对齐 Java)"
    );
}

/// 对齐 Java: `ListUtilTest.setOrPaddingNullTest()` (行 239-255)
///
/// 注:hutool `set_or_padding` 额外需要 `padding` 值(填充用),
/// Java 版内部默认 null,Rust 用空字符串近似。
#[test]
fn set_or_padding_null_test() {
    let mut list: Vec<String> = vec!["1".to_string()];

    // 替换原值
    ListUtil::set_or_padding(&mut list, 0, "a".to_string(), String::new()).unwrap();
    assert_eq!(
        list,
        vec!["a".to_string()],
        "setOrPadding(0, a) 替换 (对齐 Java)"
    );

    // append 值
    ListUtil::set_or_padding(&mut list, 1, "a".to_string(), String::new()).unwrap();
    assert_eq!(
        list,
        vec!["a".to_string(), "a".to_string()],
        "setOrPadding(1, a) append (对齐 Java)"
    );

    // padding null 后加入值
    ListUtil::set_or_padding(&mut list, 3, "a".to_string(), String::new()).unwrap();
    assert_eq!(
        list.len(),
        4,
        "setOrPadding(3, a) padding 后 size=4 (对齐 Java)"
    );
}

/// 对齐 Java: `ListUtilTest.reverseNewTest()` (行 257-262)
#[test]
fn reverse_new_test() {
    let view = vec![1, 2, 3];
    let reverse = ListUtil::reverse_new(&view);
    assert_eq!(
        reverse,
        vec![3, 2, 1],
        "reverseNew([1,2,3]) = [3,2,1] (对齐 Java)"
    );
}

/// 对齐 Java: `ListUtilTest.testMoveElementToPosition()` (行 264-283)
#[test]
fn test_move_element_to_position() {
    // Move "B" to position 2
    let mut list = vec!["A".to_string(), "B".into(), "C".into(), "D".into()];
    ListUtil::move_element(&mut list, "B".to_string(), 2).unwrap();
    assert_eq!(
        list,
        vec!["A".to_string(), "C".into(), "B".into(), "D".into()],
        "move B → 2 (对齐 Java)"
    );

    // Move "D" to position 0
    let mut list = vec!["A".to_string(), "B".into(), "C".into(), "D".into()];
    ListUtil::move_element(&mut list, "D".to_string(), 0).unwrap();
    assert_eq!(
        list,
        vec!["D".to_string(), "A".into(), "B".into(), "C".into()],
        "move D → 0 (对齐 Java)"
    );

    // Move "E" (not in list) to position 1 → 插入新元素
    let mut list = vec!["A".to_string(), "B".into(), "C".into(), "D".into()];
    ListUtil::move_element(&mut list, "E".to_string(), 1).unwrap();
    assert_eq!(
        list,
        vec![
            "A".to_string(),
            "E".into(),
            "B".into(),
            "C".into(),
            "D".into()
        ],
        "move E (不在列表) → 1 (对齐 Java)"
    );
}
// ── 扩展 list_util 测试 ──

#[test]
fn to_list_basic() {
    let result = ListUtil::to_list(vec![1, 2, 3]);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn to_linked_list_basic() {
    let result = ListUtil::to_linked_list(vec![1, 2, 3]);
    assert_eq!(result.len(), 3);
    assert_eq!(result[0], 1);
}

#[test]
fn page_basic() {
    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = ListUtil::page(&items, 0, 3).unwrap();
    assert_eq!(result, &[1, 2, 3]);
}

#[test]
fn page_second_page() {
    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = ListUtil::page(&items, 1, 3).unwrap();
    assert_eq!(result, &[4, 5, 6]);
}

#[test]
fn page_last_page() {
    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = ListUtil::page(&items, 3, 3).unwrap();
    assert_eq!(result, &[10]);
}

#[test]
fn sort_by_basic() {
    let mut items = vec![3, 1, 4, 1, 5];
    ListUtil::sort_by(&mut items, |a, b| a.cmp(b));
    assert_eq!(items, vec![1, 1, 3, 4, 5]);
}

#[test]
fn reverse_basic() {
    let mut items = vec![1, 2, 3, 4, 5];
    ListUtil::reverse(&mut items);
    assert_eq!(items, vec![5, 4, 3, 2, 1]);
}

#[test]
fn reverse_new_basic() {
    let items = vec![1, 2, 3, 4, 5];
    let result = ListUtil::reverse_new(&items);
    assert_eq!(result, vec![5, 4, 3, 2, 1]);
    assert_eq!(items, vec![1, 2, 3, 4, 5]);
}

#[test]
fn set_or_append_existing() {
    let mut items = vec![1, 2, 3];
    ListUtil::set_or_append(&mut items, 1, 10);
    assert_eq!(items, vec![1, 10, 3]);
}

#[test]
fn set_or_append_new() {
    let mut items = vec![1, 2, 3];
    ListUtil::set_or_append(&mut items, 5, 10);
    assert_eq!(items, vec![1, 2, 3, 10]);
}

#[test]
fn sub_basic() {
    let items = vec![1, 2, 3, 4, 5];
    let result = ListUtil::sub(&items, 1, 3, 1).unwrap();
    assert_eq!(result, vec![2, 3]);
}

#[test]
fn sub_with_step() {
    let items = vec![1, 2, 3, 4, 5, 6];
    let result = ListUtil::sub(&items, 0, 6, 2).unwrap();
    assert_eq!(result, vec![1, 3, 5]);
}

#[test]
fn last_index_of_basic() {
    let items = vec![1, 2, 3, 2, 1];
    assert_eq!(ListUtil::last_index_of(&items, |x| *x == 2), Some(3));
    assert_eq!(ListUtil::last_index_of(&items, |x| *x == 5), None);
}

#[test]
fn index_of_all_basic() {
    let items = vec![1, 2, 3, 2, 1];
    assert_eq!(ListUtil::index_of_all(&items, |x| *x == 2), vec![1, 3]);
    assert_eq!(
        ListUtil::index_of_all(&items, |x| *x == 5),
        Vec::<usize>::new()
    );
}

#[test]
fn partition_basic() {
    let items = vec![1, 2, 3, 4, 5];
    let p = ListUtil::partition(&items, 2).unwrap();
    assert_eq!(p.get(0), Some(&[1, 2][..]));
    assert_eq!(p.get(1), Some(&[3, 4][..]));
    assert_eq!(p.get(2), Some(&[5][..]));
}

#[test]
fn split_avg_basic() {
    let items = vec![1, 2, 3, 4, 5];
    let p = ListUtil::split_avg(&items, 3).unwrap();
    assert_eq!(p.get(0), Some(&[1, 2][..]));
    assert_eq!(p.get(1), Some(&[3, 4][..]));
    assert_eq!(p.get(2), Some(&[5][..]));
}

/// 对齐 Java: `ListUtilTest.editTest()`
#[test]
fn edit_test() {
    let a = hutool_core::ListUtil::to_linked_list(["1", "2", "3"]);
    let edited = hutool_core::CollUtil::edit(a, |s| Some(format!("edit{s}")));
    assert_eq!(edited[0], "edit1");
    assert_eq!(edited[1], "edit2");
    assert_eq!(edited[2], "edit3");
}

/// 对齐 Java: `ListUtilTest.pageTest()`
#[test]
fn page_test() {
    let a = vec![1, 2, 3, 4, 5];
    // firstPageNo=0 语义：page(0,2)=[1,2]
    let p0 = hutool_core::ListUtil::page(&a, 0, 2).unwrap();
    assert_eq!(p0, &[1, 2]);
    let p1 = hutool_core::ListUtil::page(&a, 1, 2).unwrap();
    assert_eq!(p1, &[3, 4]);
    let p2 = hutool_core::ListUtil::page(&a, 2, 2).unwrap();
    assert_eq!(p2, &[5]);
    let p3 = hutool_core::ListUtil::page(&a, 3, 2).unwrap();
    assert!(p3.is_empty());
}

/// 对齐 Java: `ListUtilTest.sortByPropertyTest()`
#[test]
fn sort_by_property_test() {
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Bean {
        name: String,
    }
    let mut list = vec![
        Bean { name: "c".into() },
        Bean { name: "a".into() },
        Bean { name: "b".into() },
    ];
    hutool_core::ListUtil::sort_by(&mut list, |x, y| x.name.cmp(&y.name));
    assert_eq!(list[0].name, "a");
    assert_eq!(list[1].name, "b");
    assert_eq!(list[2].name, "c");
}

/// 对齐 Java: `ListUtilTest.splitBenchTest()`
#[test]
fn split_bench_test() {
    let list: Vec<i32> = (0..1000).collect();
    let p = hutool_core::ListUtil::partition(&list, 100).unwrap();
    assert_eq!(p.iter().count(), 10);
}
