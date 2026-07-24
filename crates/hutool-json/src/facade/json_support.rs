use std::io::Write;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::{
    JSONArray, JSONConfig, JSONObject, JsonContainer, JsonError, Result, get_by_path, put_by_path,
};

/// Serde-backed support mixed into application types.
pub trait JSONSupport: Serialize + DeserializeOwned + Sized {
    /// Parses one instance.
    fn parse(input: &str) -> Result<Self> {
        crate::from_str(input)
    }

    /// Converts this value to a dynamic JSON value.
    fn to_json(&self) -> Result<Value> {
        Ok(serde_json::to_value(self)?)
    }

    /// Serializes this value compactly.
    fn to_json_string(&self) -> Result<String> {
        crate::to_string(self)
    }

    /// Serializes this value with indentation.
    fn to_pretty_string(&self) -> Result<String> {
        crate::to_string_pretty(self)
    }
}

impl<T: Serialize + DeserializeOwned> JSONSupport for T {}

use super::{WriterMode, normalize_writer_value};
