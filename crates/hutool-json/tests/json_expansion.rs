//! JSON 扩展 parity 测试
//! 对齐: hutool-json 多个测试类

use hutool_json::{parse, parse_object, parse_array, is_valid, minify, pretty, to_string, to_string_pretty, from_str};
use hutool_json::{json};

// ── parse 基础测试 (10 tests) ──

#[test]
fn parse_object_basic() {
    let value = parse(r#"{"key": "value"}"#).unwrap();
    assert!(value.is_object());
}

#[test]
fn parse_array_basic() {
    let value = parse(r#"[1, 2, 3]"#).unwrap();
    assert!(value.is_array());
}

#[test]
fn parse_nested_object() {
    let value = parse(r#"{"a": {"b": {"c": "deep"}}}"#).unwrap();
    assert_eq!(value["a"]["b"]["c"], "deep");
}

#[test]
fn parse_array_of_objects() {
    let arr = parse_array(r#"[{"id": 1}, {"id": 2}, {"id": 3}]"#).unwrap();
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0]["id"], 1);
}

#[test]
fn parse_with_unicode() {
    let obj = parse_object(r#"{"name": "你好世界"}"#).unwrap();
    assert_eq!(obj["name"].as_str().unwrap(), "你好世界");
}

#[test]
fn parse_with_escaped_quotes() {
    let obj = parse_object(r#"{"msg": "he said \"hello\""}"#).unwrap();
    assert_eq!(obj["msg"].as_str().unwrap(), "he said \"hello\"");
}

#[test]
fn parse_with_null() {
    let obj = parse_object(r#"{"name": "Alice", "email": null}"#).unwrap();
    assert_eq!(obj["name"].as_str().unwrap(), "Alice");
    assert!(obj["email"].is_null());
}

#[test]
fn parse_empty_object() {
    let obj = parse_object("{}").unwrap();
    assert!(obj.is_empty());
}

#[test]
fn parse_empty_array() {
    let arr = parse_array("[]").unwrap();
    assert!(arr.is_empty());
}

#[test]
fn parse_integer() {
    let value = parse("42").unwrap();
    assert_eq!(value.as_i64(), Some(42));
}

// ── is_valid 测试 (5 tests) ──

#[test]
fn is_valid_object() {
    assert!(is_valid(r#"{"key": "value"}"#));
}

#[test]
fn is_valid_array() {
    assert!(is_valid(r#"[1, 2, 3]"#));
}

#[test]
fn is_valid_invalid() {
    assert!(!is_valid("not json"));
}

#[test]
fn is_valid_empty_object() {
    assert!(is_valid("{}"));
}

#[test]
fn is_valid_empty_array() {
    assert!(is_valid("[]"));
}

// ── minify/pretty 测试 (5 tests) ──

#[test]
fn minify_pretty_json() {
    let input = r#"{
    "key": "value",
    "number": 42
}"#;
    let result = minify(input).unwrap();
    assert!(!result.contains('\n'));
    assert!(result.contains("\"key\":\"value\""));
}

#[test]
fn pretty_minified_json() {
    let input = r#"{"key":"value","number":42}"#;
    let result = pretty(input).unwrap();
    assert!(result.contains('\n'));
    assert!(result.contains("  "));
}

#[test]
fn minify_pretty_roundtrip() {
    let input = r#"{"key": "value", "arr": [1, 2, 3]}"#;
    let minified = minify(input).unwrap();
    let pretty = pretty(&minified).unwrap();
    let minified2 = minify(&pretty).unwrap();
    assert_eq!(minified, minified2);
}

#[test]
fn minify_empty_object() {
    let result = minify("{}").unwrap();
    assert_eq!(result, "{}");
}

#[test]
fn pretty_empty_object() {
    let result = pretty("{}").unwrap();
    assert_eq!(result, "{}");
}

// ── to_string 测试 (5 tests) ──

#[test]
fn to_string_basic() {
    let obj = json!({"key": "value", "num": 42});
    let s = to_string(&obj).unwrap();
    assert!(s.contains("\"key\":\"value\""));
    assert!(s.contains("\"num\":42"));
}

#[test]
fn to_string_pretty_basic() {
    let obj = json!({"key": "value"});
    let s = to_string_pretty(&obj).unwrap();
    assert!(s.contains('\n'));
    assert!(s.contains("  "));
}

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

// ── from_str 测试 (3 tests) ──

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

use serde::{Serialize, Deserialize};
