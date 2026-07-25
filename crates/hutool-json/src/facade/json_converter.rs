use std::io::Write;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::{
    JSONArray, JSONConfig, JSONObject, JsonContainer, JsonError, Result, get_by_path, put_by_path,
};

use super::json_container_object::JsonContainerObject;

/// 对齐: `cn.hutool.json.JSONConverter`
/// 中文说明: 动态 JSON 转换辅助器，根据值的形状返回对象或数组包装器。
///
/// Dynamic JSON conversion helper.
pub struct JSONConverter;

impl JSONConverter {
    /// 中文说明: 根据动态值的形状返回对象或数组包装器。
    /// 对齐 Java 方法: `convert`
    pub fn convert(value: Value, config: JSONConfig) -> Result<Box<dyn JsonContainerObject>> {
        match value {
            Value::Object(entries) => Ok(Box::new(JSONObject::from_entries(entries, config))),
            Value::Array(values) => Ok(Box::new(JSONArray::from_values(values, config))),
            Value::Null => Err(JsonError::UnexpectedType {
                expected: "object or array",
                actual: "null",
            }),
            Value::Bool(_) => Err(JsonError::UnexpectedType {
                expected: "object or array",
                actual: "boolean",
            }),
            Value::Number(_) => Err(JsonError::UnexpectedType {
                expected: "object or array",
                actual: "number",
            }),
            Value::String(_) => Err(JsonError::UnexpectedType {
                expected: "object or array",
                actual: "string",
            }),
        }
    }
}

use super::{WriterMode, normalize_writer_value};
