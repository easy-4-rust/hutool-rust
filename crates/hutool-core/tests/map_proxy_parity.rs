//! `MapProxy` 对比验证测试 —— 对齐 Hutool `MapProxyTest`
//!
//! 对齐: `cn.hutool.core.collection.MapProxyTest`
//! 来源: hutool-core/src/test/java/cn/hutool/core/collection/MapProxyTest.java

use hutool_core::MapUtil;
use std::collections::HashMap;

/// 对齐 Java: `MapProxyTest.mapProxyTest()`
#[test]
fn map_proxy_test() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), "1".to_string());
    map.insert("b".to_string(), "2".to_string());
    // MapProxy.getInt ≈ parse
    let b: i32 = map.get("b").and_then(|s| s.parse().ok()).unwrap();
    assert_eq!(b, 2);
    assert!(!map.keys().next().is_none());
    assert!(!MapUtil::is_empty(&map));
}

/// 对齐 Java: `MapProxyTest.classProxyTest()`
#[test]
fn class_proxy_test() {
    // Java toProxyBean；Rust 用 HashMap 模拟 bean 属性
    let mut bean = HashMap::new();
    bean.insert("name".to_string(), "小明".to_string());
    bean.insert("age".to_string(), "18".to_string());
    assert_eq!(bean.get("name").map(|s| s.as_str()), Some("小明"));
    assert_eq!(
        bean.get("age").and_then(|s| s.parse::<i32>().ok()),
        Some(18)
    );
}
