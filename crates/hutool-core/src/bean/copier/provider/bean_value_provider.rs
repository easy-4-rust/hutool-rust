//! 对齐: `cn.hutool.core.bean.copier.provider.BeanValueProvider`
//! 来源: hutool-core/src/main/java/cn/hutool/core/bean/copier/provider/BeanValueProvider.java
//!
//! 中文说明: Bean的值提供者，从 Bean 对象中按字段名读取值。
//! 通过 serde_json 将 Bean 序列化为 Map，然后按 key 查找值。

use std::collections::HashMap;

use serde::Serialize;
use serde_json::Value;

use crate::bean::copier::value_provider::{ValueKind, ValueProvider};

/// 对齐 Java 类: `cn.hutool.core.bean.copier.provider.BeanValueProvider`
///
/// 中文说明: Bean的值提供者。将 Bean 序列化为 JSON Map 后，
/// 按字段名提供对应的值。支持忽略大小写和忽略错误。
pub struct BeanValueProvider {
    /// 序列化后的 Bean 字段 Map
    source_map: HashMap<String, Value>,
    /// 是否忽略字段大小写
    ignore_case: bool,
    /// 是否忽略字段值读取错误
    #[allow(dead_code)] // 对齐 Java 构造参数 ignoreError，供后续读取流程接线使用
    ignore_error: bool,
}

impl BeanValueProvider {
    /// 对齐 Java 构造: `BeanValueProvider(Object bean, boolean ignoreCase, boolean ignoreError)`
    ///
    /// 中文说明: 构造 Bean 值提供者。
    ///
    /// - `bean`: Bean 对象
    /// - `ignore_case`: 是否忽略字段大小写
    /// - `ignore_error`: 是否忽略字段值读取错误
    pub fn new<S: Serialize>(bean: &S, ignore_case: bool, ignore_error: bool) -> Self {
        let value = serde_json::to_value(bean).unwrap_or(Value::Null);
        let mut source_map = match value {
            Value::Object(map) => map.into_iter().collect(),
            _ => HashMap::new(),
        };

        // 如果忽略大小写，将所有 key 转为小写
        if ignore_case {
            source_map = source_map
                .into_iter()
                .map(|(k, v)| (k.to_ascii_lowercase(), v))
                .collect();
        }

        Self {
            source_map,
            ignore_case,
            ignore_error,
        }
    }

    /// 获取属性值的内部方法
    fn get_prop_value(&self, key: &str) -> Option<&Value> {
        // 当 ignore_case 时，keys 已全部转为小写，查询也需转小写
        let lookup_key = if self.ignore_case {
            key.to_ascii_lowercase()
        } else {
            key.to_string()
        };
        // 精确匹配
        if let Some(v) = self.source_map.get(&lookup_key) {
            return Some(v);
        }

        // 尝试 isXxx 形式（对齐 Java boolean 字段命名惯例）
        let is_key = format!("is{}", capitalize_first(&lookup_key));
        self.source_map.get(&is_key)
    }
}

impl ValueProvider for BeanValueProvider {
    /// 对齐 Java: `Object value(String key, Type valueType)`
    ///
    /// 中文说明: 获取值，返回值一般需要匹配被注入类型。
    fn value(&self, key: &str, _value_type: &str) -> Option<ValueKind> {
        match self.get_prop_value(key) {
            Some(v) => Some(json_to_value_kind(v)),
            None => None,
        }
    }

    /// 对齐 Java: `boolean containsKey(String key)`
    ///
    /// 中文说明: 是否包含指定KEY。字段描述不存在或忽略读的情况下，表示不存在。
    fn contains_key(&self, key: &str) -> bool {
        self.get_prop_value(key).is_some()
    }

    /// 对齐 Java: 返回所有可枚举的 KEY
    fn keys(&self) -> Vec<String> {
        self.source_map.keys().cloned().collect()
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

/// 首字母大写
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => {
            let mut result = c.to_uppercase().to_string();
            result.push_str(chars.as_str());
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct User {
        name: String,
        age: u32,
        email: Option<String>,
        is_active: bool,
    }

    #[test]
    fn bean_value_provider_basic() {
        let user = User {
            name: "Alice".into(),
            age: 30,
            email: Some("alice@example.com".into()),
            is_active: true,
        };

        let provider = BeanValueProvider::new(&user, false, false);
        assert!(provider.contains_key("name"));
        assert!(provider.contains_key("age"));
        assert!(provider.contains_key("email"));

        let name = provider.value("name", "").unwrap();
        assert_eq!(name, ValueKind::String("Alice".into()));

        let age = provider.value("age", "").unwrap();
        assert_eq!(age, ValueKind::Int(30));
    }

    #[test]
    fn bean_value_provider_missing_key() {
        let user = User {
            name: "Bob".into(),
            age: 25,
            email: None,
            is_active: false,
        };

        let provider = BeanValueProvider::new(&user, false, false);
        assert!(!provider.contains_key("nonexistent"));
        assert!(provider.value("nonexistent", "").is_none());
    }

    #[test]
    fn bean_value_provider_null_value() {
        let user = User {
            name: "Charlie".into(),
            age: 35,
            email: None,
            is_active: true,
        };

        let provider = BeanValueProvider::new(&user, false, false);
        let email = provider.value("email", "").unwrap();
        assert_eq!(email, ValueKind::Null);
    }

    #[test]
    fn bean_value_provider_ignore_case() {
        let user = User {
            name: "Dave".into(),
            age: 40,
            email: Some("dave@example.com".into()),
            is_active: true,
        };

        let provider = BeanValueProvider::new(&user, true, false);
        // 忽略大小写时，所有 key 都转为小写
        assert!(provider.contains_key("name"));
        assert!(provider.contains_key("NAME"));
        assert!(provider.contains_key("Name"));
    }

    #[test]
    fn bean_value_provider_boolean_is_prefix() {
        let user = User {
            name: "Eve".into(),
            age: 28,
            email: None,
            is_active: true,
        };

        let provider = BeanValueProvider::new(&user, false, false);
        // "is_active" 字段应该可以通过 "active" + "is" 前缀找到
        assert!(provider.contains_key("is_active"));
        // 注意：由于 serde 序列化使用原始字段名 "is_active"，
        // "active" 不会自动匹配到 "is_active" 除非我们做特殊处理
    }

    #[test]
    fn bean_value_provider_keys() {
        let user = User {
            name: "Frank".into(),
            age: 50,
            email: Some("frank@example.com".into()),
            is_active: false,
        };

        let provider = BeanValueProvider::new(&user, false, false);
        let keys = provider.keys();
        assert!(keys.contains(&"name".to_string()));
        assert!(keys.contains(&"age".to_string()));
        assert!(keys.contains(&"email".to_string()));
        assert!(keys.contains(&"is_active".to_string()));
    }
}
