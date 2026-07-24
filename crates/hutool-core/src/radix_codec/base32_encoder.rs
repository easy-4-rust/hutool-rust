//! Configurable radix codecs and Rust-native stream/file overloads.

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

/// Base32 encoder with a validated custom ASCII alphabet and optional pad.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base32Encoder {
    alphabet: [u8; 32],
    pad: Option<char>,
}

impl Base32Encoder {
    /// Standard RFC 4648 encoder.
    pub const fn standard() -> Self {
        Self {
            alphabet: BASE32_STANDARD_BYTES,
            pad: Some('='),
        }
    }

    /// Extended Hex RFC 4648 encoder.
    pub const fn extended_hex() -> Self {
        Self {
            alphabet: BASE32_HEX_BYTES,
            pad: Some('='),
        }
    }

    /// Creates an encoder after validating length, uniqueness, ASCII, and pad ambiguity.
    pub fn new(alphabet: &str, pad: Option<char>) -> Result<Self> {
        let alphabet = validate_alphabet::<32>(alphabet, "Base32")?;
        if pad.is_some_and(|pad| !pad.is_ascii() || alphabet.contains(&(pad as u8))) {
            return Err(CoreError::Codec(
                "Base32 pad must be ASCII and outside the alphabet".into(),
            ));
        }
        Ok(Self { alphabet, pad })
    }

    /// Encodes arbitrary bytes.
    #[must_use]
    pub fn encode_bytes(&self, input: &[u8]) -> String {
        let mut output = String::with_capacity(input.len().div_ceil(5) * 8);
        let mut buffer = 0_u16;
        let mut bits = 0_u8;
        for byte in input {
            buffer = (buffer << 8) | u16::from(*byte);
            bits += 8;
            while bits >= 5 {
                bits -= 5;
                let digit = usize::from((buffer >> bits) & 0x1f);
                output.push(char::from(self.alphabet[digit]));
            }
            buffer &= (1_u16 << bits) - 1;
        }
        if bits > 0 {
            let digit = usize::from((buffer << (5 - bits)) & 0x1f);
            output.push(char::from(self.alphabet[digit]));
        }
        if let Some(pad) = self.pad {
            while output.len() % 8 != 0 {
                output.push(pad);
            }
        }
        output
    }
}

impl Encoder<[u8], String> for Base32Encoder {
    fn encode(&self, input: &[u8]) -> Result<String> {
        Ok(self.encode_bytes(input))
    }
}

use super::{BASE32_HEX_BYTES, BASE32_STANDARD_BYTES, BASE58_BITCOIN_BYTES, BASE62_GMP_BYTES, BASE62_INVERTED_BYTES, base32_decode_text, base32_decode_to_file, base32_decode_to_writer};
use super::{base32_encode_file, base32_encode_reader, base32_encode_text, base62_decode_text, base62_decode_text_gbk, base62_decode_to_file, base62_decode_to_writer, base62_encode_file};
use super::{base62_encode_reader, base62_encode_text, bcd_encode_ascii_prefix, bcd_nibble, decode_alphabet, validate_alphabet};
