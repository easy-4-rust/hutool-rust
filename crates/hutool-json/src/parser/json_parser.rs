use std::io::Read;

use serde_json::Value;

use crate::{JSONArray, JSONConfig, JSONObject, JsonError, Result};

use super::json_tokener::JSONTokener;

/// Parser facade that creates configured containers.
#[derive(Debug, Clone)]
pub struct JSONParser {
    tokener: JSONTokener,
}

impl JSONParser {
    /// Creates a parser around an owned tokenizer.
    #[must_use]
    pub const fn new(tokener: JSONTokener) -> Self {
        Self { tokener }
    }

    /// Parses an object.
    pub fn parse_object(&mut self) -> Result<JSONObject> {
        let config = self.tokener.config.clone();
        JSONObject::from_value(self.tokener.next_value()?, config)
    }

    /// Parses an array.
    pub fn parse_array(&mut self) -> Result<JSONArray> {
        let config = self.tokener.config.clone();
        JSONArray::from_value(self.tokener.next_value()?, config)
    }
}
