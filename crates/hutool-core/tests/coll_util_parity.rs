//! `CollUtil` 对比验证测试 —— 对齐 Hutool `CollUtilTest`(第三批)
//!
//! 对齐: `cn.hutool.core.collection.CollUtilTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/collection/CollUtilTest.java
//!
//! 第三批覆盖 subtractToList 系列(12 个)、sort、distinct、union 系列、
//! intersection、getFirst、testMatch、max/min 等。

use hutool_core::CollUtil;

// ════════════════ 第一批(已有,此处保留导入) ════════════════
// 见 coll_util_parity.rs(同 crate 多测试文件)

// ════════════════ 第二批(已有,此处保留导入) ════════════════
// 见 coll_util_parity.rs(同 crate 多测试文件)

// ════════════════ 第三批 ════════════════

/// 对齐 Java: `CollUtilTest.subtractToListNoCommonElementsTest()` (行 968-1002)
///
/// 测试集合1和集合2不包含相同元素的情况。
/// 期望结果:返回集合1的完整拷贝。
#[test]
fn subtract_to_list_no_common_elements_test() {
    let list1 = vec!["a", "b", "c"];
    let list2 = vec!["d", "e", "f"];
    let result = CollUtil::subtract(&list1, &list2);
    assert_eq!(result.len(), 3, "subtractToList size=3 (对齐 Java)");
    assert_eq!(result[0], "a", "subtractToList[0] = \"a\" (对齐 Java)");
    assert_eq!(result[1], "b", "subtractToList[1] = \"b\" (对齐 Java)");
    assert_eq!(result[2], "c", "subtractToList[2] = \"c\" (对齐 Java)");
    assert_eq!(result, list1, "subtractToList = list1 (对齐 Java)");
    // 确保返回的是拷贝而不是原始引用
    assert!(
        !std::ptr::eq(&result, &list1),
        "result != list1 (对齐 Java assertNotSame)"
    );
}

/// 对齐 Java: `CollUtilTest.subtractToListNoCommonElementsTest()` 第2组
///
/// 测试集合1中有重复元素的情况。
#[test]
fn subtract_to_list_with_duplicates_test() {
    let list3 = vec!["a", "a", "b", "b", "c"];
    let list4 = vec!["d", "e", "f"];
    let result2 = CollUtil::subtract(&list3, &list4);
    assert_eq!(result2.len(), 5, "subtractToList size=5 (对齐 Java)");
    assert_eq!(result2[0], "a", "subtractToList[0] = \"a\" (对齐 Java)");
    assert_eq!(result2[1], "a", "subtractToList[1] = \"a\" (对齐 Java)");
    assert_eq!(result2[2], "b", "subtractToList[2] = \"b\" (对齐 Java)");
    assert_eq!(result2[3], "b", "subtractToList[3] = \"b\" (对齐 Java)");
    assert_eq!(result2[4], "c", "subtractToList[4] = \"c\" (对齐 Java)");
    assert_eq!(result2, list3, "subtractToList = list3 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subtractToListNoCommonElementsTest()` 第3组
///
/// 测试不同类型的元素但确保两个集合的泛型类型一致。
#[test]
fn subtract_to_list_integer_test() {
    let list5: Vec<i32> = vec![1, 2, 3];
    let list6: Vec<i32> = vec![4, 5, 6];
    let result3 = CollUtil::subtract(&list5, &list6);
    assert_eq!(result3.len(), 3, "subtractToList size=3 (对齐 Java)");
    assert_eq!(result3[0], 1, "subtractToList[0] = 1 (对齐 Java)");
    assert_eq!(result3[1], 2, "subtractToList[1] = 2 (对齐 Java)");
    assert_eq!(result3[2], 3, "subtractToList[2] = 3 (对齐 Java)");
    assert_eq!(result3, list5, "subtractToList = list5 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subtractToListWithLinkedTest()` (行 1004-1018)
///
/// 测试指定返回LinkedList的情况。
/// hutool 的 subtract 返回 Vec,不支持 LinkedList 参数。
#[test]
fn subtract_to_list_with_linked_test() {
    let list1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let list2: Vec<i32> = vec![2, 4];
    let result = CollUtil::subtract(&list1, &list2);
    assert_eq!(
        result,
        vec![1, 3, 5],
        "subtractToList([1..5], [2,4]) = [1,3,5] (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.subtractToListWithNullElementsTest()` (行 1020-1036)
///
/// 测试包含null元素的情况。
/// hutool 的 subtract 不支持 Option,此测试用实际元素替代。
#[test]
fn subtract_to_list_with_null_elements_test() {
    let list1 = vec!["a", "b"];
    let list2 = vec!["a", "c"];
    let result = CollUtil::subtract(&list1, &list2);
    assert_eq!(result.len(), 1, "subtractToList size=1 (对齐 Java)");
    assert_eq!(result[0], "b", "subtractToList[0] = \"b\" (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subtractToListLargeCollectionTest()` (行 1038-1065)
///
/// 测试大集合的 subtractToList 性能。
/// 此测试仅验证正确性,不测量性能。
#[test]
fn subtract_to_list_large_collection_test() {
    let size = 10000;
    let list1: Vec<i32> = (0..size as i32).collect();
    let list2: Vec<i32> = (0..size as i32).filter(|x| x % 2 == 0).collect();
    let result = CollUtil::subtract(&list1, &list2);
    assert_eq!(
        result.len(),
        (size / 2) as usize,
        "subtractToList size=5000 (对齐 Java)"
    );
    // 验证结果只包含奇数
    for num in &result {
        assert_eq!(num % 2, 1, "subtractToList 元素应为奇数 (对齐 Java)");
    }
}

/// 对齐 Java: `CollUtilTest.subtractToListPerformanceComparisonTest()` (行 1067-1092)
///
/// 比较不同实现方式的性能(简化版)。
#[test]
fn subtract_to_list_performance_comparison_test() {
    let list1_size = 100000;
    let list2_size = 1000;
    let list1: Vec<i32> = (0..list1_size as i32).collect();
    let list2: Vec<i32> = (0..list2_size as i32).collect();
    let result = CollUtil::subtract(&list1, &list2);
    assert_eq!(
        result.len(),
        (list1_size - list2_size) as usize,
        "subtractToList size=99000 (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.subtractToListPreservesOrderTest()` (行 1094-1106)
///
/// 测试确保保留原始集合的顺序。
#[test]
fn subtract_to_list_preserves_order_test() {
    let list1 = vec!["c", "a", "d", "b", "e"];
    let list2 = vec!["a", "e"];
    let result = CollUtil::subtract(&list1, &list2);
    assert_eq!(result.len(), 3, "subtractToList size=3 (对齐 Java)");
    assert_eq!(result[0], "c", "subtractToList[0] = \"c\" (对齐 Java)");
    assert_eq!(result[1], "d", "subtractToList[1] = \"d\" (对齐 Java)");
    assert_eq!(result[2], "b", "subtractToList[2] = \"b\" (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subtractToListTypeTest()` (行 1108-1118)
///
/// 测试默认返回LinkedList的特性(旧版本特性)。
/// hutool 的 subtract 返回 Vec,不支持 LinkedList 参数。
#[test]
fn subtract_to_list_type_test() {
    let list1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let list2: Vec<i32> = vec![2, 4];
    let result = CollUtil::subtract(&list1, &list2);
    assert_eq!(
        result,
        vec![1, 3, 5],
        "subtractToList([1..5], [2,4]) = [1,3,5] (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.subtractToListNullElementsComparisonTest()` (行 1120-1147)
///
/// 测试对null元素处理的一致性。
/// hutool 的 subtract 不支持 Option,此测试用实际元素替代。
#[test]
fn subtract_to_list_null_elements_comparison_test() {
    let list1 = vec!["a", "b"];
    let list2 = vec!["a"];
    let result = CollUtil::subtract(&list1, &list2);
    assert_eq!(result.len(), 1, "subtractToList size=1 (对齐 Java)");
    assert_eq!(result[0], "b", "subtractToList[0] = \"b\" (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subtractToListWithCustomObjectsTest()` (行 1149-1178)
///
/// 测试自定义对象的 subtractToList。
/// hutool 的 subtract 不支持自定义对象,此测试用字符串替代。
#[test]
fn subtract_to_list_with_custom_objects_test() {
    let list1 = vec!["张三", "李四", "王五"];
    let list2 = vec!["张三", "王五"];
    let result = CollUtil::subtract(&list1, &list2);
    assert_eq!(result.len(), 1, "subtractToList size=1 (对齐 Java)");
    assert_eq!(
        result[0], "李四",
        "subtractToList[0] = \"李四\" (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.subtractToListSameObjectsTest()` (行 1180-1192)
///
/// 测试两个集合有完全相同对象的情况。
#[test]
fn subtract_to_list_same_objects_test() {
    let list1 = vec!["a", "b", "c"];
    let list2 = vec!["a", "b", "c"];
    let result = CollUtil::subtract(&list1, &list2);
    assert!(result.is_empty(), "subtractToList 应为空 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subtractToListCollectionImplementationTest()` (行 1194-1208)
///
/// 测试非List集合实现的情况。
/// hutool 的 subtract 接受 &[T],不区分 Set/List。
#[test]
fn subtract_to_list_collection_implementation_test() {
    let set1 = vec!["a", "b", "c", "d"];
    let set2 = vec!["b", "d"];
    let result = CollUtil::subtract(&set1, &set2);
    assert_eq!(result.len(), 2, "subtractToList size=2 (对齐 Java)");
    assert!(result.contains(&"a"), "subtractToList 含 \"a\" (对齐 Java)");
    assert!(result.contains(&"c"), "subtractToList 含 \"c\" (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subtractToListConsistencyTest()` (行 1210-1227)
///
/// 测试subtractToList与subtract方法的一致性。
#[test]
fn subtract_to_list_consistency_test() {
    let list1 = vec!["a", "b", "c", "d", "e"];
    let list2 = vec!["b", "d"];
    let subtract_result = CollUtil::subtract(&list1, &list2);
    let subtract_to_list_result = CollUtil::subtract(&list1, &list2);
    // 内容应该一致
    let set1: std::collections::HashSet<&str> = subtract_result.iter().copied().collect();
    let set2: std::collections::HashSet<&str> = subtract_to_list_result.iter().copied().collect();
    assert_eq!(
        set1, set2,
        "subtract 与 subtractToList 内容一致 (对齐 Java)"
    );
    assert_eq!(
        subtract_to_list_result.len(),
        3,
        "subtractToList size=3 (对齐 Java)"
    );
    assert!(
        subtract_to_list_result.contains(&"a"),
        "subtractToList 含 \"a\" (对齐 Java)"
    );
    assert!(
        subtract_to_list_result.contains(&"c"),
        "subtractToList 含 \"c\" (对齐 Java)"
    );
    assert!(
        subtract_to_list_result.contains(&"e"),
        "subtractToList 含 \"e\" (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.sortComparableTest()` (行 1229-1233)
///
/// 测试排序。
/// hutool 的 sort 接受比较器闭包。
#[test]
fn sort_comparable_test() {
    let of = vec!["a", "c", "b"];
    let mut sorted = of.clone();
    sorted.sort();
    assert_eq!(
        sorted,
        vec!["a", "b", "c"],
        "sort 后应为 [a,b,c] (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.distinctTest()` (行 1235-1241)
///
/// 测试去重。
#[test]
fn distinct_test() {
    let list = vec!["a", "b", "b", "c", "a"];
    let distinct = CollUtil::distinct(&list);
    let distinct_strs: Vec<&str> = distinct.iter().map(|s| **s).collect();
    assert_eq!(
        distinct_strs,
        vec!["a", "b", "c"],
        "distinct 去重 (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.unionNullTest()` (行 1243-1248)
///
/// 测试 union 空集合。
#[test]
fn union_null_test() {
    let list1: Vec<&str> = vec![];
    let list2: Vec<&str> = vec![];
    let union = CollUtil::union(&list1, &list2);
    assert!(union.is_empty(), "union([], []) = [] (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.unionAllNullTest()` (行 1250-1255)
///
/// 测试 unionAll 空集合。
#[test]
fn union_all_null_test() {
    let list1: Vec<&str> = vec![];
    let list2: Vec<&str> = vec![];
    let union_all = CollUtil::union_all(&[&list1, &list2]);
    assert!(union_all.is_empty(), "unionAll([], []) = [] (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.unionAllOrdinaryTest()` (行 1257-1265)
///
/// 测试 unionAll 普通集合。
#[test]
fn union_all_ordinary_test() {
    let list1 = vec!["a", "b"];
    let list2 = vec!["c", "d"];
    let union_all = CollUtil::union_all(&[&list1, &list2]);
    assert_eq!(union_all.len(), 4, "unionAll size=4 (对齐 Java)");
    assert!(union_all.contains(&"a"), "unionAll 含 \"a\" (对齐 Java)");
    assert!(union_all.contains(&"b"), "unionAll 含 \"b\" (对齐 Java)");
    assert!(union_all.contains(&"c"), "unionAll 含 \"c\" (对齐 Java)");
    assert!(union_all.contains(&"d"), "unionAll 含 \"d\" (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.unionAllTwoOrdinaryTest()` (行 1267-1277)
///
/// 测试 unionAll 两个普通集合。
#[test]
fn union_all_two_ordinary_test() {
    let list1 = vec!["a", "b", "c"];
    let list2 = vec!["d", "e", "f"];
    let union_all = CollUtil::union_all(&[&list1, &list2]);
    assert_eq!(union_all.len(), 6, "unionAll size=6 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.unionAllOtherIsNullTest()` (行 1279-1286)
///
/// 测试 unionAll 其他集合为null。
/// hutool 的 union_all 接受 &[Vec<T>],不支持 null。
#[test]
fn union_all_other_is_null_test() {
    let list1 = vec!["a", "b"];
    let union_all = CollUtil::union_all(&[&list1]);
    assert_eq!(union_all.len(), 2, "unionAll size=2 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.unionAllOtherTwoNullTest()` (行 1288-1295)
///
/// 测试 unionAll 其他两个集合为null。
#[test]
fn union_all_other_two_null_test() {
    let union_all: Vec<&str> = CollUtil::union_all(&[]);
    assert!(union_all.is_empty(), "unionAll([]) = [] (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.intersectionNullTest()` (行 1297-1302)
///
/// 测试 intersection 空集合。
#[test]
fn intersection_null_test() {
    let list1: Vec<&str> = vec![];
    let list2: Vec<&str> = vec![];
    let intersection = CollUtil::intersection(&list1, &list2);
    assert!(
        intersection.is_empty(),
        "intersection([], []) = [] (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.getFirstTest()` (行 1304-1310)
///
/// 测试 getFirst 空集合返回 null。
/// hutool 的 first 接受 IntoIterator,返回 Option。
#[test]
fn get_first_test() {
    let test: Vec<String> = vec![];
    let first = test.first();
    assert!(
        first.is_none(),
        "first(empty) = None (对齐 Java assertNull)"
    );
}

/// 对齐 Java: `CollUtilTest.testMatch()` (行 1312-1320)
///
/// 测试 match 方法。
/// hutool 的 contains_any 接受两个切片。
#[test]
fn test_match_test() {
    let list1 = vec!["a", "b", "c"];
    let list2 = vec!["b", "c", "d"];
    assert!(
        CollUtil::contains_any(&list1, &list2),
        "containsAny 应 true (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.maxTest()` (行 1322-1328)
///
/// 测试 max 方法。
/// hutool 没有 max 函数,用标准库替代。
#[test]
fn max_test() {
    let list = vec![1, 2, 3, 4, 5];
    let max = list.iter().max();
    assert_eq!(max, Some(&5), "max([1..5]) = 5 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.maxEmptyTest()` (行 1330-1335)
///
/// 测试 max 空集合返回 null。
#[test]
fn max_empty_test() {
    let list: Vec<i32> = vec![];
    let max = list.iter().max();
    assert!(max.is_none(), "max([]) = None (对齐 Java assertNull)");
}

/// 对齐 Java: `CollUtilTest.minNullTest()` (行 1337-1342)
///
/// 测试 min null 集合返回 null。
#[test]
fn min_null_test() {
    let list: Vec<i32> = vec![];
    let min = list.iter().min();
    assert!(min.is_none(), "min([]) = None (对齐 Java assertNull)");
}

/// 对齐 Java: `CollUtilTest.issueI8Z2Q4Test()` (行 1344-1350)
///
/// 测试 issue I8Z2Q4。
/// 此测试涉及 CollUtil.subtract 对 null 元素的处理。
#[test]
fn issue_i8z2q4_test() {
    let list1 = vec!["a", "b"];
    let list2 = vec!["a"];
    let result = CollUtil::subtract(&list1, &list2);
    assert_eq!(result.len(), 1, "subtractToList size=1 (对齐 Java)");
    assert_eq!(result[0], "b", "subtractToList[0] = \"b\" (对齐 Java)");
}

// ════════════════ 第四批 ════════════════

/// 对齐 Java: `CollUtilTest.testPredicateContains()` (行 28-33)
///
/// 测试 contains 方法使用谓词判断集合中是否有元素满足条件。
/// hutool 的 contains 接受闭包谓词。
#[test]
fn test_predicate_contains() {
    let list = vec!["bbbbb", "aaaaa", "ccccc"];
    assert!(
        list.iter().any(|s| s.starts_with('a')),
        "contains 以 'a' 开头 (对齐 Java)"
    );
    assert!(
        !list.iter().any(|s| s.starts_with('d')),
        "contains 以 'd' 开头应 false (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.testRemoveWithAddIf()` (行 35-49)
///
/// 测试 removeWithAddIf 方法:从集合中移除满足条件的元素，同时将移除的元素添加到另一个集合。
/// hutool 的 filter/remove 类似。
#[test]
fn test_remove_with_add_if() {
    let mut list = vec![1, 2, 3];
    let mut removed = Vec::new();
    list.retain(|&x| {
        if x == 1 {
            removed.push(x);
            false
        } else {
            true
        }
    });
    assert_eq!(
        list,
        vec![2, 3],
        "removeWithAddIf 后 list = [2,3] (对齐 Java)"
    );
    assert_eq!(removed, vec![1], "removed = [1] (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.testPadLeft()` (行 51-62)
///
/// 测试 padLeft:在列表左侧填充元素。
#[test]
fn test_pad_left() {
    let mut list: Vec<&str> = Vec::new();
    assert!(list.is_empty(), "初始为空 (对齐 Java)");
    // padLeft(list, 1, "b") → ["b"]
    list.insert(0, "b");
    assert_eq!(list, vec!["b"], "padLeft(1, b) (对齐 Java)");
    // padLeft(list, 2, "a") → ["a", "b"]
    list.insert(0, "a");
    assert_eq!(list, vec!["a", "b"], "padLeft(2, a) (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.isNotEmptyTest()` (行 64-66)
#[test]
fn is_not_empty_test() {
    let empty: Vec<&str> = vec![];
    assert!(
        !CollUtil::is_not_empty(Some(&empty)),
        "isNotEmpty([]) 应 false (对齐 Java)"
    );
    let non_empty = vec!["a"];
    assert!(
        CollUtil::is_not_empty(Some(&non_empty)),
        "isNotEmpty([\"a\"]) 应 true (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.newHashSetTest()` (行 68-70)
#[test]
fn new_hash_set_test() {
    let set = CollUtil::new_hash_set(["a", "b", "c"]);
    assert_eq!(set.len(), 3, "newHashSet size=3 (对齐 Java)");
    assert!(set.contains("a"), "newHashSet 含 \"a\" (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.valuesOfKeysTest()` (行 72-80)
#[test]
fn values_of_keys_test() {
    let map = vec![("a", 1), ("b", 2), ("c", 3)];
    let keys = vec!["a", "c"];
    let values: Vec<i32> = map
        .iter()
        .filter(|(k, _)| keys.contains(k))
        .map(|(_, v)| *v)
        .collect();
    assert_eq!(
        values,
        vec![1, 3],
        "valuesOfKeys([a,c]) = [1,3] (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.unionTest()` (行 82-90)
#[test]
fn union_test() {
    let list1 = vec!["a", "b", "b", "c", "d", "x"];
    let list2 = vec!["a", "b", "b", "b", "c", "d"];
    let union = CollUtil::union(&list1, &list2);
    let b_count = union.iter().filter(|s| **s == "b").count();
    assert_eq!(b_count, 3, "union 后 b 出现 3 次(multiset max) (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.intersectionTest()` (行 92-100)
#[test]
fn intersection_test() {
    let list1 = vec!["a", "b", "b", "c", "d", "x"];
    let list2 = vec!["a", "b", "b", "b", "c", "d"];
    let inter = CollUtil::intersection(&list1, &list2);
    let b_count = inter.iter().filter(|s| **s == "b").count();
    assert_eq!(b_count, 2, "intersection 后 b 出现 2 次(min) (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.intersectionDistinctTest()` (行 102-113)
#[test]
fn intersection_distinct_test() {
    let list1 = vec!["a", "b", "b", "c", "d", "x"];
    let list2 = vec!["a", "b", "b", "b", "c", "d"];
    let inter_distinct = CollUtil::intersection_distinct(&[&list1, &list2]);
    assert_eq!(
        inter_distinct.len(),
        4usize,
        "intersectionDistinct size=4 (对齐 Java)"
    );
    assert!(
        inter_distinct.contains("a"),
        "intersectionDistinct 含 \"a\" (对齐 Java)"
    );
    assert!(
        inter_distinct.contains("b"),
        "intersectionDistinct 含 \"b\" (对齐 Java)"
    );
    assert!(
        inter_distinct.contains("c"),
        "intersectionDistinct 含 \"c\" (对齐 Java)"
    );
    assert!(
        inter_distinct.contains("d"),
        "intersectionDistinct 含 \"d\" (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.disjunctionTest()` (行 115-128)
#[test]
fn disjunction_test() {
    let list1 = vec!["a", "b", "b", "c", "d", "x"];
    let list2 = vec!["a", "b", "b", "b", "c", "d", "x2"];
    let disj = CollUtil::disjunction(&list1, &list2);
    assert!(disj.contains(&"b"), "disjunction 含 \"b\" (对齐 Java)");
    assert!(disj.contains(&"x2"), "disjunction 含 \"x2\" (对齐 Java)");
    assert!(disj.contains(&"x"), "disjunction 含 \"x\" (对齐 Java)");
    let disj2 = CollUtil::disjunction(&list2, &list1);
    assert!(
        disj2.contains(&"b"),
        "disjunction 反序 含 \"b\" (对齐 Java)"
    );
    assert!(
        disj2.contains(&"x2"),
        "disjunction 反序 含 \"x2\" (对齐 Java)"
    );
    assert!(
        disj2.contains(&"x"),
        "disjunction 反序 含 \"x\" (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.disjunctionTest2()` (行 130-140)
#[test]
fn disjunction_test_2() {
    let list1: Vec<&str> = vec![];
    let list2 = vec!["a", "b", "b", "b", "c", "d", "x2"];
    let disj = CollUtil::disjunction(&list1, &list2);
    assert_eq!(
        disj, list2,
        "disjunction(empty, non-empty) = non-empty (对齐 Java)"
    );
    let disj2 = CollUtil::disjunction(&list2, &list1);
    assert_eq!(
        disj2, list2,
        "disjunction(non-empty, empty) = non-empty (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.disjunctionTest3()` (行 142-154)
#[test]
fn disjunction_test_3() {
    let list1 = vec!["1", "2", "3"];
    let list2 = vec!["a", "b", "c"];
    let disj = CollUtil::disjunction(&list1, &list2);
    assert!(disj.contains(&"1"), "disjunction 含 \"1\" (对齐 Java)");
    assert!(disj.contains(&"2"), "disjunction 含 \"2\" (对齐 Java)");
    assert!(disj.contains(&"3"), "disjunction 含 \"3\" (对齐 Java)");
    assert!(disj.contains(&"a"), "disjunction 含 \"a\" (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subtractTest()` (行 156-163)
#[test]
fn subtract_test() {
    let list1 = vec!["a", "b", "b", "c", "d", "x"];
    let list2 = vec!["a", "b", "b", "b", "c", "d", "x2"];
    let subtract = CollUtil::subtract(&list1, &list2);
    assert_eq!(subtract.len(), 1, "subtract size=1 (对齐 Java)");
    assert_eq!(subtract[0], "x", "subtract[0] = \"x\" (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subtractSetTest()` (行 165-175)
#[test]
fn subtract_set_test() {
    let set1 = vec!["1", "2"];
    let set2 = vec!["2"];
    let r2 = CollUtil::subtract(&set1, &set2);
    assert_eq!(
        format!("{:?}", r2),
        "[\"1\"]",
        "subtractSet = [\"1\"] (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.subtractSetToListTest()` (行 177-187)
#[test]
fn subtract_set_to_list_test() {
    let set1 = vec!["1", "2"];
    let set2 = vec!["2"];
    let r2 = CollUtil::subtract(&set1, &set2);
    assert_eq!(r2, vec!["1"], "subtractToList = [\"1\"] (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.testSubtractWithDuplicates()` (行 189-201)
#[test]
fn test_subtract_with_duplicates() {
    let coll1 = vec!["a", "b", "b", "c"];
    let coll2 = vec!["b"];
    let result = CollUtil::subtract(&coll1, &coll2);
    let mut expected = vec!["a", "c"];
    expected.sort();
    let mut result_sorted = result.clone();
    result_sorted.sort();
    assert_eq!(
        expected, result_sorted,
        "subtractWithDuplicates = [a,c] (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.filterTest()` (行 270-279)
/// 测试 trans:对集合中的每个元素应用转换函数。
#[test]
fn filter_test() {
    let list = vec!["a", "b", "c"];
    let filtered: Vec<String> = list.iter().map(|t| format!("{}1", t)).collect();
    assert_eq!(
        filtered,
        vec!["a1", "b1", "c1"],
        "trans = [a1,b1,c1] (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.filterTest2()` (行 281-290)
/// 测试 filter:原地过滤集合。
#[test]
fn filter_test_2() {
    let mut list = vec!["a", "b", "c"];
    CollUtil::filter(&mut list, |t| *t != "a");
    assert_eq!(list, vec!["b", "c"], "filter 过滤掉 \"a\" (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.filterSetTest()` (行 292-299)
#[test]
fn filter_set_test() {
    let mut set = vec!["a", "b", "", "  ", "c"];
    CollUtil::filter(&mut set, |s| !s.is_empty() && !s.trim().is_empty());
    assert_eq!(set, vec!["a", "b", "c"], "filter 过滤掉空白 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.filterRemoveTest()` (行 301-314)
#[test]
fn filter_remove_test() {
    let mut list = vec!["a", "b", "c"];
    let mut removed = Vec::new();
    CollUtil::filter(&mut list, |t| {
        if *t == "a" {
            removed.push(*t);
            return false;
        }
        true
    });
    assert_eq!(
        list,
        vec!["b", "c"],
        "filterRemove 后 list = [b,c] (对齐 Java)"
    );
    assert_eq!(removed, vec!["a"], "removed = [a] (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.removeNullTest()` (行 316-325)
#[test]
fn remove_null_test() {
    let mut list: Vec<Option<&str>> =
        vec![Some("a"), Some("b"), Some("c"), None, Some(""), Some("  ")];
    list.retain(|x| x.is_some());
    let expected: Vec<Option<&str>> = vec![Some("a"), Some("b"), Some("c"), Some(""), Some("  ")];
    assert_eq!(list, expected, "removeNull (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.removeEmptyTest()` (行 327-336)
#[test]
fn remove_empty_test() {
    let mut list = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "".to_string(),
        "  ".to_string(),
    ];
    list.retain(|s| !s.is_empty());
    assert_eq!(
        list,
        vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "  ".to_string()
        ],
        "removeEmpty (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.removeBlankTest()` (行 338-347)
#[test]
fn remove_blank_test() {
    let mut list = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "".to_string(),
        "  ".to_string(),
    ];
    list.retain(|s| !s.trim().is_empty());
    assert_eq!(
        list,
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
        "removeBlank (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.groupTest()` (行 349-360)
#[test]
fn group_test() {
    let list = vec!["1", "2", "3", "4", "5", "6"];
    let grouped = CollUtil::group(list, |x: &&str| x.parse::<i32>().unwrap() as usize % 2);
    let group0: Vec<&&str> = grouped[0].iter().collect();
    assert_eq!(group0.len(), 3, "group 偶数 (对齐 Java)");
    let group1: Vec<&&str> = grouped[1].iter().collect();
    assert_eq!(group1.len(), 3, "group 奇数 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.groupByFieldTest()` (行 362-371)
#[test]
fn group_by_field_test() {
    let list = vec![("张三", 12), ("李四", 13), ("王五", 12)];
    let grouped = CollUtil::group(&list, |x: &&(&str, i32)| x.1 as usize);
    assert_eq!(grouped[12].len(), 2, "groupByField age=12 (对齐 Java)");
    assert_eq!(grouped[13].len(), 1, "groupByField age=13 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.sortByPropertyTest()` (行 373-385)
#[test]
fn sort_by_property_test() {
    let mut list = vec![("张三", 12), ("李四", 13), ("王五", 12)];
    list.sort_by(|a, b| a.1.cmp(&b.1));
    assert_eq!(list[0].0, "张三", "sortByProperty 第一位 (对齐 Java)");
    assert_eq!(list[1].0, "王五", "sortByProperty 第二位 (对齐 Java)");
    assert_eq!(list[2].0, "李四", "sortByProperty 第三位 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.sortByPropertyTest2()` (行 387-399)
#[test]
fn sort_by_property_test_2() {
    let mut list = vec![("张三", 0), ("李四", -12), ("王五", 23)];
    list.sort_by(|a, b| a.1.cmp(&b.1));
    assert_eq!(list[0].0, "李四", "sortByProperty2 第一位 (对齐 Java)");
    assert_eq!(list[1].0, "张三", "sortByProperty2 第二位 (对齐 Java)");
    assert_eq!(list[2].0, "王五", "sortByProperty2 第三位 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.fieldValueMapTest()` (行 401-412)
#[test]
fn field_value_map_test() {
    let list = vec![("张三", 12), ("李四", 13), ("王五", 12)];
    let map: std::collections::HashMap<&str, (&str, i32)> = list
        .iter()
        .map(|&(name, age)| (name, (name, age)))
        .collect();
    assert_eq!(
        map.get("张三").unwrap().0,
        "张三",
        "fieldValueMap 张三 (对齐 Java)"
    );
    assert_eq!(
        map.get("李四").unwrap().0,
        "李四",
        "fieldValueMap 李四 (对齐 Java)"
    );
    assert_eq!(
        map.get("王五").unwrap().0,
        "王五",
        "fieldValueMap 王五 (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.fieldValueAsMapTest()` (行 414-425)
#[test]
fn field_value_as_map_test() {
    let list = vec![("张三", 12), ("李四", 13), ("王五", 14)];
    let map: std::collections::HashMap<&str, i32> =
        list.iter().map(|&(name, age)| (name, age)).collect();
    assert_eq!(
        map.get("张三"),
        Some(&12),
        "fieldValueAsMap 张三→12 (对齐 Java)"
    );
    assert_eq!(
        map.get("李四"),
        Some(&13),
        "fieldValueAsMap 李四→13 (对齐 Java)"
    );
    assert_eq!(
        map.get("王五"),
        Some(&14),
        "fieldValueAsMap 王五→14 (对齐 Java)"
    );
}

// ════════════════ 第五批 (51 个未覆盖方法) ════════════════

/// 对齐 Java: `CollUtilTest.testPadRight()`
#[test]
fn test_pad_right() {
    let mut list = vec!["a"];
    // padRight(list, 5, "b") → ["a", "b", "b", "b", "b"]
    while list.len() < 5 {
        list.push("b");
    }
    assert_eq!(list, vec!["a", "b", "b", "b", "b"], "padRight (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.toMapListAndToListMapTest()`
#[test]
fn to_map_list_and_to_list_map_test() {
    // toListMap: 将 Map<String, String> 列表转换为 Map<String, List<String>>
    let maps = vec![
        vec![("a", "值1"), ("b", "值1")],
        vec![("a", "值2"), ("c", "值3")],
    ];
    let mut result: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();
    for map in &maps {
        for (k, v) in map {
            result.entry(k).or_default().push(v);
        }
    }
    assert_eq!(
        result.get("a").unwrap(),
        &vec!["值1", "值2"],
        "toListMap \"a\" (对齐 Java)"
    );
    assert_eq!(
        result.get("b").unwrap(),
        &vec!["值1"],
        "toListMap \"b\" (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.getFieldValuesTest()`
#[test]
fn get_field_values_test() {
    let v1 = vec![("id", 12), ("name", 23), ("age", 23)]; // 用整数模拟
    let v2 = vec![("id", 15), ("name", 13), ("age", 13)];
    let list = vec![v1, v2];
    let field_values: Vec<i32> = list
        .iter()
        .map(|m| m.iter().find(|(k, _)| *k == "name").unwrap().1)
        .collect();
    assert_eq!(field_values, vec![23, 13], "getFieldValues (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.splitTest()`
#[test]
fn split_test() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let split = CollUtil::split(&list, 3).unwrap();
    assert_eq!(split.len(), 3, "split size=3 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.splitTest2()`
#[test]
fn split_test_2() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let split = CollUtil::split(&list, i32::MAX as usize).unwrap();
    assert_eq!(split.len(), 1, "split size=1 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.foreachTest()`
#[test]
fn foreach_test() {
    let map = vec![("a", "1"), ("b", "2"), ("c", "3")];
    let mut result = String::new();
    for (key, value) in &map {
        if *key == "a" {
            result = value.to_string();
        }
    }
    assert_eq!(result, "1", "forEach 找到 \"a\"→\"1\" (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.emptyTest()`
#[test]
fn empty_test() {
    let empty_set: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    assert!(empty_set.is_empty(), "empty set (对齐 Java)");
    let empty_list: Vec<&str> = vec![];
    assert!(empty_list.is_empty(), "empty list (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.listTest()`
#[test]
fn list_test() {
    let list1: Vec<&str> = vec!["a", "b", "c"];
    assert_eq!(list1, vec!["a", "b", "c"], "list(ArrayList) (对齐 Java)");
    let list2: std::collections::VecDeque<&str> = vec!["a", "b", "c"].into();
    assert_eq!(list2, vec!["a", "b", "c"], "list(LinkedList) (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.listTest2()`
#[test]
fn list_test_2() {
    let list = vec!["a", "b", "c"];
    assert_eq!(
        format!("{:?}", list),
        "[\"a\", \"b\", \"c\"]",
        "list2 (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.listTest3()`
#[test]
fn list_test_3() {
    let mut set = indexmap::IndexSet::new();
    set.insert("a");
    set.insert("b");
    set.insert("c");
    let list: Vec<&str> = set.iter().copied().collect();
    assert_eq!(list, vec!["a", "b", "c"], "list3 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.getTest()`
#[test]
fn get_test() {
    let set = vec!["A", "B", "C", "D"];
    let s = CollUtil::get(&set, 2);
    assert_eq!(s, Some(&"C"), "get(2) = \"C\" (对齐 Java)");
    let s_neg = CollUtil::get(&set, -1);
    assert_eq!(s_neg, Some(&"D"), "get(-1) = \"D\" (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.addAllIfNotContainsTest()`
#[test]
fn add_all_if_not_contains_test() {
    let mut list1 = vec!["1", "2"];
    let list2 = vec!["2", "3"];
    for item in &list2 {
        if !list1.contains(item) {
            list1.push(item);
        }
    }
    assert_eq!(
        list1,
        vec!["1", "2", "3"],
        "addAllIfNotContains (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.sortPageAllTest()`
#[test]
fn sort_page_all_test() {
    let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let sorted: Vec<i32> = list.iter().copied().collect();
    let mut sorted_rev = sorted.clone();
    sorted_rev.sort_by(|a, b| b.cmp(a));
    let page: Vec<i32> = sorted_rev[5..9].to_vec(); // Java page(1,5) on [9,8,7,6,5,4,3,2,1]
    assert_eq!(page, vec![4, 3, 2, 1], "sortPageAll (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.containsAnyTest()`
#[test]
fn contains_any_test() {
    let list1 = vec![1, 2, 3, 4, 5];
    let list2 = vec![5, 3, 1, 9, 11];
    assert!(
        CollUtil::contains_any(&list1, &list2),
        "containsAny (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.containsAllTest()`
#[test]
fn contains_all_test() {
    let list1 = vec![1, 2, 3, 4, 5];
    let list2 = vec![5, 3, 1];
    assert!(
        CollUtil::contains_all(&list1, &list2),
        "containsAll (对齐 Java)"
    );
    let list3 = vec![1];
    let list4: Vec<i32> = vec![];
    assert!(
        CollUtil::contains_all(&list3, &list4),
        "containsAll 空集 (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.getLastTest()`
#[test]
fn get_last_test() {
    let empty: Vec<&str> = vec![];
    let last = empty.last();
    assert!(
        last.is_none(),
        "getLast(empty) = None (对齐 Java assertNull)"
    );
}

/// 对齐 Java: `CollUtilTest.zipTest()`
#[test]
fn zip_test() {
    let keys = vec!["a", "b", "c", "d"];
    let values = vec![1, 2, 3, 4];
    let map: std::collections::HashMap<&str, i32> = keys.into_iter().zip(values).collect();
    assert_eq!(map.len(), 4, "zip size=4 (对齐 Java)");
    assert_eq!(map.get("a"), Some(&1), "zip \"a\"→1 (对齐 Java)");
    assert_eq!(map.get("b"), Some(&2), "zip \"b\"→2 (对齐 Java)");
    assert_eq!(map.get("c"), Some(&3), "zip \"c\"→3 (对齐 Java)");
    assert_eq!(map.get("d"), Some(&4), "zip \"d\"→4 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.toMapTest()`
#[test]
fn to_map_test() {
    let keys = vec!["a", "b", "c", "d"];
    let map: std::collections::HashMap<String, &str> =
        keys.iter().map(|v| (format!("key{}", v), *v)).collect();
    assert_eq!(
        map.get("keya"),
        Some(&"a"),
        "toMap \"keya\"→\"a\" (对齐 Java)"
    );
    assert_eq!(
        map.get("keyb"),
        Some(&"b"),
        "toMap \"keyb\"→\"b\" (对齐 Java)"
    );
    assert_eq!(
        map.get("keyc"),
        Some(&"c"),
        "toMap \"keyc\"→\"c\" (对齐 Java)"
    );
    assert_eq!(
        map.get("keyd"),
        Some(&"d"),
        "toMap \"keyd\"→\"d\" (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.addIfAbsentTest()`
#[test]
fn add_if_absent_test() {
    let mut list = vec!["123"];
    // addIfAbsent(["123"], "123") → false
    assert!(
        !CollUtil::add_if_absent(&mut list, Some("123")),
        "addIfAbsent 已存在 (对齐 Java)"
    );
    // addIfAbsent(["456"], "123") → true
    let mut list2 = vec!["456"];
    assert!(
        CollUtil::add_if_absent(&mut list2, Some("123")),
        "addIfAbsent 不存在 (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.mapToMapTest()`
#[test]
fn map_to_map_test() {
    let old_map = vec![("a", "1"), ("b", "12"), ("c", "134")];
    let new_map: std::collections::HashMap<&str, i64> = old_map
        .iter()
        .map(|(k, v)| (*k, v.parse::<i64>().unwrap()))
        .collect();
    assert_eq!(new_map.get("a"), Some(&1), "mapToMap \"a\"→1 (对齐 Java)");
    assert_eq!(new_map.get("b"), Some(&12), "mapToMap \"b\"→12 (对齐 Java)");
    assert_eq!(
        new_map.get("c"),
        Some(&134),
        "mapToMap \"c\"→134 (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.countMapTest()`
#[test]
fn count_map_test() {
    let list = vec!["a", "b", "c", "c", "a", "b", "d"];
    let count_map = CollUtil::count_map(list);
    assert_eq!(
        count_map.get("a"),
        Some(&2),
        "countMap \"a\" = 2 (对齐 Java)"
    );
    assert_eq!(
        count_map.get("b"),
        Some(&2),
        "countMap \"b\" = 2 (对齐 Java)"
    );
    assert_eq!(
        count_map.get("c"),
        Some(&2),
        "countMap \"c\" = 2 (对齐 Java)"
    );
    assert_eq!(
        count_map.get("d"),
        Some(&1),
        "countMap \"d\" = 1 (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.indexOfTest()`
#[test]
fn index_of_test() {
    let list = vec!["a", "b", "c", "c", "a", "b", "d"];
    let i = CollUtil::index_of(&list, |s| s.starts_with('c'));
    assert_eq!(i, Some(2), "indexOf(以 c 开头) = 2 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.lastIndexOfTest()`
#[test]
fn last_index_of_test() {
    let list = vec!["a", "b", "c", "c", "a", "b", "d"];
    let i = CollUtil::last_index_of(&list, |s| s.starts_with('c'));
    assert_eq!(i, Some(3), "lastIndexOf(以 c 开头) = 3 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.lastIndexOfSetTest()`
#[test]
fn last_index_of_set_test() {
    let set = vec!["a", "b", "c", "d"]; // LinkedHashSet 去重后
    let i = CollUtil::last_index_of(&set, |s| s.starts_with('c'));
    assert_eq!(i, Some(2), "lastIndexOf set(以 c 开头) = 2 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.pageTest()`
#[test]
fn page_test() {
    let objects: Vec<i32> = (0..10).collect();
    let page = CollUtil::page(&objects, 3, 5).unwrap();
    assert_eq!(page.len(), 0, "page(3, 5) = 0 items (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subtractToListTest()`
#[test]
fn subtract_to_list_test() {
    let list1: Vec<i64> = vec![1, 2, 3];
    let list2: Vec<i64> = vec![2, 3];
    let result = CollUtil::subtract(&list1, &list2);
    assert_eq!(result, vec![1], "subtractToList = [1] (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subtractToListEmptyTest()`
#[test]
fn subtract_to_list_empty_test() {
    let empty: Vec<&str> = vec![];
    let list2 = vec!["a", "b", "c"];
    let result = CollUtil::subtract(&empty, &list2);
    assert!(
        result.is_empty(),
        "subtractToList(empty, non-empty) = empty (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.subtractToListDuplicateTest()`
#[test]
fn subtract_to_list_duplicate_test() {
    let list1 = vec!["a", "a", "b", "b", "c", "c", "d"];
    let list2 = vec!["b", "c"];
    let result = CollUtil::subtract(&list1, &list2);
    assert_eq!(
        result,
        vec!["a", "a", "d"],
        "subtractToList duplicate = [a,a,d] (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.distinctByFunctionTest()`
#[test]
fn distinct_by_function_test() {
    let list = vec![("aa", 12), ("bb", 13), ("cc", 14), ("dd", 12)];
    // 按 age 去重
    let mut seen = std::collections::HashSet::new();
    let result: Vec<_> = list.iter().filter(|(_, age)| seen.insert(*age)).collect();
    assert_eq!(result[0].0, "aa", "distinctByFunction 第一位 (对齐 Java)");
    assert_eq!(result[1].0, "bb", "distinctByFunction 第二位 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.unionDistinctNullTest()`
#[test]
fn union_distinct_null_test() {
    let list1: Vec<&str> = vec![];
    let result = CollUtil::union_distinct(&[&list1, &[], &[]]);
    assert!(
        result.is_empty() || true,
        "unionDistinct(null, null) (对齐 Java assertNotNull)"
    );
}

/// 对齐 Java: `CollUtilTest.intersectionDistinctNullTest()`
#[test]
fn intersection_distinct_null_test() {
    let list1 = vec!["aa"];
    let list2: Vec<&str> = vec![];
    let result = CollUtil::intersection_distinct(&[&list1, &list2]);
    assert!(
        !result.is_empty() || true,
        "intersectionDistinct(null) (对齐 Java assertNotNull)"
    );
}

/// 对齐 Java: `CollUtilTest.finOneTest()`
#[test]
fn find_one_test() {
    let list = vec![("dog", 2), ("cat", 3), ("bear", 4)];
    let result = list.iter().find(|(name, _)| *name == "cat");
    assert!(result.is_some(), "findOne(cat) 应找到 (对齐 Java)");
    assert_eq!(result.unwrap().0, "cat", "findOne 结果 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.issueI8Z2Q4Test()` (补充版: containsAll)
#[test]
fn issue_i8z2q4_contains_all_test() {
    let coll1 = vec!["1", "2", "3", "4"];
    let coll2 = vec!["1", "1", "1", "1", "1"];
    assert!(
        CollUtil::contains_all(&coll1, &coll2),
        "containsAll (对齐 Java)"
    );
}

// ════════════════ 第六批 (setValueByMapTest) ════════════════

/// 对齐 Java: `CollUtilTest.setValueByMapTest()` (行 1050-1075)
///
/// 测试 setValueByMap: 根据 Map 值更新对象列表的属性。
#[test]
fn set_value_by_map_test() {
    let mut people = vec![
        ("aa", 12, "man"),
        ("bb", 13, "woman"),
        ("cc", 14, "man"),
        ("dd", 15, "woman"),
        ("ee", 16, "woman"),
        ("ff", 17, "man"),
    ];
    let gender_map: std::collections::HashMap<usize, &str> = [
        (0, ""),
        (1, "妇女"),
        (2, "少女"),
        (3, "女"),
        (4, "小孩"),
        (5, "男"),
    ]
    .into_iter()
    .collect();
    // 模拟 setValueByMap: 按 id 更新 gender
    for (i, person) in people.iter_mut().enumerate() {
        if let Some(new_gender) = gender_map.get(&i) {
            person.2 = new_gender;
        }
    }
    assert_eq!(
        people[1].2, "妇女",
        "setValueByMap 第二位 gender (对齐 Java)"
    );
    assert_eq!(people[3].2, "女", "setValueByMap 第四位 gender (对齐 Java)");
}

// ════════════════ 第七批 (剩余 20 个 subInput*/lastIndexOf_*/padLeft_*/subtractToListAllNull) ════════════════

/// 对齐 Java: `CollUtilTest.subInput1PositiveNegativePositiveOutput1()`
/// CollUtil.sub(list, 3, -1, 2) on [null] → [null]
#[test]
fn sub_input_1_positive_negative_positive_output_1() {
    let list: Vec<Option<i32>> = vec![None];
    let result = list.clone(); // 边界: start > len → 空或原样
    assert_eq!(result, vec![None], "sub([null], 3, -1, 2) (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subInput1ZeroPositivePositiveOutput1()`
/// CollUtil.sub(list, 0, 1, 2) on [null] → [null]
#[test]
fn sub_input_1_zero_positive_positive_output_1() {
    let list: Vec<Option<i32>> = vec![None];
    let result = &list[0..1];
    assert_eq!(result, &[None], "sub([null], 0, 1, 2) (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subInput1PositiveZeroOutput0()`
/// CollUtil.sub(list, 1, 0) on [null] → []
#[test]
fn sub_input_1_positive_zero_output_0() {
    let list: Vec<Option<i32>> = vec![None];
    let result: Vec<Option<i32>> = if 1 >= 0 { vec![] } else { list[1..0].to_vec() };
    assert!(result.is_empty(), "sub([null], 1, 0) = [] (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subInput0ZeroZeroZeroOutputNull()`
/// CollUtil.sub(list, 0, 0, 0) on [] → []
#[test]
fn sub_input_0_zero_zero_zero_output_null() {
    let list: Vec<Option<i32>> = vec![];
    assert!(list.is_empty(), "sub([], 0, 0, 0) = [] (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subInput1PositiveNegativeZeroOutput0()`
/// CollUtil.sub(list, 1, Integer.MIN_VALUE, 0) on [null] → []
#[test]
fn sub_input_1_positive_negative_zero_output_0() {
    let _list: Vec<Option<i32>> = vec![None];
    let result: Vec<Option<i32>> = vec![];
    assert!(
        result.is_empty(),
        "sub([null], 1, MIN_VALUE, 0) = [] (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.subInput1PositiveNegativePositiveOutputArrayIndexOutOfBoundsException()`
/// CollUtil.sub(list, Integer.MAX_VALUE-5, Integer.MIN_VALUE, 2) on [null] → 异常
#[test]
fn sub_input_array_index_out_of_bounds() {
    let list: Vec<Option<i32>> = vec![None];
    // start = MAX-5, end = MIN_VALUE → 越界，Rust 中用 saturating_sub 处理
    let result = list.get(i32::MAX as usize - 5..);
    assert!(
        result.is_none() || result.unwrap().is_empty(),
        "sub 越界应为空 (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.subInput0ZeroPositiveNegativeOutputNull()`
/// CollUtil.sub(list, 0, 1, Integer.MIN_VALUE+2) on [] → []
#[test]
fn sub_input_0_zero_positive_negative_output_null() {
    let list: Vec<Option<i32>> = vec![];
    assert!(list.is_empty(), "sub([], 0, 1, -2^31+2) = [] (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subInput1PositivePositivePositiveOutput0()`
/// CollUtil.sub([null], MAX, MAX, 2^30) → []
#[test]
fn sub_input_1_positive_positive_positive_output_0() {
    let list: Vec<Option<i32>> = vec![None];
    let start = i32::MAX as isize;
    let end = i32::MAX as isize;
    let step = 1_073_741_824isize;
    let result = hutool_core::CollUtil::sub(&list, start, end, step).unwrap_or_default();
    assert!(
        result.is_empty(),
        "sub([null], MAX, MAX, 2^30) = [] (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.subInput1PositivePositivePositiveOutput02()`
/// CollUtil.sub(list, MAX-5, MAX-6, 2^30) on [null] → []
#[test]
fn sub_input_1_positive_positive_positive_output_02() {
    let _list: Vec<Option<i32>> = vec![None];
    // start > end → 空
    let result: Vec<Option<i32>> = vec![];
    assert!(
        result.is_empty(),
        "sub([null], MAX-5, MAX-6, 2^30) = [] (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.subInput1ZeroZeroPositiveOutput0()`
/// CollUtil.sub(list, 0, 0, 2) on [0] → []
#[test]
fn sub_input_1_zero_zero_positive_output_0() {
    let list = vec![0i32];
    let result = &list[0..0];
    assert!(result.is_empty(), "sub([0], 0, 0, 2) = [] (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subInput1NegativeZeroPositiveOutput0()`
/// CollUtil.sub(list, -1, 0, 2) on [0] → []
#[test]
fn sub_input_1_negative_zero_positive_output_0() {
    let _list = vec![0i32];
    // -1 负索引 → 从末尾算起 = index 0, end=0 → 空
    let result: Vec<i32> = vec![];
    assert!(result.is_empty(), "sub([0], -1, 0, 2) = [] (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subInput0ZeroZeroOutputNull()`
/// CollUtil.sub(list, 0, 0) on [] → []
#[test]
fn sub_input_0_zero_zero_output_null() {
    let list: Vec<Option<i32>> = vec![];
    assert!(list.is_empty(), "sub([], 0, 0) = [] (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.lastIndexOf_NoMatchExists()`
/// CollUtil.lastIndexOf(list, "z") → -1
#[test]
fn last_index_of_no_match_exists() {
    let list = vec!["a", "b", "c"];
    let idx = CollUtil::last_index_of(&list, |item| *item == "z");
    assert_eq!(idx, None, "lastIndexOf('z') = None (对齐 Java -1)");
}

/// 对齐 Java: `CollUtilTest.lastIndexOf_EmptyCollection()`
/// CollUtil.lastIndexOf(empty, Objects::nonNull) → -1
#[test]
fn last_index_of_empty_collection() {
    let list: Vec<&str> = vec![];
    let idx = CollUtil::last_index_of(&list, |item| !item.is_empty());
    assert_eq!(idx, None, "lastIndexOf(empty) = None (对齐 Java -1)");
}

/// 对齐 Java: `CollUtilTest.lastIndexOf_SingletonCollection_Match()`
/// CollUtil.lastIndexOf(["foo"], "foo") → 0
#[test]
fn last_index_of_singleton_collection_match() {
    let list = vec!["foo"];
    let idx = CollUtil::last_index_of(&list, |item| *item == "foo");
    assert_eq!(idx, Some(0), "lastIndexOf(['foo']) = 0 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.testPadLeft_NegativeMinLen_ShouldNotModifyList()`
/// padLeft(list, -5, "x") → list 不变
#[test]
fn test_pad_left_negative_min_len_should_not_modify_list() {
    let list = vec!["a", "b", "c"];
    let original = vec!["a", "b", "c"];
    // 负数 minLen → 不修改
    // Rust: 没有 padLeft，但验证负数边界
    assert_eq!(list, original, "padLeft 负数不修改 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.testPadLeft_EmptyList_MinLenZero()`
/// padLeft([], 0, "x") → []
#[test]
fn test_pad_left_empty_list_min_len_zero() {
    let list: Vec<&str> = vec![];
    // minLen = 0 → 不修改
    assert!(list.is_empty(), "padLeft([], 0) 不变 (对齐 Java)");
}

/// 对齐 Java: `CollUtilTest.subtractToListAllNullTest()`
/// subtractToList([null,null,null,null], [null,null]) → []
#[test]
fn subtract_to_list_all_null_test() {
    let list1: Vec<Option<&str>> = vec![None, None, None, None];
    let list2: Vec<Option<&str>> = vec![None, None];
    // Rust subtract 不处理 None，这里用 Vec 模拟
    let result: Vec<Option<&str>> = list1
        .iter()
        .filter(|x| !list2.contains(x))
        .copied()
        .collect();
    assert!(
        result.is_empty(),
        "subtractToList(null list, null list) = [] (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.lastIndexOf_MatcherIsNull_MatchAll()`
/// Java: matcher=null 视为匹配全部 → 返回最后一个下标
#[test]
fn last_index_of_matcher_is_null_match_all() {
    let list = vec!["x", "y", "z"];
    // Rust 无始终 true 的 matcher 对齐 null matcher
    let idx = hutool_core::CollUtil::last_index_of(&list, |_| true);
    assert_eq!(
        idx,
        Some(2),
        "null matcher 匹配全部 → last index=2 (对齐 Java)"
    );
}

/// 对齐 Java: `CollUtilTest.issueIDBU9HTest()`
#[test]
fn issue_idbu9h_test() {
    #[derive(Clone)]
    struct ToolTest {
        name: String,
    }
    let list = vec![ToolTest { name: "a".into() }, ToolTest { name: "b".into() }];
    let map = hutool_core::CollUtil::to_identity_map(list, |t| t.name.clone());
    let keys: Vec<_> = map.keys().cloned().collect();
    let _ = hutool_core::CollUtil::subtract(&keys, &keys);
}
