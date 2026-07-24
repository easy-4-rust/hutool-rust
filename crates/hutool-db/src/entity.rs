//! Hutool-aligned `Entity` row / condition map.

use crate::sql::condition::{Condition, ConditionValue};
use indexmap::IndexMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{Map, Value};

/// Hutool 兼容的不可变键值实体。
///
/// 对齐 Java 类: `cn.hutool.db.Entity`
/// 来源: hutool-db/src/main/java/cn/hutool/db/Entity.java
///
/// 数据库实体：行载体和 WHERE 条件映射。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Entity {
    table_name: Option<String>,
    field_names: Vec<String>,
    fields: IndexMap<String, ConditionValue>,
    case_insensitive: bool,
}

impl Entity {
    /// Creates an empty entity without a table name.
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// Creates an entity bound to `table_name`.
    #[must_use]
    pub fn create_table(table_name: impl Into<String>) -> Self {
        Self {
            table_name: Some(table_name.into()),
            ..Self::default()
        }
    }

    /// Sets the table name.
    pub fn set_table_name(&mut self, table_name: impl Into<String>) -> &mut Self {
        self.table_name = Some(table_name.into());
        self
    }

    /// Returns the table name if set.
    #[must_use]
    pub fn table_name(&self) -> Option<&str> {
        self.table_name.as_deref()
    }

    /// 对齐 Java: `Entity.setFieldNames(String...)`.
    pub fn set_field_names(&mut self, fields: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.field_names = fields.into_iter().map(|f| f.into()).collect();
        self
    }

    /// 对齐 Java: `Entity.getFieldNames()`.
    #[must_use]
    pub fn field_names(&self) -> &[String] {
        &self.field_names
    }

    /// Sets a field value (chainable).
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<ConditionValue>) -> &mut Self {
        let key = key.into();
        let store_key = if self.case_insensitive {
            key.to_ascii_lowercase()
        } else {
            key
        };
        self.fields.insert(store_key, value.into());
        self
    }

    /// Sets a raw JSON value.
    pub fn set_value(&mut self, key: impl Into<String>, value: impl Into<Value>) -> &mut Self {
        self.set(key, ConditionValue::Raw(value.into()))
    }

    /// Sets a Condition value.
    pub fn set_condition(&mut self, key: impl Into<String>, condition: Condition) -> &mut Self {
        self.set(key, ConditionValue::Condition(condition))
    }

    /// Builder-style set that consumes and returns `Self`.
    #[must_use]
    pub fn with(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.set_value(key, value);
        self
    }

    /// Returns whether the map is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Returns field count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns true when `key` is present.
    #[must_use]
    pub fn contains_key(&self, key: &str) -> bool {
        if self.case_insensitive {
            self.fields.contains_key(&key.to_ascii_lowercase())
        } else {
            self.fields.contains_key(key)
                || self
                    .fields
                    .keys()
                    .any(|k| k.eq_ignore_ascii_case(key))
        }
    }

    /// Returns a field value.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        let entry = self.fields.get(key).or_else(|| {
            self.fields
                .iter()
                .find(|(k, _)| k.eq_ignore_ascii_case(key))
                .map(|(_, v)| v)
        })?;
        match entry {
            ConditionValue::Raw(v) => Some(v),
            ConditionValue::Condition(c) => Some(c.value()),
        }
    }

    /// Returns field as `i64` when possible.
    #[must_use]
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key).and_then(value_to_int)
    }

    /// 对齐 Java: `Entity.getLong(String)`.
    #[must_use]
    pub fn get_long(&self, key: &str) -> Option<i64> {
        self.get_int(key)
    }

    /// Returns field as owned `String` when possible.
    #[must_use]
    pub fn get_str(&self, key: &str) -> Option<String> {
        self.get(key).and_then(|v| match v {
            Value::String(s) => Some(s.clone()),
            Value::Null => None,
            other => Some(other.to_string().trim_matches('"').to_string()),
        })
    }

    /// Iterates fields in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> + '_ {
        self.fields.iter().filter_map(|(k, v)| match v {
            ConditionValue::Raw(value) => Some((k, value)),
            ConditionValue::Condition(condition) => Some((k, condition.value())),
        })
    }

    /// Iterates fields including Condition objects.
    pub fn iter_conditions(&self) -> impl Iterator<Item = (&String, &ConditionValue)> + '_ {
        self.fields.iter()
    }

    /// 对齐 Java: `Entity.parseBean(T bean)`.
    pub fn parse_bean<T: Serialize>(&mut self, bean: &T) -> Result<&mut Self, serde_json::Error> {
        self.parse_bean_with(bean, std::any::type_name::<T>(), false, false)?;
        Ok(self)
    }

    /// Parses a serializable bean into an entity.
    pub fn parse_bean_with<T: Serialize>(
        &mut self,
        bean: &T,
        type_name: &str,
        to_underline: bool,
        ignore_null: bool,
    ) -> Result<&mut Self, serde_json::Error> {
        let value = serde_json::to_value(bean)?;
        let Value::Object(map) = value else {
            return Ok(self);
        };
        if self.table_name.is_none() {
            let simple = type_name
                .rsplit("::")
                .next()
                .unwrap_or(type_name)
                .to_ascii_lowercase();
            self.table_name = Some(simple);
        }
        for (k, v) in map {
            if ignore_null && v.is_null() {
                continue;
            }
            let key = if to_underline {
                camel_to_underline(&k)
            } else {
                k
            };
            self.fields.insert(key, ConditionValue::Raw(v));
        }
        Ok(self)
    }

    /// Converts entity fields into a typed bean using case-insensitive key matching.
    pub fn to_bean_ignore_case<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        let mut map = Map::new();
        for (k, v) in &self.fields {
            let value = match v {
                ConditionValue::Raw(value) => value.clone(),
                ConditionValue::Condition(condition) => condition.value().clone(),
            };
            map.insert(k.clone(), value.clone());
            map.insert(k.to_ascii_lowercase(), value.clone());
            map.insert(k.to_ascii_uppercase(), value);
        }
        serde_json::from_value(Value::Object(map))
    }

    /// Builds an entity from an ordered field list.
    #[must_use]
    pub fn from_fields(
        table_name: Option<String>,
        fields: impl IntoIterator<Item = (String, Value)>,
    ) -> Self {
        let mut entity = Self {
            table_name,
            ..Self::default()
        };
        for (k, v) in fields {
            entity.fields.insert(k, ConditionValue::Raw(v));
        }
        entity
    }

    /// 对齐 Java: `SqlBuilder.validateEntity(Entity)`.
    pub fn validate_for_write(&self) {
        assert!(
            self.table_name.as_ref().is_some_and(|t| !t.is_empty()),
            "Entity table name is blank"
        );
        assert!(!self.is_empty(), "Entity has no fields");
    }
}

/// Converts camelCase / PascalCase to snake_case.
fn camel_to_underline(input: &str) -> String {
    let mut out = String::new();
    for (i, ch) in input.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.extend(ch.to_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn value_to_int(v: &Value) -> Option<i64> {
    match v {
        Value::Number(n) => n.as_i64().or_else(|| n.as_f64().map(|f| f as i64)),
        Value::String(s) => s.parse().ok(),
        Value::Bool(b) => Some(i64::from(*b)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Serialize, Deserialize)]
    struct User {
        id: Option<i32>,
        name: String,
    }

    #[test]
    fn parse_and_to_bean_roundtrip() {
        let user = User {
            id: Some(1),
            name: "test".into(),
        };
        let mut entity = Entity::create_table("testTable");
        entity.parse_bean(&user).unwrap();
        assert_eq!(entity.get_int("id"), Some(1));
        assert_eq!(entity.get_str("name").as_deref(), Some("test"));

        let mut entity2 = Entity::create();
        entity2.parse_bean(&user).unwrap();
        assert_eq!(entity2.table_name(), Some("user"));

        let mut entity3 = Entity::create();
        entity3
            .parse_bean_with(
                &User {
                    id: None,
                    name: "test".into(),
                },
                "User",
                false,
                true,
            )
            .unwrap();
        assert!(!entity3.contains_key("id"));

        let from = Entity::create()
            .with("ID", 2)
            .with("NAME", "testName");
        let bean: User = from.to_bean_ignore_case().unwrap();
        assert_eq!(bean.id, Some(2));
        assert_eq!(bean.name, "testName");
    }
}
