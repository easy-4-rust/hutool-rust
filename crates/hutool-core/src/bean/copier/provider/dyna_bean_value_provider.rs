//! 对齐: `cn.hutool.core.bean.copier.provider.DynaBeanValueProvider`
//! 来源: hutool-core/src/main/java/cn/hutool/core/bean/copier/provider/DynaBeanValueProvider.java
//!
//! 中文说明: DynaBean值提供者，从动态 Bean 对象中按字段名读取值。
//! Java 侧通过 DynaBean.get(key) 读取；Rust 侧使用 serde_json::Value Map 实现等效功能。

use std::collections::HashMap;

use serde_json::Value;

use crate::bean::copier::value_provider::{ValueKind, ValueProvider};

/// 对齐 Java 类: `cn.hutool.core.bean.copier.provider.DynaBeanValueProvider`
///
/// 中文说明: DynaBean值提供者。使用 serde_json::Value Map 作为底层存储，
/// 模拟 Java DynaBean 的动态属性访问能力。
pub struct DynaBeanValueProvider {
    /// 动态 Bean 的属性 Map
    data: HashMap<String, Value>,
    /// 是否忽略错误
    ignore_error: bool,
}

impl DynaBeanValueProvider {
    /// 对齐 Java 构造: `DynaBeanValueProvider(DynaBean dynaBean, boolean ignoreError)`
    ///
    /// 中文说明: 构造 DynaBean 值提供者。
    ///
    /// - `data`: 动态 Bean 的属性 Map
    /// - `ignore_error`: 是否忽略错误
    pub fn new(data: HashMap<String, Value>, ignore_error: bool) -> Self {
        Self { data, ignore_error }
    }

    /// 从 serde_json::Value 创建
    pub fn from_value(value: Value, ignore_error: bool) -> Self {
        let data = match value {
            Value::Object(map) => map.into_iter().collect(),
            _ => HashMap::new(),
        };
        Self { data, ignore_error }
    }

    /// 是否忽略错误
    pub fn is_ignore_error(&self) -> bool {
        self.ignore_error
    }
}

impl ValueProvider for DynaBeanValueProvider {
    /// 对齐 Java: `Object value(String key, Type valueType)`
    ///
    /// 中文说明: 获取值。从 Map 中按 key 查找值，返回 ValueKind 表示。
    fn value(&self, key: &str, _value_type: &str) -> Option<ValueKind> {
        match self.data.get(key) {
            Some(v) => Some(json_to_value_kind(v)),
            None => None,
        }
    }

    /// 对齐 Java: `boolean containsKey(String key)`
    ///
    /// 中文说明: 是否包含指定KEY。
    fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// 对齐 Java: 返回所有可枚举的 KEY
    fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
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
    fn dyna_bean_value_provider_basic() {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Value::String("Alice".into()));
        data.insert("age".to_string(), Value::Number(30.into()));
        data.insert("active".to_string(), Value::Bool(true));

        let provider = DynaBeanValueProvider::new(data, false);
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
    fn dyna_bean_value_provider_null() {
        let mut data = HashMap::new();
        data.insert("field".to_string(), Value::Null);

        let provider = DynaBeanValueProvider::new(data, false);
        let v = provider.value("field", "").unwrap();
        assert_eq!(v, ValueKind::Null);
    }

    #[test]
    fn dyna_bean_value_provider_missing_key() {
        let data = HashMap::new();
        let provider = DynaBeanValueProvider::new(data, false);
        assert!(provider.value("missing", "").is_none());
    }

    #[test]
    fn dyna_bean_value_provider_keys() {
        let mut data = HashMap::new();
        data.insert("a".to_string(), Value::Number(1.into()));
        data.insert("b".to_string(), Value::Number(2.into()));
        data.insert("c".to_string(), Value::Number(3.into()));

        let provider = DynaBeanValueProvider::new(data, false);
        let keys = provider.keys();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"a".to_string()));
        assert!(keys.contains(&"b".to_string()));
        assert!(keys.contains(&"c".to_string()));
    }

    #[test]
    fn dyna_bean_value_provider_from_value() {
        let value = serde_json::json!({
            "name": "Bob",
            "age": 25,
            "email": null
        });

        let provider = DynaBeanValueProvider::from_value(value, true);
        assert!(provider.is_ignore_error());
        assert!(provider.contains_key("name"));
        assert!(provider.contains_key("email"));

        let name = provider.value("name", "").unwrap();
        assert_eq!(name, ValueKind::String("Bob".into()));

        let email = provider.value("email", "").unwrap();
        assert_eq!(email, ValueKind::Null);
    }

    #[test]
    fn dyna_bean_value_provider_float() {
        let mut data = HashMap::new();
        data.insert("score".to_string(), Value::from(95.5));

        let provider = DynaBeanValueProvider::new(data, false);
        let score = provider.value("score", "").unwrap();
        assert_eq!(score, ValueKind::Float(95.5));
    }
}
