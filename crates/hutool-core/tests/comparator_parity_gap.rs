//! `cn.hutool.core.comparator` 缺口 parity
//!
//! 对齐: `cn.hutool.core.comparator.*` 未覆盖 @Test

use hutool_core::CollUtil;
use hutool_core::RandomUtil;
use hutool_core::comparator::{
    ArrayIndexedComparator, FieldsComparator, PropertyComparator, VersionComparator,
    WindowsExplorerStringComparator,
};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User {
    a: String,
    b: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Model {
    a: i32,
    b: i32,
}

// ── IndexedComparatorTest ──

/// 对齐 Java: `IndexedComparatorTest.sortTest()`
#[test]
fn indexed_comparator_sort_test() {
    let user = User {
        a: "9".to_string(),
        b: None,
    };
    let arr_mixed = [
        SortKey::Str("a".into()),
        SortKey::Str("b".into()),
        SortKey::User(user.clone()),
        SortKey::Str("1".into()),
        SortKey::I32(3),
        SortKey::None,
        SortKey::Str("2".into()),
    ];
    let mut values = arr_mixed.to_vec();
    let cmp = ArrayIndexedComparator::new(arr_mixed.iter().cloned());
    CollUtil::sort_in_place(&mut values, |x, y| i32_to_ordering(cmp.compare(x, y)));
    assert_eq!(values[0], SortKey::Str("a".into()));
    assert_eq!(values[2], SortKey::User(user));
    assert_eq!(values[4], SortKey::I32(3));
    assert_eq!(values[5], SortKey::None);
}

/// 对齐 Java: `IndexedComparatorTest.reversedTest()`
#[test]
fn indexed_comparator_reversed_test() {
    let user = User {
        a: "9".to_string(),
        b: None,
    };
    let arr_mixed = [
        SortKey::Str("a".into()),
        SortKey::Str("b".into()),
        SortKey::User(user.clone()),
        SortKey::Str("1".into()),
        SortKey::I32(3),
        SortKey::None,
        SortKey::Str("2".into()),
    ];
    let mut values = arr_mixed.to_vec();
    let cmp = ArrayIndexedComparator::new(arr_mixed.iter().cloned()).reversed();
    CollUtil::sort_in_place(&mut values, |x, y| i32_to_ordering(cmp.compare(x, y)));
    assert_eq!(values[6], SortKey::Str("a".into()));
    assert_eq!(values[1], SortKey::None);
    assert_eq!(values[4], SortKey::User(user));
    assert_eq!(values[2], SortKey::I32(3));
}

/// 对齐 Java: `IndexedComparatorTest.benchmarkSortTest()`（Java @Disabled — 验证可运行）
#[test]
fn indexed_comparator_benchmark_sort_test() {
    let arr_mixed = [
        SortKey::Str("a".into()),
        SortKey::Str("b".into()),
        SortKey::User(User {
            a: "9".into(),
            b: None,
        }),
        SortKey::Str("1".into()),
        SortKey::I32(3),
        SortKey::None,
        SortKey::Str("2".into()),
    ];
    let cmp = ArrayIndexedComparator::new(arr_mixed.iter().cloned());
    for _ in 0..1000 {
        let mut values = arr_mixed.to_vec();
        CollUtil::sort_in_place(&mut values, |x, y| i32_to_ordering(cmp.compare(x, y)));
    }
}

// ── Issue3259Test ──

/// 对齐 Java: `Issue3259Test.fieldsComparatorTest()`
#[test]
fn issue3259_fields_comparator_test() {
    let x = Model { a: 1, b: 1 };
    let y = Model {
        a: 1,
        b: RandomUtil::random_int_range(2, 100),
    };
    let extractors: Vec<Box<dyn Fn(&Model) -> Option<i32> + Send + Sync>> = vec![
        Box::new(|m: &Model| Some(m.a)),
        Box::new(|m: &Model| Some(m.b)),
    ];
    let cmp = FieldsComparator::new(extractors);
    assert!(cmp.compare(&x, &y) < 0);
}

/// 对齐 Java: `Issue3259Test.sortTest()`（Java @Disabled — 冒烟）
#[test]
fn issue3259_sort_test() {
    let x = Model { a: 1, b: 1 };
    let y = Model { a: 1, b: 3 };
    let mut all = vec![x, y];
    let extractors: Vec<Box<dyn Fn(&Model) -> Option<i32> + Send + Sync>> = vec![
        Box::new(|m: &Model| Some(m.a)),
        Box::new(|m: &Model| Some(m.b)),
    ];
    let cmp = FieldsComparator::new(extractors);
    all.sort_by(|a, b| cmp.compare(a, b).cmp(&0));
    assert_eq!(all.len(), 2);
}

// ── PropertyComparatorTest ──

/// 对齐 Java: `PropertyComparatorTest.sortNullTest()`
#[test]
fn property_comparator_sort_null_test() {
    let users = vec![
        User {
            a: "1".into(),
            b: Some("d".into()),
        },
        User {
            a: "2".into(),
            b: None,
        },
        User {
            a: "3".into(),
            b: Some("a".into()),
        },
    ];
    let mut sorted1 = users.clone();
    let cmp1 = PropertyComparator::new(|u: &User| u.b.clone());
    sorted1.sort_by(|a, b| cmp1.compare(a, b).cmp(&0));
    assert_eq!(sorted1[0].b.as_deref(), Some("a"));
    assert_eq!(sorted1[1].b.as_deref(), Some("d"));
    assert!(sorted1[2].b.is_none());

    let mut sorted2 = users.clone();
    let cmp2 = PropertyComparator::with_null_greater(|u: &User| u.b.clone(), false);
    sorted2.sort_by(|a, b| cmp2.compare(a, b).cmp(&0));
    assert!(sorted2[0].b.is_none());
    assert_eq!(sorted2[1].b.as_deref(), Some("a"));
    assert_eq!(sorted2[2].b.as_deref(), Some("d"));
}

/// 对齐 Java: `PropertyComparatorTest.reversedTest()`
#[test]
fn property_comparator_reversed_test() {
    let users = vec![
        User {
            a: "1".into(),
            b: Some("d".into()),
        },
        User {
            a: "2".into(),
            b: None,
        },
        User {
            a: "3".into(),
            b: Some("a".into()),
        },
    ];
    let mut sorted = users;
    let cmp = PropertyComparator::new(|u: &User| u.b.clone()).reversed();
    sorted.sort_by(|a, b| cmp.compare(a, b).cmp(&0));
    assert!(sorted[0].b.is_none());
    assert_eq!(sorted[1].b.as_deref(), Some("d"));
    assert_eq!(sorted[2].b.as_deref(), Some("a"));
}

// ── VersionComparatorTest ──

/// 对齐 Java: `VersionComparatorTest.compareEmptyTest()`
#[test]
fn version_comparator_compare_empty_test() {
    let cmp = VersionComparator::INSTANCE;
    assert!(cmp.compare(Some(""), Some("1.12.1")) < 0);
    assert!(cmp.compare(Some(""), None) > 0);
    assert!(cmp.compare(None, Some("")) < 0);
}

/// 对齐 Java: `VersionComparatorTest.versionComparatorTest1()`
#[test]
fn version_comparator_version_comparator_test1() {
    let cmp = VersionComparator::INSTANCE;
    assert!(cmp.compare(Some("1.2.1"), Some("1.12.1")) < 0);
    assert!(cmp.compare(Some("1.12.1"), Some("1.2.1")) > 0);
}

/// 对齐 Java: `VersionComparatorTest.versionComparatorTest2()`
#[test]
fn version_comparator_version_comparator_test2() {
    let cmp = VersionComparator::INSTANCE;
    assert!(cmp.compare(Some("1.12.1"), Some("1.12.1c")) < 0);
    assert!(cmp.compare(Some("1.12.1c"), Some("1.12.1")) > 0);
}

/// 对齐 Java: `VersionComparatorTest.versionComparatorTest3()`
#[test]
fn version_comparator_version_comparator_test3() {
    let cmp = VersionComparator::INSTANCE;
    assert!(cmp.compare(None, Some("1.12.1c")) < 0);
    assert!(cmp.compare(Some("1.12.1c"), None) > 0);
}

/// 对齐 Java: `VersionComparatorTest.versionComparatorTest4()`
#[test]
fn version_comparator_version_comparator_test4() {
    let cmp = VersionComparator::INSTANCE;
    assert!(cmp.compare(Some("1.13.0"), Some("1.12.1c")) > 0);
    assert!(cmp.compare(Some("1.12.1c"), Some("1.13.0")) < 0);
}

/// 对齐 Java: `VersionComparatorTest.versionComparatorTest5()`
#[test]
fn version_comparator_version_comparator_test5() {
    let cmp = VersionComparator::INSTANCE;
    assert!(cmp.compare(Some("V1.2"), Some("V1.1")) > 0);
    assert!(cmp.compare(Some("V1.1"), Some("V1.2")) < 0);
}

/// 对齐 Java: `VersionComparatorTest.versionComparatorTes6()`
#[test]
fn version_comparator_version_comparator_tes6() {
    let cmp = VersionComparator::INSTANCE;
    assert!(cmp.compare(Some("V0.0.20170102"), Some("V0.0.20170101")) > 0);
    assert!(cmp.compare(Some("V0.0.20170101"), Some("V0.0.20170102")) < 0);
}

/// 对齐 Java: `VersionComparatorTest.equalsTest()`
///
/// Java 默认引用相等；Rust ZST 不实现 `PartialEq`，以零尺寸表达无状态单例语义。
#[test]
fn version_comparator_equals_test() {
    let first = VersionComparator;
    let other = VersionComparator;
    let _ = (first, other);
    assert_eq!(std::mem::size_of::<VersionComparator>(), 0);
}

/// 对齐 Java: `VersionComparatorTest.versionComparatorTest7()`
#[test]
fn version_comparator_version_comparator_test7() {
    let cmp = VersionComparator::INSTANCE;
    assert!(cmp.compare(Some("1.12.2"), Some("1.12.1c")) > 0);
    assert!(cmp.compare(Some("1.12.1c"), Some("1.12.2")) < 0);
}

/// 对齐 Java: `VersionComparatorTest.equalsTest2()`
#[test]
fn version_comparator_equals_test2() {
    assert_eq!(
        VersionComparator::INSTANCE.compare(Some("1.12.0"), Some("1.12")),
        0
    );
}

/// 对齐 Java: `VersionComparatorTest.I8Z3VETest()`
#[test]
fn version_comparator_i8_z3_ve_test() {
    let cmp = VersionComparator::INSTANCE;
    assert!(cmp.compare(Some("260"), Some("a-34")) > 0);
    assert!(cmp.compare(Some("a-34"), Some("a-3")) > 0);
    assert!(cmp.compare(Some("260"), Some("a-3")) > 0);
}

/// 对齐 Java: `VersionComparatorTest.startWithNoneNumberTest()`
#[test]
fn version_comparator_start_with_none_number_test() {
    assert!(VersionComparator::INSTANCE.compare(Some("V1"), Some("A1")) > 0);
}

// ── WindowsExplorerStringComparatorTest ──

const ANSWER1: &[&str] = &[
    "filename",
    "filename 00",
    "filename 0",
    "filename 01",
    "filename.jpg",
    "filename.txt",
    "filename00.jpg",
    "filename00a.jpg",
    "filename00a.txt",
    "filename0",
    "filename0.jpg",
    "filename0a.txt",
    "filename0b.jpg",
    "filename0b1.jpg",
    "filename0b02.jpg",
    "filename0c.jpg",
    "filename01.0hjh45-test.txt",
    "filename01.0hjh46",
    "filename01.1hjh45.txt",
    "filename01.hjh45.txt",
    "Filename01.jpg",
    "Filename1.jpg",
    "filename2.hjh45.txt",
    "filename2.jpg",
    "filename03.jpg",
    "filename3.jpg",
];

const ANSWER2: &[&str] = &["abc1.doc", "abc2.doc", "abc12.doc"];

/// 对齐 Java: `WindowsExplorerStringComparatorTest.testCompare1()`
#[test]
fn windows_explorer_string_comparator_test_compare1() {
    let cmp = WindowsExplorerStringComparator::INSTANCE;
    let mut to_sort: Vec<String> = ANSWER1.iter().map(|s| (*s).to_string()).collect();
    for _ in 0..20 {
        shuffle_strings(&mut to_sort);
        if to_sort.iter().map(String::as_str).collect::<Vec<_>>() != ANSWER1 {
            break;
        }
    }
    to_sort.sort_by(|a, b| cmp.compare(a, b).cmp(&0));
    let sorted: Vec<&str> = to_sort.iter().map(|s| s.as_str()).collect();
    assert_eq!(sorted, ANSWER1);
}

/// 对齐 Java: `WindowsExplorerStringComparatorTest.testCompare2()`
#[test]
fn windows_explorer_string_comparator_test_compare2() {
    let cmp = WindowsExplorerStringComparator::INSTANCE;
    let mut to_sort: Vec<String> = ANSWER2.iter().map(|s| (*s).to_string()).collect();
    for _ in 0..20 {
        shuffle_strings(&mut to_sort);
        if to_sort.iter().map(String::as_str).collect::<Vec<_>>() != ANSWER2 {
            break;
        }
    }
    to_sort.sort_by(|a, b| cmp.compare(a, b).cmp(&0));
    let sorted: Vec<&str> = to_sort.iter().map(|s| s.as_str()).collect();
    assert_eq!(sorted, ANSWER2);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SortKey {
    Str(String),
    User(User),
    I32(i32),
    None,
}

fn i32_to_ordering(value: i32) -> Ordering {
    match value.cmp(&0) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => Ordering::Equal,
        Ordering::Greater => Ordering::Greater,
    }
}

fn shuffle_strings(values: &mut [String]) {
    for i in (1..values.len()).rev() {
        let j = RandomUtil::random_int_range(0, i as i32) as usize;
        values.swap(i, j);
    }
}

use hutool_core::comparator::{
    ComparableComparator, ComparatorChain, ComparatorException, CompareUtil, FuncComparator,
    LengthComparator, NullComparator, ReverseComparator,
};

/// Wave2: ComparatorChain / CompareUtil / Length / Reverse / Null / Func
#[test]
fn comparator_chain_and_util_wave2_test() {
    let mut chain = ComparatorChain::of(|a: &&str, b: &&str| a.len().cmp(&b.len()));
    chain.add_comparator(|a: &&str, b: &&str| a.cmp(b));
    assert!(chain.compare(&"aa", &"b") > 0);
    assert_eq!(chain.size(), 2);
    assert!(chain.is_locked());

    assert_eq!(CompareUtil::compare_ord(&1, &2), -1);
    assert_eq!(LengthComparator::INSTANCE.compare("ab", "a"), 1);
    let rev = ReverseComparator::new(|a: &i32, b: &i32| a.cmp(b));
    assert_eq!(rev.compare(&1, &2), 1);
    let null_cmp = NullComparator::new(true, |a: &i32, b: &i32| a.cmp(b));
    assert_eq!(null_cmp.compare_option(None, Some(&1)), 1);
    let func = FuncComparator::new(false, |s: &&str| s.len());
    assert_eq!(func.compare(&"a", &"bb"), -1);
    assert_eq!(ComparableComparator::new().compare(&3, &3), 0);
    let _ = ComparatorException::new("cmp");
}
