//! Bounded streaming XML reader, visitor, transformer, and writer.

/// 对齐: `cn.hutool.core.xml.XmlStream`
/// XML转换动作

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

/// Action returned by a streaming transform callback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTransformAction {
    /// Copy the current event to the target.
    Keep,
    /// Omit the current event from the target.
    Drop,
}

use super::{ParseState, begin_element, decode_name, validate_attributes, validate_event, validate_text};
