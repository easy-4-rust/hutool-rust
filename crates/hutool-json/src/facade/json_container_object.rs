use std::io::Write;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::{
    JSONArray, JSONConfig, JSONObject, JsonContainer, JsonError, Result, get_by_path, put_by_path,
};

use super::json_converter::JSONConverter;

/// Object-safe view used by [`JSONConverter`].
pub trait JsonContainerObject: std::fmt::Display + Send + Sync {
    /// Returns an owned dynamic representation.
    fn to_dynamic(&self) -> Value;
}

impl JsonContainerObject for JSONObject {
    fn to_dynamic(&self) -> Value {
        self.to_value()
    }
}

impl JsonContainerObject for JSONArray {
    fn to_dynamic(&self) -> Value {
        self.to_value()
    }
}

use super::{WriterMode, normalize_writer_value};
