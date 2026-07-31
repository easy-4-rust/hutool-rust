//! 对齐: `cn.hutool.core.util.XmlUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/XmlUtil.java
//!
//! Rust 版本基于 `quick-xml` 提供 DOM 风格 XML 操作。

use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor, Write},
    ops::ControlFlow,
};

use indexmap::IndexMap;
use quick_xml::{
    events::{BytesDecl, Event},
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{Map, Value};

use crate::xml_stream::{
    element_name, end_name, is_valid_xml_char, read_attributes, read_bounded_and_sanitize,
    resolve_reference,
};
use crate::{
    transform_xml, visit_xml, CoreError, Result, XmlEventReader, XmlEventWriter, XmlParseOptions,
    XmlTransformAction,
};

use super::xml_child::XmlChild;
use super::xml_document::XmlDocument;
use super::xml_node::XmlNode;

/// 对齐 Java: `cn.hutool.core.util.XmlUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct XmlUtil;

impl XmlUtil {
    // ── XML 验证 ──

    /// 对齐 Java: `XmlUtil.isXml(String)`
    pub fn is_xml(value: &str) -> bool {
        let trimmed = value.trim();
        trimmed.starts_with("<?xml") || trimmed.starts_with("<")
    }

    // ── XML 转义 ──

    /// 对齐 Java: `XmlUtil.escape(char)`
    pub fn escape_char(c: char) -> &'static str {
        match c {
            '&' => "&amp;",
            '<' => "&lt;",
            '>' => "&gt;",
            '"' => "&quot;",
            '\'' => "&apos;",
            _ => "",
        }
    }

    /// 对齐 Java: `XmlUtil.escape(String)`
    pub fn escape(value: &str) -> String {
        let mut result = String::with_capacity(value.len());
        for c in value.chars() {
            match c {
                '&' => result.push_str("&amp;"),
                '<' => result.push_str("&lt;"),
                '>' => result.push_str("&gt;"),
                '"' => result.push_str("&quot;"),
                '\'' => result.push_str("&apos;"),
                _ => result.push(c),
            }
        }
        result
    }

    /// 对齐 Java: `XmlUtil.unescape(String)`
    pub fn unescape(value: &str) -> String {
        value
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&apos;", "'")
    }

    // ── XML 构建 ──

    /// 构建简单的 XML 元素
    pub fn element(tag: &str, content: &str) -> String {
        format!("<{}>{}</{}>", tag, Self::escape(content), tag)
    }

    /// 构建带属性的 XML 元素
    pub fn element_with_attrs(tag: &str, attrs: &[(&str, &str)], content: &str) -> String {
        let attr_str: String = attrs
            .iter()
            .map(|(k, v)| format!(" {}=\"{}\"", k, Self::escape(v)))
            .collect();
        format!("<{}{}>{}</{}>", tag, attr_str, Self::escape(content), tag)
    }

    /// 构建自闭合 XML 元素
    pub fn self_closing_element(tag: &str) -> String {
        format!("<{} />", tag)
    }

    /// 构建带属性的自闭合 XML 元素
    pub fn self_closing_element_with_attrs(tag: &str, attrs: &[(&str, &str)]) -> String {
        let attr_str: String = attrs
            .iter()
            .map(|(k, v)| format!(" {}=\"{}\"", k, Self::escape(v)))
            .collect();
        format!("<{}{} />", tag, attr_str)
    }

    // ── XML 声明 ──

    /// 对齐 Java: `XmlUtil.xmlHeader(String)`
    pub fn xml_header(encoding: &str) -> String {
        format!("<?xml version=\"1.0\" encoding=\"{}\"?>", encoding)
    }

    /// 默认 UTF-8 编码的 XML 声明
    pub fn xml_header_utf8() -> String {
        Self::xml_header("UTF-8")
    }

    // ── CDATA ──

    /// 对齐 Java: `XmlUtil.wrapCDATA(String)`
    pub fn wrap_cdata(content: &str) -> String {
        format!("<![CDATA[{}]]>", content)
    }

    // ── XML 解析辅助 ──

    /// 提取 XML 标签内容
    pub fn get_tag_content(xml: &str, tag: &str) -> Option<String> {
        let start_tag = format!("<{}>", tag);
        let end_tag = format!("</{}>", tag);
        if let Some(start_pos) = xml.find(&start_tag) {
            let content_start = start_pos + start_tag.len();
            if let Some(end_pos) = xml[content_start..].find(&end_tag) {
                return Some(xml[content_start..content_start + end_pos].to_string());
            }
        }
        None
    }

    /// 提取 XML 属性值
    pub fn get_attribute(xml: &str, attr_name: &str) -> Option<String> {
        let pattern = format!("{}=\"", attr_name);
        if let Some(start_pos) = xml.find(&pattern) {
            let value_start = start_pos + pattern.len();
            if let Some(end_pos) = xml[value_start..].find('"') {
                return Some(xml[value_start..value_start + end_pos].to_string());
            }
        }
        None
    }

    /// 对齐 Java: `XmlUtil.parseXml(String)`
    pub fn parse_xml(xml_str: &str) -> Result<XmlDocument> {
        Self::parse_xml_with_options(xml_str, &XmlParseOptions::default())
    }

    /// Parses a string using an explicit bounded policy.
    pub fn parse_xml_with_options(xml_str: &str, options: &XmlParseOptions) -> Result<XmlDocument> {
        if xml_str.len() > options.max_input_bytes {
            return Err(CoreError::XmlLimit {
                resource: "input bytes",
                max: options.max_input_bytes,
            });
        }
        let cleaned;
        let input = if options.sanitize_invalid_chars {
            cleaned = Self::clean_invalid(xml_str);
            cleaned.as_str()
        } else {
            input_or_xml_error(xml_str)?
        };
        if input.trim().is_empty() {
            return Err(CoreError::InvalidArgument {
                name: "xml_str",
                reason: "XML content string is empty",
            });
        }
        let mut streaming_options = options.clone();
        streaming_options.sanitize_invalid_chars = false;
        Self::read_xml_from_with_options(Cursor::new(input.as_bytes()), &streaming_options)
    }

    /// 对齐 Java: `XmlUtil.readXML(String)`
    pub fn read_xml(path_or_content: &str) -> Result<XmlDocument> {
        if path_or_content.trim_start().starts_with('<') {
            return Self::parse_xml(path_or_content);
        }
        let file = File::open(path_or_content).map_err(CoreError::Io)?;
        Self::read_xml_from(BufReader::new(file))
    }

    /// Builds a DOM from a buffered source without first loading it into a `String`.
    pub fn read_xml_from<R: BufRead>(reader: R) -> Result<XmlDocument> {
        Self::read_xml_from_with_options(reader, &XmlParseOptions::default())
    }

    /// Builds a DOM from a buffered source using explicit defensive limits.
    pub fn read_xml_from_with_options<R: BufRead>(
        reader: R,
        options: &XmlParseOptions,
    ) -> Result<XmlDocument> {
        if options.sanitize_invalid_chars {
            let bytes = read_bounded_and_sanitize(reader, options)?;
            let mut streaming_options = options.clone();
            streaming_options.sanitize_invalid_chars = false;
            return Self::read_xml_from_with_options(Cursor::new(bytes), &streaming_options);
        }
        Self::read_dom_iterative(reader, options.clone())
    }

    /// 对齐 Java: `XmlUtil.cleanComment(String)`
    pub fn clean_comment(xml_content: &str) -> String {
        let mut out = String::with_capacity(xml_content.len());
        let mut idx = 0;
        while let Some(start) = xml_content[idx..].find("<!--") {
            let abs = idx + start;
            out.push_str(&xml_content[idx..abs]);
            if let Some(end_rel) = xml_content[abs + 4..].find("-->") {
                idx = abs + 4 + end_rel + 3;
            } else {
                break;
            }
        }
        out.push_str(&xml_content[idx..]);
        out
    }

    /// 对齐 Java: `XmlUtil.cleanInvalid(String)`
    pub fn clean_invalid(xml_content: &str) -> String {
        xml_content
            .chars()
            .filter(|character| is_valid_xml_char(*character))
            .collect()
    }

    /// 对齐 Java: `XmlUtil.elementText(Element, String)`
    pub fn element_text(element: &XmlNode, tag_name: &str) -> Option<String> {
        Self::get_element(element, tag_name).map(|node| node.text_content())
    }

    /// 对齐 Java: `XmlUtil.getElement(Element, String)`
    pub fn get_element<'a>(element: &'a XmlNode, tag_name: &str) -> Option<&'a XmlNode> {
        element.children.iter().find_map(|child| match child {
            XmlChild::Element(node) if name_matches(&node.tag, tag_name) => Some(node),
            _ => None,
        })
    }

    /// 对齐 Java: `XmlUtil.getByXPath(String, Object, QName)`
    pub fn get_by_xpath(expression: &str, doc: &XmlDocument) -> Option<String> {
        let path = expression.strip_prefix("//").unwrap_or(expression);
        let segments: Vec<&str> = path.split('/').filter(|part| !part.is_empty()).collect();
        let mut current = Some(&doc.root);
        for (index, segment) in segments.iter().enumerate() {
            if index == 0 && current.is_some_and(|node| name_matches(&node.tag, segment)) {
                continue;
            }
            current = current.and_then(|node| Self::find_child_element(node, segment));
        }
        current.map(|node| node.text_content())
    }

    /// 对齐 Java: `XmlUtil.xmlToMap(String)`
    pub fn xml_to_map(xml_str: &str) -> Result<IndexMap<String, Value>> {
        let doc = Self::parse_xml(xml_str)?;
        Ok(Self::xml_node_to_map(&doc.root))
    }

    /// 对齐 Java: `XmlUtil.mapToXml(Map, String)`
    pub fn map_to_xml(data: &IndexMap<String, Value>, root_name: &str) -> Result<XmlDocument> {
        let mut root = XmlNode {
            tag: root_name.to_string(),
            attributes: IndexMap::new(),
            children: Vec::new(),
        };
        Self::append_map(&mut root, data);
        Ok(XmlDocument { root })
    }

    /// 对齐 Java: `XmlUtil.mapToXmlStr(Map, boolean)`
    pub fn map_to_xml_str(
        data: &IndexMap<String, Value>,
        omit_xml_declaration: bool,
    ) -> Result<String> {
        let doc = Self::map_to_xml(data, "xml")?;
        Ok(Self::to_str(&doc, false, omit_xml_declaration))
    }

    /// 对齐 Java: `XmlUtil.toStr(Document, boolean)`
    pub fn to_str(doc: &XmlDocument, is_pretty: bool, omit_xml_declaration: bool) -> String {
        // Writing to an in-memory Vec cannot return an I/O error, and all
        // emitted content originates from UTF-8 Rust strings.
        Self::to_string_result(doc, is_pretty, omit_xml_declaration)
            .expect("writing XML to an in-memory buffer must succeed")
    }

    /// Serializes a DOM using `quick_xml::Writer`.
    pub fn to_string_result(
        doc: &XmlDocument,
        is_pretty: bool,
        omit_xml_declaration: bool,
    ) -> Result<String> {
        let mut output = Vec::new();
        Self::write_xml_to(&mut output, doc, is_pretty, omit_xml_declaration)?;
        String::from_utf8(output).map_err(|error| CoreError::Xml(error.to_string()))
    }

    /// Writes a DOM directly to an application-owned output stream.
    pub fn write_xml_to<W: Write>(
        target: W,
        doc: &XmlDocument,
        is_pretty: bool,
        omit_xml_declaration: bool,
    ) -> Result<W> {
        let mut writer = if is_pretty {
            XmlEventWriter::with_indent(target, b' ', 2)
        } else {
            XmlEventWriter::new(target)
        };
        writer.set_space_before_empty_slash(true);
        if !omit_xml_declaration {
            writer.write_event(Event::Decl(BytesDecl::new(
                "1.0",
                Some("UTF-8"),
                Some("no"),
            )))?;
        }
        write_dom_iterative(&mut writer, &doc.root)?;
        Ok(writer.into_inner())
    }

    /// 对齐 Java: `XmlUtil.format(Document)`
    pub fn format(doc: &XmlDocument) -> String {
        Self::to_str(doc, true, false)
    }

    /// 对齐 Java: `XmlUtil.beanToXml(Object, String, boolean)`
    pub fn bean_to_xml(
        bean: &impl Serialize,
        root_name: &str,
        namespace: Option<&str>,
        ignore_null: bool,
    ) -> Result<XmlDocument> {
        let value = serde_json::to_value(bean).map_err(|err| CoreError::Codec(err.to_string()))?;
        let map = Self::value_to_map(value, ignore_null);
        let mut doc = Self::map_to_xml(&map, root_name)?;
        if let Some(ns) = namespace {
            doc.root
                .attributes
                .insert("xmlns".to_string(), ns.to_string());
        }
        Ok(doc)
    }

    /// 对齐 Java: `XmlUtil.xmlToBean(Node, Class)`
    pub fn xml_to_bean<T: DeserializeOwned>(doc: &XmlDocument) -> Result<T> {
        let map = Self::xml_node_to_map(&doc.root);
        let json_map: Map<String, Value> = map.into_iter().collect();
        serde_json::from_value(Value::Object(json_map))
            .map_err(|err| CoreError::Codec(err.to_string()))
    }

    /// 对齐 Java: `XmlUtil.readBySax(InputStream, ContentHandler)`
    pub fn read_by_sax(xml: &str, mut handler: impl FnMut(&str)) -> Result<()> {
        let options = XmlParseOptions::default();
        let _: ControlFlow<()> =
            visit_xml(Cursor::new(xml.as_bytes()), options.clone(), |event| {
                if let Event::Start(start) | Event::Empty(start) = event {
                    let name = element_name(start, options.namespace_mode)?;
                    handler(&name);
                }
                Ok(ControlFlow::Continue(()))
            })?;
        Ok(())
    }

    /// Visits a buffered XML source and supports early termination.
    pub fn visit_xml<R, B, F>(
        source: R,
        options: XmlParseOptions,
        visitor: F,
    ) -> Result<ControlFlow<B>>
    where
        R: BufRead,
        F: for<'event> FnMut(&Event<'event>) -> Result<ControlFlow<B>>,
    {
        visit_xml(source, options, visitor)
    }

    /// Streams validated events through a filtering writer transform.
    pub fn transform_xml<R, W, F>(
        source: R,
        target: W,
        options: XmlParseOptions,
        transform: F,
    ) -> Result<W>
    where
        R: BufRead,
        W: Write,
        F: for<'event> FnMut(&Event<'event>) -> Result<XmlTransformAction>,
    {
        transform_xml(source, target, options, transform)
    }

    fn read_dom_iterative<R: BufRead>(source: R, options: XmlParseOptions) -> Result<XmlDocument> {
        let mut reader = XmlEventReader::new(source, options.clone());
        let mut stack = Vec::new();
        let mut root = None;
        loop {
            let version = reader.xml_version();
            let decoder = reader.decoder();
            let event = reader.read_event()?;
            match event {
                Event::Start(start) => stack.push(XmlNode {
                    tag: element_name(&start, options.namespace_mode)?,
                    attributes: read_attributes(&start, options.namespace_mode, version)?,
                    children: Vec::new(),
                }),
                Event::Empty(start) => {
                    let node = XmlNode {
                        tag: element_name(&start, options.namespace_mode)?,
                        attributes: read_attributes(&start, options.namespace_mode, version)?,
                        children: Vec::new(),
                    };
                    attach_node(node, &mut stack, &mut root)?;
                }
                Event::Text(text) => {
                    let value = text
                        .decode()
                        .map_err(|error| CoreError::Xml(error.to_string()))?
                        .into_owned();
                    if !value.is_empty() {
                        let parent = stack
                            .last_mut()
                            .ok_or_else(|| CoreError::Xml("text outside root".to_owned()))?;
                        append_text(parent, value);
                    }
                }
                Event::CData(text) => {
                    let value = text
                        .decode()
                        .map_err(|error| CoreError::Xml(error.to_string()))?
                        .into_owned();
                    let parent = stack
                        .last_mut()
                        .ok_or_else(|| CoreError::Xml("CDATA outside root".to_owned()))?;
                    append_text(parent, value);
                }
                Event::GeneralRef(reference) => {
                    let value = resolve_reference(&reference, &options)?;
                    let parent = stack.last_mut().ok_or_else(|| {
                        CoreError::Xml("general reference outside root".to_owned())
                    })?;
                    append_text(parent, value);
                }
                Event::End(end) => {
                    let expected = end_name(&end, decoder, options.namespace_mode)?;
                    let node = stack
                        .pop()
                        .ok_or_else(|| CoreError::Xml("closing tag outside root".to_owned()))?;
                    if node.tag != expected {
                        return Err(CoreError::Xml(format!(
                            "mismatched end tag: expected {}, got {expected}",
                            node.tag
                        )));
                    }
                    attach_node(node, &mut stack, &mut root)?;
                }
                Event::Eof => break,
                Event::Decl(_) | Event::PI(_) | Event::DocType(_) | Event::Comment(_) => {}
            }
        }
        let root = root.ok_or_else(|| CoreError::Xml("missing root element".to_owned()))?;
        Ok(XmlDocument { root })
    }

    fn find_child_element<'a>(node: &'a XmlNode, tag_name: &str) -> Option<&'a XmlNode> {
        node.children.iter().find_map(|child| match child {
            XmlChild::Element(child_node) if name_matches(&child_node.tag, tag_name) => {
                Some(child_node)
            }
            _ => None,
        })
    }

    fn xml_node_to_map(node: &XmlNode) -> IndexMap<String, Value> {
        let mut result = IndexMap::new();
        for child in &node.children {
            if let XmlChild::Element(element) = child {
                let key = element.tag.clone();
                let value = if element
                    .children
                    .iter()
                    .all(|item| matches!(item, XmlChild::Text(_)))
                {
                    Value::String(element.text_content())
                } else {
                    let nested = Self::xml_node_to_map(element);
                    if nested.is_empty() {
                        Value::String(element.text_content())
                    } else {
                        Value::Object(nested.into_iter().collect())
                    }
                };
                Self::insert_map_value(&mut result, key, value);
            }
        }
        result
    }

    fn insert_map_value(map: &mut IndexMap<String, Value>, key: String, value: Value) {
        if let Some(existing) = map.get(&key).cloned() {
            match existing {
                Value::Array(mut items) => {
                    items.push(value);
                    map.insert(key, Value::Array(items));
                }
                other => {
                    map.insert(key, Value::Array(vec![other, value]));
                }
            }
        } else {
            map.insert(key, value);
        }
    }

    fn append_map(node: &mut XmlNode, data: &IndexMap<String, Value>) {
        for (key, value) in data {
            Self::append_value(node, key, value);
        }
    }

    fn append_value(parent: &mut XmlNode, key: &str, value: &Value) {
        match value {
            Value::Null => parent.children.push(XmlChild::Element(XmlNode {
                tag: key.to_string(),
                attributes: IndexMap::new(),
                children: Vec::new(),
            })),
            Value::Bool(v) => parent.children.push(XmlChild::Element(XmlNode {
                tag: key.to_string(),
                attributes: IndexMap::new(),
                children: vec![XmlChild::Text(v.to_string())],
            })),
            Value::Number(v) => parent.children.push(XmlChild::Element(XmlNode {
                tag: key.to_string(),
                attributes: IndexMap::new(),
                children: vec![XmlChild::Text(v.to_string())],
            })),
            Value::String(v) => parent.children.push(XmlChild::Element(XmlNode {
                tag: key.to_string(),
                attributes: IndexMap::new(),
                children: vec![XmlChild::Text(v.clone())],
            })),
            Value::Array(items) => {
                for item in items {
                    Self::append_value(parent, key, item);
                }
            }
            Value::Object(map) => {
                let mut child = XmlNode {
                    tag: key.to_string(),
                    attributes: IndexMap::new(),
                    children: Vec::new(),
                };
                let ordered: IndexMap<String, Value> =
                    map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                Self::append_map(&mut child, &ordered);
                parent.children.push(XmlChild::Element(child));
            }
        }
    }

    fn value_to_map(value: Value, ignore_null: bool) -> IndexMap<String, Value> {
        match value {
            Value::Object(map) => map
                .into_iter()
                .filter(|(_, v)| !(ignore_null && v.is_null()))
                .collect(),
            _ => IndexMap::new(),
        }
    }
}

impl XmlNode {
    /// 返回当前节点全部文本内容。
    pub fn text_content(&self) -> String {
        self.children
            .iter()
            .filter_map(|child| match child {
                XmlChild::Text(text) => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /// 读取属性值。
    pub fn attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(String::as_str)
    }
}

use super::{append_text, attach_node, input_or_xml_error, name_matches, write_dom_iterative};
