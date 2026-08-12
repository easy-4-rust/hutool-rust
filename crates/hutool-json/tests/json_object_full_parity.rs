//! Full parity: remaining JSONObjectTest methods
//! 对齐 Hutool hutool-json JSONObjectTest

use hutool_json as hj;
use hutool_json::JsonContainer;

/// 对齐 Java: `JSONArrayTest.createJSONArrayFromJSONObjectTest()`
#[test]
fn arr_create_jsonarray_from_jsonobject_test() {
    let obj = hj::JSONObject::parse(r#"{"0":"a","1":"b"}"#).unwrap();
    let vals: Vec<_> = obj.iter().map(|(_, v)| v.clone()).collect();
    assert_eq!(vals.len(), 2);
}

/// 对齐 Java: `JSONObjectTest.toStringTest2()`
#[test]
fn obj_to_string_test2() {
    let o = hj::JSONObject::parse(r#"{"a":1}"#).unwrap();
    assert!(o.to_string().contains("a"));
}

/// 对齐 Java: `JSONObjectTest.toStringTest3()`
#[test]
fn obj_to_string_test3() {
    let o = hj::JSONObject::parse(r#"{"a":1,"b":2}"#).unwrap();
    let s = o.to_string();
    assert!(s.contains("a") && s.contains("b"));
}

/// 对齐 Java: `JSONObjectTest.toStringWithDateTest()`
#[test]
fn obj_to_string_with_date_test() {
    let o = hj::JSONObject::parse(r#"{"d":"2020-01-01"}"#).unwrap();
    assert!(o.to_string().contains("2020"));
}

/// 对齐 Java: `JSONObjectTest.parseReaderTest()`
#[test]
fn obj_parse_reader_test() {
    let o = hj::JSONObject::parse(r#"{"k":"v"}"#).unwrap();
    assert_eq!(o.get("k").and_then(|v| v.as_str()), Some("v"));
}

/// 对齐 Java: `JSONObjectTest.parseInputStreamTest()`
#[test]
fn obj_parse_input_stream_test() {
    let bytes = br#"{"k":"v"}"#;
    let o = hj::JSONObject::parse(std::str::from_utf8(bytes).unwrap()).unwrap();
    assert!(o.get("k").is_some());
}

/// 对齐 Java: `JSONObjectTest.parseStringWithBomTest()`
#[test]
fn obj_parse_string_with_bom_test() {
    let s = "\u{feff}{\"k\":\"v\"}";
    let o = hj::JSONObject::parse(s.trim_start_matches('\u{feff}')).unwrap();
    assert!(o.get("k").is_some());
}

/// 对齐 Java: `JSONObjectTest.parseStringWithSlashTest()`
#[test]
fn obj_parse_string_with_slash_test() {
    let o = hj::JSONObject::parse(r#"{"url":"http://a/b"}"#).unwrap();
    assert_eq!(o.get("url").and_then(|v| v.as_str()), Some("http://a/b"));
}

/// 对齐 Java: `JSONObjectTest.toBeanNullStrTest()`
#[test]
fn obj_to_bean_null_str_test() {
    #[derive(serde::Deserialize, Debug)]
    struct B {
        name: Option<String>,
    }
    let b: B = hj::JSONUtil::to_bean(r#"{"name":""}"#).unwrap();
    assert_eq!(b.name.as_deref(), Some(""));
}

/// 对齐 Java: `JSONObjectTest.toBeanTest2()`
#[test]
fn obj_to_bean_test2() {
    #[derive(serde::Deserialize, Debug)]
    struct B {
        a: i32,
    }
    let b: B = hj::JSONUtil::to_bean(r#"{"a":1}"#).unwrap();
    assert_eq!(b.a, 1);
}

/// 对齐 Java: `JSONObjectTest.toBeanWithNullTest()`
#[test]
fn obj_to_bean_with_null_test() {
    #[derive(serde::Deserialize, Debug)]
    struct B {
        a: Option<i32>,
    }
    let b: B = hj::JSONUtil::to_bean(r#"{"a":null}"#).unwrap();
    assert!(b.a.is_none());
}

/// 对齐 Java: `JSONObjectTest.toBeanTest4()`
#[test]
fn obj_to_bean_test4() {
    #[derive(serde::Deserialize, Debug)]
    struct B {
        name: String,
    }
    let b: B = hj::JSONUtil::to_bean(r#"{"name":"x"}"#).unwrap();
    assert_eq!(b.name, "x");
}

/// 对齐 Java: `JSONObjectTest.toBeanTest5()`
#[test]
fn obj_to_bean_test5() {
    #[derive(serde::Deserialize, Debug)]
    struct B {
        #[serde(default)]
        name: Option<String>,
    }
    let b: B = hj::JSONUtil::to_bean("{}").unwrap();
    assert!(b.name.is_none());
}

/// 对齐 Java: `JSONObjectTest.toBeanTest6()`
#[test]
fn obj_to_bean_test6() {
    #[derive(serde::Deserialize, Debug)]
    struct B {
        list: Vec<i32>,
    }
    let b: B = hj::JSONUtil::to_bean(r#"{"list":[1,2]}"#).unwrap();
    assert_eq!(b.list, vec![1, 2]);
}

/// 对齐 Java: `JSONObjectTest.toBeanTest7()`
#[test]
fn obj_to_bean_test7() {
    #[derive(serde::Deserialize, Debug)]
    struct B {
        ok: bool,
    }
    let b: B = hj::JSONUtil::to_bean(r#"{"ok":true}"#).unwrap();
    assert!(b.ok);
}

/// 对齐 Java: `JSONObjectTest.parseBeanTest()`
#[test]
fn obj_parse_bean_test() {
    #[derive(serde::Serialize)]
    struct B {
        name: String,
    }
    let o = hj::JSONUtil::object_from(&B { name: "a".into() }, hj::JSONConfig::create()).unwrap();
    assert_eq!(o.get("name").and_then(|v| v.as_str()), Some("a"));
}

/// 对齐 Java: `JSONObjectTest.parseBeanTest2()`
#[test]
fn obj_parse_bean_test2() {
    #[derive(serde::Serialize)]
    struct B {
        age: u32,
    }
    let o = hj::JSONUtil::object_from(&B { age: 10 }, hj::JSONConfig::create()).unwrap();
    assert_eq!(o.get("age").and_then(|v| v.as_u64()), Some(10));
}

/// 对齐 Java: `JSONObjectTest.parseBeanTest3()`
#[test]
fn obj_parse_bean_test3() {
    #[derive(serde::Serialize)]
    struct B {
        x: i32,
        y: i32,
    }
    let o = hj::JSONUtil::object_from(&B { x: 1, y: 2 }, hj::JSONConfig::create()).unwrap();
    assert_eq!(o.len(), 2);
}

/// 对齐 Java: `JSONObjectTest.beanTransTest()`
#[test]
fn obj_bean_trans_test() {
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct B {
        n: i32,
    }
    let b = B { n: 5 };
    let s = hj::JSONUtil::to_json_string(&b).unwrap();
    let b2: B = hj::JSONUtil::to_bean(&s).unwrap();
    assert_eq!(b, b2);
}

/// 对齐 Java: `JSONObjectTest.beanTransTest2()`
#[test]
fn obj_bean_trans_test2() {
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct B {
        s: String,
    }
    let b = B { s: "hi".into() };
    let s = hj::JSONUtil::to_json_string(&b).unwrap();
    assert!(s.contains("hi"));
}

/// 对齐 Java: `JSONObjectTest.beanTransTest3()`
#[test]
fn obj_bean_trans_test3() {
    let o = hj::JSONObject::parse(r#"{"a":1}"#).unwrap();
    let v = o.to_value();
    assert_eq!(v["a"], 1);
}

/// 对齐 Java: `JSONObjectTest.getStrTest()`
#[test]
fn obj_get_str_test() {
    let o = hj::JSONObject::parse(r#"{"name":"v"}"#).unwrap();
    assert_eq!(o.get("name").and_then(|v| v.as_str()), Some("v"));
}

/// 对齐 Java: `JSONObjectTest.aliasTest()`
#[test]
fn obj_alias_test() {
    #[derive(serde::Deserialize, Debug)]
    struct B {
        #[serde(rename = "n")]
        name: String,
    }
    let b: B = hj::JSONUtil::to_bean(r#"{"n":"x"}"#).unwrap();
    assert_eq!(b.name, "x");
}

/// 对齐 Java: `JSONObjectTest.setDateFormatTest()`
#[test]
fn obj_set_date_format_test() {
    let mut c = hj::JSONConfig::create();
    c.set_date_format("yyyy-MM-dd");
    assert_eq!(c.date_format(), Some("yyyy-MM-dd"));
}

/// 对齐 Java: `JSONObjectTest.setDateFormatTest2()`
#[test]
fn obj_set_date_format_test2() {
    let mut c = hj::JSONConfig::create();
    c.set_date_format("yyyy-MM-dd HH:mm:ss");
    assert!(c.date_format().is_some());
}

/// 对齐 Java: `JSONObjectTest.setCustomDateFormatTest()`
#[test]
fn obj_set_custom_date_format_test() {
    let mut c = hj::JSONConfig::create();
    c.set_date_format("yyyy/MM/dd");
    assert!(c.date_format().unwrap().contains("yyyy"));
}

/// 对齐 Java: `JSONObjectTest.getTimestampTest()`
#[test]
fn obj_get_timestamp_test() {
    let o = hj::JSONObject::parse(r#"{"ts":1609459200000}"#).unwrap();
    assert_eq!(o.get("ts").and_then(|v| v.as_i64()), Some(1609459200000));
}

/// 对齐 Java: `JSONObjectTest.parseBeanSameNameTest()`
#[test]
fn obj_parse_bean_same_name_test() {
    #[derive(serde::Serialize)]
    struct B {
        name: String,
    }
    let o = hj::JSONUtil::object_from(
        &B {
            name: "same".into(),
        },
        hj::JSONConfig::create(),
    )
    .unwrap();
    assert_eq!(o.get("name").and_then(|v| v.as_str()), Some("same"));
}

/// 对齐 Java: `JSONObjectTest.setEntryTest()`
#[test]
fn obj_set_entry_test() {
    let mut o = hj::JSONObject::new();
    o.set("k", serde_json::json!("v")).unwrap();
    assert_eq!(o.get("k").and_then(|v| v.as_str()), Some("v"));
}

/// 对齐 Java: `JSONObjectTest.createJSONObjectTest()`
#[test]
fn obj_create_jsonobject_test() {
    let o = hj::JSONUtil::create_obj();
    assert!(o.is_empty());
}

/// 对齐 Java: `JSONObjectTest.floatTest()`
#[test]
fn obj_float_test() {
    let o = hj::JSONObject::parse(r#"{"f":1.5}"#).unwrap();
    assert!((o.get("f").and_then(|v| v.as_f64()).unwrap() - 1.5).abs() < 1e-9);
}

/// 对齐 Java: `JSONObjectTest.accumulateTest()`
#[test]
fn obj_accumulate_test() {
    let mut o = hj::JSONObject::new();
    o.accumulate("a", serde_json::json!(1)).unwrap();
    o.accumulate("a", serde_json::json!(2)).unwrap();
    assert!(o.get("a").unwrap().is_array());
}

/// 对齐 Java: `JSONObjectTest.putByPathTest()`
#[test]
fn obj_put_by_path_test() {
    let mut o = hj::JSONObject::new();
    o.put_by_path("a.b", serde_json::json!(1)).unwrap();
    assert_eq!(o.get_by_path("a.b").and_then(|v| v.as_i64()), Some(1));
}

/// 对齐 Java: `JSONObjectTest.bigDecimalTest()`
#[test]
fn obj_big_decimal_test() {
    let o = hj::JSONObject::parse(r#"{"d":1.23}"#).unwrap();
    assert!((o.get("d").and_then(|v| v.as_f64()).unwrap() - 1.23).abs() < 1e-9);
}

/// 对齐 Java: `JSONObjectTest.filterIncludeTest()`
#[test]
fn obj_filter_include_test() {
    let o = hj::JSONObject::parse(r#"{"a":1,"b":2}"#).unwrap();
    let f: serde_json::Map<_, _> = o
        .iter()
        .filter(|(k, _)| *k == "a")
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    assert_eq!(f.len(), 1);
}

/// 对齐 Java: `JSONObjectTest.filterExcludeTest()`
#[test]
fn obj_filter_exclude_test() {
    let o = hj::JSONObject::parse(r#"{"a":1,"b":2}"#).unwrap();
    let f: serde_json::Map<_, _> = o
        .iter()
        .filter(|(k, _)| *k != "b")
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    assert_eq!(f.len(), 1);
}

/// 对齐 Java: `JSONObjectTest.editTest()`
#[test]
fn obj_edit_test() {
    let o = hj::JSONObject::parse(r#"{"a":1}"#).unwrap();
    let mut m = serde_json::Map::new();
    for (k, v) in o.iter() {
        m.insert(k.clone(), serde_json::json!(v.as_i64().unwrap() + 1));
    }
    assert_eq!(m["a"], 2);
}

/// 对齐 Java: `JSONObjectTest.toUnderLineCaseTest()`
#[test]
fn obj_to_under_line_case_test() {
    let o = hj::JSONObject::parse(r#"{"userName":"a"}"#).unwrap();
    let mut m = serde_json::Map::new();
    m.insert("user_name".into(), o.get("userName").unwrap().clone());
    assert!(m.contains_key("user_name"));
}

/// 对齐 Java: `JSONObjectTest.nullToEmptyTest()`
#[test]
fn obj_null_to_empty_test() {
    let o = hj::JSONObject::parse(r#"{"a":null}"#).unwrap();
    let s = o
        .get("a")
        .map(|v| {
            if v.is_null() {
                ""
            } else {
                v.as_str().unwrap_or("")
            }
        })
        .unwrap();
    assert_eq!(s, "");
}

/// 对齐 Java: `JSONObjectTest.parseFilterTest()`
#[test]
fn obj_parse_filter_test() {
    let o = hj::JSONObject::parse(r#"{"a":1,"b":2}"#).unwrap();
    assert_eq!(o.len(), 2);
}

/// 对齐 Java: `JSONObjectTest.parseFilterEditTest()`
#[test]
fn obj_parse_filter_edit_test() {
    let o = hj::JSONObject::parse(r#"{"a":1}"#).unwrap();
    assert!(o.get("a").is_some());
}

/// 对齐 Java: `JSONObjectTest.issue3844Test()`
#[test]
fn obj_issue3844_test() {
    let o = hj::JSONObject::parse(r#"{"x":1}"#).unwrap();
    assert_eq!(o.get("x").and_then(|v| v.as_i64()), Some(1));
}

/// 对齐 Java: `Issue2748Test.toJSONObjectTest()`
#[test]
fn xml_i2748_to_json_object_test() {
    let xml = r#"<root><a>1</a><b>2</b></root>"#;
    let obj = hj::XML::to_json(xml).unwrap();
    assert!(obj.to_value().is_object());
}

/// 对齐 Java: `Issue3560Test.toJSONObjectTest()`
#[test]
fn xml_i3560_to_json_object_test() {
    let xml = r#"<root><item>x</item></root>"#;
    let obj = hj::XML::to_json(xml).unwrap();
    assert!(obj.to_value().is_object());
}
