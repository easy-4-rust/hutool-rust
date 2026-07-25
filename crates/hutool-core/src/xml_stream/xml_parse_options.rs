//! Bounded streaming XML reader, visitor, transformer, and writer.

/// 对齐: `cn.hutool.core.xml.XmlStream`
/// XML解析选项

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

use super::namespace_mode::NamespaceMode;

/// Defensive limits and parsing policy for XML input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlParseOptions {
    /// Maximum number of input bytes consumed by the parser.
    pub max_input_bytes: usize,
    /// Maximum element nesting depth.
    pub max_depth: usize,
    /// Maximum number of elements, including empty elements.
    pub max_nodes: usize,
    /// Maximum number of attributes on one element.
    pub max_attributes_per_element: usize,
    /// Maximum cumulative decoded text and CDATA bytes.
    pub max_text_bytes: usize,
    /// Whether insignificant leading and trailing text whitespace is removed.
    pub trim_text: bool,
    /// Namespace name handling used by DOM construction.
    pub namespace_mode: NamespaceMode,
    /// Whether a document type declaration may be observed.
    pub allow_doctype: bool,
    /// Whether unknown named general references are retained literally.
    ///
    /// Numeric references and the five predefined XML references are always
    /// accepted. This option never expands declarations from a DTD.
    pub allow_general_references: bool,
    /// Whether illegal XML control characters are removed before string parsing.
    ///
    /// Generic `BufRead` input is buffered only when this is explicitly enabled.
    pub sanitize_invalid_chars: bool,
}

impl Default for XmlParseOptions {
    fn default() -> Self {
        Self {
            max_input_bytes: 16 * 1024 * 1024,
            max_depth: 128,
            max_nodes: 100_000,
            max_attributes_per_element: 256,
            max_text_bytes: 8 * 1024 * 1024,
            trim_text: true,
            namespace_mode: NamespaceMode::Preserve,
            allow_doctype: false,
            allow_general_references: false,
            sanitize_invalid_chars: false,
        }
    }
}

impl Default for ParseState {
    fn default() -> Self {
        Self {
            depth: 0,
            nodes: 0,
            text_bytes: 0,
            root_seen: false,
            root_closed: false,
            version: XmlVersion::Implicit1_0,
        }
    }
}

use super::{ParseState, begin_element, decode_name, validate_attributes, validate_event, validate_text};
