//! xml_util parity tests
//! 对齐: hutool-core XmlUtilTest

#![allow(non_snake_case)] // 对齐 Java 测试 Bean 字段名（ReqCode/AccountName 等大写风格）

use hutool_core::XmlUtil;

// ── XML 验证 ──

#[test]
fn is_xml_valid() {
    assert!(XmlUtil::is_xml("<?xml version=\"1.0\"?>"));
    assert!(XmlUtil::is_xml("<root>content</root>"));
    assert!(XmlUtil::is_xml("<tag/>"));
}

#[test]
fn is_xml_invalid() {
    assert!(!XmlUtil::is_xml("not xml"));
    assert!(!XmlUtil::is_xml(""));
}

// ── XML 转义 ──

#[test]
fn escape_char_basic() {
    assert_eq!(XmlUtil::escape_char('&'), "&amp;");
    assert_eq!(XmlUtil::escape_char('<'), "&lt;");
    assert_eq!(XmlUtil::escape_char('>'), "&gt;");
    assert_eq!(XmlUtil::escape_char('"'), "&quot;");
    assert_eq!(XmlUtil::escape_char('\''), "&apos;");
}

#[test]
fn escape_char_normal() {
    assert_eq!(XmlUtil::escape_char('a'), "");
    assert_eq!(XmlUtil::escape_char('1'), "");
}

#[test]
fn escape_basic() {
    assert_eq!(XmlUtil::escape("<tag>value</tag>"), "&lt;tag&gt;value&lt;/tag&gt;");
}

#[test]
fn escape_special_chars() {
    assert_eq!(XmlUtil::escape("a & b"), "a &amp; b");
    assert_eq!(XmlUtil::escape("\"hello\""), "&quot;hello&quot;");
    assert_eq!(XmlUtil::escape("it's"), "it&apos;s");
}

#[test]
fn unescape_basic() {
    assert_eq!(XmlUtil::unescape("&lt;tag&gt;value&lt;/tag&gt;"), "<tag>value</tag>");
}

#[test]
fn unescape_special_chars() {
    assert_eq!(XmlUtil::unescape("a &amp; b"), "a & b");
    assert_eq!(XmlUtil::unescape("&quot;hello&quot;"), "\"hello\"");
    assert_eq!(XmlUtil::unescape("it&apos;s"), "it's");
}

#[test]
fn escape_unescape_roundtrip() {
    let inputs = vec![
        "<tag>value</tag>",
        "a & b",
        "\"hello\"",
        "it's",
        "<script>alert('xss')</script>",
    ];
    for input in inputs {
        let escaped = XmlUtil::escape(input);
        let unescaped = XmlUtil::unescape(&escaped);
        assert_eq!(unescaped, input, "roundtrip failed for {:?}", input);
    }
}

// ── XML 构建 ──

#[test]
fn element_basic() {
    assert_eq!(XmlUtil::element("name", "Alice"), "<name>Alice</name>");
}

#[test]
fn element_with_special_chars() {
    assert_eq!(XmlUtil::element("tag", "<value>"), "<tag>&lt;value&gt;</tag>");
}

#[test]
fn element_with_attrs() {
    let result = XmlUtil::element_with_attrs("tag", &[("id", "1"), ("class", "test")], "value");
    assert_eq!(result, "<tag id=\"1\" class=\"test\">value</tag>");
}

#[test]
fn self_closing_element_basic() {
    assert_eq!(XmlUtil::self_closing_element("br"), "<br />");
}

#[test]
fn self_closing_element_with_attrs() {
    let result = XmlUtil::self_closing_element_with_attrs("img", &[("src", "test.png"), ("alt", "test")]);
    assert_eq!(result, "<img src=\"test.png\" alt=\"test\" />");
}

// ── XML 声明 ──

#[test]
fn xml_header_utf8() {
    assert_eq!(XmlUtil::xml_header_utf8(), "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
}

#[test]
fn xml_header_custom() {
    assert_eq!(XmlUtil::xml_header("ISO-8859-1"), "<?xml version=\"1.0\" encoding=\"ISO-8859-1\"?>");
}

// ── CDATA ──

#[test]
fn wrap_cdata_basic() {
    assert_eq!(XmlUtil::wrap_cdata("hello"), "<![CDATA[hello]]>");
}

#[test]
fn wrap_cdata_with_special_chars() {
    assert_eq!(XmlUtil::wrap_cdata("<tag>&amp;</tag>"), "<![CDATA[<tag>&amp;</tag>]]>");
}

// ── XML 解析辅助 ──

#[test]
fn get_tag_content_basic() {
    let xml = "<root><name>Alice</name><age>30</age></root>";
    assert_eq!(XmlUtil::get_tag_content(xml, "name"), Some("Alice".to_string()));
    assert_eq!(XmlUtil::get_tag_content(xml, "age"), Some("30".to_string()));
}

#[test]
fn get_tag_content_missing() {
    let xml = "<root><name>Alice</name></root>";
    assert_eq!(XmlUtil::get_tag_content(xml, "age"), None);
}

#[test]
fn get_attribute_basic() {
    let xml = "<tag id=\"1\" class=\"test\">value</tag>";
    assert_eq!(XmlUtil::get_attribute(xml, "id"), Some("1".to_string()));
    assert_eq!(XmlUtil::get_attribute(xml, "class"), Some("test".to_string()));
}

#[test]
fn get_attribute_missing() {
    let xml = "<tag id=\"1\">value</tag>";
    assert_eq!(XmlUtil::get_attribute(xml, "class"), None);
}

// ── Hutool TEST parity gap wave ──
// ── Hutool XmlUtilTest remaining gaps ──

const RETURN_SMS_XML: &str = "<?xml version=\"1.0\" encoding=\"utf-8\" ?><returnsms><returnstatus>Success</returnstatus><message>ok</message><remainpoint>1490</remainpoint><taskID>885</taskID><successCounts>1</successCounts></returnsms>";

/// 对齐 Java: `XmlUtilTest.parseTest()`
#[test]
fn parse_test() {
    let doc = XmlUtil::parse_xml(RETURN_SMS_XML).unwrap();
    assert_eq!(
        Some("Success".to_string()),
        XmlUtil::element_text(&doc.root, "returnstatus")
    );
}

/// 对齐 Java: `XmlUtilTest.writeTest()`
#[test]
fn write_test() {
    let doc = XmlUtil::parse_xml(RETURN_SMS_XML).unwrap();
    let xml = XmlUtil::to_str(&doc, false, false);
    assert!(xml.contains("<returnstatus>Success</returnstatus>"));
}

/// 对齐 Java: `XmlUtilTest.xpathTest()`
#[test]
fn xpath_test() {
    let xml = "<?xml version=\"1.0\" encoding=\"utf-8\" ?><returnsms><returnstatus>Success（成功）</returnstatus><message>ok</message><remainpoint>1490</remainpoint><taskID>885</taskID><successCounts>1</successCounts></returnsms>";
    let doc = XmlUtil::parse_xml(xml).unwrap();
    assert_eq!(Some("ok".to_string()), XmlUtil::get_by_xpath("//returnsms/message", &doc));
}

/// 对齐 Java: `XmlUtilTest.xpathTest2()`
#[test]
fn xpath_test2() {
    let xml = include_str!("resources/test.xml");
    let doc = XmlUtil::parse_xml(xml).unwrap();
    assert_eq!(Some("ok".to_string()), XmlUtil::get_by_xpath("//returnsms/message", &doc));
}

/// 对齐 Java: `XmlUtilTest.xmlToMapTest()`
#[test]
fn xml_to_map_test() {
    let xml = "<?xml version=\"1.0\" encoding=\"utf-8\" ?><returnsms><returnstatus>Success</returnstatus><message>ok</message><remainpoint>1490</remainpoint><taskID>885</taskID><successCounts>1</successCounts><newNode><sub>subText</sub></newNode></returnsms>";
    let map = XmlUtil::xml_to_map(xml).unwrap();
    assert_eq!(6, map.len());
    assert_eq!(Some(&serde_json::json!("Success")), map.get("returnstatus"));
    assert_eq!(Some(&serde_json::json!("ok")), map.get("message"));
    assert_eq!(Some(&serde_json::json!("1490")), map.get("remainpoint"));
    assert_eq!(Some(&serde_json::json!("885")), map.get("taskID"));
    assert_eq!(Some(&serde_json::json!("1")), map.get("successCounts"));
    assert_eq!(
        Some(&serde_json::json!({"sub": "subText"})),
        map.get("newNode")
    );
}

/// 对齐 Java: `XmlUtilTest.xmlToMapTest2()`
#[test]
fn xml_to_map_test2() {
    let xml = "<root><name>张三</name><name>李四</name></root>";
    let map = XmlUtil::xml_to_map(xml).unwrap();
    assert_eq!(1, map.len());
    assert_eq!(
        Some(&serde_json::json!(["张三", "李四"])),
        map.get("name")
    );
}

/// 对齐 Java: `XmlUtilTest.mapToXmlTest()`
#[test]
fn map_to_xml_test() {
    let mut map = indexmap::IndexMap::new();
    map.insert("name".to_string(), serde_json::json!("张三"));
    map.insert("age".to_string(), serde_json::json!(12));
    let mut game = indexmap::IndexMap::new();
    game.insert("昵称".to_string(), serde_json::json!("Looly"));
    game.insert("level".to_string(), serde_json::json!(14));
    map.insert("game".to_string(), serde_json::json!(game));
    let doc = XmlUtil::map_to_xml(&map, "user").unwrap();
    assert_eq!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><user><name>张三</name><age>12</age><game><昵称>Looly</昵称><level>14</level></game></user>",
        XmlUtil::to_str(&doc, false, false)
    );
}

/// 对齐 Java: `XmlUtilTest.mapToXmlTest2()`
#[test]
fn map_to_xml_test2() {
    let mut map = indexmap::IndexMap::new();
    map.insert(
        "Town".to_string(),
        serde_json::json!(["town1", "town2"]),
    );
    let doc = XmlUtil::map_to_xml(&map, "City").unwrap();
    assert_eq!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?><City><Town>town1</Town><Town>town2</Town></City>",
        XmlUtil::to_str(&doc, false, false)
    );
}

/// 对齐 Java: `XmlUtilTest.readTest()`
#[test]
fn read_test() {
    let doc = XmlUtil::read_xml(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/resources/test.xml"
    ))
    .unwrap();
    assert_eq!("returnsms", doc.root.tag);
}

/// 对齐 Java: `XmlUtilTest.readBySaxTest()`
#[test]
fn read_by_sax_test() {
    let xml = include_str!("resources/test.xml");
    let mut seen = std::collections::HashSet::new();
    XmlUtil::read_by_sax(xml, |name| {
        seen.insert(name.to_string());
    })
    .unwrap();
    for name in [
        "returnsms",
        "returnstatus",
        "message",
        "remainpoint",
        "taskID",
        "successCounts",
    ] {
        assert!(seen.contains(name), "missing element {name}");
    }
}

/// 对齐 Java: `XmlUtilTest.mapToXmlTestWithOmitXmlDeclaration()`
#[test]
fn map_to_xml_test_with_omit_xml_declaration() {
    let mut map = indexmap::IndexMap::new();
    map.insert("name".to_string(), serde_json::json!("ddatsh"));
    assert_eq!(
        "<xml><name>ddatsh</name></xml>",
        XmlUtil::map_to_xml_str(&map, true).unwrap()
    );
}

/// 对齐 Java: `XmlUtilTest.getByPathTest()`
#[test]
fn get_by_path_test() {
    let xml_str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<soap:Envelope xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\">\n  <soap:Body>\n    <ns2:testResponse xmlns:ns2=\"http://ws.xxx.com/\">\n      <return>2020/04/15 21:01:21</return>\n    </ns2:testResponse>\n  </soap:Body>\n</soap:Envelope>\n";
    let doc = XmlUtil::parse_xml(xml_str).unwrap();
    assert_eq!(
        Some("2020/04/15 21:01:21".to_string()),
        XmlUtil::get_by_xpath(
            "//soap:Envelope/soap:Body/ns2:testResponse/return",
            &doc
        )
    );
}

/// 对齐 Java: `XmlUtilTest.beanToXmlIgnoreNullTest()`
#[test]
fn bean_to_xml_ignore_null_test() {
    #[derive(serde::Serialize)]
    struct TestBean {
        ReqCode: String,
        AccountName: String,
        Operator: String,
        ProjectCode: Option<String>,
        BankCode: String,
    }
    let bean = TestBean {
        ReqCode: "1111".to_string(),
        AccountName: "账户名称".to_string(),
        Operator: "cz".to_string(),
        ProjectCode: None,
        BankCode: "00001".to_string(),
    };
    let doc = XmlUtil::bean_to_xml(&bean, "TestBean", None, false).unwrap();
    assert!(XmlUtil::get_element(&doc.root, "ProjectCode").is_some());
    let doc = XmlUtil::bean_to_xml(&bean, "TestBean", None, true).unwrap();
    assert!(XmlUtil::get_element(&doc.root, "ProjectCode").is_none());
}

/// 对齐 Java: `XmlUtilTest.xmlToBeanTest()`
#[test]
fn xml_to_bean_test() {
    #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
    struct TestBean {
        ReqCode: String,
        AccountName: String,
        Operator: String,
        ProjectCode: String,
        BankCode: String,
    }
    let bean = TestBean {
        ReqCode: "1111".to_string(),
        AccountName: "账户名称".to_string(),
        Operator: "cz".to_string(),
        ProjectCode: "123".to_string(),
        BankCode: "00001".to_string(),
    };
    let doc = XmlUtil::bean_to_xml(&bean, "TestBean", None, false).unwrap();
    assert_eq!("TestBean", doc.root.tag);
    let parsed: TestBean = XmlUtil::xml_to_bean(&doc).unwrap();
    assert_eq!(bean, parsed);
}

/// 对齐 Java: `XmlUtilTest.xmlToBeanTest2()`
#[test]
fn xml_to_bean_test2() {
    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct SmsRes {
        code: String,
    }
    let xml_str = "<?xml version=\"1.0\" encoding=\"gbk\" ?><response><code>02</code></response>";
    let doc = XmlUtil::parse_xml(xml_str).unwrap();
    let res: SmsRes = XmlUtil::xml_to_bean(&doc).unwrap();
    assert_eq!("02", res.code);
}

/// 对齐 Java: `XmlUtilTest.cleanCommentTest()`
#[test]
fn clean_comment_test() {
    let xml_content = "<info><title>hutool</title><!-- 这是注释 --><lang>java</lang></info>";
    assert_eq!(
        "<info><title>hutool</title><lang>java</lang></info>",
        XmlUtil::clean_comment(xml_content)
    );
}

/// 对齐 Java: `XmlUtilTest.formatTest()`
#[test]
fn format_test() {
    let mut map = indexmap::IndexMap::new();
    map.insert("name".to_string(), serde_json::json!("走位"));
    let doc = XmlUtil::map_to_xml(&map, "NODES").unwrap();
    let formatted = XmlUtil::format(&doc);
    assert!(formatted.contains("<NODES>"));
    assert!(formatted.contains("<name>走位</name>"));
}

/// 对齐 Java: `XmlUtilTest.getParamTest()`
#[test]
fn get_param_test() {
    let xml = "<Config name=\"aaaa\">\n    <url>222222</url>\n</Config>";
    let doc = XmlUtil::parse_xml(xml).unwrap();
    assert_eq!(Some("aaaa"), doc.root.attribute("name"));
}

/// 对齐 Java: `XmlUtilTest.xmlStrToBeanTest()`
#[test]
fn xml_str_to_bean_test() {
    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct UserInfo {
        name: String,
        age: String,
        email: String,
    }
    let xml = "<userInfo><name>张三</name><age>20</age><email>zhangsan@example.com</email></userInfo>";
    let doc = XmlUtil::parse_xml(xml).unwrap();
    let user: UserInfo = XmlUtil::xml_to_bean(&doc).unwrap();
    assert_eq!(
        UserInfo {
            name: "张三".to_string(),
            age: "20".to_string(),
            email: "zhangsan@example.com".to_string(),
        },
        user
    );
}

/// 对齐 Java: `XmlUtilTest.issue3139Test()`
#[test]
fn issue3139_test() {
    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct C {
        s: String,
        p: String,
    }
    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct R {
        #[serde(deserialize_with = "one_or_many")]
        c: Vec<C>,
    }
    fn one_or_many<'de, D>(deserializer: D) -> std::result::Result<Vec<C>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum OneOrMany {
            One(C),
            Many(Vec<C>),
        }
        match OneOrMany::deserialize(deserializer)? {
            OneOrMany::One(item) => Ok(vec![item]),
            OneOrMany::Many(items) => Ok(items),
        }
    }
    let xml = "<r><c><s>1</s><p>str</p></c></r>";
    let doc = XmlUtil::parse_xml(xml).unwrap();
    let parsed: R = XmlUtil::xml_to_bean(&doc).unwrap();
    assert_eq!("1", parsed.c[0].s);
    assert_eq!("str", parsed.c[0].p);
}

/// 对齐 Java: `XmlUtilTest.escapeTest()`
#[test]
fn escape_test() {
    assert_eq!(XmlUtil::escape("<a>&"), "&lt;a&gt;&amp;");
}
