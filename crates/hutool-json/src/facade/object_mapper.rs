use std::io::Write;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::{
    JSONArray, JSONConfig, JSONObject, JsonContainer, JsonError, Result, get_by_path, put_by_path,
};

use super::json_util::JSONUtil;

/// Serde-backed object mapper.
pub struct ObjectMapper;

impl ObjectMapper {
    /// Maps a serializable value to a configured object.
    pub fn to_object<T: Serialize + ?Sized>(value: &T, config: JSONConfig) -> Result<JSONObject> {
        JSONUtil::object_from(value, config)
    }

    /// Maps a serializable value to a configured array.
    pub fn to_array<T: Serialize + ?Sized>(value: &T, config: JSONConfig) -> Result<JSONArray> {
        JSONUtil::array_from(value, config)
    }
}

use super::{WriterMode, normalize_writer_value};
