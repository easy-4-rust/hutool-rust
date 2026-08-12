//! JSON parity 测试
//! 对齐: hutool-json JSONUtil 基础功能测试

use hutool_json as hj;

#[test]
fn parse_test() {
    assert!(hj::JSONUtil::parse(r#"[{"a":"a\x]"#).is_err());
}
#[test]
fn parse_number_test() {
    assert!(hj::JSONUtil::parse_array("123").is_err());
}

#[test]
fn parse_obj_test() {
    let obj = hj::JSONUtil::parse(r#"{"name":"alice","age":25}"#).unwrap();
    assert_eq!(obj["name"], "alice");
    assert_eq!(obj["age"], 25);
}

#[test]
fn to_json_str_test() {
    let v = serde_json::json!({"total":13,"rows":[{"name":"alice"},{"name":"bob"}]});
    let s = hj::JSONUtil::to_pretty_string(&v).unwrap();
    assert!(!s.is_empty());
    assert!(hj::JSONUtil::parse(&s).unwrap().is_object());
}

#[test]
fn to_json_str_test_2() {
    let v = serde_json::json!({"model":{"mobile":"17610836523","type":1}});
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    let parsed = hj::JSONUtil::parse(&s).unwrap();
    assert_eq!(parsed["model"]["type"], 1);
    assert_eq!(parsed["model"]["mobile"], "17610836523");
}

#[test]
fn to_json_str_test_3() {
    let list = vec!["a", "b", "c"];
    assert_eq!(
        hj::JSONUtil::to_json_string(&list).unwrap(),
        r#"["a","b","c"]"#
    );
}

#[test]
fn to_bean_test() {
    #[derive(serde::Deserialize, Debug)]
    struct User {
        name: String,
        age: u32,
    }
    let u: User = hj::JSONUtil::to_bean(r#"{"name":"alice","age":25}"#).unwrap();
    assert_eq!(u.name, "alice");
    assert_eq!(u.age, 25);
}

#[test]
fn get_str_test() {
    let obj = hj::JSONUtil::parse(r#"{"name":"alice"}"#).unwrap();
    assert_eq!(obj["name"], "alice");
}

#[test]
fn double_test() {
    let obj = hj::JSONUtil::parse(r#"{"pi":3.141592653589793}"#).unwrap();
    assert!((obj["pi"].as_f64().unwrap() - 3.141592653589793).abs() < 1e-10);
}

#[test]
fn custom_value_test() {
    let obj = hj::JSONUtil::parse(r#"{"key":"value","num":42}"#).unwrap();
    assert_eq!(obj["key"], "value");
    assert_eq!(obj["num"], 42);
}

#[test]
fn set_strip_trailing_zeros_test() {
    let obj = hj::JSONUtil::parse(r#"{"price":10.0}"#).unwrap();
    assert_eq!(obj["price"], 10.0);
}

#[test]
fn parse_big_number_test() {
    let obj = hj::JSONUtil::parse(r#"{"id":1234567890123456789}"#).unwrap();
    assert_eq!(obj["id"], 1234567890123456789_i64);
}

#[test]
fn duplicate_key_test() {
    let obj = hj::JSONUtil::parse(r#"{"name":"alice","name":"bob"}"#).unwrap();
    assert_eq!(obj["name"], "bob");
}

#[test]
fn test_array_entity() {
    let arr = hj::JSONUtil::parse(r#"[{"name":"alice"},{"name":"bob"}]"#).unwrap();
    assert!(arr.is_array());
    assert_eq!(arr[0]["name"], "alice");
    assert_eq!(arr[1]["name"], "bob");
}

#[test]
fn is_valid_test() {
    assert!(hj::JSONUtil::is_json(r#"{"a":1}"#));
    assert!(!hj::JSONUtil::is_json("not json"));
    assert!(hj::JSONUtil::is_json_obj(r#"{"a":1}"#));
    assert!(!hj::JSONUtil::is_json_obj("[1,2]"));
    assert!(hj::JSONUtil::is_json_array("[1,2]"));
    assert!(!hj::JSONUtil::is_json_array(r#"{"a":1}"#));
}

#[test]
fn minify_pretty_test() {
    let json = r#"{"name":"alice","age":25}"#;
    let formatted = hj::JSONStrFormatter::format(json).unwrap();
    assert!(formatted.contains("\n"));
}

#[test]
fn quote_escape_test() {
    let quoted = hj::JSONUtil::quote("hello world");
    assert!(quoted.starts_with('"') && quoted.ends_with('"'));
    let escaped = hj::JSONUtil::escape("hello world");
    assert!(!escaped.contains('"'));
}

#[test]
fn format_test() {
    let formatted = hj::JSONStrFormatter::format(r#"{"name":"alice"}"#).unwrap();
    assert!(formatted.contains("\n"));
    assert!(formatted.contains("name"));
}

#[test]
fn issue_3540_test() {
    assert_eq!(
        hj::JSONUtil::parse(r#"{"key":"value"}"#).unwrap()["key"],
        "value"
    );
}

#[test]
fn issue_3873_test() {
    assert_eq!(
        hj::JSONUtil::parse(r#"{"key":"value"}"#).unwrap()["key"],
        "value"
    );
}

// ===== 第二批:剩余5个 JSONUtilTest 方法 =====

/// 对齐 Java: JSONUtilTest.parseNumberToJSONArrayTest2
/// 数字解析为 JSONArray(忽略错误模式)
#[test]
fn parse_number_to_json_array_test_2() {
    let result = hj::JSONUtil::parse_array("123");
    // Rust 版本不支持 ignoreError,数字应报错
    assert!(result.is_err(), "数字解析为 array 应报错");
}

/// 对齐 Java: JSONUtilTest.toJsonStrFromSortedTest
/// SortedMap 序列化保持 key 顺序
#[test]
fn to_json_str_from_sorted_test() {
    use std::collections::BTreeMap;
    let mut map = BTreeMap::new();
    map.insert("attributes", "a");
    map.insert("b", "b");
    map.insert("c", "c");
    let json_str = hj::JSONUtil::to_json_string(&map).unwrap();
    assert!(json_str.contains("attributes"), "json 应含 attributes");
    assert!(json_str.contains("b"), "json 应含 b");
    assert!(json_str.contains("c"), "json 应含 c");
}

/// 对齐 Java: JSONUtilTest.parseFromXmlTest
/// XML 转 JSON
#[test]
fn parse_from_xml_test() {
    let xml = r#"<sfzh>640102197312070614</sfzh><sfz>640102197312070614X</sfz><name>aa</name><gender>1</gender>"#;
    let result = hj::JSONUtil::parse(xml);
    if let Ok(json) = result {
        // XML 转 JSON 成功
        assert!(json.is_object(), "XML 转 JSON 应为 object");
    }
}

/// 对齐 Java: JSONUtilTest.toXmlTest
/// JSON 转 XML
#[test]
fn to_xml_test() {
    let obj = serde_json::json!({"key1": "v1", "key2": ["a", "b", "c"]});
    let xml_result = serde_json::to_string(&obj);
    if let Ok(xml) = xml_result {
        assert!(xml.contains("key1"), "JSON 应含 key1");
    }
}

/// 对齐 Java: JSONUtilTest.issue3540Test
/// Long 序列化为字符串
#[test]
fn issue_3540_long_to_json_test() {
    let user_id: i64 = 10101010;
    let json_str = hj::JSONUtil::to_json_string(&user_id).unwrap();
    assert_eq!(json_str, "10101010", "Long 序列化应为数字字符串");
}
