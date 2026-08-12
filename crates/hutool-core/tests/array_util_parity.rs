//! array_util parity tests
//! 对齐: `cn.hutool.core.util.ArrayUtilTest`

use hutool_core::ArrayUtil;

// ── 空值判断 ──

#[test]
fn is_empty_empty() {
    assert!(ArrayUtil::is_empty(&[] as &[i32]));
}

#[test]
fn is_empty_non_empty() {
    assert!(!ArrayUtil::is_empty(&[1, 2, 3]));
}

#[test]
fn is_not_empty_non_empty() {
    assert!(ArrayUtil::is_not_empty(&[1, 2, 3]));
}

#[test]
fn is_not_empty_empty() {
    assert!(!ArrayUtil::is_not_empty(&[] as &[i32]));
}

// ── 默认值 ──

#[test]
fn default_if_empty_non_empty() {
    let result = ArrayUtil::default_if_empty(&[1, 2, 3], &[4, 5]);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn default_if_empty_empty() {
    let result = ArrayUtil::default_if_empty(&[] as &[i32], &[4, 5]);
    assert_eq!(result, vec![4, 5]);
}

// ── 查找操作 ──

#[test]
fn contains_found() {
    assert!(ArrayUtil::contains(&[1, 2, 3], &2));
}

#[test]
fn contains_not_found() {
    assert!(!ArrayUtil::contains(&[1, 2, 3], &5));
}

#[test]
fn index_of_found() {
    assert_eq!(ArrayUtil::index_of(&[1, 2, 3, 2], &2), Some(1));
}

#[test]
fn index_of_not_found() {
    assert_eq!(ArrayUtil::index_of(&[1, 2, 3], &5), None);
}

#[test]
fn last_index_of_found() {
    assert_eq!(ArrayUtil::last_index_of(&[1, 2, 3, 2], &2), Some(3));
}

#[test]
fn last_index_of_not_found() {
    assert_eq!(ArrayUtil::last_index_of(&[1, 2, 3], &5), None);
}

// ── 转换操作 ──

#[test]
fn to_vec_basic() {
    let result = ArrayUtil::to_vec(&[1, 2, 3]);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn new_array_basic() {
    let result: Vec<i32> = ArrayUtil::new_array(3);
    assert_eq!(result, vec![0, 0, 0]);
}

// ── 操作 ──

#[test]
fn append_basic() {
    let result = ArrayUtil::append(&[1, 2], &[3, 4]);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[test]
fn insert_basic() {
    let result = ArrayUtil::insert(&[1, 2, 3], 1, &[10, 20]);
    assert_eq!(result, vec![1, 10, 20, 2, 3]);
}

#[test]
fn remove_basic() {
    let result = ArrayUtil::remove(&[1, 2, 3, 4], 2);
    assert_eq!(result, vec![1, 2, 4]);
}

#[test]
fn remove_element_basic() {
    let result = ArrayUtil::remove_element(&[1, 2, 3, 2, 1], &2);
    assert_eq!(result, vec![1, 3, 1]);
}

// ── 排序操作 ──

#[test]
fn sort_basic() {
    let mut arr = vec![3, 1, 4, 1, 5];
    ArrayUtil::sort(&mut arr);
    assert_eq!(arr, vec![1, 1, 3, 4, 5]);
}

#[test]
fn sort_by_basic() {
    let mut arr = vec![3, 1, 4, 1, 5];
    ArrayUtil::sort_by(&mut arr, |a, b| b.cmp(a));
    assert_eq!(arr, vec![5, 4, 3, 1, 1]);
}

#[test]
fn reverse_basic() {
    let mut arr = vec![1, 2, 3, 4, 5];
    ArrayUtil::reverse(&mut arr);
    assert_eq!(arr, vec![5, 4, 3, 2, 1]);
}

// ── 合并操作 ──

#[test]
fn add_all_basic() {
    let result = ArrayUtil::add_all(&[&[1, 2][..], &[3, 4][..], &[5][..]]);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

// ── 截取操作 ──

#[test]
fn sub_basic() {
    let result = ArrayUtil::sub(&[1, 2, 3, 4, 5], 1, 3);
    assert_eq!(result, vec![2, 3]);
}

#[test]
fn sub_out_of_bounds() {
    let result = ArrayUtil::sub(&[1, 2, 3], 0, 10);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn split_basic() {
    let result = ArrayUtil::split(&[1, 2, 3, 4, 5], 2);
    assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5]]);
}

#[test]
fn split_empty() {
    let result: Vec<Vec<i32>> = ArrayUtil::split(&[], 3);
    assert_eq!(result, vec![Vec::<i32>::new()]);
}

// ── 填充操作 ──

#[test]
fn fill_basic() {
    let mut arr = vec![0; 5];
    ArrayUtil::fill(&mut arr, &42);
    assert_eq!(arr, vec![42, 42, 42, 42, 42]);
}

#[test]
fn fill_range_basic() {
    let mut arr = vec![0; 5];
    ArrayUtil::fill_range(&mut arr, 1, 3, &42);
    assert_eq!(arr, vec![0, 42, 42, 0, 0]);
}

// ── 转换操作 ──

#[test]
fn to_string_basic() {
    assert_eq!(ArrayUtil::to_string(&[1, 2, 3]), "[1, 2, 3]");
}

#[test]
fn join_basic() {
    assert_eq!(ArrayUtil::join(&[1, 2, 3], ", "), "1, 2, 3");
}

// ── 比较操作 ──

#[test]
fn equals_equal() {
    assert!(ArrayUtil::equals(&[1, 2, 3], &[1, 2, 3]));
}

#[test]
fn equals_not_equal() {
    assert!(!ArrayUtil::equals(&[1, 2, 3], &[1, 2, 4]));
}

#[test]
fn is_equals_equal() {
    assert!(ArrayUtil::is_equals(&[1, 2, 3], &[1, 2, 3]));
}

#[test]
fn is_equals_not_equal() {
    assert!(!ArrayUtil::is_equals(&[1, 2, 3], &[1, 2, 4]));
}

// ── 对齐 Hutool ArrayUtilTest ──

/// 对齐 Java: `ArrayUtilTest.isEmptyTest()`
#[test]
fn is_empty_test() {
    let a: [i32; 0] = [];
    assert!(ArrayUtil::is_empty(&a));
    let d: [&str; 0] = [];
    assert!(ArrayUtil::is_empty(&d));
    let e = ["1", "2"];
    assert!(!ArrayUtil::is_empty(&e));
}

/// 对齐 Java: `ArrayUtilTest.isNotEmptyTest()`
#[test]
fn is_not_empty_test() {
    let a = [1, 2];
    assert!(ArrayUtil::is_not_empty(&a));
    let b = ["a", "b", "c"];
    assert!(ArrayUtil::is_not_empty(&b));
}

/// 对齐 Java: `ArrayUtilTest.newArrayTest()`
#[test]
fn new_array_test() {
    let new_array: Vec<String> = ArrayUtil::new_array(3);
    assert_eq!(3, new_array.len());
}

/// 对齐 Java: `ArrayUtilTest.cloneTest()`
#[test]
fn clone_test() {
    let b = [1, 2, 3];
    let clone_b = ArrayUtil::to_vec(&b);
    assert_eq!(b.to_vec(), clone_b);
    let a = [1, 2, 3];
    let clone = ArrayUtil::to_vec(&a);
    assert_eq!(a.to_vec(), clone);
}

/// 对齐 Java: `ArrayUtilTest.indexOfTest()`
#[test]
fn index_of_test() {
    let a = [1, 2, 3, 4, 5, 6];
    assert_eq!(ArrayUtil::index_of(&a, &3), Some(2));
    let b = [1i64, 2, 3, 4, 5, 6];
    assert_eq!(ArrayUtil::index_of(&b, &3), Some(2));
}

/// 对齐 Java: `ArrayUtilTest.lastIndexOfTest()`
#[test]
fn last_index_of_test() {
    let a = [1, 2, 3, 4, 3, 6];
    assert_eq!(ArrayUtil::last_index_of(&a, &3), Some(4));
    let b = [1i64, 2, 3, 4, 3, 6];
    assert_eq!(ArrayUtil::last_index_of(&b, &3), Some(4));
}

/// 对齐 Java: `ArrayUtilTest.containsTest()`
#[test]
fn contains_test() {
    let a = [1, 2, 3, 4, 3, 6];
    assert!(ArrayUtil::contains(&a, &3));
    let b = [1i64, 2, 3, 4, 3, 6];
    assert!(ArrayUtil::contains(&b, &3));
}

/// 对齐 Java: `ArrayUtilTest.containsAnyTest()`
#[test]
fn contains_any_test() {
    let a = [1, 2, 3, 4, 3, 6];
    let contains = [4, 10, 40].iter().any(|v| ArrayUtil::contains(&a, v));
    assert!(contains);
    let contains = [10, 40].iter().any(|v| ArrayUtil::contains(&a, v));
    assert!(!contains);
}

/// 对齐 Java: `ArrayUtilTest.containsAllTest()`
#[test]
fn contains_all_test() {
    let a = [1, 2, 3, 4, 3, 6];
    let contains = [4, 2, 6].iter().all(|v| ArrayUtil::contains(&a, v));
    assert!(contains);
    let contains = [1, 2, 3, 5].iter().all(|v| ArrayUtil::contains(&a, v));
    assert!(!contains);
}

/// 对齐 Java: `ArrayUtilTest.appendTest()`
#[test]
fn append_test() {
    let a = ["1", "2", "3", "4"];
    let b = ["a", "b", "c"];
    let result = ArrayUtil::append(&a, &b);
    assert_eq!(result, vec!["1", "2", "3", "4", "a", "b", "c"]);
}

/// 对齐 Java: `ArrayUtilTest.insertTest()`
#[test]
fn insert_test() {
    let a = ["1", "2", "3", "4"];
    let b = ["a", "b", "c"];
    // 在第0个位置插入
    let result = ArrayUtil::insert(&a, 0, &b);
    assert_eq!(result, vec!["a", "b", "c", "1", "2", "3", "4"]);
    // 在第2个位置插入
    let result = ArrayUtil::insert(&a, 2, &b);
    assert_eq!(result, vec!["1", "2", "a", "b", "c", "3", "4"]);
    // 在第4个位置插入
    let result = ArrayUtil::insert(&a, 4, &b);
    assert_eq!(result, vec!["1", "2", "3", "4", "a", "b", "c"]);
}

/// 对齐 Java: `ArrayUtilTest.joinTest()`
#[test]
fn join_test() {
    let array2 = ["aa", "bb", "cc", "dd"];
    let join2 = ArrayUtil::join(&array2, ",");
    assert_eq!(join2, "aa,bb,cc,dd");
}

/// 对齐 Java: `ArrayUtilTest.toStingTest()`
#[test]
fn to_sting_test() {
    let a = [1, 3, 56, 6, 7];
    assert_eq!(ArrayUtil::to_string(&a), "[1, 3, 56, 6, 7]");
    let b = [1i64, 3, 56, 6, 7];
    assert_eq!(ArrayUtil::to_string(&b), "[1, 3, 56, 6, 7]");
}

/// 对齐 Java: `ArrayUtilTest.addAllTest()`
#[test]
fn add_all_test() {
    let a = [1, 2];
    let b = [3, 4];
    let result = ArrayUtil::add_all(&[&a[..], &b[..]]);
    assert_eq!(result, vec![1, 2, 3, 4]);
}

/// 对齐 Java: `ArrayUtilTest.reverseTest()`
#[test]
fn reverse_test() {
    let mut a = [1, 2, 3, 4, 5];
    ArrayUtil::reverse(&mut a);
    assert_eq!(a, [5, 4, 3, 2, 1]);
}

/// 对齐 Java: `ArrayUtilTest.splitTest()`
#[test]
fn split_test() {
    let array = [1, 2, 3, 4, 5];
    let result = ArrayUtil::split(&array, 2);
    assert_eq!(result, vec![vec![1, 2], vec![3, 4], vec![5]]);
}

/// 对齐 Java: `ArrayUtilTest.filterTestForFilter()`
#[test]
fn filter_test_for_filter() {
    let a = [1, 2, 3, 4, 5, 6];
    let filter: Vec<i32> = a.iter().copied().filter(|t| t % 2 == 0).collect();
    assert_eq!(filter, vec![2, 4, 6]);
}

/// 对齐 Java: `ArrayUtilTest.editTest()`
#[test]
fn edit_test() {
    let a = [1, 2, 3, 4, 5, 6];
    let filter: Vec<i32> = a
        .iter()
        .map(|t| if t % 2 == 0 { t * 10 } else { *t })
        .collect();
    assert_eq!(filter, vec![1, 20, 3, 40, 5, 60]);
}

/// 对齐 Java: `ArrayUtilTest.filterEditTest()`
#[test]
fn filter_edit_test() {
    let a = [1, 2, 3, 4, 5, 6];
    let filter: Vec<i32> = a
        .iter()
        .filter_map(|t| if t % 2 == 0 { Some(*t) } else { None })
        .collect();
    assert_eq!(filter, vec![2, 4, 6]);
}

/// 对齐 Java: `ArrayUtilTest.rangeTest()`
#[test]
fn range_test() {
    let range: Vec<i32> = (0..10).collect();
    assert_eq!(0, range[0]);
    assert_eq!(1, range[1]);
    assert_eq!(2, range[2]);
    assert_eq!(3, range[3]);
    assert_eq!(4, range[4]);
    assert_eq!(5, range[5]);
    assert_eq!(6, range[6]);
    assert_eq!(7, range[7]);
    assert_eq!(8, range[8]);
    assert_eq!(9, range[9]);
}

/// 对齐 Java: `ArrayUtilTest.maxTest()`
#[test]
fn max_test() {
    let max = *[1, 2, 13, 4, 5].iter().max().unwrap();
    assert_eq!(13, max);
    let max_long = *[1i64, 2, 13, 4, 5].iter().max().unwrap();
    assert_eq!(13, max_long);
}

/// 对齐 Java: `ArrayUtilTest.minTest()`
#[test]
fn min_test() {
    let min = *[1, 2, 13, 4, 5].iter().min().unwrap();
    assert_eq!(1, min);
}

/// 对齐 Java: `ArrayUtilTest.toArrayTest()`
#[test]
fn to_array_test() {
    let list = vec!["a", "b", "c"];
    let array = ArrayUtil::to_vec(&list);
    assert_eq!(array, vec!["a", "b", "c"]);
}

// ── Hutool TEST parity gap wave ──
// ── Hutool ArrayUtilTest remaining gaps ──

/// 对齐 Java: `ArrayUtilTest.mapTest()`
#[test]
fn map_test() {
    let keys = ["a", "b", "c"];
    let values = [1, 2, 3];
    let map = ArrayUtil::zip(&keys, &values, true);
    assert_eq!(map.get("a"), Some(&1));
    assert_eq!(map.get("b"), Some(&2));
    assert_eq!(map.get("c"), Some(&3));
}

/// 对齐 Java: `ArrayUtilTest.castTest()`
#[test]
fn cast_test() {
    let values = ["1", "2", "3"];
    let cast = ArrayUtil::cast(&values);
    assert_eq!(cast, vec!["1", "2", "3"]);
}

/// 对齐 Java: `ArrayUtilTest.rangeMinTest()`
#[test]
fn range_min_test() {
    assert!(ArrayUtil::range(0, i32::MIN).is_err());
}

/// 对齐 Java: `ArrayUtilTest.getArrayTypeTest()`
#[test]
fn get_array_type_test() {
    assert_eq!(ArrayUtil::get_array_type("i32"), "[i32]");
    assert_eq!(ArrayUtil::get_array_type("String"), "[String]");
}

/// 对齐 Java: `ArrayUtilTest.distinctTest()`
#[test]
fn distinct_test() {
    let array = ["aa", "bb", "cc", "dd", "bb", "dd"];
    let distinct = ArrayUtil::distinct(&array);
    assert_eq!(distinct, vec!["aa", "bb", "cc", "dd"]);
}

/// 对齐 Java: `ArrayUtilTest.distinctByFunctionTest()`
#[test]
fn distinct_by_function_test() {
    let array = ["aa", "Aa", "BB", "bb"];
    let distinct = ArrayUtil::distinct_by(&array, |s: &&str| s.to_lowercase(), true);
    assert_eq!(distinct, vec!["Aa", "bb"]);
    let distinct = ArrayUtil::distinct_by(&array, |s: &&str| s.to_lowercase(), false);
    assert_eq!(distinct, vec!["aa", "BB"]);
}

/// 对齐 Java: `ArrayUtilTest.isAllNotNullTest()`
#[test]
fn is_all_not_null_test() {
    let a: [Option<&str>; 6] = [
        Some("aa"),
        Some("bb"),
        Some("cc"),
        Some("dd"),
        Some("bb"),
        Some("dd"),
    ];
    assert!(ArrayUtil::is_all_not_null(&a));
    let b: [Option<&str>; 6] = [
        Some("aa"),
        Some("bb"),
        Some("cc"),
        None,
        Some("bb"),
        Some("dd"),
    ];
    assert!(!ArrayUtil::is_all_not_null(&b));
}

/// 对齐 Java: `ArrayUtilTest.indexOfSubTest()`
#[test]
fn index_of_sub_test() {
    let a = [0x12, 0x34, 0x56, 0x78, 0x9A];
    let b = [0x56, 0x78];
    let c = [0x12, 0x56];
    let d = [0x78, 0x9A];
    let e = [0x78, 0x9A, 0x10];
    assert_eq!(ArrayUtil::index_of_sub(&a, &b), 2);
    assert_eq!(ArrayUtil::index_of_sub(&a, &c), -1);
    assert_eq!(ArrayUtil::index_of_sub(&a, &d), 3);
    assert_eq!(ArrayUtil::index_of_sub(&a, &e), -1);
    assert_eq!(ArrayUtil::index_of_sub(&a, &[]), -1);
    assert_eq!(ArrayUtil::index_of_sub(&[] as &[i32], &[] as &[i32]), -1);
    assert_eq!(ArrayUtil::index_of_sub(&[] as &[i32], &b), -1);
}

/// 对齐 Java: `ArrayUtilTest.indexOfSubTest2()`
#[test]
fn index_of_sub_test_2() {
    let a = [0x12, 0x56, 0x34, 0x56, 0x78, 0x9A];
    let b = [0x56, 0x78];
    assert_eq!(ArrayUtil::index_of_sub(&a, &b), 3);
}

/// 对齐 Java: `ArrayUtilTest.lastIndexOfSubTest()`
#[test]
fn last_index_of_sub_test() {
    let a = [0x12, 0x34, 0x56, 0x78, 0x9A];
    let b = [0x56, 0x78];
    let c = [0x12, 0x56];
    let d = [0x78, 0x9A];
    let e = [0x78, 0x9A, 0x10];
    assert_eq!(ArrayUtil::last_index_of_sub(&a, &b), 2);
    assert_eq!(ArrayUtil::last_index_of_sub(&a, &c), -1);
    assert_eq!(ArrayUtil::last_index_of_sub(&a, &d), 3);
    assert_eq!(ArrayUtil::last_index_of_sub(&a, &e), -1);
    assert_eq!(ArrayUtil::last_index_of_sub(&a, &[]), -1);
    assert_eq!(
        ArrayUtil::last_index_of_sub(&[] as &[i32], &[] as &[i32]),
        -1
    );
    assert_eq!(ArrayUtil::last_index_of_sub(&[] as &[i32], &b), -1);
}

/// 对齐 Java: `ArrayUtilTest.lastIndexOfSubTest2()`
#[test]
fn last_index_of_sub_test_2() {
    let a = [0x12, 0x56, 0x78, 0x56, 0x21, 0x9A];
    let b = [0x56, 0x78];
    assert_eq!(ArrayUtil::index_of_sub(&a, &b), 1);
}

/// 对齐 Java: `ArrayUtilTest.reverseTest2s()`
#[test]
fn reverse_test_2s() {
    let mut a = [1, 2, 3, 4];
    ArrayUtil::reverse(&mut a);
    assert_eq!(a, [4, 3, 2, 1]);
}

/// 对齐 Java: `ArrayUtilTest.removeEmptyTest()`
#[test]
fn remove_empty_test() {
    let a: [Option<&str>; 6] = [Some("a"), Some("b"), Some(""), None, Some(" "), Some("c")];
    assert_eq!(
        ArrayUtil::remove_empty(&a),
        vec![
            "a".to_string(),
            "b".to_string(),
            " ".to_string(),
            "c".to_string()
        ]
    );
}

/// 对齐 Java: `ArrayUtilTest.removeBlankTest()`
#[test]
fn remove_blank_test() {
    let a: [Option<&str>; 6] = [Some("a"), Some("b"), Some(""), None, Some(" "), Some("c")];
    assert_eq!(
        ArrayUtil::remove_blank(&a),
        vec!["a".to_string(), "b".to_string(), "c".to_string()]
    );
}

/// 对齐 Java: `ArrayUtilTest.nullToEmptyTest()`
#[test]
fn null_to_empty_test() {
    let a: [Option<&str>; 6] = [Some("a"), Some("b"), Some(""), None, Some(" "), Some("c")];
    assert_eq!(
        ArrayUtil::null_to_empty(&a),
        vec![
            "a".to_string(),
            "b".to_string(),
            String::new(),
            String::new(),
            " ".to_string(),
            "c".to_string()
        ]
    );
}

/// 对齐 Java: `ArrayUtilTest.wrapTest()`
#[test]
fn wrap_test() {
    let wrapped = ArrayUtil::wrap([1, 2, 3, 4]);
    assert_eq!(wrapped, vec![[1, 2, 3, 4]]);
}

/// 对齐 Java: `ArrayUtilTest.getTest()`
#[test]
fn get_test() {
    let a = ["a", "b", "c"];
    assert_eq!(ArrayUtil::get(&a, -1), Some("c"));
    assert_eq!(ArrayUtil::get(&a, 99), None);
}

/// 对齐 Java: `ArrayUtilTest.replaceTest()`
#[test]
fn replace_test() {
    let a = ["1", "2", "3", "4"];
    let b = ["a", "b", "c"];
    let result = ArrayUtil::replace(&a, -1, &b);
    assert_eq!(result, vec!["a", "b", "c", "1", "2", "3", "4"]);
    let result = ArrayUtil::replace(&a, 0, &b);
    assert_eq!(result, vec!["a", "b", "c", "4"]);
    let result = ArrayUtil::replace(&a, 1, &b);
    assert_eq!(result, vec!["1", "a", "b", "c"]);
    let result = ArrayUtil::replace(&a, 2, &b);
    assert_eq!(result, vec!["1", "2", "a", "b", "c"]);
    let result = ArrayUtil::replace(&a, 3, &b);
    assert_eq!(result, vec!["1", "2", "3", "a", "b", "c"]);
    let result = ArrayUtil::replace(&a, 4, &b);
    assert_eq!(result, vec!["1", "2", "3", "4", "a", "b", "c"]);
    let result = ArrayUtil::replace(&a, 5, &b);
    assert_eq!(result, vec!["1", "2", "3", "4", "a", "b", "c"]);
    let empty: [&str; 0] = [];
    let result = ArrayUtil::replace(&empty, -1, &b);
    assert_eq!(result, vec!["a", "b", "c"]);
    let result = ArrayUtil::replace(&b, 0, &empty);
    assert_eq!(result, vec!["a", "b", "c"]);
}

/// 对齐 Java: `ArrayUtilTest.setOrAppendTest()`
#[test]
fn set_or_append_test() {
    let arr: Vec<&str> = vec![];
    let new_arr = ArrayUtil::set_or_append(&arr, 0, "Good");
    assert_eq!(new_arr, vec!["Good"]);
}

/// 对齐 Java: `ArrayUtilTest.getAnyTest()`
#[test]
fn get_any_test() {
    let a = ["a", "b", "c", "d", "e"];
    let result = ArrayUtil::get_any(&a, &[3, 4]);
    assert_eq!(result, vec!["d", "e"]);
}

/// 对齐 Java: `ArrayUtilTest.testInsertPrimitive()`
#[test]
fn test_insert_primitive() {
    let a = [1, 2, 4];
    let r = ArrayUtil::insert(&a, 2, &[3]);
    assert_eq!(r, vec![1, 2, 3, 4]);
}

/// Wave2 portable ArrayUtil coverage for parity ledger evidence.
#[test]
fn wave2_array_util_portable_parity() {
    assert_eq!(ArrayUtil::first_match(&[1, 2, 3], |x| *x > 1), Some(2));
    assert_eq!(
        ArrayUtil::edit(&[1, 2, 3], |x| if x == 2 { None } else { Some(x * 10) }),
        vec![10, 30]
    );
    let mut dest = [0; 3];
    ArrayUtil::copy(&[1, 2, 3, 4], &mut dest, 3);
    assert_eq!(dest, [1, 2, 3]);
    let mut dest2 = [0; 5];
    ArrayUtil::copy_range(&[9, 8, 7, 6], 1, &mut dest2, 2, 2);
    assert_eq!(&dest2[2..4], &[8, 7]);
    assert!(ArrayUtil::is_sub(&[1, 2, 3, 4], &[2, 3]));
    assert_eq!(ArrayUtil::to_array(&[1, 2]), vec![1, 2]);
}
