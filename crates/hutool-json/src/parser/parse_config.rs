use std::io::Read;

use serde_json::Value;

use crate::{JSONArray, JSONConfig, JSONObject, JsonError, Result};

/// Defensive parser options corresponding to Hutool's `ParseConfig`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseConfig {
    keep_strings: bool,
    max_nesting_depth: usize,
    pub(crate) max_input_bytes: usize,
}

impl Default for ParseConfig {
    fn default() -> Self {
        Self {
            keep_strings: false,
            max_nesting_depth: 128,
            max_input_bytes: 16 * 1024 * 1024,
        }
    }
}

impl ParseConfig {
    /// Creates production defaults.
    #[must_use]
    pub fn create() -> Self {
        Self::default()
    }

    /// Returns whether scalar XML values remain strings.
    #[must_use]
    pub const fn is_keep_strings(&self) -> bool {
        self.keep_strings
    }

    /// Configures scalar XML conversion.
    pub const fn set_keep_strings(&mut self, value: bool) -> &mut Self {
        self.keep_strings = value;
        self
    }

    /// Returns the maximum structured nesting depth.
    #[must_use]
    pub const fn max_nesting_depth(&self) -> usize {
        self.max_nesting_depth
    }

    /// Replaces the structured nesting limit.
    pub const fn set_max_nesting_depth(&mut self, value: usize) -> &mut Self {
        self.max_nesting_depth = value;
        self
    }

    /// Returns the maximum encoded input size.
    #[must_use]
    pub const fn max_input_bytes(&self) -> usize {
        self.max_input_bytes
    }

    /// Replaces the encoded input limit.
    pub const fn set_max_input_bytes(&mut self, value: usize) -> &mut Self {
        self.max_input_bytes = value;
        self
    }

    pub(crate) fn validate(&self, input: &str) -> Result<()> {
        if input.len() > self.max_input_bytes {
            return Err(JsonError::Limit("input bytes"));
        }
        let mut depth = 0_usize;
        let mut quoted = false;
        let mut escaped = false;
        for character in input.chars() {
            if quoted {
                if escaped {
                    escaped = false;
                } else if character == '\\' {
                    escaped = true;
                } else if character == '"' {
                    quoted = false;
                }
                continue;
            }
            match character {
                '"' => quoted = true,
                '{' | '[' => {
                    depth = depth.saturating_add(1);
                    if depth > self.max_nesting_depth {
                        return Err(JsonError::Limit("nesting depth"));
                    }
                }
                '}' | ']' => depth = depth.saturating_sub(1),
                _ => {}
            }
        }
        Ok(())
    }
}
