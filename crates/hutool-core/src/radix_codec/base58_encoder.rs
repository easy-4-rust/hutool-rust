//! Configurable radix codecs and Rust-native stream/file overloads.

/// 对齐: `cn.hutool.core.codec.Base58`
/// Base58 编码器

use crate::{
    Encoder, Result,
    advanced_codec::{convert_base, translate_digits},
};

/// Base58 encoder with a custom validated alphabet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base58Encoder {
    alphabet: [u8; 58],
}

impl Base58Encoder {
    /// Bitcoin alphabet encoder used by Hutool.
    pub const fn bitcoin() -> Self {
        Self {
            alphabet: BASE58_BITCOIN_BYTES,
        }
    }

    /// Creates an encoder for a 58-character unique ASCII alphabet.
    pub fn new(alphabet: &str) -> Result<Self> {
        Ok(Self {
            alphabet: validate_alphabet::<58>(alphabet, "Base58")?,
        })
    }

    /// Encodes arbitrary bytes.
    #[must_use]
    pub fn encode_bytes(&self, input: &[u8]) -> String {
        translate_digits(&convert_base(input, 256, 58), &self.alphabet)
    }
}

impl Encoder<[u8], String> for Base58Encoder {
    fn encode(&self, input: &[u8]) -> Result<String> {
        Ok(self.encode_bytes(input))
    }
}

use super::{BASE58_BITCOIN_BYTES, validate_alphabet};
