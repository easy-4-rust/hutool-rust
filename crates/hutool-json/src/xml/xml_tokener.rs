use crate::{JSONObject, ParseConfig, Result};

use super::xml::XML;

/// 对齐: `cn.hutool.json.XMLTokener`
/// 中文说明: 基于有界转换引擎的 XML 分词器。
///
/// XML tokenizer backed by the bounded conversion engine.
pub struct XMLTokener;

impl XMLTokener {
    /// 中文说明: 通过共享有界引擎解析 XML。
    /// 对齐 Java 方法: `parse`
    pub fn parse(input: &str, config: ParseConfig) -> Result<JSONObject> {
        XML::to_json_with(input, config)
    }
}
