//! Configurable radix codecs and Rust-native stream/file overloads.

/// 对齐: `cn.hutool.core.codec.Base62`
/// Base62 解码器

use crate::{Decoder, Result};

/// Base62 decoder with a custom validated byte alphabet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base62Decoder {
    alphabet: [u8; 62],
}

impl Base62Decoder {
    /// GMP-style alphabet decoder.
    pub const fn gmp() -> Self {
        Self {
            alphabet: BASE62_GMP_BYTES,
        }
    }

    /// Case-inverted alphabet decoder.
    pub const fn inverted() -> Self {
        Self {
            alphabet: BASE62_INVERTED_BYTES,
        }
    }

    /// Creates a decoder for a 62-character unique ASCII alphabet.
    pub fn new(alphabet: &str) -> Result<Self> {
        Ok(Self {
            alphabet: validate_alphabet::<62>(alphabet, "Base62")?,
        })
    }

    /// Decodes ASCII Base62 bytes.
    pub fn decode_bytes(&self, input: &[u8]) -> Result<Vec<u8>> {
        decode_alphabet(input, &self.alphabet, 62)
    }
}

impl Decoder<[u8], Vec<u8>> for Base62Decoder {
    fn decode(&self, input: &[u8]) -> Result<Vec<u8>> {
        self.decode_bytes(input)
    }
}

use super::{BASE62_GMP_BYTES, BASE62_INVERTED_BYTES, decode_alphabet, validate_alphabet};
