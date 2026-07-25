use std::io::Write;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::{
    JSONArray, JSONConfig, JSONObject, JsonContainer, JsonError, Result, get_by_path, put_by_path,
};

/// 对齐: `cn.hutool.json.JSON`
/// 中文说明: 混入应用类型的 Serde 支持 trait，提供解析和序列化方法。
///
/// Serde-backed support mixed into application types.
pub trait JSONSupport: Serialize + DeserializeOwned + Sized {
    /// 中文说明: 解析 JSON 文本为当前类型实例。
    /// 对齐 Java 方法: `parse`
    fn parse(input: &str) -> Result<Self> {
        crate::from_str(input)
    }

    /// 中文说明: 将当前值转换为动态 JSON 值。
    /// 对齐 Java 方法: `toJSON`
    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::to_value(self)?)
    }

    /// 中文说明: 将当前值序列化为紧凑 JSON 字符串。
    /// 对齐 Java 方法: `toString`
    fn to_json_string(&self) -> Result<String> {
        crate::to_string(self)
    }

    /// 中文说明: 将当前值序列化为带缩进的 JSON 字符串。
    /// 对齐 Java 方法: `toStringPretty`
    fn to_pretty_string(&self) -> Result<String> {
        crate::to_string_pretty(self)
    }
}

impl<T: Serialize + DeserializeOwned> JSONSupport for T {}

use super::{WriterMode, normalize_writer_value};
