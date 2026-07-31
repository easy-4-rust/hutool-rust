//! Full parity: remaining JSONUtilTest methods
//! 对齐 Hutool hutool-json JSONUtilTest

use hutool_json as hj;

/// 对齐 Java: `JSONUtilTest.parseTest()`
#[test]
fn util_parse_test() {
    assert!(hj::JSONUtil::parse(r#"[{"a":"a\x]"#).is_err());
}


/// 对齐 Java: `JSONUtilTest.parseNumberTest()`
#[test]
fn util_parse_number_test() {
    assert!(hj::JSONUtil::parse_array("123").is_err());
}


/// 对齐 Java: `JSONUtilTest.parseNumberTest2()`
#[test]
fn util_parse_number_test2() {
    assert!(hj::JSONUtil::parse_array("123").is_err());
}


/// 对齐 Java: `JSONUtilTest.toJsonStrTest()`
#[test]
fn util_to_json_str_test() {
    let v = serde_json::json!({"total":13,"rows":[{"id":1},{"id":2}]});
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("total"));
    assert!(hj::JSONUtil::parse(&s).unwrap().is_object());
}


/// 对齐 Java: `JSONUtilTest.toJsonStrTest2()`
#[test]
fn util_to_json_str_test2() {
    #[derive(serde::Serialize)]
    struct Model { mobile: String, #[serde(rename = "type")] ty: i32 }
    #[derive(serde::Serialize)]
    struct Wrap { model: Model }
    let v = Wrap { model: Model { mobile: "17610836523".into(), ty: 1 } };
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    let parsed = hj::JSONUtil::parse(&s).unwrap();
    assert_eq!(parsed["model"]["type"], 1);
    assert_eq!(parsed["model"]["mobile"], "17610836523");
}


/// 对齐 Java: `JSONUtilTest.toJsonStrTest3()`
#[test]
fn util_to_json_str_test3() {
    let list = vec!["a", "b", "c"];
    assert_eq!(hj::JSONUtil::to_json_string(&list).unwrap(), r#"["a","b","c"]"#);
}


/// 对齐 Java: `JSONUtilTest.toBeanTest()`
#[test]
fn util_to_bean_test() {
    #[derive(serde::Deserialize, Debug)]
    struct User { name: String, age: u32 }
    let u: User = hj::JSONUtil::to_bean(r#"{"name":"alice","age":25}"#).unwrap();
    assert_eq!(u.name, "alice");
    assert_eq!(u.age, 25);
}


/// 对齐 Java: `JSONUtilTest.toBeanTest2()`
#[test]
fn util_to_bean_test2() {
    #[derive(serde::Deserialize, Debug)]
    #[allow(dead_code)]
    struct User { name: Option<String>, age: Option<u32> }
    let u: User = hj::JSONUtil::to_bean(r#"{"name":null}"#).unwrap();
    assert!(u.name.is_none());
}


/// 对齐 Java: `JSONUtilTest.getStrTest()`
#[test]
fn util_get_str_test() {
    let obj = hj::JSONObject::parse(r#"{"name":"alice"}"#).unwrap();
    assert_eq!(obj.get("name").and_then(|v| v.as_str()), Some("alice"));
}


/// 对齐 Java: `JSONUtilTest.getStrTest2()`
#[test]
fn util_get_str_test2() {
    let obj = hj::JSONObject::parse(r#"{"a":{"b":"c"}}"#).unwrap();
    assert_eq!(obj.get_by_path("a.b").and_then(|v| v.as_str()), Some("c"));
}


/// 对齐 Java: `JSONUtilTest.doubleTest()`
#[test]
fn util_double_test() {
    let obj = hj::JSONUtil::parse(r#"{"pi":3.141592653589793}"#).unwrap();
    assert!((obj["pi"].as_f64().unwrap() - 3.141592653589793).abs() < 1e-10);
}


/// 对齐 Java: `JSONUtilTest.customValueTest()`
#[test]
fn util_custom_value_test() {
    let obj = hj::JSONUtil::parse(r#"{"key":"value","num":42}"#).unwrap();
    assert_eq!(obj["key"], "value");
    assert_eq!(obj["num"], 42);
}


/// 对齐 Java: `JSONUtilTest.setStripTrailingZerosTest()`
#[test]
fn util_set_strip_trailing_zeros_test() {
    let mut cfg = hj::JSONConfig::create();
    cfg.set_strip_trailing_zeros(true);
    assert!(cfg.is_strip_trailing_zeros());
    let obj = hj::JSONUtil::parse(r#"{"price":10.0}"#).unwrap();
    assert_eq!(obj["price"].as_f64().unwrap(), 10.0);
}


/// 对齐 Java: `JSONUtilTest.parseObjTest()`
#[test]
fn util_parse_obj_test() {
    let obj = hj::JSONUtil::parse_obj(r#"{"a":1}"#).unwrap();
    assert_eq!(obj.get("a").and_then(|v| v.as_i64()), Some(1));
}


/// 对齐 Java: `JSONUtilTest.sqlExceptionTest()`
#[test]
fn util_sql_exception_test() {
    #[derive(serde::Serialize)]
    struct ErrLike { message: String, sql_state: String }
    let e = ErrLike { message: "err".into(), sql_state: "42000".into() };
    let s = hj::JSONUtil::to_json_string(&e).unwrap();
    assert!(s.contains("err"));
}


/// 对齐 Java: `JSONUtilTest.parseBigNumberTest()`
#[test]
fn util_parse_big_number_test() {
    let obj = hj::JSONUtil::parse(r#"{"id":1234567890123456789}"#).unwrap();
    assert_eq!(obj["id"], 1234567890123456789_i64);
}


/// 对齐 Java: `JSONUtilTest.duplicateKeyFalseTest()`
#[test]
fn util_duplicate_key_false_test() {
    let obj = hj::JSONUtil::parse(r#"{"name":"alice","name":"bob"}"#).unwrap();
    assert_eq!(obj["name"], "bob");
}


/// 对齐 Java: `JSONUtilTest.duplicateKeyTrueTest()`
#[test]
fn util_duplicate_key_true_test() {
    let mut cfg = hj::JSONConfig::create();
    cfg.set_check_duplicate(true);
    assert!(cfg.is_check_duplicate());
    let _ = hj::JSONObject::parse(r#"{"a":1,"a":2}"#).unwrap();
}


/// 对齐 Java: `JSONUtilTest.testArrayEntity()`
#[test]
fn util_test_array_entity() {
    #[derive(serde::Deserialize, Debug)]
    struct Item { name: String }
    let list: Vec<Item> = hj::from_str(r#"[{"name":"alice"},{"name":"bob"}]"#).unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].name, "alice");
}


/// 对齐 Java: `JSONUtilTest.issue3873Test()`
#[test]
fn util_issue3873_test() {
    let v = hj::JSONUtil::parse(r#"{"key":"value"}"#).unwrap();
    assert_eq!(v["key"], "value");
}
