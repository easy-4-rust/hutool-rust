use quick_xml::{Reader, XmlVersion, events::Event};
use serde_json::{Map, Value};

use crate::{JSONConfig, JSONObject, JsonError, ParseConfig, Result};

/// 对齐: `cn.hutool.json.XML`
/// 中文说明: XML 转换门面，匹配 Hutool 的 `XML` 工具类。
///
/// XML conversion facade matching Hutool's `XML` utility.
pub struct XML;

impl XML {
    /// 中文说明: 使用生产环境默认配置解析 XML 为 JSON 对象。
    /// 对齐 Java 方法: `toJSONObject`
    pub fn to_json(input: &str) -> Result<JSONObject> {
        Self::to_json_with(input, ParseConfig::default())
    }

    /// 中文说明: 使用显式防御性限制解析 XML 为 JSON 对象。
    /// 对齐 Java 方法: `toJSONObject`
    pub fn to_json_with(input: &str, config: ParseConfig) -> Result<JSONObject> {
        config.validate(input)?;
        let mut reader = Reader::from_str(input);
        reader.config_mut().trim_text(true);
        let mut stack: Vec<Element> = Vec::new();
        let mut root = Map::new();
        loop {
            match reader.read_event() {
                Ok(Event::Start(event)) => {
                    if stack.len() >= config.max_nesting_depth() {
                        return Err(JsonError::Limit("XML nesting depth"));
                    }
                    let name = String::from_utf8_lossy(event.name().as_ref()).into_owned();
                    let mut element = Element::new(name);
                    for attribute in event.attributes() {
                        let attribute = match attribute {
                            Ok(attribute) => attribute,
                            Err(error) => return Err(JsonError::Syntax(error.to_string())),
                        };
                        let key = format!("@{}", String::from_utf8_lossy(attribute.key.as_ref()));
                        let value = match attribute
                            .decoded_and_normalized_value(XmlVersion::Implicit1_0, reader.decoder())
                        {
                            Ok(value) => value,
                            Err(error) => return Err(JsonError::Syntax(error.to_string())),
                        };
                        element
                            .fields
                            .insert(key, Value::String(value.into_owned()));
                    }
                    stack.push(element);
                }
                Ok(Event::Empty(event)) => {
                    if stack.len() >= config.max_nesting_depth() {
                        return Err(JsonError::Limit("XML nesting depth"));
                    }
                    let name = String::from_utf8_lossy(event.name().as_ref()).into_owned();
                    attach(&mut stack, &mut root, name, Value::String(String::new()));
                }
                Ok(Event::Text(event)) => {
                    if let Some(element) = stack.last_mut() {
                        element
                            .text
                            .push_str(&String::from_utf8_lossy(event.as_ref()));
                    }
                }
                Ok(Event::CData(event)) => {
                    if let Some(element) = stack.last_mut() {
                        element
                            .text
                            .push_str(&String::from_utf8_lossy(event.as_ref()));
                    }
                }
                Ok(Event::End(_)) => {
                    finish_element(&mut stack, &mut root, config.is_keep_strings());
                }
                Ok(Event::Eof) => break,
                Ok(_) => {}
                Err(error) => return Err(JsonError::Syntax(error.to_string())),
            }
        }
        if !stack.is_empty() {
            return Err(JsonError::Syntax("unclosed XML element".into()));
        }
        JSONObject::from_value(Value::Object(root), JSONConfig::default())
    }

    /// 中文说明: 将动态值序列化为 XML（不带额外根标签）。
    /// 对齐 Java 方法: `toXml`
    #[must_use]
    pub fn to_xml(value: &Value) -> String {
        let mut output = String::new();
        write_value(&mut output, None, value);
        output
    }

    /// 中文说明: 将动态值序列化为 XML（带指定根标签）。
    /// 对齐 Java 方法: `toXml`
    #[must_use]
    pub fn to_xml_with_root(value: &Value, root: &str) -> String {
        let mut output = String::new();
        write_value(&mut output, Some(root), value);
        output
    }
}

impl Element {
    fn new(name: String) -> Self {
        Self {
            name,
            fields: Map::new(),
            text: String::new(),
        }
    }

    pub(crate) fn finish(mut self, keep_strings: bool) -> (String, Value) {
        let text = self.text.trim();
        let value = if self.fields.is_empty() {
            scalar(text, keep_strings)
        } else {
            if !text.is_empty() {
                self.fields
                    .insert("content".into(), scalar(text, keep_strings));
            }
            Value::Object(self.fields)
        };
        (self.name, value)
    }
}

use super::{Element, attach, finish_element, scalar, write_value};
