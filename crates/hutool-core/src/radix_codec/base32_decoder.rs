//! Configurable radix codecs and Rust-native stream/file overloads.

/// 对齐: `cn.hutool.core.codec.Base32`
/// Base32 解码器

use crate::{Decoder, Result};

/// Base32 decoder with a validated custom ASCII alphabet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base32Decoder {
    lookup: [i8; 128],
}

impl Base32Decoder {
    /// Standard RFC 4648 decoder.
    pub fn standard() -> Self {
        Self::from_valid_alphabet(BASE32_STANDARD_BYTES)
    }

    /// Extended Hex RFC 4648 decoder.
    pub fn extended_hex() -> Self {
        Self::from_valid_alphabet(BASE32_HEX_BYTES)
    }

    /// Creates a case-tolerant decoder for an ASCII alphabet.
    pub fn new(alphabet: &str) -> Result<Self> {
        let alphabet = validate_alphabet::<32>(alphabet, "Base32")?;
        Ok(Self::from_valid_alphabet(alphabet))
    }

    fn from_valid_alphabet(alphabet: [u8; 32]) -> Self {
        let mut lookup = [-1; 128];
        for (digit, byte) in (0_i8..).zip(alphabet.iter().copied()) {
            lookup[usize::from(byte)] = digit;
            // Hutool adds lowercase aliases only for uppercase alphabet entries.
            if byte.is_ascii_uppercase() {
                lookup[usize::from(byte.to_ascii_lowercase())] = digit;
            }
        }
        Self { lookup }
    }

    /// Decodes while ignoring padding and characters outside the configured alphabet.
    #[must_use]
    pub fn decode_text(&self, input: &str) -> Vec<u8> {
        let mut output = Vec::with_capacity(input.len().saturating_mul(5) / 8);
        let mut buffer = 0_u16;
        let mut bits = 0_u8;
        for byte in input.bytes() {
            let digit = self
                .lookup
                .get(usize::from(byte))
                .copied()
                .filter(|digit| *digit >= 0);
            let Some(digit) = digit else {
                continue;
            };
            buffer = (buffer << 5) | u16::from(u8::try_from(digit).unwrap_or_default());
            bits += 5;
            if bits >= 8 {
                bits -= 8;
                output.push(u8::try_from((buffer >> bits) & 0xff).unwrap_or_default());
                buffer &= (1_u16 << bits) - 1;
            }
        }
        output
    }
}

impl Decoder<str, Vec<u8>> for Base32Decoder {
    fn decode(&self, input: &str) -> Result<Vec<u8>> {
        Ok(self.decode_text(input))
    }
}

use super::{BASE32_HEX_BYTES, BASE32_STANDARD_BYTES, validate_alphabet};
