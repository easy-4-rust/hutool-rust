//! 对齐: `cn.hutool.core.lang.Dict`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Dict.java
//!
//! Rust 以 `HashMap<String, serde_json::Value>` 表达 Hutool Dict 动态袋；
//! Serde 路径覆盖 `parse` / `toBean`，反射字段注入保持 planned。

use std::collections::HashMap;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use super::dict::Dict;

/// 对齐 Java: `cn.hutool.core.lang.Dict` 工厂与取值门面。
#[derive(Debug, Clone, Copy, Default)]
pub struct DictUtil;

impl DictUtil {
    /// 对齐 Java: `Dict.create()`
    #[must_use]
    pub fn create() -> Dict {
        HashMap::new()
    }

    /// 对齐 Java: `Dict.of(Pair...)` / 键值对批量构造。
    #[must_use]
    pub fn of(pairs: &[(&str, Value)]) -> Dict {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), v.clone()))
            .collect()
    }

    /// 对齐 Java: `Dict.parse(T)` / `parseBean` — Serde 序列化为 Dict。
    pub fn parse<T: Serialize>(bean: &T) -> Result<Dict, serde_json::Error> {
        let value = serde_json::to_value(bean)?;
        Ok(match value {
            Value::Object(map) => map.into_iter().collect(),
            other => {
                let mut d = Dict::new();
                d.insert("_value".into(), other);
                d
            }
        })
    }

    /// 对齐 Java: `Dict.toBean(T)` / `toBeanIgnoreCase` / `toBeanWithCamelCase` 的 Serde 路径。
    pub fn to_bean<T: DeserializeOwned>(dict: &Dict) -> Result<T, serde_json::Error> {
        serde_json::from_value(Value::Object(
            dict.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        ))
    }

    /// 对齐 Java: `Dict.set(String, Object)`
    pub fn set(dict: &mut Dict, key: &str, value: Value) {
        dict.insert(key.to_string(), value);
    }

    /// 对齐 Java: `Dict.setIgnoreNull(String, Object)`
    pub fn set_ignore_null(dict: &mut Dict, key: &str, value: Option<Value>) {
        if let Some(v) = value {
            if !v.is_null() {
                dict.insert(key.to_string(), v);
            }
        }
    }

    /// 对齐 Java: `Dict.put` / `putIfAbsent` / `putAll` 语义门面。
    pub fn put(dict: &mut Dict, key: impl Into<String>, value: Value) -> Option<Value> {
        dict.insert(key.into(), value)
    }

    /// 对齐 Java: `Dict.putIfAbsent`
    pub fn put_if_absent(dict: &mut Dict, key: impl Into<String>, value: Value) -> Option<Value> {
        use std::collections::hash_map::Entry;
        match dict.entry(key.into()) {
            Entry::Occupied(o) => Some(o.get().clone()),
            Entry::Vacant(v) => {
                v.insert(value);
                None
            }
        }
    }

    /// 对齐 Java: `Dict.putAll`
    pub fn put_all(dict: &mut Dict, other: &Dict) {
        for (k, v) in other {
            dict.insert(k.clone(), v.clone());
        }
    }

    /// 对齐 Java: `Dict.filter(String... keys)`
    #[must_use]
    pub fn filter(dict: &Dict, keys: &[&str]) -> Dict {
        keys.iter()
            .filter_map(|k| dict.get(*k).map(|v| ((*k).to_string(), v.clone())))
            .collect()
    }

    /// 对齐 Java: `Dict.removeEqual(T, String...)` — 移除与另一 Dict 等值的键。
    pub fn remove_equal(dict: &mut Dict, other: &Dict, without_names: &[&str]) {
        let skip: std::collections::HashSet<&str> = without_names.iter().copied().collect();
        let keys: Vec<String> = dict
            .keys()
            .filter(|k| !skip.contains(k.as_str()))
            .cloned()
            .collect();
        for k in keys {
            if other.get(&k) == dict.get(&k) {
                dict.remove(&k);
            }
        }
    }

    /// 对齐 Java: `Dict.remove(String)`
    pub fn remove(dict: &mut Dict, key: &str) -> Option<Value> {
        dict.remove(key)
    }

    /// 对齐 Java: `Dict.containsKey(String)`
    #[must_use]
    pub fn contains_key(dict: &Dict, key: &str) -> bool {
        dict.contains_key(key)
    }

    /// 对齐 Java: `Dict.getObj(String)`
    #[must_use]
    pub fn get_obj<'a>(dict: &'a Dict, key: &str) -> Option<&'a Value> {
        dict.get(key)
    }

    /// 对齐 Java: `Dict.get(String, T defaultValue)`
    #[must_use]
    pub fn get_or(dict: &Dict, key: &str, default: Value) -> Value {
        dict.get(key).cloned().unwrap_or(default)
    }

    /// 对齐 Java: `Dict.getOrDefault`
    #[must_use]
    pub fn get_or_default(dict: &Dict, key: &str, default: Value) -> Value {
        Self::get_or(dict, key, default)
    }

    /// 对齐 Java: `Dict.getStr(String)`
    #[must_use]
    pub fn get_str(dict: &Dict, key: &str) -> Option<String> {
        dict.get(key).map(|v| match v {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        })
    }

    /// 对齐 Java: `Dict.getInt(String)`
    #[must_use]
    pub fn get_int(dict: &Dict, key: &str) -> Option<i64> {
        dict.get(key).and_then(|v| v.as_i64())
    }

    /// 对齐 Java: `Dict.getLong(String)`
    #[must_use]
    pub fn get_long(dict: &Dict, key: &str) -> Option<i64> {
        Self::get_int(dict, key)
    }

    /// 对齐 Java: `Dict.getFloat` / `getDouble` / `getNumber`
    #[must_use]
    pub fn get_float(dict: &Dict, key: &str) -> Option<f64> {
        dict.get(key).and_then(|v| v.as_f64().or_else(|| v.as_i64().map(|i| i as f64)))
    }

    /// 对齐 Java: `Dict.getDouble(String)`
    #[must_use]
    pub fn get_double(dict: &Dict, key: &str) -> Option<f64> {
        Self::get_float(dict, key)
    }

    /// 对齐 Java: `Dict.getBool(String)`
    #[must_use]
    pub fn get_bool(dict: &Dict, key: &str) -> Option<bool> {
        dict.get(key).and_then(|v| v.as_bool())
    }

    /// 对齐 Java: `Dict.getByte` / `getShort` / `getChar` — 数值窄化。
    #[must_use]
    pub fn get_byte(dict: &Dict, key: &str) -> Option<u8> {
        Self::get_int(dict, key).and_then(|n| u8::try_from(n).ok())
    }

    /// 对齐 Java: `Dict.getShort(String)`
    #[must_use]
    pub fn get_short(dict: &Dict, key: &str) -> Option<i16> {
        Self::get_int(dict, key).and_then(|n| i16::try_from(n).ok())
    }

    /// 对齐 Java: `Dict.getChar(String)`
    #[must_use]
    pub fn get_char(dict: &Dict, key: &str) -> Option<char> {
        Self::get_str(dict, key).and_then(|s| s.chars().next())
    }

    /// 对齐 Java: `Dict.getBigDecimal` / `getBigInteger` — 用字符串/数字表达。
    #[must_use]
    pub fn get_big_decimal(dict: &Dict, key: &str) -> Option<String> {
        dict.get(key).map(|v| match v {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            other => other.to_string(),
        })
    }

    /// 对齐 Java: `Dict.getBigInteger(String)`
    #[must_use]
    pub fn get_big_integer(dict: &Dict, key: &str) -> Option<i128> {
        dict.get(key).and_then(|v| {
            v.as_i64()
                .map(i128::from)
                .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
        })
    }

    /// 对齐 Java: `Dict.getDate` / `getTime` / `getTimestamp` — 毫秒时间戳。
    #[must_use]
    pub fn get_date_ms(dict: &Dict, key: &str) -> Option<i64> {
        Self::get_long(dict, key)
    }

    /// 对齐 Java: `Dict.getByPath(String)` — 点分路径浅层取值。
    #[must_use]
    pub fn get_by_path<'a>(dict: &'a Dict, path: &str) -> Option<&'a Value> {
        let mut parts = path.split('.');
        let first = parts.next()?;
        let mut cur = dict.get(first)?;
        for part in parts {
            cur = cur.as_object()?.get(part)?;
        }
        Some(cur)
    }

    /// 对齐 Java: `Dict.getBean(String)` — 子对象反序列化。
    pub fn get_bean<T: DeserializeOwned>(dict: &Dict, key: &str) -> Result<Option<T>, serde_json::Error> {
        match dict.get(key) {
            None | Some(Value::Null) => Ok(None),
            Some(v) => Ok(Some(serde_json::from_value(v.clone())?)),
        }
    }

    /// 对齐 Java: `Dict.replace` / `compute` / `merge` 的 HashMap 语义包装。
    pub fn replace(dict: &mut Dict, key: impl Into<String>, value: Value) -> Option<Value> {
        let key = key.into();
        if dict.contains_key(&key) {
            dict.insert(key, value)
        } else {
            None
        }
    }

    /// 对齐 Java: `Dict.clone()`
    #[must_use]
    pub fn clone_dict(dict: &Dict) -> Dict {
        dict.clone()
    }
}
