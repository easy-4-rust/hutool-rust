use serde::Serialize;

use crate::{JSONArray, JSONConfig, JSONObject, Result};

use super::json_util::JSONUtil;

/// 对齐: `cn.hutool.json.ObjectMapper`
/// 中文说明: 基于 Serde 的对象映射器。
///
/// Serde-backed object mapper.
pub struct ObjectMapper;

impl ObjectMapper {
    /// 中文说明: 将可序列化的值映射为配置的 JSON 对象。
    /// 对齐 Java 方法: `toObject`
    pub fn to_object<T: Serialize + ?Sized>(value: &T, config: JSONConfig) -> Result<JSONObject> {
        JSONUtil::object_from(value, config)
    }

    /// 中文说明: 将可序列化的值映射为配置的 JSON 数组。
    /// 对齐 Java 方法: `toArray`
    pub fn to_array<T: Serialize + ?Sized>(value: &T, config: JSONConfig) -> Result<JSONArray> {
        JSONUtil::array_from(value, config)
    }
}
