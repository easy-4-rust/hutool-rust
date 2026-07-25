//! 对齐: `cn.hutool.json.JSONUtil`
//! 来源: hutool-json/src/main/java/cn/hutool/json/JSONUtil.java
//! 中文说明: JSON 工具类，提供创建、解析、序列化、判断等静态方法。

use std::io::Write;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::{
    JSONArray, JSONConfig, JSONObject, JsonContainer, JsonError, Result, get_by_path, put_by_path,
};

/// 对齐: `cn.hutool.json.JSONUtil`
/// 中文说明: Hutool 兼容的 JSON 工具类，提供静态便捷方法。
pub struct JSONUtil;

impl JSONUtil {
    /// 中文说明: 创建空的 JSON 对象。
    /// 对齐 Java 方法: `createObj`
    #[must_use]
    pub fn create_obj() -> JSONObject {
        JSONObject::new()
    }

    /// 中文说明: 使用指定配置创建空的 JSON 对象。
    /// 对齐 Java 方法: `createObj`
    #[must_use]
    pub fn create_obj_with(config: JSONConfig) -> JSONObject {
        JSONObject::with_config(config)
    }

    /// 中文说明: 创建空的 JSON 数组。
    /// 对齐 Java 方法: `createArray`
    #[must_use]
    pub fn create_array() -> JSONArray {
        JSONArray::new()
    }

    /// 中文说明: 使用指定配置创建空的 JSON 数组。
    /// 对齐 Java 方法: `createArray`
    #[must_use]
    pub fn create_array_with(config: JSONConfig) -> JSONArray {
        JSONArray::with_config(config)
    }

    /// 中文说明: 解析 JSON 文本为对象。
    /// 对齐 Java 方法: `parseObj`
    pub fn parse_obj(input: &str) -> Result<JSONObject> {
        JSONObject::parse(input)
    }

    /// 中文说明: 将可序列化的值转换为 JSON 对象。
    /// 对齐 Java 方法: `toBean(JSONObject.class, ...)`
    pub fn object_from<T: Serialize + ?Sized>(value: &T, config: JSONConfig) -> Result<JSONObject> {
        JSONObject::from_value(serde_json::to_value(value)?, config)
    }

    /// 中文说明: 解析 JSON 文本为数组。
    /// 对齐 Java 方法: `parseArray`
    pub fn parse_array(input: &str) -> Result<JSONArray> {
        JSONArray::parse(input)
    }

    /// 中文说明: 将可序列化的值转换为 JSON 数组。
    /// 对齐 Java 方法: `toBean(JSONArray.class, ...)`
    pub fn array_from<T: Serialize + ?Sized>(value: &T, config: JSONConfig) -> Result<JSONArray> {
        JSONArray::from_value(serde_json::to_value(value)?, config)
    }

    /// 中文说明: 解析任意 JSON 值。
    /// 对齐 Java 方法: `parse`
    pub fn parse(input: &str) -> Result<Value> {
        crate::parse(input)
    }

    /// 中文说明: 将值序列化为紧凑 JSON 字符串。
    /// 对齐 Java 方法: `toJsonStr`
    pub fn to_json_string<T: Serialize + ?Sized>(value: &T) -> Result<String> {
        crate::to_string(value)
    }

    /// 中文说明: 将值序列化为带缩进的 JSON 字符串。
    /// 对齐 Java 方法: `toJsonPrettyStr`
    pub fn to_pretty_string<T: Serialize + ?Sized>(value: &T) -> Result<String> {
        crate::to_string_pretty(value)
    }

    /// 中文说明: 将 JSON 反序列化为指定类型的 Rust 值。
    /// 对齐 Java 方法: `toBean`
    pub fn to_bean<T: DeserializeOwned>(input: &str) -> Result<T> {
        crate::from_str(input)
    }

    /// 中文说明: 将 JSON 数组的每个元素反序列化为指定类型。
    /// 对齐 Java 方法: `toList`
    pub fn to_list<T: DeserializeOwned>(array: &JSONArray) -> Result<Vec<T>> {
        Ok(serde_json::from_value(array.to_value())?)
    }

    /// 中文说明: 通过 JSON 路径借用值。
    /// 对齐 Java 方法: `getByPath`
    #[must_use]
    pub fn get_by_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
        get_by_path(value, path)
    }

    /// 中文说明: 通过 JSON 路径写入值。
    /// 对齐 Java 方法: `putByPath`
    pub fn put_by_path(value: &mut Value, path: &str, replacement: Value) -> Result<()> {
        put_by_path(value, path, replacement)
    }

    /// 中文说明: 将字符串包装为 JSON 带引号格式。
    /// 对齐 Java 方法: `quote`
    #[must_use]
    pub fn quote(value: &str) -> String {
        Value::String(value.to_owned()).to_string()
    }

    /// 中文说明: 转义字符串（不带外层引号）。
    /// 对齐 Java 方法: `escape`
    #[must_use]
    pub fn escape(value: &str) -> String {
        let quoted = Self::quote(value);
        quoted[1..quoted.len() - 1].to_owned()
    }

    /// 中文说明: 将 JSON 文本格式化为带缩进的表示。
    /// 对齐 Java 方法: `formatJsonStr`
    pub fn format_json_str(value: &str) -> Result<String> {
        crate::pretty(value)
    }

    /// 中文说明: 判断输入是否为合法的 JSON。
    /// 对齐 Java 方法: `isJson`
    #[must_use]
    pub fn is_json(value: &str) -> bool {
        crate::is_valid(value)
    }

    /// 中文说明: 判断输入是否为 JSON 对象。
    /// 对齐 Java 方法: `isJsonObj`
    #[must_use]
    pub fn is_json_obj(value: &str) -> bool {
        crate::is_json_object(value)
    }

    /// 中文说明: 判断输入是否为 JSON 数组。
    /// 对齐 Java 方法: `isJsonArray`
    #[must_use]
    pub fn is_json_array(value: &str) -> bool {
        crate::is_json_array(value)
    }

    /// 中文说明: 判断动态值是否为 JSON null。
    /// 对齐 Java 方法: `isNull`
    #[must_use]
    pub fn is_null(value: &Value) -> bool {
        value.is_null()
    }
}

use super::{WriterMode, normalize_writer_value};
