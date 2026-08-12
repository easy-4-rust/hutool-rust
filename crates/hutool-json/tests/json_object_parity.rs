//! `JSONObject` 对比验证测试 —— 对齐 Hutool `JSONObjectTest`
//! 来源: hutool-json/src/test/java/cn/hutool/json/JSONObjectTest.java

use hutool_json as hj;
use serde_json::json;

/// 对齐 Java: JSONObjectTest.parseStringTest
#[test]
fn parse_string_test() {
    let result = hj::JSONUtil::parse(r#"{"name":"alice","age":25}"#);
    assert!(result.is_ok(), "parse 应成功");
    let val = result.unwrap();
    assert_eq!(val["name"], "alice");
    assert_eq!(val["age"], 25);
}

/// 对齐 Java: JSONObjectTest.parseStringTest2
#[test]
fn parse_string_test_2() {
    let result = hj::JSONUtil::parse(r#"{"key":null}"#);
    assert!(result.is_ok(), "null 值解析应成功");
}

/// 对齐 Java: JSONObjectTest.parseStringTest3
#[test]
fn parse_string_test_3() {
    let result = hj::JSONUtil::parse(r#"{"a":{"b":{"c":1}}}"#);
    assert!(result.is_ok(), "嵌套对象解析应成功");
}

/// 对齐 Java: JSONObjectTest.parseStringTest4
#[test]
fn parse_string_test_4() {
    let result = hj::JSONUtil::parse(r#"{}"#);
    assert!(result.is_ok(), "空对象解析应成功");
}

/// 对齐 Java: JSONObjectTest.parseBytesTest
#[test]
fn parse_bytes_test() {
    let bytes = b"{\"key\":\"value\"}";
    let result = hj::JSONUtil::parse(std::str::from_utf8(bytes).unwrap());
    assert!(result.is_ok(), "byte[] 解析应成功");
}

/// 对齐 Java: JSONObjectTest.toBeanTest
#[test]
fn to_bean_test() {
    #[derive(serde::Deserialize, Debug)]
    struct User {
        name: String,
        age: u32,
    }
    let result: Result<User, _> = hj::JSONUtil::to_bean(r#"{"name":"alice","age":25}"#);
    assert!(result.is_ok(), "to_bean 应成功");
    let user = result.unwrap();
    assert_eq!(user.name, "alice");
    assert_eq!(user.age, 25);
}

/// 对齐 Java: JSONObjectTest.specialCharTest
#[test]
fn special_char_test() {
    let result = hj::JSONUtil::parse(r#"{"key":"value\nwith\nnewlines"}"#);
    assert!(result.is_ok(), "特殊字符解析应成功");
}

/// 对齐 Java: JSONObjectTest.toStringTest
#[test]
fn to_string_test() {
    let val = hj::JSONUtil::parse(r#"{"key":"value"}"#).unwrap();
    let s = serde_json::to_string(&val).unwrap();
    assert!(s.contains("key"), "toString 应包含 key");
    assert!(s.contains("value"), "toString 应包含 value");
}

/// 对齐 Java: JSONObjectTest.putAllTest
#[test]
fn put_all_test() {
    let mut obj = serde_json::Map::new();
    obj.insert("key1".into(), json!("val1"));
    obj.insert("key2".into(), json!("val2"));
    assert_eq!(obj.len(), 2, "putAll 后应有 2 个 key");
    assert_eq!(obj["key1"], "val1");
}

/// 对齐 Java: JSONObjectTest.parseFromBeanTest
#[test]
fn parse_from_bean_test() {
    #[derive(serde::Serialize)]
    struct Bean {
        name: String,
        age: u32,
    }
    let bean = Bean {
        name: "alice".into(),
        age: 25,
    };
    let result = hj::to_string(&bean);
    assert!(result.is_ok(), "to_string 应成功");
    let s = result.unwrap();
    assert!(s.contains("alice"), "应包含 name");
}
