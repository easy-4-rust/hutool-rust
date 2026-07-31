//! 对齐: `cn.hutool.json` 包中的门面工具类
//! 来源: hutool-json/src/main/java/cn/hutool/json/JSONUtil.java, JSONWriter.java 等
//! 中文说明: 提供 JSONUtil、JSONWriter、JSONConverter 等门面工具。

use serde_json::Value;

use crate::JSONConfig;

mod json_util;
mod json_str_formatter;
mod json_support;
mod json_converter;
mod json_container_object;
mod object_mapper;
mod json_writer;

pub use json_util::JSONUtil;
pub use json_str_formatter::JSONStrFormatter;
pub use json_support::JSONSupport;
pub use json_converter::JSONConverter;
pub use json_container_object::JsonContainerObject;
pub use object_mapper::ObjectMapper;
pub use json_writer::JSONWriter;

#[derive(PartialEq)]
enum WriterMode {
    Object,
    Array,
}

fn normalize_writer_value(value: &Value, config: &JSONConfig) -> Value {
    if config.is_write_long_as_string() {
        if let Value::Number(number) = value {
            if number.as_i64().is_some() || number.as_u64().is_some() {
                return Value::String(number.to_string());
            }
        }
    }
    value.clone()
}
