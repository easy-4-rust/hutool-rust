//! 对齐: `cn.hutool.json` 包中的解析器组件
//! 来源: hutool-json/src/main/java/cn/hutool/json/JSONParser.java, JSONTokener.java
//! 中文说明: 提供有状态的 JSON 分词器和解析器。

use std::io::Read;

use serde_json::Value;

use crate::{JSONArray, JSONConfig, JSONObject, JsonError, Result};

mod parse_config;
mod json_tokener;
mod json_parser;

pub use parse_config::ParseConfig;
pub use json_tokener::JSONTokener;
pub use json_parser::JSONParser;
