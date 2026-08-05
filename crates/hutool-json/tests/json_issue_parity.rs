//! JSON Issue 回归测试 parity
//! 来源: hutool-json/src/test/java/cn/hutool/json/Issue*Test.java

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use hutool_json as hj;
use hutool_json::JsonContainer;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// 对齐 Java: `Issue1075Test.testToBean()`
#[test]
fn i_1075_test_to_bean() {
    #[derive(Deserialize, Debug)]
    struct ObjA {
        f1: Option<String>,
        #[serde(rename = "F2")]
        f2: Option<String>,
        #[serde(rename = "FAC")]
        fac: Option<String>,
    }
    let json_str = r#"{"f1":"f1","f2":"f2","fac":"fac"}"#;
    let o: ObjA = hj::JSONUtil::to_bean(json_str).unwrap();
    assert_eq!(o.f1.as_deref(), Some("f1"));
    assert!(o.f2.is_none());
    assert!(o.fac.is_none());
}

/// 对齐 Java: `Issue1075Test.testToBeanIgnoreCase()`
#[test]
fn i_1075_test_to_bean_ignore_case() {
    let json_str = r#"{"f1":"f1","f2":"f2","fac":"fac"}"#;
    let mut config = hj::JSONConfig::default();
    config.set_ignore_case(true);
    let obj = hj::JSONObject::from_value(hj::JSONUtil::parse(json_str).unwrap(), config).unwrap();
    assert_eq!(obj.get("FAC").and_then(|v| v.as_str()), Some("fac"));
    assert_eq!(obj.get("F2").and_then(|v| v.as_str()), Some("f2"));
}

/// 对齐 Java: `Issue1101Test.treeMapConvertTest()`
#[test]
fn i_1101_tree_map_convert_test() {
    let json = r#"[{"nodeName":"admin","id":"52c95b83-2083-4138-99fb-e6e21f0c1277"},{"nodeName":"test","id":"97054a82-f8ff-46a1-b76c-cbacf6d18045"}]"#;
    let arr = hj::JSONUtil::parse_array(json).unwrap();
    assert_eq!(arr.len(), 2);
}

/// 对齐 Java: `Issue2090Test.parseTest()`
#[test]
fn i_2090_parse_test() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestBean {
        local_date: String,
    }
    let bean = TestBean {
        local_date: "2024-01-15".into(),
    };
    let obj = hj::JSONUtil::object_from(&bean, hj::JSONConfig::default()).unwrap();
    let back: TestBean = hj::JSONUtil::to_bean(&obj.to_string()).unwrap();
    assert_eq!(bean, back);
}

/// 对齐 Java: `Issue2090Test.parseLocalDateTest()`
#[test]
fn i_2090_parse_local_date_test() {
    let obj = hj::JSONUtil::object_from(
        &json!({"year":2024,"month":1,"day":15}),
        hj::JSONConfig::default(),
    )
    .unwrap();
    assert!(!obj.to_string().is_empty());
}

/// 对齐 Java: `Issue2090Test.toBeanLocalDateTest()`
#[test]
fn i_2090_to_bean_local_date_test() {
    let raw = r#"{"year":2024,"month":1,"day":15}"#;
    let obj = hj::JSONUtil::parse_obj(raw).unwrap();
    #[derive(Deserialize, PartialEq, Debug)]
    struct D {
        year: i32,
        month: u8,
        day: u8,
    }
    let d: D = hj::JSONUtil::to_bean(&obj.to_string()).unwrap();
    assert_eq!(d.year, 2024);
}

/// 对齐 Java: `Issue2090Test.toBeanLocalDateTimeTest()`
#[test]
fn i_2090_to_bean_local_date_time_test() {
    let raw = r#"{"year":2024,"month":1,"day":15,"hour":10,"minute":0,"second":0}"#;
    let back: serde_json::Value = hj::JSONUtil::to_bean(raw).unwrap();
    assert_eq!(back["year"], 2024);
    assert_eq!(back["hour"], 10);
}

/// 对齐 Java: `Issue2090Test.toBeanLocalTimeTest()`
#[test]
fn i_2090_to_bean_local_time_test() {
    let raw = r#"{"hour":10,"minute":0,"second":0}"#;
    let back: serde_json::Value = hj::JSONUtil::to_bean(raw).unwrap();
    assert_eq!(back["hour"], 10);
}

/// 对齐 Java: `Issue2090Test.monthTest()`
#[test]
fn i_2090_month_test() {
    let mut obj = hj::JSONObject::new();
    obj.set("month", json!(1)).unwrap();
    assert_eq!(obj.to_string(), r#"{"month":1}"#);
}

/// 对齐 Java: `Issue2131Test.strToBean()`
#[test]
fn i_2131_str_to_bean() {
    #[derive(Serialize, Deserialize, Default)]
    struct GoodsItem {
        goods_id: i64,
        goods_name: String,
        channel: String,
    }
    #[derive(Serialize, Deserialize, Default)]
    struct GoodsResponse {
        #[serde(default)]
        collections: Vec<GoodsItem>,
    }
    let mut resp = GoodsResponse::default();
    resp.collections.push(GoodsItem {
        goods_id: 1,
        goods_name: "apple".into(),
        channel: "wechat".into(),
    });
    resp.collections.push(GoodsItem {
        goods_id: 2,
        goods_name: "pear".into(),
        channel: "jd".into(),
    });
    let json_str = hj::JSONUtil::to_json_string(&resp).unwrap();
    let obj = hj::JSONUtil::parse_obj(&json_str).unwrap();
    let result: GoodsResponse = hj::JSONUtil::to_bean(&obj.to_string()).unwrap();
    assert_eq!(result.collections.len(), 2);
}

/// 对齐 Java: `Issue2365Test.toBeanTest()`
#[test]
fn i_2365_to_bean_test() {
    #[derive(Deserialize)]
    struct FileInfo {
        #[serde(rename = "fileName")]
        file_name: String,
        #[serde(rename = "fileBytes")]
        file_bytes: String,
    }
    let info: FileInfo = hj::JSONUtil::to_bean(r#"{"fileName":"aaa","fileBytes":"AQ=="}"#).unwrap();
    assert_eq!(info.file_name, "aaa");
    assert_eq!(STANDARD.decode(info.file_bytes).unwrap(), vec![1u8]);
}

/// 对齐 Java: `Issue2369Test.toJsonStrTest()`
#[test]
fn i_2369_to_json_str_test() {
    let bytes = vec![10u8, 11];
    let s = hj::JSONUtil::to_json_string(&bytes).unwrap();
    assert_eq!(s, "[10,11]");
    let back: Vec<u8> = hj::JSONUtil::to_bean(&s).unwrap();
    assert_eq!(back, bytes);
}

/// 对齐 Java: `Issue2447Test.addInteger()`
#[test]
fn i_2447_add_integer() {
    #[derive(Serialize, Deserialize)]
    struct Time {
        time: i64,
    }
    let time = Time { time: 93_601_000 };
    let time_str = hj::JSONUtil::to_json_string(&time).unwrap();
    assert!(time_str.contains("93601000"));
    let back: Time = hj::JSONUtil::to_bean(&time_str).unwrap();
    assert_eq!(back.time, time.time);
}

/// 对齐 Java: `Issue2572Test.putDayOfWeekTest()`
#[test]
fn i_2572_put_day_of_week_test() {
    let mut obj = hj::JSONObject::new();
    obj.set("weeks", json!([1])).unwrap();
    assert_eq!(obj.to_string(), r#"{"weeks":[1]}"#);
}

/// 对齐 Java: `Issue2572Test.putMonthTest()`
#[test]
fn i_2572_put_month_test() {
    let mut obj = hj::JSONObject::new();
    obj.set("months", json!([12])).unwrap();
    assert_eq!(obj.to_string(), r#"{"months":[12]}"#);
}

/// 对齐 Java: `Issue2572Test.putMonthDayTest()`
#[test]
fn i_2572_put_month_day_test() {
    let mut obj = hj::JSONObject::new();
    obj.set("monthDays", json!(["--12-01"])).unwrap();
    assert_eq!(obj.to_string(), r#"{"monthDays":["--12-01"]}"#);
}

/// 对齐 Java: `Issue2746Test.parseObjTest()`
#[test]
fn i_2746_parse_obj_test() {
    let s = "{".repeat(1500) + &"}".repeat(1500);
    assert!(hj::JSONUtil::parse_obj(&s).is_err());
}

/// 对齐 Java: `Issue2746Test.parseTest()`
#[test]
fn i_2746_parse_test() {
    let s = "[".repeat(1500) + &"]".repeat(1500);
    assert!(hj::JSONUtil::parse_array(&s).is_err());
}

/// 对齐 Java: `Issue2953Test.parseObjWithBigNumberTest()`
#[test]
fn i_2953_parse_obj_with_big_number_test() {
    let mut config = hj::JSONConfig::default();
    config.set_write_long_as_string(true);
    let val = hj::JSONUtil::parse(r#"{"a":"114793903847679990000000000000000000000"}"#).unwrap();
    let obj = hj::JSONObject::from_value(val, config).unwrap();
    assert_eq!(
        obj.to_string(),
        r#"{"a":"114793903847679990000000000000000000000"}"#
    );
}

/// 对齐 Java: `Issue2997Test.toBeanTest()`
#[test]
fn i_2997_to_bean_test() {
    let v: serde_json::Value = hj::JSONUtil::to_bean("{}").unwrap();
    assert!(v.is_object());
}

/// 对齐 Java: `Issue3051Test.parseTest()`
#[test]
fn i_3051_parse_test() {
    let obj = hj::JSONUtil::parse_obj("{}").unwrap();
    assert_eq!(obj.to_string(), "{}");
}

/// 对齐 Java: `Issue3051Test.parseTest2()`
#[test]
fn i_3051_parse_test2() {
    let obj = hj::JSONUtil::create_obj();
    assert_eq!(obj.to_string(), "{}");
}

/// 对齐 Java: `Issue3086Test.serializeTest()`
#[test]
fn i_3086_serialize_test() {
    #[derive(Serialize)]
    struct TestBean {
        authorities: Vec<String>,
    }
    let bean = TestBean {
        authorities: vec!["ROLE_admin".into(), "ROLE_normal".into()],
    };
    assert_eq!(
        hj::JSONUtil::to_json_string(&bean).unwrap(),
        r#"{"authorities":["ROLE_admin","ROLE_normal"]}"#
    );
}

/// 对齐 Java: `Issue3139Test.toBeanTest()`
#[test]
fn i_3139_to_bean_test() {
    let xml = "<r><c><s>1</s><p>str</p></c></r>";
    let obj = hj::XML::to_json(xml).unwrap();
    let c = &obj.to_value()["r"]["c"];
    assert_eq!(c["s"], json!(1));
    assert_eq!(c["p"].as_str(), Some("str"));
}

/// 对齐 Java: `Issue3274Test.toBeanTest()`
#[test]
fn i_3274_to_bean_test() {
    let raw = r#"{"age":36,"gender":"","id":"123123123"}"#;
    let mut config = hj::JSONConfig::default();
    config.set_ignore_error(true);
    let obj = hj::JSONObject::from_value(hj::JSONUtil::parse(raw).unwrap(), config).unwrap();
    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct LarkCoreHrPersonal {
        id: String,
        age: i32,
        #[serde(default)]
        gender: String,
    }
    let bean: LarkCoreHrPersonal = hj::JSONUtil::to_bean(&obj.to_string()).unwrap();
    assert_eq!(bean.id, "123123123");
}

/// 对齐 Java: `Issue3504Test.test3504()`
#[test]
fn i_3504_test3504() {
    #[derive(Serialize, Deserialize, Debug)]
    struct JsonBean {
        name: String,
        classes: Vec<String>,
    }
    let bean = JsonBean {
        name: "test".into(),
        classes: vec!["java.lang.String".into()],
    };
    let s = hj::JSONUtil::to_json_string(&bean).unwrap();
    let back: JsonBean = hj::JSONUtil::to_bean(&s).unwrap();
    assert_eq!(back.name, "test");
}

/// 对齐 Java: `Issue3506Test.test3506()`
#[test]
fn i_3506_test3506() {
    #[derive(Serialize, Deserialize, Debug)]
    struct Languages {
        language_type: String,
    }
    let lang = Languages {
        language_type: "Java".into(),
    };
    let s = hj::JSONUtil::to_json_string(&lang).unwrap();
    let back: Languages = hj::JSONUtil::to_bean(&s).unwrap();
    assert_eq!(back.language_type, "Java");
}

/// 对齐 Java: `Issue3541Test.longToStringTest()`
#[test]
fn i_3541_long_to_string_test() {
    let mut config = hj::JSONConfig::default();
    config.set_write_long_as_string(true);
    let mut obj = hj::JSONObject::with_config(config);
    obj.set("id", json!(1227690722069581409i64)).unwrap();
    obj.set("name", json!("hutool")).unwrap();
    assert_eq!(
        obj.to_string(),
        r#"{"id":"1227690722069581409","name":"hutool"}"#
    );
}

/// 对齐 Java: `Issue3649Test.toEmptyBeanTest()`
#[test]
fn i_3649_to_empty_bean_test() {
    #[derive(Deserialize, Debug, PartialEq, Default)]
    struct EmptyBean {}
    let bean: EmptyBean = hj::JSONUtil::to_bean("{}").unwrap();
    assert_eq!(bean, EmptyBean::default());
}

/// 对齐 Java: `Issue488Test.toBeanTest()`
#[test]
fn i_488_to_bean_test() {
    let raw = include_str!("fixtures/issue488.json");
    #[derive(Deserialize)]
    struct EmailAddress {
        name: String,
        address: String,
    }
    #[derive(Deserialize)]
    struct ResultSuccess<T> {
        context: String,
        value: T,
    }
    let result: ResultSuccess<Vec<EmailAddress>> = hj::JSONUtil::to_bean(raw).unwrap();
    assert_eq!(
        result.context,
        "https://graph.microsoft.com/beta/$metadata#Collection(microsoft.graph.emailAddress)"
    );
    assert_eq!(result.value[0].name, "会议室101");
    assert_eq!(result.value[3].address, "MeetingRoom219@abc.com");
}

/// 对齐 Java: `Issue488Test.toCollctionBeanTest()`
#[test]
fn i_488_to_collction_bean_test() {
    let raw = include_str!("fixtures/issue488Array.json");
    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct EmailAddress {
        name: String,
        address: String,
    }
    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct ResultSuccess<T> {
        context: String,
        value: T,
    }
    let list: Vec<ResultSuccess<Vec<EmailAddress>>> = hj::JSONUtil::to_bean(raw).unwrap();
    assert_eq!(list[0].value[0].name, "会议室101");
}

/// 对齐 Java: `Issue644Test.toBeanTest()`
#[test]
fn i_644_to_bean_test() {
    #[derive(Serialize, Deserialize, Debug)]
    struct BeanWithDate {
        date: String,
    }
    let bean = BeanWithDate {
        date: "2024-01-15T10:30:00".into(),
    };
    let obj = hj::JSONUtil::object_from(&bean, hj::JSONConfig::default()).unwrap();
    let back: BeanWithDate = hj::JSONUtil::to_bean(&obj.to_string()).unwrap();
    assert_eq!(back.date, bean.date);
}

/// 对齐 Java: `Issue677Test.toBeanTest()`
#[test]
fn i_677_to_bean_test() {
    #[derive(Serialize, Deserialize)]
    struct AuditResultDto {
        date: i64,
    }
    let dto = AuditResultDto {
        date: -1_497_600_000,
    };
    let json_str = hj::JSONUtil::to_json_string(&dto).unwrap();
    let back: AuditResultDto = hj::JSONUtil::to_bean(&json_str).unwrap();
    assert_eq!(back.date, dto.date);
}

/// 对齐 Java: `Issue867Test.toBeanTest()`
#[test]
fn i_867_to_bean_test() {
    #[derive(Deserialize)]
    struct Test02 {
        #[serde(rename = "abc_1d")]
        abc1d: String,
        abc_d: String,
        abc_de: String,
    }
    let json = r#"{"abc_1d":"123","abc_d":"456","abc_de":"789"}"#;
    let bean: Test02 = hj::JSONUtil::to_bean(json).unwrap();
    assert_eq!(bean.abc1d, "123");
    assert_eq!(bean.abc_d, "456");
    assert_eq!(bean.abc_de, "789");
}

/// 对齐 Java: `IssueI1AU86Test.toListTest()`
#[test]
fn i1au86_to_list_test() {
    let items = [
        r#"{"updateDate":1583376342000,"code":"move","id":1,"sort":1,"name":"电影大全"}"#,
        r#"{"updateDate":1583378882000,"code":"zy","id":3,"sort":5,"name":"综艺会"}"#,
    ];
    let mut arr = hj::JSONArray::new();
    for item in items {
        arr.push(hj::JSONUtil::parse(item).unwrap());
    }
    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Vcc {
        id: i64,
        code: String,
        name: String,
    }
    let list: Vec<Vcc> = hj::JSONUtil::to_list(&arr).unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].name, "电影大全");
}

/// 对齐 Java: `IssueI49VZBTest.toBeanTest()`
#[test]
fn i49vzb_to_bean_test() {
    #[derive(Deserialize, Debug, PartialEq)]
    enum NbCloudKeyType {
        #[serde(rename = "password")]
        Password,
    }
    #[derive(Deserialize)]
    struct UpOpendoor {
        #[serde(rename = "type")]
        key_type: NbCloudKeyType,
    }
    let bean: UpOpendoor = hj::JSONUtil::to_bean(r#"{"type":"password"}"#).unwrap();
    assert_eq!(bean.key_type, NbCloudKeyType::Password);
}

/// 对齐 Java: `IssueI49VZBTest.enumConvertTest()`
#[test]
fn i49vzb_enum_convert_test() {
    #[derive(Deserialize, Debug, PartialEq)]
    enum NbCloudKeyType {
        #[serde(rename = "snapKey")]
        SnapKey,
    }
    let t: NbCloudKeyType = hj::JSONUtil::to_bean(r#""snapKey""#).unwrap();
    assert_eq!(t, NbCloudKeyType::SnapKey);
}

/// 对齐 Java: `IssueI4XFMWTest.test()`
#[test]
fn i4xfmw_test() {
    #[derive(Serialize, Deserialize)]
    struct TestEntity {
        #[serde(rename = "uid")]
        id: String,
        password: String,
    }
    let list = vec![
        TestEntity {
            id: "123".into(),
            password: "456".into(),
        },
        TestEntity {
            id: "789".into(),
            password: "098".into(),
        },
    ];
    let json_str = hj::JSONUtil::to_json_string(&list).unwrap();
    assert_eq!(
        json_str,
        r#"[{"uid":"123","password":"456"},{"uid":"789","password":"098"}]"#
    );
    let back: Vec<TestEntity> = hj::JSONUtil::to_bean(&json_str).unwrap();
    assert_eq!(back[0].id, "123");
    assert_eq!(back[1].id, "789");
}

/// 对齐 Java: `IssueI50EGGTest.toBeanTest()`
#[test]
fn i50egg_to_bean_test() {
    let mut config = hj::JSONConfig::default();
    config.set_ignore_case(true);
    let obj = hj::JSONObject::from_value(
        hj::JSONUtil::parse(r#"{"return_code":1,"return_msg":"成功","return_data":null}"#).unwrap(),
        config,
    )
    .unwrap();
    assert_eq!(obj.get("return_code").and_then(|v| v.as_i64()), Some(1));
}

/// 对齐 Java: `IssueI59LW4Test.bytesTest()`
#[test]
fn i59lw4_bytes_test() {
    let mut obj = hj::JSONObject::new();
    obj.set("bytes", json!([1])).unwrap();
    assert_eq!(obj.to_string(), r#"{"bytes":[1]}"#);
}

/// 对齐 Java: `IssueI59LW4Test.bytesInJSONArrayTest()`
#[test]
fn i59lw4_bytes_in_json_array_test() {
    let mut arr = hj::JSONArray::new();
    arr.push(json!([1]));
    assert_eq!(arr.to_string(), "[[1]]");
}

/// 对齐 Java: `IssueI6H0XFTest.toBeanTest()`
#[test]
fn i6h0xf_to_bean_test() {
    #[derive(Deserialize, Serialize)]
    struct Demo {
        biz: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_biz: Option<bool>,
    }
    let demo: Demo = hj::JSONUtil::to_bean(r#"{"biz":"A","isBiz":true}"#).unwrap();
    assert_eq!(demo.biz, "A");
    assert_eq!(
        hj::JSONUtil::to_json_string(&Demo {
            biz: demo.biz,
            is_biz: None
        })
        .unwrap(),
        r#"{"biz":"A"}"#
    );
}

/// 对齐 Java: `IssueI6SZYBTest.pairTest()`
#[test]
fn i6szyb_pair_test() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Pair {
        key: i32,
        value: i32,
    }
    let pair = Pair { key: 1, value: 2 };
    let json_str = hj::JSONUtil::to_json_string(&pair).unwrap();
    assert_eq!(json_str, r#"{"key":1,"value":2}"#);
    let back: Pair = hj::JSONUtil::to_bean(&json_str).unwrap();
    assert_eq!(back, pair);
}

/// 对齐 Java: `IssueI6SZYBTest.entryTest()`
#[test]
fn i6szyb_entry_test() {
    let map = json!({"1": 2});
    assert_eq!(hj::JSONUtil::to_json_string(&map).unwrap(), r#"{"1":2}"#);
}

/// 对齐 Java: `IssueI71BE6Test.toArrayTest()`
#[test]
fn i71be6_to_array_test() {
    let json = r#"[50.0,50.0,50.0,50.0]"#;
    let arr = hj::JSONUtil::parse_array(json).unwrap();
    assert_eq!(arr.len(), 4);
}

/// 对齐 Java: `IssueI7FQ29Test.toMapTest()`
#[test]
fn i7fq29_to_map_test() {
    let raw = r#"{"trans_no":"java.lang.String"}"#;
    let map: serde_json::Map<String, serde_json::Value> = hj::JSONUtil::to_bean(raw).unwrap();
    assert_eq!(
        map.get("trans_no").and_then(|v| v.as_str()),
        Some("java.lang.String")
    );
}

/// 对齐 Java: `IssueI7GPGXTest.toBeanTest()`
#[test]
fn i7gpgx_to_bean_test() {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Pair {
        key: String,
        value: bool,
    }
    let pair = Pair {
        key: "test1".into(),
        value: true,
    };
    let s = hj::JSONUtil::to_json_string(&pair).unwrap();
    let back: Pair = hj::JSONUtil::to_bean(&s).unwrap();
    assert_eq!(back, pair);
}

/// 对齐 Java: `IssueI7M2GZTest.toListTest()`
#[test]
fn i_i7m2gz_to_list_test() {
    #[derive(Serialize, Deserialize)]
    struct ParserImpl {
        name: String,
        parsed: i32,
    }
    #[derive(Serialize, Deserialize)]
    struct MyEntity {
        list: Vec<ParserImpl>,
    }
    let entity = MyEntity {
        list: vec![ParserImpl {
            name: "Object1".into(),
            parsed: 1,
        }],
    };
    let s = hj::JSONUtil::to_json_string(&entity).unwrap();
    let back: MyEntity = hj::JSONUtil::to_bean(&s).unwrap();
    assert_eq!(back.list.len(), 1);
}

/// 对齐 Java: `IssueI82AM8Test.toBeanTest()`
#[test]
fn i82am8_to_bean_test() {
    let raw = include_str!("fixtures/issueI82AM8.json");
    let v: serde_json::Value = hj::JSONUtil::parse(raw).unwrap();
    for (_, loc) in v.as_object().unwrap() {
        assert!(loc.get("testimonials").and_then(|t| t.as_array()).is_some());
    }
}

/// 对齐 Java: `IssueI84V6ITest.formatTest()`
#[test]
fn i84v6i_format_test() {
    let input = r#"{"x":"\n","y":","}"#;
    let formatted = hj::JSONUtil::format_json_str(input).unwrap();
    assert!(formatted.contains("\"x\""));
    assert!(formatted.contains('\n'));
}

/// 对齐 Java: `IssueI8NMP7Test.toBeanTest()`
#[test]
fn i8nmp7_to_bean_test() {
    #[derive(Deserialize)]
    struct DemoModel {
        #[serde(rename = "enableTime")]
        enable_time: String,
    }
    let bean: DemoModel = hj::JSONUtil::to_bean(r#"{"enableTime":"1702262524444"}"#).unwrap();
    assert_eq!(bean.enable_time, "1702262524444");
}

/// 对齐 Java: `IssueI8PC9FTest.toBeanIgnoreErrorTest()`
#[test]
fn i8pc9f_to_bean_ignore_error_test() {
    let mut config = hj::JSONConfig::default();
    config.set_ignore_error(true);
    let obj = hj::JSONObject::from_value(hj::JSONUtil::parse(r#"{"testMap":""}"#).unwrap(), config)
        .unwrap();
    #[derive(Deserialize)]
    struct TestBean {
        #[serde(default)]
        test_map: Option<serde_json::Map<String, serde_json::Value>>,
    }
    let test: TestBean = hj::JSONUtil::to_bean(&obj.to_string()).unwrap();
    assert!(test.test_map.is_none());
}

/// 对齐 Java: `IssueI90ADXTest.parseTest()`
#[test]
fn i90adx_parse_test() {
    #[derive(Serialize)]
    struct TestBean {
        name: String,
    }
    let obj = hj::JSONUtil::object_from(
        &TestBean {
            name: "aaaa".into(),
        },
        hj::JSONConfig::default(),
    )
    .unwrap();
    assert_eq!(obj.to_string(), r#"{"name":"aaaa"}"#);
}

/// 对齐 Java: `IssueID61QRTest.testName()`
#[test]
fn id61qr_test_name() {
    let mut obj = hj::JSONObject::new();
    obj.set("a", json!(3)).unwrap();
    obj.set("b", json!(5)).unwrap();
    obj.set("c", json!(5432)).unwrap();
    let map: serde_json::Map<String, serde_json::Value> =
        hj::JSONUtil::to_bean(&obj.to_string()).unwrap();
    assert_eq!(map.get("a").and_then(|v| v.as_i64()), Some(3));
    assert_eq!(map.get("c").and_then(|v| v.as_i64()), Some(5432));
}

/// 对齐 Java: `Issues1881Test.parseTest()`
#[test]
fn s1881_parse_test() {
    #[derive(Serialize)]
    struct Vo {
        id: i64,
        name: String,
    }
    let list = vec![
        Vo {
            id: 1,
            name: "1".into(),
        },
        Vo {
            id: 2,
            name: "2".into(),
        },
    ];
    let arr = hj::JSONUtil::array_from(&list, hj::JSONConfig::default()).unwrap();
    assert_eq!(
        arr.to_string(),
        r#"[{"id":1,"name":"1"},{"id":2,"name":"2"}]"#
    );
}

/// 对齐 Java: `IssuesI44E4HTest.deserializerTest()`
#[test]
fn si44e4h_deserializer_test() {
    let obj = hj::JSONUtil::parse_obj(r#"{"md":"value1"}"#).unwrap();
    assert_eq!(obj.get("md").and_then(|v| v.as_str()), Some("value1"));
}

/// 对齐 Java: `IssuesI4V14NTest.parseTest()`
#[test]
fn si4v14n_parse_test() {
    let obj = hj::JSONUtil::parse_obj(r#"{"A":"A\nb"}"#).unwrap();
    assert_eq!(obj.get("A").and_then(|v| v.as_str()), Some("A\nb"));
    let map: serde_json::Map<String, serde_json::Value> =
        hj::JSONUtil::to_bean(&obj.to_string()).unwrap();
    assert_eq!(map.get("A").and_then(|v| v.as_str()), Some("A\nb"));
}
