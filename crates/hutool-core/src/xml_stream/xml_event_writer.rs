//! Bounded streaming XML reader, visitor, transformer, and writer.

/// 对齐: `cn.hutool.core.xml.XmlStream`
/// XML事件写入器

use std::io::Write;

use quick_xml::{
    events::Event,
    Writer,
};

use crate::{CoreError, Result};

/// Streaming XML writer backed by `quick_xml::Writer`.
pub struct XmlEventWriter<W: Write> {
    writer: Writer<W>,
}

impl<W: Write> std::fmt::Debug for XmlEventWriter<W> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("XmlEventWriter")
            .finish_non_exhaustive()
    }
}

impl<W: Write> XmlEventWriter<W> {
    /// Creates a compact writer.
    #[must_use]
    pub fn new(target: W) -> Self {
        Self {
            writer: Writer::new(target),
        }
    }

    /// Creates a writer that indents structural events.
    #[must_use]
    pub fn with_indent(target: W, indent_char: u8, indent_size: usize) -> Self {
        Self {
            writer: Writer::new_with_indent(target, indent_char, indent_size),
        }
    }

    /// Selects whether empty elements are written as `<tag />` instead of `<tag/>`.
    pub fn set_space_before_empty_slash(&mut self, enabled: bool) {
        self.writer
            .config_mut()
            .add_space_before_slash_in_empty_elements = enabled;
    }

    /// Writes one XML event.
    pub fn write_event(&mut self, event: Event<'_>) -> Result<()> {
        self.writer.write_event(event).map_err(CoreError::Io)
    }

    /// Returns the underlying writer.
    #[must_use]
    pub fn into_inner(self) -> W {
        self.writer.into_inner()
    }
}
