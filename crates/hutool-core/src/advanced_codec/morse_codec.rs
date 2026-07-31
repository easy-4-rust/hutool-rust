//! Hutool-aligned binary and text codecs with Rust-native error handling.

/// 对齐: `cn.hutool.core.codec.Morse`
/// 摩尔斯电码

use crate::{CoreError, Result};

use super::{morse_bits, morse_character};

/// Configurable Morse encoder compatible with Hutool's binary dictionary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MorseCodec {
    dit: char,
    dah: char,
    separator: char,
}

impl MorseCodec {
    /// Creates a codec with custom dot, dash, and symbol separator characters.
    pub fn new(dit: char, dah: char, separator: char) -> Result<Self> {
        if dit == dah || dit == separator || dah == separator {
            return Err(CoreError::Codec(
                "Morse markers and separator must be distinct".into(),
            ));
        }
        Ok(Self {
            dit,
            dah,
            separator,
        })
    }

    /// Encodes Unicode text; unknown symbols use their binary code point.
    #[must_use]
    pub fn encode(self, input: &str) -> String {
        let mut output = String::new();
        for code_unit in input.to_uppercase().encode_utf16() {
            let bits = char::from_u32(u32::from(code_unit))
                .and_then(morse_bits)
                .map_or_else(|| format!("{code_unit:b}"), str::to_owned);
            for bit in bits.bytes() {
                output.push(if bit == b'0' { self.dit } else { self.dah });
            }
            output.push(self.separator);
        }
        output
    }

    /// Decodes Morse text and validates every input character.
    pub fn decode(self, input: &str) -> Result<String> {
        if input
            .chars()
            .any(|value| value != self.dit && value != self.dah && value != self.separator)
        {
            return Err(CoreError::Codec("incorrect Morse input".into()));
        }
        let mut output = Vec::new();
        for word in input.split(self.separator).filter(|word| !word.is_empty()) {
            let bits: String = word
                .chars()
                .map(|value| if value == self.dit { '0' } else { '1' })
                .collect();
            let code_unit = if let Some(character) = morse_character(&bits) {
                u16::try_from(u32::from(character))
                    .map_err(|_| CoreError::Codec("invalid Morse code point".into()))?
            } else {
                u16::from_str_radix(&bits, 2)
                    .map_err(|_| CoreError::Codec("invalid Morse code point".into()))?
            };
            output.push(code_unit);
        }
        String::from_utf16(&output).map_err(|error| CoreError::Codec(error.to_string()))
    }
}

impl Default for MorseCodec {
    fn default() -> Self {
        Self {
            dit: '.',
            dah: '-',
            separator: '/',
        }
    }
}
