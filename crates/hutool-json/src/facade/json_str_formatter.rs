use std::io::Write;

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::{
    JSONArray, JSONConfig, JSONObject, JsonContainer, JsonError, Result, get_by_path, put_by_path,
};

/// Pretty-formatting facade corresponding to Hutool's `JSONStrFormatter`.
pub struct JSONStrFormatter;

impl JSONStrFormatter {
    /// Formats one complete JSON document.
    pub fn format(input: &str) -> Result<String> {
        crate::pretty(input)
    }
}

use super::{WriterMode, normalize_writer_value};
