use std::io::Write;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::{
    JSONArray, JSONConfig, JSONObject, JsonContainer, JsonError, Result, get_by_path, put_by_path,
};

/// 对齐: `cn.hutool.json.JSONStrFormatter`
/// 中文说明: JSON 字符串格式化门面，提供美化输出功能。
///
/// Pretty-formatting facade corresponding to Hutool's `JSONStrFormatter`.
pub struct JSONStrFormatter;

impl JSONStrFormatter {
    /// 中文说明: 格式化一个完整的 JSON 文档。
    /// 对齐 Java 方法: `format`
    pub fn format(input: &str) -> Result<String> {
        crate::pretty(input)
    }
}

use super::{WriterMode, normalize_writer_value};
