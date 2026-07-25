use std::fmt::Write as _;

use quick_xml::{Reader, XmlVersion, events::Event};
use serde_json::{Map, Number, Value};

use crate::{JSONConfig, JSONObject, JsonError, ParseConfig, Result};

use super::xml::XML;

/// 对齐: `cn.hutool.json.JSONXMLSerializer`
/// 中文说明: 为 Hutool 迁移保留的序列化器类型别名，等同于 [`XML`]。
///
/// Serializer alias retained for Hutool migration.
pub type JSONXMLSerializer = XML;

use super::{Element, attach, display_scalar, escape_name, escape_text, finish_element, scalar, write_value};
