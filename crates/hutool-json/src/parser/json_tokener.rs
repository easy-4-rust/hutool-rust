use std::io::Read;

use serde_json::Value;

use crate::{JSONArray, JSONConfig, JSONObject, JsonError, Result};

use super::parse_config::ParseConfig;

/// Stateful Unicode tokenizer retaining Hutool's migration surface.
#[derive(Debug, Clone)]
pub struct JSONTokener {
    input: String,
    position: usize,
    previous: Option<char>,
    pub(crate) config: JSONConfig,
}

impl JSONTokener {
    /// Creates a tokenizer with default defensive limits.
    pub fn new(input: &str, config: JSONConfig) -> Result<Self> {
        Self::with_limits(input, config, ParseConfig::default())
    }

    /// Creates a tokenizer with explicit defensive limits.
    pub fn with_limits(input: &str, config: JSONConfig, limits: ParseConfig) -> Result<Self> {
        limits.validate(input)?;
        Ok(Self {
            input: input.to_owned(),
            position: 0,
            previous: None,
            config,
        })
    }

    /// Reads bounded UTF-8 bytes into a tokenizer.
    pub fn from_reader(
        reader: &mut dyn Read,
        config: JSONConfig,
        limits: ParseConfig,
    ) -> Result<Self> {
        let mut bytes = Vec::new();
        reader
            .take(limits.max_input_bytes as u64 + 1)
            .read_to_end(&mut bytes)?;
        if bytes.len() > limits.max_input_bytes {
            return Err(JsonError::Limit("input bytes"));
        }
        let input = String::from_utf8(bytes)?;
        Self::with_limits(&input, config, limits)
    }

    /// Returns the JSON container configuration.
    #[must_use]
    pub const fn config(&self) -> &JSONConfig {
        &self.config
    }

    /// Returns whether all input is consumed.
    #[must_use]
    pub fn end(&self) -> bool {
        self.position >= self.input.len()
    }

    /// Returns whether another character is available.
    #[must_use]
    pub fn more(&self) -> bool {
        !self.end()
    }

    /// Returns the next Unicode scalar.
    pub fn next_char(&mut self) -> Option<char> {
        let character = self.input[self.position..].chars().next()?;
        self.position += character.len_utf8();
        self.previous = Some(character);
        Some(character)
    }

    /// Moves back one previously read scalar.
    pub fn back(&mut self) -> Result<()> {
        let previous = self
            .previous
            .take()
            .ok_or_else(|| JsonError::Syntax("cannot step back twice".into()))?;
        self.position = self.position.saturating_sub(previous.len_utf8());
        Ok(())
    }

    /// Requires the next scalar to equal `expected`.
    pub fn next_expected(&mut self, expected: char) -> Result<char> {
        let Some(actual) = self.next_char() else {
            return Err(JsonError::Syntax(format!("expected {expected}, found end")));
        };
        if actual != expected {
            return Err(JsonError::Syntax(format!(
                "expected {expected}, found {actual}"
            )));
        }
        Ok(actual)
    }

    /// Reads exactly `count` Unicode scalars.
    pub fn next_n(&mut self, count: usize) -> Result<String> {
        let mut output = String::new();
        for _ in 0..count {
            output.push(
                self.next_char()
                    .ok_or_else(|| JsonError::Syntax("unexpected end".into()))?,
            );
        }
        Ok(output)
    }

    /// Skips whitespace and returns the next scalar.
    pub fn next_clean(&mut self) -> Option<char> {
        loop {
            let character = self.next_char()?;
            if !character.is_whitespace() {
                return Some(character);
            }
        }
    }

    /// Reads a quoted JSON string after its opening quote was consumed.
    pub fn next_string(&mut self, quote: char) -> Result<String> {
        let mut output = String::new();
        while let Some(character) = self.next_char() {
            if character == quote {
                return Ok(output);
            }
            if character != '\\' {
                output.push(character);
                continue;
            }
            let escaped = self
                .next_char()
                .ok_or_else(|| JsonError::Syntax("unterminated escape".into()))?;
            match escaped {
                'b' => output.push('\u{8}'),
                'f' => output.push('\u{c}'),
                'n' => output.push('\n'),
                'r' => output.push('\r'),
                't' => output.push('\t'),
                '"' | '\\' | '/' => output.push(escaped),
                _ => return Err(JsonError::Syntax("unsupported escape".into())),
            }
        }
        Err(JsonError::Syntax("unterminated string".into()))
    }

    /// Reads text until one delimiter without consuming it.
    #[must_use]
    pub fn next_to(&mut self, delimiters: &str) -> String {
        let start = self.position;
        while let Some(character) = self.next_char() {
            if delimiters.contains(character) {
                let end = self.position - character.len_utf8();
                self.position = end;
                self.previous = None;
                return self.input[start..end].trim().to_owned();
            }
        }
        self.input[start..].trim().to_owned()
    }

    /// Skips through and including a marker.
    pub fn skip_past(&mut self, marker: &str) -> bool {
        if let Some(offset) = self.input[self.position..].find(marker) {
            self.position += offset + marker.len();
            self.previous = None;
            true
        } else {
            self.position = self.input.len();
            false
        }
    }

    /// Skips to a scalar without consuming it.
    pub fn skip_to(&mut self, target: char) -> Option<char> {
        let offset = self.input[self.position..].find(target)?;
        self.position += offset;
        self.previous = None;
        Some(target)
    }

    /// Parses the remaining input as one complete JSON value.
    pub fn next_value(&mut self) -> Result<Value> {
        let remaining = self.input[self.position..].trim();
        let value = crate::parse(remaining)?;
        self.position = self.input.len();
        self.previous = None;
        Ok(value)
    }

    /// Parses the remaining input as an array.
    pub fn to_array(&mut self) -> Result<JSONArray> {
        JSONArray::from_value(self.next_value()?, self.config.clone())
    }
}
