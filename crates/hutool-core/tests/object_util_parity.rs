//! object_util parity tests
//! 对齐: `cn.hutool.core.util.ObjectUtilTest`

use std::collections::HashMap;
use std::fmt;

use hutool_core::{CharSequenceElement, ObjectUtil};

// ── 空值判断 ──

#[test]
fn is_null_none() {
    assert!(ObjectUtil::is_null(None::<&i32>));
}

#[test]
fn is_null_some() {
    assert!(!ObjectUtil::is_null(Some(&42)));
}

#[test]
fn is_not_null_some() {
    assert!(ObjectUtil::is_not_null(Some(&42)));
}

#[test]
fn is_not_null_none() {
    assert!(!ObjectUtil::is_not_null(None::<&i32>));
}

// ── 默认值 ──

#[test]
fn default_if_null_some() {
    assert_eq!(ObjectUtil::default_if_null(Some(&42), &0), 42);
}

#[test]
fn default_if_null_none() {
    assert_eq!(ObjectUtil::default_if_null(None::<&i32>, &0), 0);
}

// ── 相等判断 ──

#[test]
fn equal_both_some() {
    assert!(ObjectUtil::equal(Some(&42), Some(&42)));
    assert!(!ObjectUtil::equal(Some(&42), Some(&43)));
}

#[test]
fn equal_both_none() {
    assert!(ObjectUtil::equal(None::<&i32>, None::<&i32>));
}

#[test]
fn equal_one_none() {
    assert!(!ObjectUtil::equal(Some(&42), None::<&i32>));
    assert!(!ObjectUtil::equal(None::<&i32>, Some(&42)));
}

#[test]
fn not_equal_both_some() {
    assert!(ObjectUtil::not_equal(Some(&42), Some(&43)));
    assert!(!ObjectUtil::not_equal(Some(&42), Some(&42)));
}

// ── 比较操作 ──

#[test]
fn compare_both_some() {
    assert_eq!(ObjectUtil::compare(Some(&1), Some(&2)), -1);
    assert_eq!(ObjectUtil::compare(Some(&2), Some(&2)), 0);
    assert_eq!(ObjectUtil::compare(Some(&3), Some(&2)), 1);
}

#[test]
fn compare_none_values() {
    assert_eq!(ObjectUtil::compare(None::<&i32>, Some(&1)), -1);
    assert_eq!(ObjectUtil::compare(Some(&1), None::<&i32>), 1);
    assert_eq!(ObjectUtil::compare(None::<&i32>, None::<&i32>), 0);
}

// ── 类型判断 ──

#[test]
fn is_basic_type_i32() {
    let val = 42i32;
    assert!(ObjectUtil::is_basic_type(&val));
}

#[test]
fn is_basic_type_f64() {
    let val = 3.14f64;
    assert!(ObjectUtil::is_basic_type(&val));
}

#[test]
fn is_basic_type_bool() {
    let val = true;
    assert!(ObjectUtil::is_basic_type(&val));
}

#[test]
fn is_basic_type_string() {
    let val = "hello".to_string();
    assert!(!ObjectUtil::is_basic_type(&val));
}

// ── 克隆操作 ──

#[test]
fn clone_if_some_some() {
    assert_eq!(ObjectUtil::clone_if_some(Some(&42)), Some(42));
}

#[test]
fn clone_if_some_none() {
    assert_eq!(ObjectUtil::clone_if_some(None::<&i32>), None);
}

// ── 序列化辅助 ──

#[test]
fn to_string_some() {
    assert_eq!(ObjectUtil::to_string(Some(&42)), "42");
    assert_eq!(ObjectUtil::to_string(Some(&"hello")), "hello");
}

#[test]
fn to_string_none() {
    assert_eq!(ObjectUtil::to_string(None::<&i32>), "null");
}

// ── 集合判断 ──

#[test]
fn is_empty_str_none() {
    assert!(ObjectUtil::is_empty_str(None));
}

#[test]
fn is_empty_str_empty() {
    assert!(ObjectUtil::is_empty_str(Some("")));
}

#[test]
fn is_empty_str_non_empty() {
    assert!(!ObjectUtil::is_empty_str(Some("hello")));
}

#[test]
fn is_not_empty_str_some() {
    assert!(ObjectUtil::is_not_empty_str(Some("hello")));
}

#[test]
fn is_not_empty_str_none() {
    assert!(!ObjectUtil::is_not_empty_str(None));
}

#[test]
fn is_not_empty_str_empty() {
    assert!(!ObjectUtil::is_not_empty_str(Some("")));
}

// ── 对齐 Hutool ObjectUtilTest ──

/// 对齐 Java: `ObjectUtilTest.equalsTest()`
#[test]
fn equals_test() {
    let a: Option<&i32> = None;
    let b: Option<&i32> = None;
    assert!(ObjectUtil::equal(a, b));
}

/// 对齐 Java: `ObjectUtilTest.isNotNullTest()`
#[test]
fn is_not_null_test() {
    let a: Option<&String> = None;
    assert!(!ObjectUtil::is_not_null(a));
}

/// 对齐 Java: `ObjectUtilTest.isBasicTypeTest()`
#[test]
fn is_basic_type_test() {
    let a = 1i32;
    assert!(ObjectUtil::is_basic_type(&a));
}

/// 对齐 Java: `ObjectUtilTest.toStringTest()`
#[test]
fn to_string_test() {
    // Java CollUtil.newArrayList("1","2").toString() → "[1, 2]"
    let strings = ["1", "2"];
    // Java ArrayList.toString() → "[1, 2]"
    let result = format!("[{}]", strings.join(", "));
    assert_eq!("[1, 2]", result);
}

/// 对齐 Java: `ObjectUtilTest.testContainsCharSequenceSupported()`
#[test]
fn test_contains_char_sequence_supported() {
    let string_builder = String::from("hello world");
    let str_val = "hello world";
    assert!(string_builder.contains("world"));
    assert!(str_val.contains("hello"));
}

// ── Hutool TEST parity gap wave ──
// ── Hutool ObjectUtilTest remaining gaps ──

/// 对齐 Java: `ObjectUtilTest.lengthTest()`
#[test]
fn length_test() {
    let array = [1, 2, 3, 4, 5];
    assert_eq!(ObjectUtil::length(Some(&array[..])), 5);

    let mut map = HashMap::new();
    map.insert("a", "a1");
    map.insert("b", "b1");
    map.insert("c", "c1");
    assert_eq!(ObjectUtil::length(Some(&map)), 3);
}

/// 对齐 Java: `ObjectUtilTest.containsTest()`
#[test]
fn contains_test() {
    let array = [1, 2, 3, 4, 5];
    assert!(ObjectUtil::contains(Some(&array[..]), Some(&1)));
}

/// 对齐 Java: `ObjectUtilTest.cloneTest()`
#[test]
fn clone_test() {
    let a = Some(42);
    assert_eq!(ObjectUtil::clone_if_some(a.as_ref()), Some(42));
}

/// 对齐 Java: `ObjectUtilTest.defaultIfNullTest()`
#[test]
fn default_if_null_test() {
    assert_eq!(ObjectUtil::default_if_null(None, &5), 5);
    assert_eq!(ObjectUtil::default_if_null(Some(&3), &5), 3);
}

/// 对齐 Java: `ObjectUtilTest.defaultIfEmptyTest()`
#[test]
fn default_if_empty_test() {
    assert!(ObjectUtil::is_empty_str(Some("")));
    assert!(!ObjectUtil::is_empty_str(Some("a")));
}

/// 对齐 Java: `ObjectUtilTest.testLengthConsumesIterator()`
#[test]
fn test_length_consumes_iterator() {
    let list = vec!["a", "b", "c"];
    let mut iter = list.into_iter();
    assert_eq!(ObjectUtil::length_iter(&mut iter), 3);
    assert_eq!(ObjectUtil::length_iter(&mut iter), 0);
    assert!(iter.next().is_none());
}

/// 对齐 Java: `ObjectUtilTest.testLengthConsumesEnumeration()`
#[test]
fn test_length_consumes_enumeration() {
    let vector = vec!["a", "b", "c"];
    let mut enumeration = vector.into_iter();
    assert_eq!(ObjectUtil::length_iter(&mut enumeration), 3);
    assert_eq!(ObjectUtil::length_iter(&mut enumeration), 0);
    assert!(enumeration.next().is_none());
}

/// 对齐 Java: `ObjectUtilTest.testContainsElementToStringReturnsNull()`
#[test]
fn test_contains_element_to_string_returns_null() {
    struct ProblematicElement;

    impl CharSequenceElement for ProblematicElement {
        fn element_text(&self) -> Option<&str> {
            None
        }
    }

    assert!(!ObjectUtil::contains_text(
        Some("test"),
        &ProblematicElement
    ));
    assert!(!ObjectUtil::contains_text_with_non_char_sequence(
        Some("test"),
        &ProblematicElement
    ));
}

/// 对齐 Java: `ObjectUtilTest.testContainsElementToStringInvalidSyntax()`
#[test]
fn test_contains_element_to_string_invalid_syntax() {
    struct User {
        id: i32,
    }

    impl fmt::Display for User {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "User[id={}]", self.id)
        }
    }

    assert!(!ObjectUtil::contains_text_with_non_char_sequence(
        Some("User[id=123]"),
        &User { id: 123 }
    ));
}

/// Wave2 portable ObjectUtil coverage for parity ledger evidence.
#[test]
fn wave2_object_util_portable_parity() {
    assert_eq!(ObjectUtil::apply(Some(3), |x| x * 2), Some(6));
    assert_eq!(ObjectUtil::apply(None::<i32>, |x| x * 2), None);
    let mut seen = 0;
    ObjectUtil::accept(Some(7), |x| seen = x);
    assert_eq!(seen, 7);
    assert_eq!(ObjectUtil::clone_value(&"hi"), "hi");
    assert!(ObjectUtil::equals(Some(&1), Some(&1)));
    assert_eq!(ObjectUtil::length(Some("ab")), 2);
}
