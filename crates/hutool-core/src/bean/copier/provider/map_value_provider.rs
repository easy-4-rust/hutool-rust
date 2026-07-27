//! 对齐: `cn.hutool.core.bean.copier.provider.MapValueProvider`
//! 来源: hutool-core/src/main/java/cn/hutool/core/bean/copier/provider/MapValueProvider.java
//!
//! 中文说明: Map值提供者，从 Map 对象中按字段名读取值。
//! Java 侧通过 Map.get(key) 读取；Rust 侧使用 HashMap<String, Value> 实现。

use std::collections::HashMap;

use serde_json::Value;

use crate::bean::copier::value_provider::{ValueKind, ValueProvider};

/// 对齐 Java 类: `cn.hutool.core.bean.copier.provider.MapValueProvider`
///
/// 中文说明: Map值提供者。使用 HashMap<String, Value> 作为底层存储，
/// 按 key 查找并返回对应的值。
pub struct MapValueProvider {
    /// Map 数据
    map: HashMap<String, Value>,
}

impl MapValueProvider {
    /// 对齐 Java 构造: `MapValueProvider(Map map)`
    ///
    /// 中文说明: 构造 Map 值提供者。
    ///
    /// - `map`: Map 数据源
    pub fn new(map: HashMap<String, Value>) -> Self {
        Self { map }
    }

    /// 从 serde_json::Value 创建
    pub fn from_value(value: Value) -> Self {
        let map = match value {
            Value::Object(m) => m.into_iter().collect(),
            _ => HashMap::new(),
        };
        Self { map }
    }
}

impl ValueProvider for MapValueProvider {
    /// 对齐 Java: `Object value(String key, Type valueType)`
    ///
    /// 中文说明: 获取值。从 Map 中按 key 查找值。
    fn value(&self, key: &str, _value_type: &str) -> Option<ValueKind> {
        match self.map.get(key) {
            Some(v) => Some(json_to_value_kind(v)),
            None => None,
        }
    }

    /// 对齐 Java: `boolean containsKey(String key)`
    ///
    /// 中文说明: 是否包含指定KEY。
    fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    /// 对齐 Java: 返回所有可枚举的 KEY
    fn keys(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }
}

/// 将 serde_json::Value 转为 ValueKind
fn json_to_value_kind(v: &Value) -> ValueKind {
    match v {
        Value::Null => ValueKind::Null,
        Value::Bool(b) => ValueKind::Bool(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                ValueKind::Int(i)
            } else if let Some(f) = n.as_f64() {
                ValueKind::Float(f)
            } else {
                ValueKind::Other(n.to_string())
            }
        }
        Value::String(s) => ValueKind::String(s.clone()),
        other => ValueKind::Other(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_value_provider_basic() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), Value::String("Alice".into()));
        map.insert("age".to_string(), Value::Number(30.into()));
        map.insert("active".to_string(), Value::Bool(true));

        let provider = MapValueProvider::new(map);
        assert!(provider.contains_key("name"));
        assert!(provider.contains_key("age"));
        assert!(!provider.contains_key("missing"));

        let name = provider.value("name", "").unwrap();
        assert_eq!(name, ValueKind::String("Alice".into()));

        let age = provider.value("age", "").unwrap();
        assert_eq!(age, ValueKind::Int(30));

        let active = provider.value("active", "").unwrap();
        assert_eq!(active, ValueKind::Bool(true));
    }

    #[test]
    fn map_value_provider_null() {
        let mut map = HashMap::new();
        map.insert("field".to_string(), Value::Null);

        let provider = MapValueProvider::new(map);
        let v = provider.value("field", "").unwrap();
        assert_eq!(v, ValueKind::Null);
    }

    #[test]
    fn map_value_provider_missing_key() {
        let map = HashMap::new();
        let provider = MapValueProvider::new(map);
        assert!(provider.value("missing", "").is_none());
    }

    #[test]
    fn map_value_provider_keys() {
        let mut map = HashMap::new();
        map.insert("x".to_string(), Value::Number(1.into()));
        map.insert("y".to_string(), Value::Number(2.into()));

        let provider = MapValueProvider::new(map);
        let keys = provider.keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"x".to_string()));
        assert!(keys.contains(&"y".to_string()));
    }

    #[test]
    fn map_value_provider_from_value() {
        let value = serde_json::json!({
            "name": "Bob",
            "score": 95.5,
            "passed": true
        });

        let provider = MapValueProvider::from_value(value);
        assert!(provider.contains_key("name"));
        assert!(provider.contains_key("score"));

        let name = provider.value("name", "").unwrap();
        assert_eq!(name, ValueKind::String("Bob".into()));

        let score = provider.value("score", "").unwrap();
        assert_eq!(score, ValueKind::Float(95.5));

        let passed = provider.value("passed", "").unwrap();
        assert_eq!(passed, ValueKind::Bool(true));
    }

    #[test]
    fn map_value_provider_string_array() {
        let mut map = HashMap::new();
        map.insert(
            "tags".to_string(),
            Value::Array(vec![
                Value::String("a".into()),
                Value::String("b".into()),
            ]),
        );

        let provider = MapValueProvider::new(map);
        let tags = provider.value("tags", "").unwrap();
        // 数组类型映射为 Other
        assert!(matches!(tags, ValueKind::Other(_)));
    }
}
