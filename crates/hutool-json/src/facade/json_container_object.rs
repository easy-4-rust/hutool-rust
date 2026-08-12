use serde_json::Value;

use crate::{JSONArray, JSONObject, JsonContainer};

/// 对齐: `cn.hutool.json.JSON` (对象安全视图)
/// 中文说明: 供 [`JSONConverter`] 使用的对象安全视图 trait。
///
/// Object-safe view used by [`JSONConverter`].
pub trait JsonContainerObject: std::fmt::Display + Send + Sync {
    /// 中文说明: 返回动态 JSON 表示的副本。
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
