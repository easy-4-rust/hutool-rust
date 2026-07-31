//! 对齐: `cn.hutool.json` 包中的 XML 互转组件
//! 来源: hutool-json/src/main/java/cn/hutool/json/XML.java, XMLTokener.java
//! 中文说明: 提供 JSON 与 XML 之间的解析和序列化功能。

use std::fmt::Write as _;

use serde_json::{Map, Number, Value};

mod xml;
mod jsonxml_parser;
mod jsonxml_serializer;
mod xml_tokener;

pub use xml::XML;
pub use jsonxml_parser::JSONXMLParser;
pub use jsonxml_serializer::JSONXMLSerializer;
pub use xml_tokener::XMLTokener;

fn finish_element(
    stack: &mut Vec<Element>,
    root: &mut Map<String, Value>,
    keep_strings: bool,
) -> bool {
    let Some(element) = stack.pop() else {
        return false;
    };
    let (name, value) = element.finish(keep_strings);
    attach(stack, root, name, value);
    true
}

struct Element {
    name: String,
    fields: Map<String, Value>,
    text: String,
}

fn attach(stack: &mut [Element], root: &mut Map<String, Value>, name: String, value: Value) {
    let fields = stack.last_mut().map_or(root, |parent| &mut parent.fields);
    match fields.remove(&name) {
        None => {
            fields.insert(name, value);
        }
        Some(Value::Array(mut values)) => {
            values.push(value);
            fields.insert(name, Value::Array(values));
        }
        Some(previous) => {
            fields.insert(name, Value::Array(vec![previous, value]));
        }
    }
}

fn scalar(text: &str, keep_strings: bool) -> Value {
    if keep_strings {
        return Value::String(text.to_owned());
    }
    match text {
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        "null" => Value::Null,
        value => value
            .parse::<i64>()
            .map(Number::from)
            .map(Value::Number)
            .or_else(|_| {
                value
                    .parse::<f64>()
                    .ok()
                    .and_then(Number::from_f64)
                    .map(Value::Number)
                    .ok_or(())
            })
            .unwrap_or_else(|()| Value::String(value.to_owned())),
    }
}

fn write_value(output: &mut String, tag: Option<&str>, value: &Value) {
    if let Value::Array(values) = value {
        for value in values {
            write_value(output, tag, value);
        }
        return;
    }
    if let Some(tag) = tag {
        let _ = write!(output, "<{}>", escape_name(tag));
    }
    if let Value::Object(object) = value {
        for (key, value) in object {
            if !key.starts_with('@') && key != "content" {
                write_value(output, Some(key), value);
            } else if key == "content" {
                output.push_str(&escape_text(&display_scalar(value)));
            }
        }
    } else {
        output.push_str(&escape_text(&display_scalar(value)));
    }
    if let Some(tag) = tag {
        let _ = write!(output, "</{}>", escape_name(tag));
    }
}

fn display_scalar(value: &Value) -> String {
    value
        .as_str()
        .map_or_else(|| value.to_string(), str::to_owned)
}

fn escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn escape_name(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_alphanumeric() || matches!(character, '_' | '-' | ':'))
        .collect()
}
