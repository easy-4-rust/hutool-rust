use std::fmt::Write as _;

use quick_xml::{Reader, XmlVersion, events::Event};
use serde_json::{Map, Number, Value};

use crate::{JSONConfig, JSONObject, JsonError, ParseConfig, Result};

use super::xml::XML;

/// Serializer alias retained for Hutool migration.
pub type JSONXMLSerializer = XML;

use super::{Element, attach, display_scalar, escape_name, escape_text, finish_element, scalar, write_value};
