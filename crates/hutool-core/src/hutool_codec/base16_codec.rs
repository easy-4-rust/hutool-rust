//! Hutool-compatible codec facades whose configuration exceeds the small core helpers.

use std::{
    collections::BTreeSet,
    io::{Read, Write},
    path::Path,
};

use base64::Engine as _;
use encoding_rs::Encoding;

use crate::{CoreError, Result};

use super::decoder::Decoder;
use super::encoder::Encoder;

/// Configurable lower- or uppercase Base16 codec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Base16Codec {
    lower_case: bool,
}

impl Base16Codec {
    /// Lowercase hexadecimal codec.
    pub const LOWER: Self = Self::new(true);
    /// Uppercase hexadecimal codec.
    pub const UPPER: Self = Self::new(false);

    /// Creates a codec with the requested alphabet case.
    #[must_use]
    pub const fn new(lower_case: bool) -> Self {
        Self { lower_case }
    }

    /// Encodes bytes as Base16 text.
    #[must_use]
    pub fn encode_bytes(self, input: &[u8]) -> String {
        let encoded = hex::encode(input);
        if self.lower_case {
            encoded
        } else {
            encoded.to_ascii_uppercase()
        }
    }

    /// Decodes Base16 after removing Unicode whitespace and padding an odd nibble.
    pub fn decode_text(self, input: &str) -> Result<Vec<u8>> {
        let mut cleaned: String = input
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect();
        if cleaned.len() % 2 != 0 {
            cleaned.insert(0, '0');
        }
        hex::decode(cleaned).map_err(Into::into)
    }

    /// Formats a Rust character as one or two Java-compatible UTF-16 escapes.
    #[must_use]
    pub fn to_unicode_hex(self, character: char) -> String {
        character
            .encode_utf16(&mut [0; 2])
            .iter()
            .map(|code_unit| {
                if self.lower_case {
                    format!("\\u{code_unit:04x}")
                } else {
                    format!("\\u{code_unit:04X}")
                }
            })
            .collect()
    }

    /// Appends one byte as two hexadecimal characters.
    pub fn append_hex(self, output: &mut String, byte: u8) {
        output.push_str(&self.encode_bytes(&[byte]));
    }
}

impl Encoder<[u8], String> for Base16Codec {
    fn encode(&self, input: &[u8]) -> Result<String> {
        Ok(self.encode_bytes(input))
    }
}

impl Decoder<str, Vec<u8>> for Base16Codec {
    fn decode(&self, input: &str) -> Result<Vec<u8>> {
        self.decode_text(input)
    }
}

use super::{base64_sextet};
