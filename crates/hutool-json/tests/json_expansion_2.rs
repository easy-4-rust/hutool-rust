//! JSON 扩展 parity 测试 2
//! 对齐: hutool-json 多个测试类

use hutool_json::{Value, json};
use hutool_json::{
    from_str, is_valid, parse, parse_array, parse_object, to_string, to_string_pretty,
};

// ── parse 扩展测试 (10 tests) ──

#[test]
fn parse_float() {
    let value = parse("3.14").unwrap();
    assert_eq!(value.as_f64(), Some(3.14));
}

#[test]
fn parse_negative() {
    let value = parse("-1").unwrap();
    assert_eq!(value.as_i64(), Some(-1));
}

#[test]
fn parse_boolean_true() {
    let value = parse("true").unwrap();
    assert_eq!(value.as_bool(), Some(true));
}

#[test]
fn parse_boolean_false() {
    let value = parse("false").unwrap();
    assert_eq!(value.as_bool(), Some(false));
}

#[test]
fn parse_null() {
    let value = parse("null").unwrap();
    assert!(value.is_null());
}

#[test]
fn parse_string() {
    let value = parse("\"hello\"").unwrap();
    assert_eq!(value.as_str(), Some("hello"));
}

#[test]
fn parse_mixed_array() {
    let arr = parse_array(r#"[1, "two", true, null]"#).unwrap();
    assert_eq!(arr.len(), 4);
    assert_eq!(arr[0].as_i64(), Some(1));
    assert_eq!(arr[1].as_str(), Some("two"));
    assert_eq!(arr[2].as_bool(), Some(true));
    assert!(arr[3].is_null());
}

#[test]
fn parse_nested_array() {
    let arr = parse_array(r#"[[1, 2], [3, 4]]"#).unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0][0], 1);
    assert_eq!(arr[1][1], 4);
}

#[test]
fn parse_special_chars() {
    let obj = parse_object(r#"{"key": "value\nwith\nnewlines"}"#).unwrap();
    assert!(obj["key"].as_str().unwrap().contains('\n'));
}

#[test]
fn parse_large_number() {
    let value = parse("9007199254740991").unwrap();
    assert_eq!(value.as_i64(), Some(9007199254740991));
}

// ── is_valid 扩展测试 (5 tests) ──

#[test]
fn is_valid_nested() {
    assert!(is_valid(r#"{"a": {"b": [1, 2, 3]}}"#));
}

#[test]
fn is_valid_string_only() {
    assert!(is_valid("\"hello\""));
}

#[test]
fn is_valid_number_only() {
    assert!(is_valid("42"));
}

#[test]
fn is_valid_boolean_only() {
    assert!(is_valid("true"));
}

#[test]
fn is_valid_null_only() {
    assert!(is_valid("null"));
}

// ── Value 操作测试 (5 tests) ──

#[test]
fn value_as_str() {
    let v = Value::String("hello".to_string());
    assert_eq!(v.as_str(), Some("hello"));
}

#[test]
fn value_as_i64() {
    let v = Value::Number(42.into());
    assert_eq!(v.as_i64(), Some(42));
}

#[test]
fn value_as_bool() {
    assert_eq!(Value::Bool(true).as_bool(), Some(true));
    assert_eq!(Value::Bool(false).as_bool(), Some(false));
}

#[test]
fn value_is_null() {
    assert!(Value::Null.is_null());
    assert!(!Value::Bool(true).is_null());
}

#[test]
fn value_array_access() {
    let arr = json!([1, 2, 3]);
    assert_eq!(arr[0], 1);
    assert_eq!(arr[2], 3);
}

// ── 序列化测试 (5 tests) ──

#[test]
fn to_string_array() {
    let arr = json!([1, 2, 3]);
    let s = to_string(&arr).unwrap();
    assert_eq!(s, "[1,2,3]");
}

#[test]
fn to_string_nested() {
    let obj = json!({"a": {"b": "c"}});
    let s = to_string(&obj).unwrap();
    assert!(s.contains("\"a\""));
    assert!(s.contains("\"b\":\"c\""));
}

#[test]
fn to_string_null() {
    let obj = json!({"key": null});
    let s = to_string(&obj).unwrap();
    assert!(s.contains("null"));
}

#[test]
fn to_string_pretty_nested() {
    let obj = json!({"a": {"b": "c"}});
    let s = to_string_pretty(&obj).unwrap();
    assert!(s.contains('\n'));
}

#[test]
fn to_string_special_chars() {
    let obj = json!({"key": "value\nwith\nnewlines"});
    let s = to_string(&obj).unwrap();
    assert!(s.contains("\\n"));
}

// ── 反序列化测试 (3 tests) ──

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct SimpleStruct {
    name: String,
    age: i32,
}

#[test]
fn from_str_basic() {
    let input = r#"{"name": "Alice", "age": 30}"#;
    let s: SimpleStruct = from_str(input).unwrap();
    assert_eq!(s.name, "Alice");
    assert_eq!(s.age, 30);
}

#[test]
fn from_str_array() {
    let input = r#"[1, 2, 3]"#;
    let arr: Vec<i32> = from_str(input).unwrap();
    assert_eq!(arr, vec![1, 2, 3]);
}

#[test]
fn from_str_nested() {
    let input = r#"{"items": [1, 2, 3]}"#;
    let obj: std::collections::HashMap<String, Vec<i32>> = from_str(input).unwrap();
    assert_eq!(obj.get("items").unwrap(), &vec![1, 2, 3]);
}

use serde::{Deserialize, Serialize};
