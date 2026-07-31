//! Bounded streaming XML reader, visitor, transformer, and writer.

/// 对齐: `cn.hutool.core.xml.XmlStream`
/// XML事件读取器

use std::io::{BufRead, Take};

use quick_xml::{
    events::Event,
    Reader, XmlVersion,
};

use crate::{CoreError, Result};

use super::xml_parse_options::XmlParseOptions;

/// Pull reader that reuses one allocation buffer and enforces [`XmlParseOptions`].
pub struct XmlEventReader<R: BufRead> {
    reader: Reader<Take<R>>,
    buffer: Vec<u8>,
    options: XmlParseOptions,
    state: ParseState,
}

impl<R: BufRead> std::fmt::Debug for XmlEventReader<R> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("XmlEventReader")
            .field("buffer_capacity", &self.buffer.capacity())
            .field("options", &self.options)
            .field("depth", &self.state.depth)
            .field("nodes", &self.state.nodes)
            .field("text_bytes", &self.state.text_bytes)
            .finish_non_exhaustive()
    }
}

impl<R: BufRead> XmlEventReader<R> {
    /// Creates a bounded reader. No input is consumed until [`Self::read_event`].
    #[must_use]
    pub fn new(source: R, options: XmlParseOptions) -> Self {
        let limit = u64::try_from(options.max_input_bytes)
            .unwrap_or(u64::MAX)
            .saturating_add(1);
        let mut reader = Reader::from_reader(source.take(limit));
        reader.config_mut().trim_text(options.trim_text);
        Self {
            reader,
            buffer: Vec::new(),
            options,
            state: ParseState::default(),
        }
    }

    /// Reads the next borrowed event.
    ///
    /// The returned event borrows the reader's single reusable buffer, so it
    /// must be dropped before reading another event.
    pub fn read_event(&mut self) -> Result<Event<'_>> {
        self.buffer.clear();
        let event = self.reader.read_event_into(&mut self.buffer);
        let position = usize::try_from(self.reader.buffer_position()).unwrap_or(usize::MAX);
        if position > self.options.max_input_bytes {
            return Err(CoreError::XmlLimit {
                resource: "input bytes",
                max: self.options.max_input_bytes,
            });
        }
        let event = event.map_err(|error| CoreError::Xml(error.to_string()))?;
        validate_event(&event, &self.options, &mut self.state)?;
        Ok(event)
    }

    /// Returns the current reusable buffer capacity.
    #[must_use]
    pub fn buffer_capacity(&self) -> usize {
        self.buffer.capacity()
    }

    /// Returns the active parsing policy.
    #[must_use]
    pub fn options(&self) -> &XmlParseOptions {
        &self.options
    }

    /// Returns the XML version observed in the declaration, or implicit XML 1.0.
    #[must_use]
    pub(crate) fn xml_version(&self) -> XmlVersion {
        self.state.version
    }

    /// Returns the decoder selected by the XML declaration.
    #[must_use]
    pub(crate) fn decoder(&self) -> quick_xml::encoding::Decoder {
        self.reader.decoder()
    }
}

use super::{ParseState, validate_event};