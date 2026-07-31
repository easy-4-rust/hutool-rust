//! Configurable radix codecs and Rust-native stream/file overloads.

/// 对齐: `cn.hutool.core.codec.Base62`
/// Base62 编码器

use crate::{
    Encoder, Result,
    advanced_codec::convert_base,
};

/// Base62 encoder with a custom validated byte alphabet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base62Encoder {
    alphabet: [u8; 62],
}

impl Base62Encoder {
    /// GMP-style alphabet encoder.
    pub const fn gmp() -> Self {
        Self {
            alphabet: BASE62_GMP_BYTES,
        }
    }

    /// Case-inverted alphabet encoder.
    pub const fn inverted() -> Self {
        Self {
            alphabet: BASE62_INVERTED_BYTES,
        }
    }

    /// Creates an encoder for a 62-character unique ASCII alphabet.
    pub fn new(alphabet: &str) -> Result<Self> {
        Ok(Self {
            alphabet: validate_alphabet::<62>(alphabet, "Base62")?,
        })
    }

    /// Encodes bytes to ASCII Base62 bytes.
    #[must_use]
    pub fn encode_bytes(&self, input: &[u8]) -> Vec<u8> {
        convert_base(input, 256, 62)
            .iter()
            .map(|digit| self.alphabet[usize::from(*digit)])
            .collect()
    }
}

impl Encoder<[u8], Vec<u8>> for Base62Encoder {
    fn encode(&self, input: &[u8]) -> Result<Vec<u8>> {
        Ok(self.encode_bytes(input))
    }
}

use super::{BASE62_GMP_BYTES, BASE62_INVERTED_BYTES, validate_alphabet};
