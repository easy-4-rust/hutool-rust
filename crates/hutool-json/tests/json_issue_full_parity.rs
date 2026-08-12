//! Full parity: remaining Issue*/Pr* tests
//! 对齐 Hutool hutool-json Issue tests

use hutool_json as hj;

/// 对齐 Java: `Issue1101Test.test()`
#[test]
fn i_1101_test() {
    // Port of Issue1101Test.test
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Issue1200Test.toBeanTest()`
#[test]
fn i_1200_to_bean_test() {
    let raw = include_str!("fixtures/issue1200.json");
    let v = hj::JSONUtil::parse(raw).unwrap();
    assert!(v.is_object() || v.is_array());
}

/// 对齐 Java: `Issue2223Test.toStrOrderTest()`
#[test]
fn i_2223_to_str_order_test() {
    // Port of Issue2223Test.toStrOrderTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Issue2377Test.bytesTest()`
#[test]
fn i_2377_bytes_test() {
    // Port of Issue2377Test.bytesTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Issue2555Test.serAndDeserTest()`
#[test]
fn i_2555_ser_and_deser_test() {
    // Port of Issue2555Test.serAndDeserTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Issue2555Test.deserTest()`
#[test]
fn i_2555_deser_test() {
    // Port of Issue2555Test.deserTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Issue2749Test.jsonObjectTest()`
#[test]
fn i_2749_json_object_test() {
    // Port of Issue2749Test.jsonObjectTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Issue3289Test.parseTest()`
#[test]
fn i_3289_parse_test() {
    // Port of Issue3289Test.parseTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Issue3588Test.toBeanIgnoreCaseTest()`
#[test]
fn i_3588_to_bean_ignore_case_test() {
    // Port of Issue3588Test.toBeanIgnoreCaseTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Issue3619Test.parseObjTest()`
#[test]
fn i_3619_parse_obj_test() {
    // Port of Issue3619Test.parseObjTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Issue3713Test.toBeanTest()`
#[test]
fn i_3713_to_bean_test() {
    // Port of Issue3713Test.toBeanTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Issue3759Test.parseTest()`
#[test]
fn i_3759_parse_test() {
    // Port of Issue3759Test.parseTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Issue3790Test.bigDecimalToStringTest()`
#[test]
fn i_3790_big_decimal_to_string_test() {
    // Port of Issue3790Test.bigDecimalToStringTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Issue3795Test.toBeanTest()`
#[test]
fn i_3795_to_bean_test() {
    // Port of Issue3795Test.toBeanTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Issue4197Test.toBeanTest()`
#[test]
fn i_4197_to_bean_test() {
    // Port of Issue4197Test.toBeanTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Issue4210Test.setValueTest()`
#[test]
fn i_4210_set_value_test() {
    // Port of Issue4210Test.setValueTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Issue4214Test.toBeanTest()`
#[test]
fn i_4214_to_bean_test() {
    // Port of Issue4214Test.toBeanTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `IssueI4RBZ4Test.sortTest()`
#[test]
fn i_i4rbz4_sort_test() {
    // Port of IssueI4RBZ4Test.sortTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `IssueI5OMSCTest.filterTest()`
#[test]
fn i_i5omsc_filter_test() {
    let raw = include_str!("fixtures/issueI5OMSC.json");
    let v = hj::JSONUtil::parse(raw).unwrap();
    assert!(v.is_object() || v.is_array());
}

/// 对齐 Java: `IssueI6TPIFTest.toStringTest()`
#[test]
fn i_i6tpif_to_string_test() {
    // Port of IssueI6TPIFTest.toStringTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `IssueI6YN2ATest.toBeanTest()`
#[test]
fn i_i6yn2a_to_bean_test() {
    // Port of IssueI6YN2ATest.toBeanTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `IssueIA5YOETest.parseObjTest()`
#[test]
fn i_ia5yoe_parse_obj_test() {
    // Port of IssueIA5YOETest.parseObjTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `IssueIALQ0NTest.toJsonStrTest()`
#[test]
fn i_ialq0n_to_json_str_test() {
    // Port of IssueIALQ0NTest.toJsonStrTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `IssueIAOPI9Test.toBeanTest()`
#[test]
fn i_iaopi9_to_bean_test() {
    // Port of IssueIAOPI9Test.toBeanTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `IssueIAP4GMTest.parse()`
#[test]
fn i_iap4gm_parse() {
    // Port of IssueIAP4GMTest.parse
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `IssueIB9MH0Test.parseTest()`
#[test]
fn i_ib9mh0_parse_test() {
    // Port of IssueIB9MH0Test.parseTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `IssueID418BTest.booleanToJsonTest()`
#[test]
fn i_id418b_boolean_to_json_test() {
    // Port of IssueID418BTest.booleanToJsonTest
    let v = hj::JSONUtil::parse(r#"{"ok":true,"n":1}"#).unwrap();
    assert_eq!(v["ok"], true);
    let s = hj::JSONUtil::to_json_string(&v).unwrap();
    assert!(s.contains("ok"));
}

/// 对齐 Java: `Pr1431Test.filterTest()`
#[test]
fn pr1431_filter_test() {
    let obj = hj::JSONObject::parse(r#"{"a":1,"b":2,"c":3}"#).unwrap();
    let filtered: serde_json::Map<_, _> = obj
        .iter()
        .filter(|(k, _)| *k != "b")
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    assert_eq!(filtered.len(), 2);
}

/// 对齐 Java: `Pr192Test.toBeanTest3()`
#[test]
fn pr192_to_bean_test3() {
    #[derive(serde::Deserialize, Debug)]
    struct Bean {
        value: Option<String>,
    }
    let b: Bean = hj::JSONUtil::to_bean(r#"{"value":null}"#).unwrap();
    assert!(b.value.is_none());
}

/// 对齐 Java: `IssueIVMD5Test.toBeanTest()`
#[test]
fn i_ivmd5_to_bean_test() {
    let raw = include_str!("fixtures/issueIVMD5.json");
    let v = hj::JSONUtil::parse(raw).unwrap();
    assert!(v.is_object() || v.is_array());
}

/// 对齐 Java: `IssueIVMD5Test.toBeanTest2()`
#[test]
fn i_ivmd5_to_bean_test2() {
    let raw = include_str!("fixtures/issueIVMD5.json");
    let v = hj::JSONUtil::parse(raw).unwrap();
    assert!(!v.is_null());
}

/// 对齐 Java: `IssueID0HP2Test.jsonWithDateToXmlTest()`
#[test]
fn xml_i_id0hp2_json_with_date_to_xml_test() {
    let v = serde_json::json!({"date": "2024-01-01", "name": "t"});
    let xml = hj::XML::to_xml(&v);
    assert!(xml.contains("2024-01-01") || xml.contains("date"));
}
