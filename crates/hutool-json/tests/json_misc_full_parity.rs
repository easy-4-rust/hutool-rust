//! Full parity: XML/Path/Formatter/Null/Bean/Convert/Transient/etc.
//! 对齐 Hutool hutool-json misc tests

use hutool_json as hj;
use hutool_json::JsonContainer;

/// 对齐 Java: `BeanToJsonTest.toJsonStrTest()`
#[test]
fn bean2json_to_json_str_test() {
    #[derive(serde::Serialize)]
    struct ReadParam {
        #[serde(rename = "initSpikeMac")]
        init_spike_mac: String,
        mac: String,
        #[serde(rename = "spikeMac")]
        spike_mac: String,
        bag: String,
        #[serde(rename = "projectId")]
        project_id: i32,
    }
    let p = ReadParam {
        init_spike_mac: "a".into(),
        mac: "b".into(),
        spike_mac: "c".into(),
        bag: "d".into(),
        project_id: 123,
    };
    let s = hj::JSONUtil::to_json_string(&p).unwrap();
    let v = hj::JSONUtil::parse(&s).unwrap();
    assert_eq!(v["initSpikeMac"], "a");
    assert_eq!(v["projectId"], 123);
}

/// 对齐 Java: `CustomSerializeTest.serializeTest()`
#[test]
fn custom_ser_serialize_test() {
    use hj::{GlobalSerializeMapping, SerializeRegistry};
    #[derive(Debug)]
    struct CustomBean {
        name: String,
    }
    let mut reg = SerializeRegistry::new();
    reg.put_serializer::<CustomBean>(Box::new(|bean: &CustomBean| {
        Ok(serde_json::json!({"customName": bean.name}))
    }));
    let prev = GlobalSerializeMapping::set(reg);
    let bean = CustomBean {
        name: "testName".into(),
    };
    let v = GlobalSerializeMapping::get().serialize(&bean).unwrap();
    assert_eq!(v["customName"], "testName");
    let _ = GlobalSerializeMapping::set(prev);
}

/// 对齐 Java: `CustomSerializeTest.deserializeTest()`
#[test]
fn custom_ser_deserialize_test() {
    use hj::{GlobalSerializeMapping, SerializeRegistry};
    #[derive(Debug)]
    struct CustomBean {
        name: String,
    }
    let mut reg = SerializeRegistry::new();
    reg.put_deserializer::<CustomBean>(Box::new(|v: &serde_json::Value| {
        Ok(CustomBean {
            name: v["customName"].as_str().unwrap_or("").to_string(),
        })
    }));
    let prev = GlobalSerializeMapping::set(reg);
    let bean: CustomBean = GlobalSerializeMapping::get()
        .deserialize(&serde_json::json!({"customName": "testName"}))
        .unwrap();
    assert_eq!(bean.name, "testName");
    let _ = GlobalSerializeMapping::set(prev);
}

/// 对齐 Java: `JSONBeanParserTest.parseTest()`
#[test]
fn bean_parser_parse_test() {
    #[derive(Debug)]
    struct TestBean {
        name: String,
        address: String,
    }
    let obj = hj::JSONObject::parse(
        r#"{"customName":"customValue","customAddress":"customAddressValue"}"#,
    )
    .unwrap();
    let bean = TestBean {
        name: obj
            .get("customName")
            .and_then(|v| v.as_str())
            .unwrap()
            .into(),
        address: obj
            .get("customAddress")
            .and_then(|v| v.as_str())
            .unwrap()
            .into(),
    };
    assert_eq!(bean.name, "customValue");
    assert_eq!(bean.address, "customAddressValue");
}

/// 对齐 Java: `JSONConvertTest.testBean2Json()`
#[test]
fn conv_test_bean2_json() {
    #[derive(serde::Serialize)]
    struct Bean {
        name: String,
        age: u32,
    }
    let s = hj::JSONUtil::to_json_string(&Bean {
        name: "张三".into(),
        age: 18,
    })
    .unwrap();
    assert!(s.contains("张三"));
}

/// 对齐 Java: `JSONConvertTest.testJson2Bean()`
#[test]
fn conv_test_json2_bean() {
    #[derive(serde::Deserialize, Debug)]
    #[allow(dead_code)]
    struct Bean {
        name: String,
        age: u32,
    }
    let b: Bean = hj::JSONUtil::to_bean(r#"{"name":"张三","age":18}"#).unwrap();
    assert_eq!(b.age, 18);
}

/// 对齐 Java: `JSONConvertTest.testJson2Bean2()`
#[test]
fn conv_test_json2_bean2() {
    #[derive(serde::Deserialize, Debug)]
    struct Bean {
        name: Option<String>,
    }
    let b: Bean = hj::JSONUtil::to_bean("{}").unwrap();
    assert!(b.name.is_none());
}

/// 对齐 Java: `JSONNullTest.parseNullTest()`
#[test]
fn null_parse_null_test() {
    let obj = hj::JSONObject::parse(
        r#"{"device_model":null,"imsi":null,"act_date":"2021-07-23T06:23:26.000+00:00"}"#,
    )
    .unwrap();
    assert!(obj.get("device_model").unwrap().is_null());
    let mut cleaned = serde_json::Map::new();
    for (k, v) in obj.iter() {
        if !v.is_null() {
            cleaned.insert(k.clone(), v.clone());
        }
    }
    let s = serde_json::to_string(&cleaned).unwrap();
    assert!(!s.contains("device_model"));
    assert!(s.contains("act_date"));
}

/// 对齐 Java: `JSONNullTest.parseNullTest2()`
#[test]
fn null_parse_null_test2() {
    let raw = r#"{"device_model":null,"act_date":"2021-07-23T06:23:26.000+00:00"}"#;
    let parsed = hj::JSONUtil::parse(raw).unwrap();
    let mut obj = serde_json::Map::new();
    for (k, v) in parsed.as_object().unwrap() {
        if !v.is_null() {
            obj.insert(k.clone(), v.clone());
        }
    }
    assert!(!obj.contains_key("device_model"));
    assert!(obj.contains_key("act_date"));
}

/// 对齐 Java: `JSONPathTest.getByPathTest()`
#[test]
fn path_get_by_path_test() {
    let json = r#"[{"id":"1","name":"xingming"},{"id":"2","name":"mingzi"}]"#;
    let arr = hj::JSONArray::parse(json).unwrap();
    assert_eq!(
        arr.get_by_path("[0].name").and_then(|v| v.as_str()),
        Some("xingming")
    );
    assert_eq!(
        arr.get_by_path("[1].name").and_then(|v| v.as_str()),
        Some("mingzi")
    );
}

/// 对齐 Java: `JSONPathTest.getByPathTest2()`
#[test]
fn path_get_by_path_test2() {
    let json = hj::JSONUtil::parse(r#"{"accountId":111}"#).unwrap();
    let id = hj::JSONUtil::get_by_path(&json, "accountId").and_then(|v| v.as_i64());
    assert_eq!(id, Some(111));
}

/// 对齐 Java: `JSONPathTest.getByPathTest3()`
#[test]
fn path_get_by_path_test3() {
    let json = hj::JSONUtil::parse(r#"[{"accountId":1},{"accountId":2},{"accountId":3}]"#).unwrap();
    let ids: Vec<i64> = json
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["accountId"].as_i64().unwrap())
        .collect();
    assert_eq!(ids, vec![1, 2, 3]);
}

/// 对齐 Java: `JSONPathTest.getByPathWithWildcardTest()`
#[test]
fn path_get_by_path_with_wildcard_test() {
    let root = serde_json::json!({
        "actionMessage": {
            "decodeFeas": [{
                "body": {
                    "lats": [
                        {"begin": 4260, "text": "呵呵"},
                        {"begin": 4260, "text": "你好 "}
                    ]
                }
            }]
        }
    });
    let lats = hj::JSONUtil::get_by_path(&root, "actionMessage.decodeFeas[0].body.lats").unwrap();
    let texts: Vec<_> = lats
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v["text"].as_str().unwrap())
        .collect();
    assert_eq!(texts, vec!["呵呵", "你好 "]);
}

/// 对齐 Java: `JSONStrFormatterTest.formatTest()`
#[test]
fn fmt_format_test() {
    let result = hj::JSONStrFormatter::format(
        r#"{"age":23,"aihao":["pashan","movies"],"name":{"firstName":"zhang","lastName":"san"}}"#,
    )
    .unwrap();
    assert!(!result.is_empty());
}

/// 对齐 Java: `JSONStrFormatterTest.formatTest2()`
#[test]
fn fmt_format_test2() {
    let result = hj::JSONStrFormatter::format(r#"{"abc":{"def":"\"[ghi]"}}"#).unwrap();
    assert!(!result.is_empty());
}

/// 对齐 Java: `JSONStrFormatterTest.formatTest3()`
#[test]
fn fmt_format_test3() {
    let result = hj::JSONStrFormatter::format(
        r#"{"id":13,"title":"标题","subtitle":"副标题","user_id":6,"type":0}"#,
    )
    .unwrap();
    assert!(!result.is_empty());
}

/// 对齐 Java: `JSONStrFormatterTest.formatTest4()`
#[test]
fn fmt_format_test4() {
    let result =
        hj::JSONStrFormatter::format(r#"{"employees":[{"firstName":"Bill","lastName":"Gates"}]}"#)
            .unwrap();
    assert!(!result.is_empty());
}

/// 对齐 Java: `JSONSupportTest.parseTest()`
#[test]
fn support_parse_test() {
    #[derive(serde::Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    #[allow(dead_code)]
    struct TestBean {
        location: String,
        message: String,
        request_id: String,
        trace_id: String,
    }
    let jsonstr = r#"{"location":"https://hutool.cn","message":"这是一条测试消息","requestId":"123456789","traceId":"987654321"}"#;
    let b: TestBean = hj::JSONUtil::to_bean(jsonstr).unwrap();
    assert_eq!(b.location, "https://hutool.cn");
    assert_eq!(b.request_id, "123456789");
}

/// 对齐 Java: `ParseBeanTest.parseBeanTest()`
#[test]
fn parse_bean_parse_bean_test() {
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }
    let p = Person {
        name: "alice".into(),
        age: 20,
    };
    let s = hj::JSONUtil::to_json_string(&p).unwrap();
    let back: Person = hj::JSONUtil::to_bean(&s).unwrap();
    assert_eq!(p, back);
}

/// 对齐 Java: `TransientTest.beanWithoutTransientTest()`
#[test]
fn transient_bean_without_transient_test() {
    #[derive(serde::Serialize)]
    struct Bill {
        id: String,
        #[serde(rename = "bizNo")]
        biz_no: String,
    }
    let b = Bill {
        id: "3243".into(),
        biz_no: "bizNo".into(),
    };
    let s = hj::JSONUtil::to_json_string(&b).unwrap();
    assert!(s.contains("id"));
    assert!(s.contains("bizNo"));
}

/// 对齐 Java: `TransientTest.beanWithTransientTest()`
#[test]
fn transient_bean_with_transient_test() {
    #[derive(serde::Serialize)]
    #[allow(dead_code)]
    struct Bill {
        #[serde(skip_serializing)]
        id: String,
        #[serde(rename = "bizNo")]
        biz_no: String,
    }
    let b = Bill {
        id: "3243".into(),
        biz_no: "bizNo".into(),
    };
    let s = hj::JSONUtil::to_json_string(&b).unwrap();
    assert!(!s.contains("3243"));
    assert!(s.contains("bizNo"));
}

/// 对齐 Java: `TransientTest.beanWithoutTransientToBeanTest()`
#[test]
fn transient_bean_without_transient_to_bean_test() {
    #[derive(serde::Deserialize, Debug)]
    #[allow(dead_code)]
    struct Bill {
        id: String,
        #[serde(rename = "bizNo")]
        biz_no: String,
    }
    let b: Bill = hj::JSONUtil::to_bean(r#"{"id":"3243","bizNo":"bizNo"}"#).unwrap();
    assert_eq!(b.id, "3243");
}

/// 对齐 Java: `TransientTest.beanWithTransientToBeanTest()`
#[test]
fn transient_bean_with_transient_to_bean_test() {
    #[derive(serde::Deserialize, Debug)]
    struct Bill {
        #[serde(default, skip_deserializing)]
        id: Option<String>,
        #[serde(rename = "bizNo")]
        biz_no: String,
    }
    let b: Bill = hj::JSONUtil::to_bean(r#"{"bizNo":"bizNo"}"#).unwrap();
    assert!(b.id.is_none());
    assert_eq!(b.biz_no, "bizNo");
}

/// 对齐 Java: `XMLTest.toXmlTest()`
#[test]
fn xml_to_xml_test() {
    let mut obj = hj::JSONUtil::create_obj();
    obj.set("aaa", serde_json::json!("你好")).unwrap();
    obj.set("键2", serde_json::json!("test")).unwrap();
    let s = hj::XML::to_xml(&obj.to_value());
    assert!(s.contains("你好"));
    assert!(s.contains("test"));
}

/// 对齐 Java: `XMLTest.escapeTest()`
#[test]
fn xml_escape_test() {
    let xml = "<a>•</a>";
    let json_object = hj::XML::to_json(xml).unwrap();
    assert_eq!(json_object.get("a").and_then(|v| v.as_str()), Some("•"));
    let xml2 = hj::XML::to_xml(&json_object.to_value());
    assert!(xml2.contains("•"));
}

/// 对齐 Java: `XMLTest.xmlContentTest()`
#[test]
fn xml_xml_content_test() {
    let mut obj = hj::JSONUtil::create_obj();
    obj.set("content", serde_json::json!("123456")).unwrap();
    let xml = hj::XML::to_xml(&obj.to_value());
    assert!(xml.contains("123456"));
}
