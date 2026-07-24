use std::io::Write;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::{
    JSONArray, JSONConfig, JSONObject, JsonContainer, JsonError, Result, get_by_path, put_by_path,
};

/// Static Hutool-compatible convenience facade.
pub struct JSONUtil;

impl JSONUtil {
    /// Creates an empty object.
    #[must_use]
    pub fn create_obj() -> JSONObject {
        JSONObject::new()
    }

    /// Creates an empty configured object.
    #[must_use]
    pub fn create_obj_with(config: JSONConfig) -> JSONObject {
        JSONObject::with_config(config)
    }

    /// Creates an empty array.
    #[must_use]
    pub fn create_array() -> JSONArray {
        JSONArray::new()
    }

    /// Creates an empty configured array.
    #[must_use]
    pub fn create_array_with(config: JSONConfig) -> JSONArray {
        JSONArray::with_config(config)
    }

    /// Parses an object.
    pub fn parse_obj(input: &str) -> Result<JSONObject> {
        JSONObject::parse(input)
    }

    /// Converts a serializable value to an object.
    pub fn object_from<T: Serialize + ?Sized>(value: &T, config: JSONConfig) -> Result<JSONObject> {
        JSONObject::from_value(serde_json::to_value(value)?, config)
    }

    /// Parses an array.
    pub fn parse_array(input: &str) -> Result<JSONArray> {
        JSONArray::parse(input)
    }

    /// Converts a serializable value to an array.
    pub fn array_from<T: Serialize + ?Sized>(value: &T, config: JSONConfig) -> Result<JSONArray> {
        JSONArray::from_value(serde_json::to_value(value)?, config)
    }

    /// Parses any JSON value.
    pub fn parse(input: &str) -> Result<Value> {
        crate::parse(input)
    }

    /// Serializes a value compactly.
    pub fn to_json_string<T: Serialize + ?Sized>(value: &T) -> Result<String> {
        crate::to_string(value)
    }

    /// Serializes a value with indentation.
    pub fn to_pretty_string<T: Serialize + ?Sized>(value: &T) -> Result<String> {
        crate::to_string_pretty(value)
    }

    /// Deserializes a typed Rust value.
    pub fn to_bean<T: DeserializeOwned>(input: &str) -> Result<T> {
        crate::from_str(input)
    }

    /// Deserializes every array element to a typed Rust value.
    pub fn to_list<T: DeserializeOwned>(array: &JSONArray) -> Result<Vec<T>> {
        Ok(serde_json::from_value(array.to_value())?)
    }

    /// Borrows a value at a JSON path.
    #[must_use]
    pub fn get_by_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
        get_by_path(value, path)
    }

    /// Writes a value at a JSON path.
    pub fn put_by_path(value: &mut Value, path: &str, replacement: Value) -> Result<()> {
        put_by_path(value, path, replacement)
    }

    /// Quotes a JSON string.
    #[must_use]
    pub fn quote(value: &str) -> String {
        Value::String(value.to_owned()).to_string()
    }

    /// Escapes a string without surrounding quotes.
    #[must_use]
    pub fn escape(value: &str) -> String {
        let quoted = Self::quote(value);
        quoted[1..quoted.len() - 1].to_owned()
    }

    /// Formats valid JSON with indentation.
    pub fn format_json_str(value: &str) -> Result<String> {
        crate::pretty(value)
    }

    /// Returns whether the complete input is JSON.
    #[must_use]
    pub fn is_json(value: &str) -> bool {
        crate::is_valid(value)
    }

    /// Returns whether the complete input is an object.
    #[must_use]
    pub fn is_json_obj(value: &str) -> bool {
        crate::is_json_object(value)
    }

    /// Returns whether the complete input is an array.
    #[must_use]
    pub fn is_json_array(value: &str) -> bool {
        crate::is_json_array(value)
    }

    /// Returns whether a dynamic value is JSON null.
    #[must_use]
    pub fn is_null(value: &Value) -> bool {
        value.is_null()
    }
}

use super::{WriterMode, normalize_writer_value};
