//! Configurable radix codecs and Rust-native stream/file overloads.

/// 对齐: `cn.hutool.core.codec.Base58`
/// Base58 解码器

use crate::{Decoder, Result};

/// Base58 decoder with a custom validated alphabet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base58Decoder {
    alphabet: [u8; 58],
}

impl Base58Decoder {
    /// Bitcoin alphabet decoder used by Hutool.
    pub const fn bitcoin() -> Self {
        Self {
            alphabet: BASE58_BITCOIN_BYTES,
        }
    }

    /// Creates a decoder for a 58-character unique ASCII alphabet.
    pub fn new(alphabet: &str) -> Result<Self> {
        Ok(Self {
            alphabet: validate_alphabet::<58>(alphabet, "Base58")?,
        })
    }

    /// Decodes custom-alphabet Base58.
    pub fn decode_text(&self, input: &str) -> Result<Vec<u8>> {
        decode_alphabet(input.as_bytes(), &self.alphabet, 58)
    }
}

impl Decoder<str, Vec<u8>> for Base58Decoder {
    fn decode(&self, input: &str) -> Result<Vec<u8>> {
        self.decode_text(input)
    }
}

use super::{BASE58_BITCOIN_BYTES, decode_alphabet, validate_alphabet};
