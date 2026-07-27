//! 对齐: `cn.hutool.json` 包中的核心类型
//! 来源: hutool-json/src/main/java/cn/hutool/json/JSONObject.java, JSONArray.java, JSONConfig.java, JSONNull.java
//! 中文说明: 提供 Hutool 兼容的 JSONObject、JSONArray、JSONConfig、JSONNull 以及 JSON 路径操作。

mod json_config;
mod json_null;

use std::{fmt, ops::Index};

use serde::Serialize;
use serde_json::{Map, Number, Value};

use crate::{JsonError, Result};

pub use self::json_config::JSONConfig;
pub use self::json_null::JSONNull;

/// 对齐: `cn.hutool.json.JSONPath`
/// 中文说明: JSON 路径解析或遍历时产生的错误。
///
/// Error raised by JSON path parsing or traversal.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PathError {
    /// 中文说明: 路径语法无效。
    #[error("invalid JSON path: {0}")]
    Invalid(String),
    /// 中文说明: 遍历遇到形状不兼容的值。
    #[error("cannot traverse JSON path segment: {0}")]
    Type(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PathToken {
    Key(String),
    Index(usize),
}

fn parse_path(path: &str) -> std::result::Result<Vec<PathToken>, PathError> {
    let path = path.strip_prefix('$').unwrap_or(path);
    let bytes = path.as_bytes();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'.' {
            index += 1;
            continue;
        }
        if bytes[index] == b'[' {
            let end = path[index + 1..]
                .find(']')
                .map(|offset| index + 1 + offset)
                .ok_or_else(|| PathError::Invalid(path.to_owned()))?;
            let value = path[index + 1..end]
                .parse::<usize>()
                .map_err(|_| PathError::Invalid(path.to_owned()))?;
            tokens.push(PathToken::Index(value));
            index = end + 1;
            continue;
        }
        let end = path[index..]
            .find(['.', '['])
            .map_or(bytes.len(), |offset| index + offset);
        tokens.push(PathToken::Key(path[index..end].to_owned()));
        index = end;
    }
    Ok(tokens)
}

/// 中文说明: 使用点号和数组索引符号借用嵌套值。
/// 对齐 Java 方法: `cn.hutool.json.JSONUtil.getByPath`
pub fn get_by_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let tokens = parse_path(path).ok()?;
    let mut current = value;
    for token in tokens {
        current = match token {
            PathToken::Key(key) => current.get(key)?,
            PathToken::Index(index) => current.get(index)?,
        };
    }
    Some(current)
}

/// 中文说明: 写入嵌套值，自动创建缺失的对象和数组。
/// 对齐 Java 方法: `cn.hutool.json.JSONUtil.putByPath`
pub fn put_by_path(value: &mut Value, path: &str, replacement: Value) -> Result<()> {
    let tokens = parse_path(path)?;
    if tokens.is_empty() {
        *value = replacement;
        return Ok(());
    }
    let mut current = value;
    for (position, token) in tokens.iter().enumerate() {
        let last = position + 1 == tokens.len();
        match token {
            PathToken::Key(key) => {
                if current.is_null() {
                    *current = Value::Object(Map::new());
                }
                let Value::Object(object) = current else {
                    return Err(PathError::Type(key.clone()).into());
                };
                if last {
                    object.insert(key.clone(), replacement.clone());
                    break;
                }
                current = object.entry(key.clone()).or_insert(Value::Null);
            }
            PathToken::Index(index) => {
                if current.is_null() {
                    *current = Value::Array(Vec::new());
                }
                let Value::Array(array) = current else {
                    return Err(PathError::Type(index.to_string()).into());
                };
                if array.len() <= *index {
                    array.resize(*index + 1, Value::Null);
                }
                if last {
                    array[*index] = replacement.clone();
                    break;
                }
                current = &mut array[*index];
            }
        }
    }
    Ok(())
}

/// 对齐: `cn.hutool.json.JSON` (JSONObject/JSONArray 共有行为)
/// 中文说明: JSON 对象和数组包装器共享的公共行为。
///
/// Common behavior shared by JSON object and array wrappers.
pub trait JsonContainer: Clone + fmt::Display {
    /// 中文说明: 返回容器的配置。
    fn config(&self) -> &JSONConfig;
    /// 中文说明: 返回动态 JSON 表示的副本。
    fn to_value(&self) -> Value;
    /// 中文说明: 使用可选缩进宽度序列化容器。
    fn to_json_string(&self, indent: usize) -> Result<String> {
        if indent == 0 {
            crate::to_string(&self.to_value())
        } else {
            crate::to_string_pretty(&self.to_value())
        }
    }
}

/// 可变的 JSON 对象，兼容 Hutool。
///
/// 对齐 Java 类: `cn.hutool.json.JSONObject`'s `JSONObject`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JSONObject {
    entries: Map<String, Value>,
    config: JSONConfig,
}

impl Default for JSONObject {
    fn default() -> Self {
        Self::new()
    }
}

impl JSONObject {
    pub(crate) fn from_entries(entries: Map<String, Value>, config: JSONConfig) -> Self {
        Self { entries, config }
    }
    /// 中文说明: 创建空的 JSON 对象。
    /// 对齐 Java 方法: `new JSONObject()`
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(JSONConfig::default())
    }

    /// 中文说明: 使用指定配置创建空的 JSON 对象。
    /// 对齐 Java 方法: `new JSONObject(JSONConfig)`
    #[must_use]
    pub fn with_config(config: JSONConfig) -> Self {
        Self {
            entries: Map::new(),
            config,
        }
    }

    /// 中文说明: 将可序列化的 Rust 值转换为 JSON 对象。
    /// 对齐 Java 方法: `new JSONObject(Object)`
    pub fn from_serializable<T: Serialize + ?Sized>(value: &T) -> Result<Self> {
        let value = serde_json::to_value(value)?;
        Self::from_value(value, JSONConfig::default())
    }

    /// 中文说明: 从动态值构建 JSON 对象。
    /// 对齐 Java 方法: `new JSONObject(JSONTokener)`
    pub fn from_value(value: Value, config: JSONConfig) -> Result<Self> {
        match value {
            Value::Object(entries) => Ok(Self { entries, config }),
            value => Err(JsonError::UnexpectedType {
                expected: "object",
                actual: value_type(&value),
            }),
        }
    }

    /// 中文说明: 解析 JSON 文本为对象。
    /// 对齐 Java 方法: `new JSONObject(String)`
    pub fn parse(input: &str) -> Result<Self> {
        Self::from_value(crate::parse(input)?, JSONConfig::default())
    }

    /// 中文说明: 返回字段数量。
    /// 对齐 Java 方法: `size`
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// 中文说明: 判断对象是否为空。
    /// 对齐 Java 方法: `isEmpty`
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 中文说明: 获取字段值，支持配置的 ASCII 大小写不敏感查找。
    /// 对齐 Java 方法: `get`
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries.get(key).or_else(|| {
            self.config.is_ignore_case().then(|| {
                self.entries
                    .iter()
                    .find(|(candidate, _)| candidate.eq_ignore_ascii_case(key))
                    .map(|(_, value)| value)
            })?
        })
    }

    /// 中文说明: 获取字段值，不存在时返回默认值。
    /// 对齐 Java 方法: `getOrDefault`
    #[must_use]
    pub fn get_or<'a>(&'a self, key: &str, default: &'a Value) -> &'a Value {
        self.get(key).unwrap_or(default)
    }

    /// 中文说明: 根据配置插入或替换字段。
    /// 对齐 Java 方法: `set`
    pub fn set(&mut self, key: &str, value: Value) -> Result<&mut Self> {
        if self.config.is_ignore_null_value() && value.is_null() {
            self.entries.remove(key);
            return Ok(self);
        }
        if self.config.is_check_duplicate() && self.entries.contains_key(key) {
            return Err(PathError::Invalid(format!("duplicate key: {key}")).into());
        }
        self.entries
            .insert(key.to_owned(), normalize_value(value, &self.config));
        Ok(self)
    }

    /// 中文说明: 仅在键非空且值非 null 时插入字段。
    /// 对齐 Java 方法: `putOpt`
    pub fn put_opt(&mut self, key: &str, value: Value) -> Result<&mut Self> {
        if !key.is_empty() && !value.is_null() {
            self.set(key, value)?;
        }
        Ok(self)
    }

    /// 中文说明: 仅在字段不存在时插入（防止重复键）。
    /// 对齐 Java 方法: `putOnce`
    pub fn put_once(&mut self, key: &str, value: Value) -> Result<&mut Self> {
        if self.entries.contains_key(key) {
            return Err(PathError::Invalid(format!("duplicate key: {key}")).into());
        }
        self.set(key, value)
    }

    /// 中文说明: 将重复值累积到数组中。
    /// 对齐 Java 方法: `accumulate`
    pub fn accumulate(&mut self, key: &str, value: Value) -> Result<&mut Self> {
        match self.entries.remove(key) {
            None => self.set(key, value),
            Some(Value::Array(mut values)) => {
                values.push(value);
                self.set(key, Value::Array(values))
            }
            Some(previous) => self.set(key, Value::Array(vec![previous, value])),
        }
    }

    /// 中文说明: 向已有的数组字段追加值。
    /// 对齐 Java 方法: `append`
    pub fn append(&mut self, key: &str, value: Value) -> Result<&mut Self> {
        let values = self
            .entries
            .get_mut(key)
            .and_then(Value::as_array_mut)
            .ok_or_else(|| PathError::Type(key.to_owned()))?;
        values.push(value);
        Ok(self)
    }

    /// 中文说明: 自增数值字段，缺失时视为零。
    /// 对齐 Java 方法: `increment`
    pub fn increment(&mut self, key: &str) -> Result<&mut Self> {
        let next = match self.entries.get(key) {
            None => 1,
            Some(Value::Number(number)) => number
                .as_i64()
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| PathError::Type(key.to_owned()))?,
            Some(_) => return Err(PathError::Type(key.to_owned()).into()),
        };
        self.entries
            .insert(key.to_owned(), Value::Number(Number::from(next)));
        Ok(self)
    }

    /// 中文说明: 通过路径借用嵌套值。
    /// 对齐 Java 方法: `getByPath`
    #[must_use]
    pub fn get_by_path(&self, path: &str) -> Option<&Value> {
        get_object_path(self, path)
    }

    /// 中文说明: 通过路径写入嵌套值。
    /// 对齐 Java 方法: `putByPath`
    pub fn put_by_path(&mut self, path: &str, value: Value) -> Result<()> {
        let mut root = self.to_value();
        put_by_path(&mut root, path, value)?;
        self.entries = root.as_object().cloned().ok_or(JsonError::UnexpectedType {
            expected: "object",
            actual: value_type(&root),
        })?;
        Ok(())
    }

    /// 中文说明: 移除并返回指定字段。
    /// 对齐 Java 方法: `remove`
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.entries.remove(key)
    }

    /// 中文说明: 按确定性映射顺序迭代字段。
    /// 对齐 Java 方法: `entrySet`
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Value)> {
        self.entries.iter()
    }
}

fn get_object_path<'a>(object: &'a JSONObject, path: &str) -> Option<&'a Value> {
    let tokens = parse_path(path).ok()?;
    let mut tokens = tokens.into_iter();
    let first = tokens.next()?;
    let mut current = match first {
        PathToken::Key(key) => object.get(&key)?,
        PathToken::Index(_) => return None,
    };
    for token in tokens {
        current = match token {
            PathToken::Key(key) => current.get(key)?,
            PathToken::Index(index) => current.get(index)?,
        };
    }
    Some(current)
}

impl JsonContainer for JSONObject {
    fn config(&self) -> &JSONConfig {
        &self.config
    }
    fn to_value(&self) -> Value {
        Value::Object(self.entries.clone())
    }
}

impl fmt::Display for JSONObject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.to_value())
    }
}

impl Index<&str> for JSONObject {
    type Output = Value;
    fn index(&self, index: &str) -> &Self::Output {
        self.get(index).unwrap_or(&Value::Null)
    }
}

/// 可变的 JSON 数组，兼容 Hutool。
///
/// 对齐 Java 类: `cn.hutool.json.JSONArray`'s `JSONArray`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JSONArray {
    values: Vec<Value>,
    config: JSONConfig,
}

impl Default for JSONArray {
    fn default() -> Self {
        Self::new()
    }
}

impl JSONArray {
    pub(crate) fn from_values(values: Vec<Value>, config: JSONConfig) -> Self {
        Self { values, config }
    }
    /// 中文说明: 创建空的 JSON 数组。
    /// 对齐 Java 方法: `new JSONArray()`
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(JSONConfig::default())
    }

    /// 中文说明: 使用指定配置创建空的 JSON 数组。
    /// 对齐 Java 方法: `new JSONArray(JSONConfig)`
    #[must_use]
    pub fn with_config(config: JSONConfig) -> Self {
        Self {
            values: Vec::new(),
            config,
        }
    }

    /// 中文说明: 从动态值构建 JSON 数组。
    /// 对齐 Java 方法: `new JSONArray(JSONTokener)`
    pub fn from_value(value: Value, config: JSONConfig) -> Result<Self> {
        match value {
            Value::Array(values) => Ok(Self { values, config }),
            value => Err(JsonError::UnexpectedType {
                expected: "array",
                actual: value_type(&value),
            }),
        }
    }

    /// 中文说明: 解析 JSON 文本为数组。
    /// 对齐 Java 方法: `new JSONArray(String)`
    pub fn parse(input: &str) -> Result<Self> {
        Self::from_value(crate::parse(input)?, JSONConfig::default())
    }

    /// 中文说明: 返回元素数量。
    /// 对齐 Java 方法: `size`
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// 中文说明: 判断数组是否为空。
    /// 对齐 Java 方法: `isEmpty`
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// 中文说明: 借用指定索引的元素。
    /// 对齐 Java 方法: `get`
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.values.get(index)
    }

    /// 中文说明: 追加值，若配置忽略 null 则跳过 null 值。
    /// 对齐 Java 方法: `add`
    pub fn push(&mut self, value: Value) -> &mut Self {
        if !(self.config.is_ignore_null_value() && value.is_null()) {
            self.values.push(normalize_value(value, &self.config));
        }
        self
    }

    /// 中文说明: 设置指定索引的值，缺失位置用 null 填充。
    /// 对齐 Java 方法: `set`
    pub fn set(&mut self, index: usize, value: Value) -> &mut Self {
        if self.values.len() <= index {
            self.values.resize(index + 1, Value::Null);
        }
        self.values[index] = normalize_value(value, &self.config);
        self
    }

    /// 中文说明: 移除并返回指定索引的元素。
    /// 对齐 Java 方法: `remove`
    pub fn remove(&mut self, index: usize) -> Option<Value> {
        if index < self.values.len() {
            Some(self.values.remove(index))
        } else {
            None
        }
    }

    /// 中文说明: 将显示值用分隔符连接（不带 JSON 字符串引号）。
    /// 对齐 Java 方法: `join`
    #[must_use]
    pub fn join(&self, separator: &str) -> String {
        self.values
            .iter()
            .map(display_value)
            .collect::<Vec<_>>()
            .join(separator)
    }

    /// 中文说明: 通过路径借用嵌套值。
    /// 对齐 Java 方法: `getByPath`
    #[must_use]
    pub fn get_by_path(&self, path: &str) -> Option<&Value> {
        let tokens = parse_path(path).ok()?;
        let mut current: Option<&Value> = None;
        for token in tokens {
            current = Some(match (current, token) {
                (None, PathToken::Index(index)) => self.values.get(index)?,
                (Some(value), PathToken::Index(index)) => value.get(index)?,
                (Some(value), PathToken::Key(key)) => value.get(key)?,
                (None, PathToken::Key(_)) => return None,
            });
        }
        current
    }

    /// 中文说明: 通过路径写入嵌套值。
    /// 对齐 Java 方法: `putByPath`
    pub fn put_by_path(&mut self, path: &str, value: Value) -> Result<()> {
        let mut root = self.to_value();
        put_by_path(&mut root, path, value)?;
        self.values = root.as_array().cloned().ok_or(JsonError::UnexpectedType {
            expected: "array",
            actual: value_type(&root),
        })?;
        Ok(())
    }

    /// 中文说明: 迭代数组元素。
    /// 对齐 Java 方法: `iterator`
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.values.iter()
    }
}

impl JsonContainer for JSONArray {
    fn config(&self) -> &JSONConfig {
        &self.config
    }
    fn to_value(&self) -> Value {
        Value::Array(self.values.clone())
    }
}

impl fmt::Display for JSONArray {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.to_value())
    }
}

impl Index<usize> for JSONArray {
    type Output = Value;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

fn normalize_value(value: Value, config: &JSONConfig) -> Value {
    if config.is_write_long_as_string() {
        if let Value::Number(number) = &value {
            if number.as_i64().is_some() || number.as_u64().is_some() {
                return Value::String(number.to_string());
            }
        }
    }
    value
}

fn display_value(value: &Value) -> String {
    value
        .as_str()
        .map_or_else(|| value.to_string(), str::to_owned)
}

const fn value_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct FlexibleSerialize(bool);

    impl Serialize for FlexibleSerialize {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            if self.0 {
                Err(serde::ser::Error::custom(
                    "intentional serialization failure",
                ))
            } else {
                json!({"value": 3}).serialize(serializer)
            }
        }
    }

    #[test]
    fn config_exposes_every_hutool_option() {
        let mut config = JSONConfig::create();
        assert!(config.is_order());
        assert!(!config.is_ignore_error());
        assert!(!config.is_ignore_case());
        assert_eq!(config.date_format(), None);
        assert!(!config.is_ignore_null_value());
        assert!(config.is_transient_support());
        assert!(config.is_strip_trailing_zeros());
        assert!(!config.is_check_duplicate());
        assert!(!config.is_write_long_as_string());
        assert!(!config.has_nature_key_comparator());
        config
            .set_order(false)
            .set_nature_key_comparator()
            .set_ignore_error(true)
            .set_ignore_case(true)
            .set_date_format("yyyy-MM-dd")
            .set_ignore_null_value(true)
            .set_transient_support(false)
            .set_strip_trailing_zeros(false)
            .set_check_duplicate(true)
            .set_write_long_as_string(true);
        assert!(config.has_nature_key_comparator());
        assert!(config.is_ignore_error());
        assert!(config.is_ignore_case());
        assert_eq!(config.date_format(), Some("yyyy-MM-dd"));
        assert!(config.is_ignore_null_value());
        assert!(!config.is_transient_support());
        assert!(!config.is_strip_trailing_zeros());
        assert!(config.is_check_duplicate());
        assert!(config.is_write_long_as_string());
        config.set_date_format("");
        assert_eq!(config.date_format(), None);
    }

    #[test]
    fn paths_read_write_create_and_validate_nested_shapes() {
        let mut value = json!({"user": {"items": [1]}});
        assert_eq!(get_by_path(&value, "$.user.items[0]"), Some(&json!(1)));
        assert_eq!(get_by_path(&value, "user.missing"), None);
        assert_eq!(get_by_path(&value, "[bad]"), None);
        assert_eq!(get_by_path(&json!([1]), "[2]"), None);
        put_by_path(&mut value, "user.items[2].name", json!("third")).unwrap();
        assert_eq!(value["user"]["items"][1], Value::Null);
        assert_eq!(value["user"]["items"][2]["name"], "third");
        put_by_path(&mut value, "user.active", json!(true)).unwrap();
        assert_eq!(value["user"]["active"], true);
        put_by_path(&mut value, "", json!([1, 2])).unwrap();
        assert_eq!(value, json!([1, 2]));
        assert!(put_by_path(&mut value, "[", Value::Null).is_err());
        assert!(put_by_path(&mut value, "[x]", Value::Null).is_err());
        let mut object_conflict = json!({"value": 1});
        assert!(put_by_path(&mut object_conflict, "value.name", json!(2)).is_err());
        let mut array_conflict = json!([1]);
        assert!(put_by_path(&mut array_conflict, "[0][0]", json!(2)).is_err());
        assert_eq!(
            PathError::Type("x".into()).to_string(),
            "cannot traverse JSON path segment: x"
        );
    }

    #[test]
    fn object_mutations_cover_case_null_duplicates_arrays_and_numbers() {
        let mut config = JSONConfig::default();
        config.set_ignore_case(true).set_write_long_as_string(true);
        let mut object = JSONObject::with_config(config.clone());
        object.set("Name", json!("Alice")).unwrap();
        object
            .set("identifier", json!(9_007_199_254_740_991_i64))
            .unwrap();
        assert_eq!(object.get("name"), Some(&json!("Alice")));
        assert_eq!(object["identifier"], "9007199254740991");
        assert_eq!(object.get_or("missing", &json!(false)), &json!(false));
        object
            .put_opt("", json!(1))
            .unwrap()
            .put_opt("none", Value::Null)
            .unwrap();
        object.put_opt("present", json!(1)).unwrap();
        object.put_once("once", json!(1)).unwrap();
        assert!(object.put_once("once", json!(2)).is_err());
        object.accumulate("tag", json!("a")).unwrap();
        object.accumulate("tag", json!("b")).unwrap();
        object.accumulate("tag", json!("c")).unwrap();
        object.append("tag", json!("d")).unwrap();
        assert_eq!(object["tag"], json!(["a", "b", "c", "d"]));
        assert!(object.append("Name", json!(1)).is_err());
        object
            .increment("visits")
            .unwrap()
            .increment("visits")
            .unwrap();
        assert_eq!(object["visits"], 2);
        object.set("not-number", json!(true)).unwrap();
        assert!(object.increment("not-number").is_err());
        let mut overflow = JSONObject::new();
        overflow.set("max", json!(i64::MAX)).unwrap();
        assert!(overflow.increment("max").is_err());
        object.put_by_path("nested.values[1]", json!(7)).unwrap();
        assert_eq!(object.get_by_path("nested.values[1]"), Some(&json!(7)));
        assert_eq!(object.get_by_path("[0]"), None);
        assert_eq!(object.remove("once"), Some(json!("1")));
        assert_eq!(object.remove("missing"), None);
        assert!(object.iter().any(|(key, _)| key == "Name"));
        assert!(!object.is_empty());
        assert_eq!(object.len(), object.iter().count());
        assert_eq!(object.config(), &config);
        assert!(object.to_json_string(0).unwrap().starts_with('{'));
        assert!(object.to_json_string(2).unwrap().contains('\n'));
        assert!(object.to_string().starts_with('{'));

        let mut ignore_null = JSONConfig::default();
        ignore_null.set_ignore_null_value(true);
        let mut object = JSONObject::with_config(ignore_null);
        object.set("removed", json!(1)).unwrap();
        object.set("removed", Value::Null).unwrap();
        assert!(object.is_empty());

        let mut duplicate = JSONConfig::default();
        duplicate.set_check_duplicate(true);
        let mut object = JSONObject::with_config(duplicate);
        object.set("x", json!(1)).unwrap();
        assert!(object.set("x", json!(2)).is_err());
        assert!(object.put_opt("x", json!(2)).is_err());

        let mut ignore_case = JSONConfig::default();
        ignore_case.set_ignore_case(true);
        assert_eq!(JSONObject::with_config(ignore_case).get("missing"), None);

        let mut object = JSONObject::new();
        assert!(object.put_by_path("[", json!(1)).is_err());
        assert!(object.put_by_path("", json!(1)).is_err());
        assert_eq!(object.get_by_path("["), None);
        assert_eq!(object.get_by_path(""), None);
        assert_eq!(object.get_by_path("missing"), None);
        object.set("nested", json!({"array": [1]})).unwrap();
        assert_eq!(object.get_by_path("nested.missing"), None);
        assert_eq!(object.get_by_path("nested.array[2]"), None);
        assert_eq!(object.get_by_path("nested.array.name"), None);
    }

    #[test]
    fn object_construction_rejects_shapes_and_serializes_structs() {
        let object = JSONObject::from_serializable(&FlexibleSerialize(false)).unwrap();
        assert!(JSONObject::from_serializable(&FlexibleSerialize(true)).is_err());
        assert_eq!(object["value"], 3);
        assert_eq!(JSONObject::parse(r#"{"a":1}"#).unwrap()["a"], 1);
        assert!(JSONObject::parse("[]").is_err());
        assert!(JSONObject::parse("{").is_err());
        assert!(JSONObject::from_value(Value::Null, JSONConfig::default()).is_err());
        for value in [json!(false), json!(1), json!("x")] {
            assert!(JSONObject::from_value(value, JSONConfig::default()).is_err());
        }
        assert_eq!(JSONObject::default(), JSONObject::new());
        assert_eq!(object["missing"], Value::Null);
    }

    #[test]
    fn array_mutations_paths_and_formatting_are_complete() {
        let mut config = JSONConfig::default();
        config
            .set_ignore_null_value(true)
            .set_write_long_as_string(true);
        let mut array = JSONArray::with_config(config.clone());
        array.push(Value::Null).push(json!(1)).push(json!("two"));
        assert_eq!(array.len(), 2);
        assert!(!array.is_empty());
        array.set(3, json!(4));
        array.set(0, json!(1));
        assert_eq!(array[2], Value::Null);
        assert_eq!(array[3], "4");
        assert_eq!(array.get(9), None);
        assert_eq!(array.join("|"), "1|two|null|4");
        array.put_by_path("[2].nested[0]", json!(true)).unwrap();
        assert_eq!(array.get_by_path("[2].nested[0]"), Some(&json!(true)));
        assert_eq!(array.get_by_path("name"), None);
        assert_eq!(array.remove(99), None);
        assert_eq!(array.remove(0), Some(json!("1")));
        assert_eq!(array.iter().count(), 3);
        assert_eq!(array.config(), &config);
        assert!(array.to_json_string(2).unwrap().contains('\n'));
        assert!(array.to_json_string(0).unwrap().starts_with('['));
        assert!(array.to_string().starts_with('['));
        assert_eq!(JSONArray::parse("[1]").unwrap()[0], 1);
        assert!(JSONArray::parse("{}").is_err());
        assert!(JSONArray::parse("[").is_err());
        assert!(JSONArray::from_value(Value::Null, JSONConfig::default()).is_err());
        assert!(JSONArray::from_value(json!(true), JSONConfig::default()).is_err());
        assert_eq!(JSONArray::default(), JSONArray::new());
        assert!(JSONArray::new().is_empty());

        let mut invalid = JSONArray::new();
        assert!(invalid.put_by_path("[", json!(1)).is_err());
        assert!(invalid.put_by_path("", json!({})).is_err());
        assert_eq!(invalid.get_by_path("["), None);
        assert_eq!(invalid.get_by_path("[2]"), None);
        let nested = JSONArray::parse(r#"[[1], {"x": 1}]"#).unwrap();
        assert_eq!(nested.get_by_path("[0][2]"), None);
        assert_eq!(nested.get_by_path("[1].missing"), None);

        let mut unsigned = JSONConfig::default();
        unsigned.set_write_long_as_string(true);
        assert_eq!(
            JSONArray::with_config(unsigned).push(json!(u64::MAX))[0],
            u64::MAX.to_string()
        );
        let mut fractional = JSONConfig::default();
        fractional.set_write_long_as_string(true);
        assert_eq!(normalize_value(json!(1.5), &fractional), json!(1.5));
    }

    #[test]
    fn json_null_matches_json_text() {
        assert_eq!(JSONNull.to_string(), "null");
        assert_eq!(format!("{JSONNull:?}"), "JSONNull");
    }
}
