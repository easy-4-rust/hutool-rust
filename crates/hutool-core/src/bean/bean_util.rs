//! 对齐: `cn.hutool.core.bean.BeanUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/bean/BeanUtil.java
//!
//! **策略**: 仅提供 Serde / `From` 惯用路径（`beanToMap` / `mapToBean` / `toBean` / `copyProperties`）。
//! Java Introspector / 反射读写字段 = `planned` 或 `unsafe-to-copy`，不在此实现。

#![allow(dead_code, unused_variables, clippy::new_without_default)]

use std::collections::HashMap;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use super::bean_exception::BeanException;

/// 对齐 Java 类: `cn.hutool.core.bean.BeanUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct BeanUtil;

impl BeanUtil {
    /// 对齐桩 sentinel（保留，禁止删除）。
    pub fn pending_alignment() -> &'static str {
        "pending"
    }

    // ── Serde / From 惯用路径 ──

    /// 对齐 Java: `BeanUtil.beanToMap(Object, String...)` — 全量字段；`properties` 过滤可选。
    pub fn bean_to_map<T: Serialize>(
        bean: &T,
        properties: &[&str],
    ) -> Result<HashMap<String, Value>, BeanException> {
        let value = serde_json::to_value(bean).map_err(|e| BeanException::new(e.to_string()))?;
        let mut map = match value {
            Value::Object(o) => o.into_iter().collect::<HashMap<_, _>>(),
            other => {
                let mut m = HashMap::new();
                m.insert("_value".into(), other);
                m
            }
        };
        if !properties.is_empty() {
            map.retain(|k, _| properties.contains(&k.as_str()));
        }
        Ok(map)
    }

    /// 对齐 Java: `BeanUtil.beanToMap(Object, boolean isToUnderlineCase, boolean ignoreNullValue)`
    pub fn bean_to_map_opts<T: Serialize>(
        bean: &T,
        to_underline: bool,
        ignore_null: bool,
    ) -> Result<HashMap<String, Value>, BeanException> {
        let mut map = Self::bean_to_map(bean, &[])?;
        if ignore_null {
            map.retain(|_, v| !v.is_null());
        }
        if to_underline {
            map = map
                .into_iter()
                .map(|(k, v)| (camel_to_underline(&k), v))
                .collect();
        }
        Ok(map)
    }

    /// 对齐 Java: `BeanUtil.mapToBean(Map, Class, boolean)` / Serde 反序列化。
    pub fn map_to_bean<T: DeserializeOwned>(
        map: &HashMap<String, Value>,
        _ignore_error: bool,
    ) -> Result<T, BeanException> {
        let obj = Value::Object(Map::from_iter(
            map.iter().map(|(k, v)| (k.clone(), v.clone())),
        ));
        serde_json::from_value(obj).map_err(|e| BeanException::new(e.to_string()))
    }

    /// 对齐 Java: `BeanUtil.mapToBeanIgnoreCase` — 键小写归一后反序列化。
    pub fn map_to_bean_ignore_case<T: DeserializeOwned>(
        map: &HashMap<String, Value>,
        ignore_error: bool,
    ) -> Result<T, BeanException> {
        let lowered: HashMap<String, Value> = map
            .iter()
            .map(|(k, v)| (k.to_ascii_lowercase(), v.clone()))
            .collect();
        Self::map_to_bean(&lowered, ignore_error)
    }

    /// 对齐 Java: `BeanUtil.fillBeanWithMap(Map, T, ...)` — 合并进已有 Value 再反序列化。
    pub fn fill_bean_with_map<T: Serialize + DeserializeOwned>(
        map: &HashMap<String, Value>,
        bean: &T,
        ignore_error: bool,
    ) -> Result<T, BeanException> {
        let mut base = Self::bean_to_map(bean, &[])?;
        for (k, v) in map {
            if ignore_error && v.is_null() {
                continue;
            }
            base.insert(k.clone(), v.clone());
        }
        Self::map_to_bean(&base, ignore_error)
    }

    /// 对齐 Java: `BeanUtil.toBean(Object, Class)` — 源经 Serialize 再 Deserialize。
    pub fn to_bean<S: Serialize, T: DeserializeOwned>(source: &S) -> Result<T, BeanException> {
        let value = serde_json::to_value(source).map_err(|e| BeanException::new(e.to_string()))?;
        serde_json::from_value(value).map_err(|e| BeanException::new(e.to_string()))
    }

    /// 对齐 Java: `BeanUtil.toBeanIgnoreError` — 失败返回 `T::default`。
    pub fn to_bean_ignore_error<S: Serialize, T: DeserializeOwned + Default>(source: &S) -> T {
        Self::to_bean(source).unwrap_or_default()
    }

    /// 对齐 Java: `BeanUtil.copyProperties(Object, Class, String...)`
    pub fn copy_properties_new<S: Serialize, T: DeserializeOwned>(
        source: &S,
        ignore_properties: &[&str],
    ) -> Result<T, BeanException> {
        let mut map = Self::bean_to_map(source, &[])?;
        for k in ignore_properties {
            map.remove(*k);
        }
        Self::map_to_bean(&map, false)
    }

    /// 对齐 Java: `BeanUtil.copyProperties(Object, Object, String...)` — 合并字段到目标类型。
    pub fn copy_properties_into<S: Serialize, T: Serialize + DeserializeOwned>(
        source: &S,
        target: &T,
        ignore_properties: &[&str],
    ) -> Result<T, BeanException> {
        let mut dst = Self::bean_to_map(target, &[])?;
        let src = Self::bean_to_map(source, &[])?;
        for (k, v) in src {
            if ignore_properties.contains(&k.as_str()) {
                continue;
            }
            dst.insert(k, v);
        }
        Self::map_to_bean(&dst, false)
    }

    /// 对齐 Java: `BeanUtil.copyToList(Collection, Class)`
    pub fn copy_to_list<S: Serialize, T: DeserializeOwned>(
        collection: &[S],
    ) -> Result<Vec<T>, BeanException> {
        collection.iter().map(|s| Self::to_bean(s)).collect()
    }

    /// 对齐 Java: `BeanUtil.isEmpty` / `isNotEmpty` — 经 Serde 后无字段或全 null。
    pub fn is_empty_bean<T: Serialize>(
        bean: &T,
        ignore_field_names: &[&str],
    ) -> Result<bool, BeanException> {
        let map = Self::bean_to_map(bean, &[])?;
        Ok(map.iter().all(|(k, v)| {
            ignore_field_names.contains(&k.as_str())
                || v.is_null()
                || v == &Value::String(String::new())
        }))
    }

    /// 对齐 Java: `BeanUtil.isNotEmpty`
    pub fn is_not_empty_bean<T: Serialize>(
        bean: &T,
        ignore_field_names: &[&str],
    ) -> Result<bool, BeanException> {
        Ok(!Self::is_empty_bean(bean, ignore_field_names)?)
    }

    /// 对齐 Java: `BeanUtil.isCommonFieldsEqual` — 公共键值比较（Serde Map）。
    pub fn is_common_fields_equal<A: Serialize, B: Serialize>(
        source: &A,
        target: &B,
        ignore_properties: &[&str],
    ) -> Result<bool, BeanException> {
        let a = Self::bean_to_map(source, &[])?;
        let b = Self::bean_to_map(target, &[])?;
        for (k, va) in &a {
            if ignore_properties.contains(&k.as_str()) {
                continue;
            }
            if let Some(vb) = b.get(k) {
                if va != vb {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    /// 对齐 Java: `BeanUtil.getFieldName(String)` — getter/setter 名 → 字段名。
    #[must_use]
    pub fn get_field_name(getter_or_setter: &str) -> String {
        let name = getter_or_setter
            .strip_prefix("get")
            .or_else(|| getter_or_setter.strip_prefix("set"))
            .or_else(|| getter_or_setter.strip_prefix("is"))
            .unwrap_or(getter_or_setter);
        let mut chars = name.chars();
        match chars.next() {
            None => String::new(),
            Some(c) => {
                let lower = c.to_ascii_lowercase();
                let rest: String = chars.collect();
                format!("{lower}{rest}")
            }
        }
    }

    /// 对齐 Java: `BeanUtil.isMatchName` — 简单类型名匹配。
    #[must_use]
    pub fn is_match_name(type_name: &str, bean_class_name: &str, is_simple: bool) -> bool {
        if is_simple {
            type_name
                .rsplit("::")
                .next()
                .or_else(|| type_name.rsplit('.').next())
                == bean_class_name
                    .rsplit("::")
                    .next()
                    .or_else(|| bean_class_name.rsplit('.').next())
        } else {
            type_name == bean_class_name
        }
    }
}

/// camelCase → snake_case（下划线）。
fn camel_to_underline(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 4);
    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod bean_util_serde_parity {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
    struct User {
        name: String,
        age: i32,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        city: Option<String>,
    }

    /// 对齐 Java BeanUtil Serde 路径：map↔bean、copy、fieldName。
    #[test]
    fn bean_util_serde_map_copy_and_field_name() {
        let u = User {
            name: "Ada".into(),
            age: 36,
            city: Some("London".into()),
        };
        let map = BeanUtil::bean_to_map(&u, &[]).unwrap();
        assert_eq!(map.get("name").and_then(|v| v.as_str()), Some("Ada"));
        let back: User = BeanUtil::map_to_bean(&map, false).unwrap();
        assert_eq!(back, u);

        let mut other = User {
            name: "Bob".into(),
            age: 1,
            city: None,
        };
        other = BeanUtil::copy_properties_into(&u, &other, &["age"]).unwrap();
        assert_eq!(other.name, "Ada");
        assert_eq!(other.age, 1);

        assert_eq!(BeanUtil::get_field_name("getUserName"), "userName");
        assert!(BeanUtil::is_common_fields_equal(&u, &u, &[]).unwrap());
        assert!(!BeanUtil::is_empty_bean(&u, &[]).unwrap());
    }
}
