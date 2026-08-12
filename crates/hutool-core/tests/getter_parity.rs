//! getter module parity tests
//! 对齐: hutool-core Getter tests

use hutool_core::{BasicTypeGetter, OptBasicTypeGetter, StringMapGetter};

// ── StringMapGetter ──

#[test]
fn string_map_getter_raw() {
    let getter = StringMapGetter::new(vec![
        ("name".to_string(), "Alice".to_string()),
        ("age".to_string(), "30".to_string()),
    ]);
    assert_eq!(getter.raw(&"name"), Some("Alice"));
    assert_eq!(getter.raw(&"age"), Some("30"));
    assert_eq!(getter.raw(&"missing"), None);
}

#[test]
fn string_map_getter_get_typed() {
    let getter = StringMapGetter::new(vec![("age".to_string(), "30".to_string())]);
    let age: Option<i32> = getter.get(&"age");
    assert_eq!(age, Some(30));
    let missing: Option<i32> = getter.get(&"missing");
    assert_eq!(missing, None);
}

#[test]
fn string_map_getter_get_or() {
    let getter = StringMapGetter::new(vec![("age".to_string(), "30".to_string())]);
    let age: i32 = getter.get_or(&"age", 0);
    assert_eq!(age, 30);
    let missing: i32 = getter.get_or(&"missing", 0);
    assert_eq!(missing, 0);
}
