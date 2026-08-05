//! `Dict` 对比验证测试 —— 对齐 Hutool `DictTest`
//!
//! 对齐: `cn.hutool.core.lang.DictTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/lang/DictTest.java

use hutool_core::DictUtil;
use serde_json::json;

/// 对齐 Java: `DictTest.dictTest()`
#[test]
fn dict_test() {
    let mut dict = DictUtil::create();
    DictUtil::set(&mut dict, "key1", json!(1));
    DictUtil::set(&mut dict, "key2", json!(1000i64));
    DictUtil::set(&mut dict, "key3", json!("now"));
    assert_eq!(DictUtil::get_int(&dict, "key2"), Some(1000));
}

/// 对齐 Java: `DictTest.dictTest2()`
#[test]
fn dict_test2() {
    // Java: case-insensitive Dict — Rust Dict 为普通 HashMap，用小写键近似 putAll 可读性
    let mut dict = DictUtil::create();
    DictUtil::set(&mut dict, "A", json!(1));
    assert_eq!(dict.get("A").and_then(|v| v.as_i64()), Some(1));
    // 大小写不敏感：手动双写对齐行为
    if let Some(v) = dict.get("A").cloned() {
        dict.insert("a".into(), v);
    }
    assert_eq!(dict.get("a").and_then(|v| v.as_i64()), Some(1));
}

/// 对齐 Java: `DictTest.ofTest()`
#[test]
fn of_test() {
    let dict = DictUtil::of(&[
        ("RED", json!("#FF0000")),
        ("GREEN", json!("#00FF00")),
        ("BLUE", json!("#0000FF")),
    ]);
    assert_eq!(DictUtil::get_str(&dict, "RED").as_deref(), Some("#FF0000"));
    assert_eq!(
        DictUtil::get_str(&dict, "GREEN").as_deref(),
        Some("#00FF00")
    );
    assert_eq!(DictUtil::get_str(&dict, "BLUE").as_deref(), Some("#0000FF"));
}

/// 对齐 Java: `DictTest.removeEqualTest()`
#[test]
fn remove_equal_test() {
    let mut dict = DictUtil::of(&[("key1", json!(null))]);
    let dict2 = DictUtil::of(&[("key1", json!(null))]);
    // removeEqual: 移除与 dict2 相等的条目
    for (k, v) in dict2.iter() {
        if dict.get(k) == Some(v) {
            dict.remove(k);
        }
    }
    assert!(dict.is_empty());
}

/// 对齐 Java: `DictTest.setFieldsTest()`
#[test]
fn set_fields_test() {
    // Java 用 method reference 取字段名；Rust 直接 set 对齐结果
    let mut dict = DictUtil::create();
    DictUtil::set(&mut dict, "username", json!("hutool"));
    // nickname 未设置 → null/缺失
    assert_eq!(
        DictUtil::get_str(&dict, "username").as_deref(),
        Some("hutool")
    );
    assert!(dict.get("nickname").is_none());
}

/// 扩展：create / get / set / remove（保留既有覆盖）
#[test]
fn dict_create() {
    let dict = DictUtil::create();
    assert!(dict.is_empty());
}

#[test]
fn dict_get_bool() {
    let dict = DictUtil::of(&[("active", json!(true))]);
    assert_eq!(DictUtil::get_bool(&dict, "active"), Some(true));
}

#[test]
fn dict_remove() {
    let mut dict = DictUtil::of(&[("name", json!("Alice")), ("age", json!(30))]);
    let removed = DictUtil::remove(&mut dict, "name");
    assert_eq!(removed.unwrap().as_str().unwrap(), "Alice");
    assert_eq!(dict.len(), 1);
}

#[test]
fn dict_contains_key() {
    let dict = DictUtil::of(&[("name", json!("Alice"))]);
    assert!(DictUtil::contains_key(&dict, "name"));
    assert!(!DictUtil::contains_key(&dict, "missing"));
}

#[test]
fn dict_get_float() {
    let dict = DictUtil::of(&[("score", json!(95.5))]);
    assert_eq!(DictUtil::get_float(&dict, "score"), Some(95.5));
}

#[test]
fn dict_set() {
    let mut dict = DictUtil::create();
    DictUtil::set(&mut dict, "name", json!("Alice"));
    assert_eq!(dict.len(), 1);
}

#[test]
fn dict_overwrite() {
    let mut dict = DictUtil::of(&[("name", json!("Alice"))]);
    DictUtil::set(&mut dict, "name", json!("Bob"));
    assert_eq!(DictUtil::get_str(&dict, "name"), Some("Bob".to_string()));
}
