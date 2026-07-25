//! Configurable radix codecs and Rust-native stream/file overloads.

/// 对齐: `cn.hutool.core.codec.Base58`
/// Base58 解码器

use std::{
    io::{Read, Write},
    path::Path,
};

use encoding_rs::{Encoding, GBK};

use crate::{
    CoreError, Decoder, Encoder, Result,
    advanced_codec::{convert_base, translate_digits},
    base32_decode, base32_encode, base32_hex_decode, base32_hex_encode, base62_decode,
    base62_encode, base62_inverted_decode, base62_inverted_encode,
};

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

use super::{BASE32_HEX_BYTES, BASE32_STANDARD_BYTES, BASE58_BITCOIN_BYTES, BASE62_GMP_BYTES, BASE62_INVERTED_BYTES, base32_decode_text, base32_decode_to_file, base32_decode_to_writer};
use super::{base32_encode_file, base32_encode_reader, base32_encode_text, base62_decode_text, base62_decode_text_gbk, base62_decode_to_file, base62_decode_to_writer, base62_encode_file};
use super::{base62_encode_reader, base62_encode_text, bcd_encode_ascii_prefix, bcd_nibble, decode_alphabet, validate_alphabet};
