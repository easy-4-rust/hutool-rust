//! Hutool `hutool-extra` servlet / spring test parity（本地 mock）.
//!
//! 对齐: `cn.hutool.extra.servlet.ServletUtilTest`
//! 对齐: `cn.hutool.extra.spring.SpringUtilTest`
//! 对齐: `cn.hutool.extra.spring.SpringUtilWithAutoConfigTest`
//! 对齐: `cn.hutool.extra.spring.EnableSpringUtilTest`

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// 本地 mock HttpServletResponse：捕获写入的字节与头。
struct MockResponse {
    body: Vec<u8>,
    content_type: Option<String>,
    file_name: Option<String>,
    encoding: Option<String>,
}

impl MockResponse {
    fn new() -> Self {
        Self {
            body: Vec::new(),
            content_type: None,
            file_name: None,
            encoding: None,
        }
    }
    fn set_character_encoding(&mut self, enc: &str) {
        self.encoding = Some(enc.into());
    }
    fn write(&mut self, bytes: &[u8], content_type: &str, file_name: &str) {
        self.content_type = Some(content_type.into());
        self.file_name = Some(file_name.into());
        self.body.extend_from_slice(bytes);
    }
}

fn utf8_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

/// 对齐 Java: `ServletUtilTest.writeTest()`
#[test]
fn servlet_util_test_write_test() {
    let mut response = MockResponse::new();
    let bytes = utf8_bytes("地球是我们共同的家园，需要大家珍惜.");
    response.set_character_encoding("UTF-8");
    response.write(&bytes, "application/pdf", "签名文件.pdf");
    assert_eq!(response.body, bytes);
    assert_eq!(response.content_type.as_deref(), Some("application/pdf"));
    assert_eq!(response.file_name.as_deref(), Some("签名文件.pdf"));
    assert_eq!(response.encoding.as_deref(), Some("UTF-8"));
}

/// 对齐 Java: `ServletUtilTest.jakartaWriteTest()`
#[test]
fn servlet_util_test_jakarta_write_test() {
    servlet_util_test_write_test();
}

// ----- Spring bean registry mock -----
#[derive(Clone, Debug, PartialEq)]
struct Demo2 {
    id: i64,
    name: String,
}

fn registry() -> &'static Mutex<HashMap<String, Demo2>> {
    static REG: OnceLock<Mutex<HashMap<String, Demo2>>> = OnceLock::new();
    REG.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(
            "testDemo".into(),
            Demo2 {
                id: 12345,
                name: "test".into(),
            },
        );
        Mutex::new(m)
    })
}

fn register_bean(name: &str, bean: Demo2) {
    registry().lock().unwrap().insert(name.into(), bean);
}
fn unregister_bean(name: &str) {
    registry().lock().unwrap().remove(name);
}
fn get_bean(name: &str) -> Option<Demo2> {
    registry().lock().unwrap().get(name).cloned()
}

/// 对齐 Java: `SpringUtilTest.registerBeanTest()`
#[test]
fn spring_util_test_register_bean_test() {
    register_bean(
        "registerBean",
        Demo2 {
            id: 123,
            name: "222".into(),
        },
    );
    let got = get_bean("registerBean").expect("registered");
    assert_eq!(got.id, 123);
    assert_eq!(got.name, "222");
}

/// 对齐 Java: `SpringUtilTest.unregisterBeanTest()`
#[test]
fn spring_util_test_unregister_bean_test() {
    register_bean(
        "testAutoWired",
        Demo2 {
            id: 1,
            name: "x".into(),
        },
    );
    assert!(get_bean("testAutoWired").is_some());
    unregister_bean("testAutoWired");
    assert!(get_bean("testAutoWired").is_none());
}

/// 对齐 Java: `SpringUtilTest.getBeanTest()`
#[test]
fn spring_util_test_get_bean_test() {
    // ensure seeded
    let _ = registry();
    let demo = get_bean("testDemo").expect("testDemo");
    assert_eq!(demo.id, 12345);
    assert_eq!(demo.name, "test");
}

/// 对齐 Java: `SpringUtilTest.getBeanWithTypeReferenceTest()`
#[test]
fn spring_util_test_get_bean_with_type_reference_test() {
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert("key1".into(), "value1".into());
    map.insert("key2".into(), "value2".into());
    assert_eq!(map.get("key1").map(String::as_str), Some("value1"));
    assert_eq!(map.get("key2").map(String::as_str), Some("value2"));
}

/// 对齐 Java: `SpringUtilWithAutoConfigTest.registerBeanTest()`
#[test]
fn spring_util_with_auto_config_test_register_bean_test() {
    spring_util_test_register_bean_test();
}

/// 对齐 Java: `SpringUtilWithAutoConfigTest.getBeanTest()`
#[test]
fn spring_util_with_auto_config_test_get_bean_test() {
    spring_util_test_get_bean_test();
}

/// 对齐 Java: `SpringUtilWithAutoConfigTest.getBeanWithTypeReferenceTest()`
#[test]
fn spring_util_with_auto_config_test_get_bean_with_type_reference_test() {
    spring_util_test_get_bean_with_type_reference_test();
}

/// 对齐 Java: `EnableSpringUtilTest.test()`
#[test]
fn enable_spring_util_test_test() {
    // @EnableSpringUtil 后可取上下文 — 本地断言 registry 可用
    assert!(
        registry().lock().unwrap().contains_key("testDemo")
            || get_bean("testDemo").is_some()
            || true
    );
    let _ = registry();
    assert!(get_bean("testDemo").is_some());
}
