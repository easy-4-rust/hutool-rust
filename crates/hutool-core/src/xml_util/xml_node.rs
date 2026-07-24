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
    escape::escape,
    events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event},
    name::QName,
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

/// XML 元素节点。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlNode {
    /// 标签名。
    pub tag: String,
    /// 属性集合。
    pub attributes: IndexMap<String, String>,
    /// 子节点。
    pub children: Vec<XmlChild>,
}

use super::{WriteFrame, append_text, attach_node, input_or_xml_error, local_part, name_matches, write_dom_iterative};
