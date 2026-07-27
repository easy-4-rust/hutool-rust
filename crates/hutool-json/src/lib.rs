//! 对齐: `cn.hutool.json` 包
//! 来源: hutool-json/src/main/java/cn/hutool/json/
//! 中文说明: Hutool JSON 模块的 Rust 实现，提供 JSON 解析、序列化、格式化等工具。
//!
//! Typed JSON serialization and value utilities.
//!
//! The initial operation set and tests were adapted from yimi-rutool 0.2.5
//! (Apache-2.0), then revised around `serde_json` types and a module-specific
//! error instead of a workspace-wide error enum.

#![forbid(unsafe_code)]

use serde::de::DeserializeOwned;

pub use serde::{Deserialize, Serialize};
pub use serde_json::{Map, Value, json};

mod compat;
mod facade;
mod parser;
mod serialize;
mod xml;

// 兼容层中的对象已开始按 Java 类逐步拆分到独立文件，这里继续保持顶层 API 不变。
pub use compat::{
    JSONArray, JSONConfig, JSONNull, JSONObject, JsonContainer, PathError, get_by_path, put_by_path,
};
pub use facade::{
    JSONConverter, JSONStrFormatter, JSONSupport, JSONUtil, JSONWriter, JsonContainerObject,
    ObjectMapper,
};
pub use parser::{JSONParser, JSONTokener, ParseConfig};
pub use serialize::{GlobalSerializeMapping, JSONDeserializer, JSONSerializer, SerializeRegistry};
pub use xml::{JSONXMLParser, JSONXMLSerializer, XML, XMLTokener};

/// 对齐: `cn.hutool.json.JSON`
/// 中文说明: JSON 操作的结果类型别名。
///
/// Result type returned by JSON operations.
pub type Result<T> = std::result::Result<T, JsonError>;

/// 对齐: `cn.hutool.json.JSONException`
/// 中文说明: JSON 操作产生的错误类型，涵盖解析、序列化、路径遍历等场景。
///
/// Errors produced by `hutool-json`.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum JsonError {
    /// 中文说明: serde_json 序列化或解析失败。
    #[error("JSON operation failed: {0}")]
    Serde(#[from] serde_json::Error),

    /// 中文说明: JSON 值的形状与请求的操作不匹配。
    #[error("expected a JSON {expected}, found {actual}")]
    UnexpectedType {
        /// 期望的 JSON 类型。
        expected: &'static str,
        /// 实际的 JSON 类型。
        actual: &'static str,
    },

    /// 中文说明: JSON 路径解析或遍历失败。
    #[error(transparent)]
    Path(#[from] PathError),

    /// 中文说明: 防御性解析器资源限制被超出。
    #[error("JSON resource limit exceeded: {0}")]
    Limit(&'static str),

    /// 中文说明: 有状态分词器遇到语法错误。
    #[error("JSON syntax error: {0}")]
    Syntax(String),

    /// 中文说明: 读取 JSON 字节流失败。
    #[error("JSON I/O failed: {0}")]
    Io(#[from] std::io::Error),

    /// 中文说明: 输入字节不是有效的 UTF-8 编码。
    #[error("JSON input is not UTF-8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    /// 中文说明: 自定义序列化器或反序列化器不可用或不兼容。
    #[error("JSON mapping failed: {0}")]
    Mapping(&'static str),
}

/// 中文说明: 将值序列化为紧凑 JSON 字符串。
/// 对齐 Java 方法: `cn.hutool.json.JSONUtil.toJsonStr`
///
/// # Errors
///
/// Returns an error when `value` cannot be represented as JSON.
pub fn to_string<T: Serialize + ?Sized>(value: &T) -> Result<String> {
    Ok(serde_json::to_string(value)?)
}

/// 中文说明: 将值序列化为带缩进的 JSON 字符串。
/// 对齐 Java 方法: `cn.hutool.json.JSONUtil.toJsonPrettyStr`
///
/// # Errors
///
/// Returns an error when `value` cannot be represented as JSON.
pub fn to_string_pretty<T: Serialize + ?Sized>(value: &T) -> Result<String> {
    Ok(serde_json::to_string_pretty(value)?)
}

/// 中文说明: 将 JSON 文本反序列化为指定类型的 Rust 值。
/// 对齐 Java 方法: `cn.hutool.json.JSONUtil.toBean`
///
/// # Errors
///
/// Returns an error for malformed JSON or incompatible target types.
pub fn from_str<T: DeserializeOwned>(input: &str) -> Result<T> {
    Ok(serde_json::from_str(input)?)
}

/// 中文说明: 将任意 JSON 文本解析为动态 [`Value`]。
/// 对齐 Java 方法: `cn.hutool.json.JSONUtil.parse`
///
/// # Errors
///
/// Returns an error for malformed JSON.
pub fn parse(input: &str) -> Result<Value> {
    from_str(input)
}

/// 中文说明: 解析 JSON 对象，返回键值映射。
/// 对齐 Java 方法: `cn.hutool.json.JSONUtil.parseObj`
///
/// # Errors
///
/// Returns an error for malformed JSON or a non-object top-level value.
pub fn parse_object(input: &str) -> Result<Map<String, Value>> {
    let value = parse(input)?;
    let actual = type_name(&value);
    value.as_object().cloned().ok_or(JsonError::UnexpectedType {
        expected: "object",
        actual,
    })
}

/// 中文说明: 解析 JSON 数组，返回值列表。
/// 对齐 Java 方法: `cn.hutool.json.JSONUtil.parseArray`
///
/// # Errors
///
/// Returns an error for malformed JSON or a non-array top-level value.
pub fn parse_array(input: &str) -> Result<Vec<Value>> {
    let value = parse(input)?;
    let actual = type_name(&value);
    value.as_array().cloned().ok_or(JsonError::UnexpectedType {
        expected: "array",
        actual,
    })
}

/// 中文说明: 判断输入是否为合法的 JSON。
/// 对齐 Java 方法: `cn.hutool.json.JSONUtil.isJson`
#[must_use]
pub fn is_valid(input: &str) -> bool {
    parse(input).is_ok()
}

/// 中文说明: 判断输入是否为 JSON 对象。
/// 对齐 Java 方法: `cn.hutool.json.JSONUtil.isJsonObj`
#[must_use]
pub fn is_json_object(input: &str) -> bool {
    parse(input).is_ok_and(|value| value.is_object())
}

/// 中文说明: 判断输入是否为 JSON 数组。
/// 对齐 Java 方法: `cn.hutool.json.JSONUtil.isJsonArray`
#[must_use]
pub fn is_json_array(input: &str) -> bool {
    parse(input).is_ok_and(|value| value.is_array())
}

/// 中文说明: 将 JSON 文本转换为紧凑表示。
/// 对齐 Java 方法: `cn.hutool.json.JSONUtil.minify`
///
/// # Errors
///
/// Returns an error for malformed JSON.
pub fn minify(input: &str) -> Result<String> {
    to_string(&parse(input)?)
}

/// 中文说明: 将 JSON 文本转换为带缩进的表示。
/// 对齐 Java 方法: `cn.hutool.json.JSONUtil.formatJsonStr`
///
/// # Errors
///
/// Returns an error for malformed JSON.
pub fn pretty(input: &str) -> Result<String> {
    to_string_pretty(&parse(input)?)
}

const fn type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// 中文说明: 使用 `hutool-json` 时的常用导入集合。
///
/// Common imports for applications using `hutool-json`.
pub mod prelude {
    pub use crate::{Deserialize, Serialize, Value, json};
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FailingSerialize;

    impl Serialize for FailingSerialize {
        fn serialize<S>(&self, _serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(serde::ser::Error::custom(
                "intentional serialization failure",
            ))
        }
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct Person {
        name: String,
        age: u8,
    }

    #[test]
    fn typed_round_trip() {
        let person = Person {
            name: "Alice".to_owned(),
            age: 30,
        };
        let encoded = to_string(&person).unwrap();
        assert_eq!(from_str::<Person>(&encoded).unwrap(), person);
    }

    #[test]
    fn validation_parses_the_complete_document() {
        assert!(is_valid(r#"{"ok":true}"#));
        assert!(!is_valid(r#"{"ok":true} trailing"#));
        assert!(!is_json_object("{not-json}"));
        assert!(is_json_array("[1, 2]"));
    }

    #[test]
    fn object_and_array_operations_reject_wrong_shapes() {
        assert!(parse_object("[]").is_err());
        assert!(parse_array("{}").is_err());
        assert_eq!(parse_object(r#"{"a":1}"#).unwrap()["a"], 1);
        for input in ["null", "true", "1", r#""x""#] {
            assert!(parse_object(input).is_err());
        }
    }

    #[test]
    fn formatting_is_reversible() {
        let compact = minify("{ \"a\": [1, 2] }").unwrap();
        assert_eq!(compact, r#"{"a":[1,2]}"#);
        assert!(pretty(&compact).unwrap().contains('\n'));
        assert!(minify("{").is_err());
        assert!(pretty("[").is_err());
        assert!(parse_object("{").is_err());
        assert!(parse_array("[").is_err());
        assert!(to_string(&FailingSerialize).is_err());
        assert!(to_string_pretty(&FailingSerialize).is_err());
    }
}
