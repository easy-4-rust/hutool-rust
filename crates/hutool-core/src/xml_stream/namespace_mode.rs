//! Bounded streaming XML reader, visitor, transformer, and writer.

/// 对齐: `cn.hutool.core.xml.XmlStream`
/// 命名空间模式

use std::{
    io::{BufRead, Read, Take, Write},
    ops::ControlFlow,
};

use indexmap::IndexMap;
use quick_xml::{
    escape::resolve_predefined_entity,
    events::{BytesEnd, BytesRef, BytesStart, Event},
    Reader, Writer, XmlVersion,
};

use crate::{CoreError, Result};

/// Namespace handling applied when XML names are copied into the DOM.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NamespaceMode {
    /// Keep qualified names such as `soap:Envelope`.
    #[default]
    Preserve,
    /// Keep only the local part such as `Envelope`.
    LocalName,
}

use super::{ParseState, begin_element, decode_name, validate_attributes, validate_event, validate_text};
