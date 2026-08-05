//! 对齐: `cn.hutool.core.util.XmlUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/XmlUtil.java
//!
//! Rust 版本基于 `quick-xml` 提供 DOM 风格 XML 操作。

use std::io::Write;

use quick_xml::{
    escape::escape,
    events::{BytesEnd, BytesStart, BytesText, Event},
    name::QName,
};

use crate::xml_stream::is_valid_xml_char;
use crate::{CoreError, Result, XmlEventWriter};

mod xml_child;
mod xml_document;
mod xml_node;
mod xml_util;

pub use xml_child::XmlChild;
pub use xml_document::XmlDocument;
pub use xml_node::XmlNode;
pub use xml_util::XmlUtil;

fn input_or_xml_error(input: &str) -> Result<&str> {
    if input.chars().all(is_valid_xml_char) {
        Ok(input)
    } else {
        Err(CoreError::Xml("illegal XML character".to_owned()))
    }
}

fn attach_node(node: XmlNode, stack: &mut [XmlNode], root: &mut Option<XmlNode>) -> Result<()> {
    if let Some(parent) = stack.last_mut() {
        parent.children.push(XmlChild::Element(node));
        return Ok(());
    }
    if root.replace(node).is_some() {
        return Err(CoreError::Xml("multiple root elements".to_owned()));
    }
    Ok(())
}

fn append_text(node: &mut XmlNode, value: String) {
    if let Some(XmlChild::Text(text)) = node.children.last_mut() {
        text.push_str(&value);
    } else {
        node.children.push(XmlChild::Text(value));
    }
}

fn name_matches(actual: &str, requested: &str) -> bool {
    actual == requested || local_part(actual) == local_part(requested)
}

fn local_part(name: &str) -> &str {
    name.split_once(':')
        .map_or(name, |(_, local_name)| local_name)
}

enum WriteFrame<'node> {
    Node(&'node XmlNode),
    Text(&'node str),
    End(&'node str),
}

fn write_dom_iterative<W: Write>(writer: &mut XmlEventWriter<W>, root: &XmlNode) -> Result<()> {
    let mut stack = vec![WriteFrame::Node(root)];
    while let Some(frame) = stack.pop() {
        match frame {
            WriteFrame::Node(node) => {
                let mut start = BytesStart::new(node.tag.as_str());
                for (key, value) in &node.attributes {
                    start.push_attribute(quick_xml::events::attributes::Attribute {
                        key: QName(key.as_bytes()),
                        value: escape(value).into_owned().into_bytes().into(),
                    });
                }
                if node.children.is_empty() {
                    writer.write_event(Event::Empty(start))?;
                    continue;
                }
                writer.write_event(Event::Start(start))?;
                stack.push(WriteFrame::End(&node.tag));
                for child in node.children.iter().rev() {
                    match child {
                        XmlChild::Element(element) => {
                            stack.push(WriteFrame::Node(element));
                        }
                        XmlChild::Text(text) => stack.push(WriteFrame::Text(text)),
                    }
                }
            }
            WriteFrame::Text(text) => {
                writer.write_event(Event::Text(BytesText::new(text)))?;
            }
            WriteFrame::End(name) => {
                writer.write_event(Event::End(BytesEnd::new(name)))?;
            }
        }
    }
    Ok(())
}
