use std::io::Write;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::{
    JSONArray, JSONConfig, JSONObject, JsonContainer, JsonError, Result, get_by_path, put_by_path,
};

use super::json_container_object::JsonContainerObject;

/// Dynamic JSON conversion helper.
pub struct JSONConverter;

impl JSONConverter {
    /// Returns an object or array wrapper matching the dynamic shape.
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
