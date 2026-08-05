//! JSON extended parity tests
//! 对齐: hutool-json JSONUtilTest/JSONObjectTest/JSONArrayTest

use hutool_json::{Value, json};
use hutool_json::{
    from_str, minify, parse, parse_array, parse_object, pretty, to_string, to_string_pretty,
};
use serde::{Deserialize, Serialize};

// ── parse edge cases ──

#[test]
fn parse_nested_object() {
    let input = r#"{"a": {"b": {"c": "deep"}}}"#;
    let value = parse(input).unwrap();
    assert_eq!(value["a"]["b"]["c"], "deep");
}

#[test]
fn parse_array_of_objects() {
    let input = r#"[{"id": 1}, {"id": 2}, {"id": 3}]"#;
    let arr = parse_array(input).unwrap();
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0]["id"], 1);
    assert_eq!(arr[2]["id"], 3);
}

#[test]
fn parse_with_unicode() {
    let input = r#"{"name": "你好世界"}"#;
    let obj = parse_object(input).unwrap();
    assert_eq!(obj["name"].as_str().unwrap(), "你好世界");
}

#[test]
fn parse_with_escaped_quotes() {
    let input = r#"{"msg": "he said \"hello\""}"#;
    let obj = parse_object(input).unwrap();
    assert_eq!(obj["msg"].as_str().unwrap(), "he said \"hello\"");
}

// ── Value operations ──

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
fn value_as_f64() {
    let v = Value::Number(serde_json::Number::from_f64(3.14).unwrap());
    assert_eq!(v.as_f64(), Some(3.14));
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

// ── to_string / to_string_pretty ──

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

// ── from_str ──

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

// ── minify / pretty roundtrip ──

#[test]
fn minify_pretty_roundtrip() {
    let input = r#"{"key": "value", "arr": [1, 2, 3]}"#;
    let minified = minify(input).unwrap();
    let pretty = pretty(&minified).unwrap();
    let minified2 = minify(&pretty).unwrap();
    assert_eq!(minified, minified2);
}

// ── JSON with null values ──

#[test]
fn parse_with_null() {
    let input = r#"{"name": "Alice", "email": null}"#;
    let obj = parse_object(input).unwrap();
    assert_eq!(obj["name"].as_str().unwrap(), "Alice");
    assert!(obj["email"].is_null());
}

// ── Empty structures ──

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

// ── Number types ──

#[test]
fn parse_integer() {
    let value = parse("42").unwrap();
    assert_eq!(value.as_i64(), Some(42));
}

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
fn parse_large_number() {
    let value = parse("9007199254740991").unwrap();
    assert_eq!(value.as_i64(), Some(9007199254740991));
}
