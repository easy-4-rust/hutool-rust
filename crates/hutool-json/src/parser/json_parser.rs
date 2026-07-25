use std::io::Read;

use serde_json::Value;

use crate::{JSONArray, JSONConfig, JSONObject, JsonError, Result};

use super::json_tokener::JSONTokener;

/// 对齐: `cn.hutool.json.JSONParser`
/// 中文说明: 解析器门面，创建配置好的容器对象。
///
/// Parser facade that creates configured containers.
#[derive(Debug, Clone)]
pub struct JSONParser {
    tokener: JSONTokener,
}

impl JSONParser {
    /// 中文说明: 创建基于拥有所有权的分词器的解析器。
    /// 对齐 Java 方法: `new JSONParser(JSONTokener)`
    #[must_use]
    pub const fn new(tokener: JSONTokener) -> Self {
        Self { tokener }
    }

    /// 中文说明: 解析 JSON 对象。
    /// 对齐 Java 方法: `parseObject`
    pub fn parse_object(&mut self) -> Result<JSONObject> {
        let config = self.tokener.config.clone();
        JSONObject::from_value(self.tokener.next_value()?, config)
    }

    /// 中文说明: 解析 JSON 数组。
    /// 对齐 Java 方法: `parseArray`
    pub fn parse_array(&mut self) -> Result<JSONArray> {
        let config = self.tokener.config.clone();
        JSONArray::from_value(self.tokener.next_value()?, config)
    }
}
