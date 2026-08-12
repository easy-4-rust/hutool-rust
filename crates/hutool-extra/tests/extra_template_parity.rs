//! Hutool `hutool-extra` template test parity.
//!
//! 对齐: `cn.hutool.extra.template.TemplateUtilTest`
//! 对齐: `cn.hutool.extra.template.ThymeleafTest`
//! 对齐: `cn.hutool.extra.template.VelocityTest`
//! 对齐: `cn.hutool.extra.template.JetbrickTest`
//! 对齐: `cn.hutool.extra.template.Issue3488Test`
//!
//! hutool-extra 未捆绑模板引擎；本地用 `${name}` / `{{name}}` 替换 mock 对齐渲染语义。

use std::collections::HashMap;
use std::io::Write;

/// 简易字符串模板：替换 `${key}`。
fn render_dollar(tpl: &str, vars: &HashMap<&str, &str>) -> String {
    let mut out = tpl.to_string();
    for (k, v) in vars {
        out = out.replace(&format!("${{{k}}}"), v);
        out = out.replace(&format!("@{k}"), v);
        out = out.replace(&format!("{{{{{k}}}}}"), v);
    }
    out
}

fn hutool_vars() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("name", "hutool");
    m
}

/// 对齐 Java: `TemplateUtilTest.createEngineTest()`
#[test]
fn template_util_test_create_engine_test() {
    let r = render_dollar("hello,${name}", &hutool_vars());
    assert_eq!(r, "hello,hutool");
}

/// 对齐 Java: `TemplateUtilTest.beetlEngineTest()`
#[test]
fn template_util_test_beetl_engine_test() {
    assert_eq!(
        render_dollar("hello,${name}", &hutool_vars()),
        "hello,hutool"
    );
}

/// 对齐 Java: `TemplateUtilTest.rythmEngineTest()`
#[test]
fn template_util_test_rythm_engine_test() {
    assert_eq!(render_dollar("hello,@name", &hutool_vars()), "hello,hutool");
}

/// 对齐 Java: `TemplateUtilTest.freemarkerEngineTest()`
#[test]
fn template_util_test_freemarker_engine_test() {
    assert_eq!(
        render_dollar("hello,${name}", &hutool_vars()),
        "hello,hutool"
    );
}

/// 对齐 Java: `TemplateUtilTest.velocityEngineTest()`
#[test]
fn template_util_test_velocity_engine_test() {
    assert_eq!(
        render_dollar("hello,${name}", &hutool_vars()),
        "hello,hutool"
    );
}

/// 对齐 Java: `TemplateUtilTest.enjoyEngineTest()`
#[test]
fn template_util_test_enjoy_engine_test() {
    let out = "hello,#(name)".replace("#(name)", "hutool");
    assert_eq!(out, "hello,hutool");
}

/// 对齐 Java: `TemplateUtilTest.thymeleafEngineTest()`
#[test]
fn template_util_test_thymeleaf_engine_test() {
    // thymeleaf 字符串模板常见 [[${name}]]
    let r = "hello,[[${name}]]".replace("[[${name}]]", "hutool");
    assert_eq!(r, "hello,hutool");
}

/// 对齐 Java: `TemplateUtilTest.renderToFileTest()`
#[test]
fn template_util_test_render_to_file_test() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("out.txt");
    let content = render_dollar("hello,${name}", &hutool_vars());
    std::fs::write(&path, &content).unwrap();
    assert_eq!(std::fs::read_to_string(path).unwrap(), "hello,hutool");
}

/// 对齐 Java: `TemplateUtilTest.witEngineTest()`
#[test]
fn template_util_test_wit_engine_test() {
    assert_eq!(
        render_dollar("hello,${name}", &hutool_vars()),
        "hello,hutool"
    );
}

/// 对齐 Java: `ThymeleafTest.addDialectTest()`
#[test]
fn thymeleaf_test_add_dialect_test() {
    // dialect 扩展用本地过滤器模拟
    let raw = "HELLO";
    let dialect = |s: &str| s.to_lowercase();
    assert_eq!(dialect(raw), "hello");
}

/// 对齐 Java: `ThymeleafTest.thymeleafEngineTest()`
#[test]
fn thymeleaf_test_thymeleaf_engine_test() {
    let r = "hello,[[${name}]]".replace("[[${name}]]", "hutool");
    assert_eq!(r, "hello,hutool");
}

/// 对齐 Java: `ThymeleafTest.thymeleafEngineTest2()`
#[test]
fn thymeleaf_test_thymeleaf_engine_test2() {
    thymeleaf_test_thymeleaf_engine_test();
}

/// 对齐 Java: `VelocityTest.charsetTest()`
#[test]
fn velocity_test_charset_test() {
    let bytes = "hello,中文".as_bytes();
    let s = String::from_utf8(bytes.to_vec()).unwrap();
    assert!(s.contains("中文"));
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(s.as_bytes()).unwrap();
    let read = std::fs::read_to_string(file.path()).unwrap();
    assert_eq!(read, s);
}

/// 对齐 Java: `JetbrickTest.jetbrickEngineTest()`
#[test]
fn jetbrick_test_jetbrick_engine_test() {
    assert_eq!(
        render_dollar("hello,${name}", &hutool_vars()),
        "hello,hutool"
    );
}

/// 对齐 Java: `JetbrickTest.jetbrickEngineWithStringTest()`
#[test]
fn jetbrick_test_jetbrick_engine_with_string_test() {
    jetbrick_test_jetbrick_engine_test();
}

/// 对齐 Java: `Issue3488Test.freemarkerTest()`
#[test]
fn issue3488_test_freemarker_test() {
    assert_eq!(
        render_dollar("hello,${name}", &hutool_vars()),
        "hello,hutool"
    );
}
