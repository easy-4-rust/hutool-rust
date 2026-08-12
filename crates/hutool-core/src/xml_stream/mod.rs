//! Bounded streaming XML reader, visitor, transformer, and writer.

/// 对齐: `cn.hutool.core.xml.XmlStream`
/// XML 流解析
use std::{
    io::{BufRead, Read, Write},
    ops::ControlFlow,
};

use indexmap::IndexMap;
use quick_xml::{
    XmlVersion,
    escape::resolve_predefined_entity,
    events::{BytesEnd, BytesRef, BytesStart, Event},
};

use crate::{CoreError, Result};

mod namespace_mode;
mod xml_event_reader;
mod xml_event_writer;
mod xml_parse_options;
mod xml_transform_action;

pub use namespace_mode::NamespaceMode;
pub use xml_event_reader::XmlEventReader;
pub use xml_event_writer::XmlEventWriter;
pub use xml_parse_options::XmlParseOptions;
pub use xml_transform_action::XmlTransformAction;

struct ParseState {
    depth: usize,
    nodes: usize,
    text_bytes: usize,
    root_seen: bool,
    root_closed: bool,
    version: XmlVersion,
}

/// 流式访问 XML 事件。
///
/// # Errors
///
/// 解析出错时返回 [`CoreError`]。
pub fn visit_xml<R, B, F>(
    source: R,
    options: XmlParseOptions,
    mut visitor: F,
) -> Result<ControlFlow<B>>
where
    R: BufRead,
    F: for<'event> FnMut(&Event<'event>) -> Result<ControlFlow<B>>,
{
    let mut reader = XmlEventReader::new(source, options);
    loop {
        let event = reader.read_event()?;
        if matches!(event, Event::Eof) {
            return Ok(ControlFlow::Continue(()));
        }
        if let ControlFlow::Break(value) = visitor(&event)? {
            return Ok(ControlFlow::Break(value));
        }
    }
}

/// 流式转换 XML 事件到 Writer。
///
/// # Errors
///
/// 解析或写出出错时返回 [`CoreError`]。
pub fn transform_xml<R, W, F>(
    source: R,
    target: W,
    options: XmlParseOptions,
    mut transform: F,
) -> Result<W>
where
    R: BufRead,
    W: Write,
    F: for<'event> FnMut(&Event<'event>) -> Result<XmlTransformAction>,
{
    let mut reader = XmlEventReader::new(source, options);
    let mut writer = XmlEventWriter::new(target);
    loop {
        let event = reader.read_event()?;
        if matches!(event, Event::Eof) {
            return Ok(writer.into_inner());
        }
        if transform(&event)? == XmlTransformAction::Keep {
            writer.write_event(event.borrow())?;
        }
    }
}

fn validate_event(
    event: &Event<'_>,
    options: &XmlParseOptions,
    state: &mut ParseState,
) -> Result<()> {
    match event {
        Event::Decl(declaration) => {
            let version = declaration
                .version()
                .map_err(|error| CoreError::Xml(error.to_string()))?;
            state.version = if version.as_ref() == b"1.1" {
                XmlVersion::Explicit1_1
            } else {
                XmlVersion::Explicit1_0
            };
        }
        Event::Start(start) => {
            begin_element(start, options, state)?;
            state.depth += 1;
            if state.depth > options.max_depth {
                return Err(CoreError::XmlLimit {
                    resource: "depth",
                    max: options.max_depth,
                });
            }
        }
        Event::Empty(start) => {
            begin_element(start, options, state)?;
            if state.depth.saturating_add(1) > options.max_depth {
                return Err(CoreError::XmlLimit {
                    resource: "depth",
                    max: options.max_depth,
                });
            }
            if state.depth == 0 {
                state.root_closed = true;
            }
        }
        Event::End(_) => {
            if state.depth == 0 {
                return Err(CoreError::Xml(
                    "closing tag outside the root element".to_owned(),
                ));
            }
            state.depth -= 1;
            if state.depth == 0 {
                state.root_closed = true;
            }
        }
        Event::Text(text) => {
            let value = text
                .decode()
                .map_err(|error| CoreError::Xml(error.to_string()))?;
            validate_text(value.as_ref(), options, state)?;
        }
        Event::CData(text) => {
            let value = text
                .decode()
                .map_err(|error| CoreError::Xml(error.to_string()))?;
            validate_text(value.as_ref(), options, state)?;
        }
        Event::DocType(_) if !options.allow_doctype => {
            return Err(CoreError::XmlForbidden("DOCTYPE"));
        }
        Event::GeneralRef(reference) => {
            let value = resolve_reference(reference, options)?;
            validate_text(&value, options, state)?;
        }
        Event::Eof => {
            if state.depth != 0 {
                return Err(CoreError::Xml(
                    "unexpected EOF inside an element".to_owned(),
                ));
            }
            if !state.root_seen {
                return Err(CoreError::Xml("missing root element".to_owned()));
            }
        }
        Event::Comment(_) | Event::PI(_) | Event::DocType(_) => {}
    }
    Ok(())
}

fn begin_element(
    start: &BytesStart<'_>,
    options: &XmlParseOptions,
    state: &mut ParseState,
) -> Result<()> {
    if state.depth == 0 {
        if state.root_seen || state.root_closed {
            return Err(CoreError::Xml("multiple root elements".to_owned()));
        }
        state.root_seen = true;
    }
    state.nodes += 1;
    if state.nodes > options.max_nodes {
        return Err(CoreError::XmlLimit {
            resource: "node count",
            max: options.max_nodes,
        });
    }
    validate_attributes(start, options, state.version)?;
    Ok(())
}

fn validate_attributes(
    start: &BytesStart<'_>,
    options: &XmlParseOptions,
    version: XmlVersion,
) -> Result<()> {
    let mut count = 0;
    for attribute in start.attributes() {
        count += 1;
        if count > options.max_attributes_per_element {
            return Err(CoreError::XmlLimit {
                resource: "attributes per element",
                max: options.max_attributes_per_element,
            });
        }
        let attribute = attribute.map_err(|error| CoreError::Xml(error.to_string()))?;
        let value = attribute
            .decoded_and_normalized_value(version, start.decoder())
            .map_err(|error| CoreError::Xml(error.to_string()))?;
        validate_xml_chars(value.as_ref())?;
    }
    Ok(())
}

fn validate_text(value: &str, options: &XmlParseOptions, state: &mut ParseState) -> Result<()> {
    validate_xml_chars(value)?;
    if state.depth == 0 && !value.trim().is_empty() {
        return Err(CoreError::Xml("text outside the root element".to_owned()));
    }
    state.text_bytes = state.text_bytes.saturating_add(value.len());
    if state.text_bytes > options.max_text_bytes {
        return Err(CoreError::XmlLimit {
            resource: "text bytes",
            max: options.max_text_bytes,
        });
    }
    Ok(())
}

pub(crate) fn element_name(event: &BytesStart<'_>, mode: NamespaceMode) -> Result<String> {
    decode_name(
        event.decoder(),
        event.name().as_ref(),
        event.local_name().as_ref(),
        mode,
    )
}

pub(crate) fn end_name(
    event: &BytesEnd<'_>,
    decoder: quick_xml::encoding::Decoder,
    mode: NamespaceMode,
) -> Result<String> {
    decode_name(
        decoder,
        event.name().as_ref(),
        event.local_name().as_ref(),
        mode,
    )
}

fn decode_name(
    decoder: quick_xml::encoding::Decoder,
    qualified: &[u8],
    local: &[u8],
    mode: NamespaceMode,
) -> Result<String> {
    let bytes = match mode {
        NamespaceMode::Preserve => qualified,
        NamespaceMode::LocalName => local,
    };
    decoder
        .decode(bytes)
        .map(|value| value.into_owned())
        .map_err(|error| CoreError::Xml(error.to_string()))
}

pub(crate) fn read_attributes(
    event: &BytesStart<'_>,
    mode: NamespaceMode,
    version: XmlVersion,
) -> Result<IndexMap<String, String>> {
    let mut attributes = IndexMap::new();
    for attribute in event.attributes() {
        let attribute = attribute.map_err(|error| CoreError::Xml(error.to_string()))?;
        let qualified = attribute.key.as_ref();
        let local = qualified
            .splitn(2, |byte| *byte == b':')
            .nth(1)
            .unwrap_or(qualified);
        let name_bytes = if qualified == b"xmlns" || qualified.starts_with(b"xmlns:") {
            qualified
        } else {
            match mode {
                NamespaceMode::Preserve => qualified,
                NamespaceMode::LocalName => local,
            }
        };
        let key = event
            .decoder()
            .decode(name_bytes)
            .map(|value| value.into_owned())
            .map_err(|error| CoreError::Xml(error.to_string()))?;
        let value = attribute
            .decoded_and_normalized_value(version, event.decoder())
            .map_err(|error| CoreError::Xml(error.to_string()))?
            .into_owned();
        validate_xml_chars(&value)?;
        attributes.insert(key, value);
    }
    Ok(attributes)
}

pub(crate) fn resolve_reference(
    reference: &BytesRef<'_>,
    options: &XmlParseOptions,
) -> Result<String> {
    if let Some(character) = reference
        .resolve_char_ref()
        .map_err(|error| CoreError::Xml(error.to_string()))?
    {
        validate_xml_chars(&character.to_string())?;
        return Ok(character.to_string());
    }
    let name = reference
        .decode()
        .map_err(|error| CoreError::Xml(error.to_string()))?;
    if let Some(value) = resolve_predefined_entity(name.as_ref()) {
        return Ok(value.to_owned());
    }
    if options.allow_general_references {
        Ok(format!("&{name};"))
    } else {
        Err(CoreError::XmlForbidden("unknown general reference"))
    }
}

pub(crate) fn validate_xml_chars(value: &str) -> Result<()> {
    if value.chars().all(is_valid_xml_char) {
        Ok(())
    } else {
        Err(CoreError::Xml("illegal XML character".to_owned()))
    }
}

pub(crate) fn is_valid_xml_char(character: char) -> bool {
    matches!(
        character,
        '\u{9}'
            | '\u{A}'
            | '\u{D}'
            | '\u{20}'..='\u{D7FF}'
            | '\u{E000}'..='\u{FFFD}'
            | '\u{10000}'..='\u{10FFFF}'
    )
}

pub(crate) fn read_bounded_and_sanitize<R: BufRead>(
    source: R,
    options: &XmlParseOptions,
) -> Result<Vec<u8>> {
    let limit = u64::try_from(options.max_input_bytes)
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    let mut bytes = Vec::new();
    source
        .take(limit)
        .read_to_end(&mut bytes)
        .map_err(CoreError::Io)?;
    if bytes.len() > options.max_input_bytes {
        return Err(CoreError::XmlLimit {
            resource: "input bytes",
            max: options.max_input_bytes,
        });
    }
    let text = String::from_utf8(bytes).map_err(|error| CoreError::Xml(error.to_string()))?;
    Ok(text
        .chars()
        .filter(|character| is_valid_xml_char(*character))
        .collect::<String>()
        .into_bytes())
}
