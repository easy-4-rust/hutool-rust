//! JSON util parity tests
//! 对齐: hutool-json JSONUtilTest

use hutool_json::{
    is_json_array, is_json_object, is_valid, minify, parse, parse_array, parse_object, pretty,
};

// ── is_valid ──

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

// ── is_json_object ──

#[test]
fn is_json_object_valid() {
    assert!(is_json_object(r#"{"key": "value"}"#));
}

#[test]
fn is_json_object_array() {
    assert!(!is_json_object(r#"[1, 2, 3]"#));
}

// ── is_json_array ──

#[test]
fn is_json_array_valid() {
    assert!(is_json_array(r#"[1, 2, 3]"#));
}

#[test]
fn is_json_array_object() {
    assert!(!is_json_array(r#"{"key": "value"}"#));
}

// ── minify ──

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

// ── pretty ──

#[test]
fn pretty_minified_json() {
    let input = r#"{"key":"value","number":42}"#;
    let result = pretty(input).unwrap();
    assert!(result.contains('\n'));
    assert!(result.contains("  "));
}

// ── parse ──

#[test]
fn parse_object_value() {
    let value = parse(r#"{"key": "value"}"#).unwrap();
    assert!(value.is_object());
}

#[test]
fn parse_array_value() {
    let value = parse(r#"[1, 2, 3]"#).unwrap();
    assert!(value.is_array());
}

// ── parse_object ──

#[test]
fn parse_object_basic() {
    let obj = parse_object(r#"{"key": "value", "num": 42}"#).unwrap();
    assert_eq!(obj.get("key").unwrap().as_str().unwrap(), "value");
    assert_eq!(obj.get("num").unwrap().as_i64().unwrap(), 42);
}

// ── parse_array ──

#[test]
fn parse_array_basic() {
    let arr = parse_array(r#"[1, "two", true]"#).unwrap();
    assert_eq!(arr.len(), 3);
    assert_eq!(arr[0].as_i64().unwrap(), 1);
    assert_eq!(arr[1].as_str().unwrap(), "two");
    assert!(arr[2].as_bool().unwrap());
}
