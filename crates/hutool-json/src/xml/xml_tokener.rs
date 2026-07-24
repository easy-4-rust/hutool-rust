use std::fmt::Write as _;

use quick_xml::{Reader, XmlVersion, events::Event};
use serde_json::{Map, Number, Value};

use crate::{JSONConfig, JSONObject, JsonError, ParseConfig, Result};

use super::xml::XML;

/// XML tokenizer backed by the bounded conversion engine.
pub struct XMLTokener;

impl XMLTokener {
    /// Parses XML through the shared bounded engine.
    pub fn parse(input: &str, config: ParseConfig) -> Result<JSONObject> {
        XML::to_json_with(input, config)
    }
}

use super::{Element, attach, display_scalar, escape_name, escape_text, finish_element, scalar, write_value};
