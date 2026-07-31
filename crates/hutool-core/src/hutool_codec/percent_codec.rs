//! Hutool-compatible codec facades whose configuration exceeds the small core helpers.

/// 对齐: `cn.hutool.core.codec.PercentCodec`
/// 百分号编码器

use encoding_rs::Encoding;
use std::collections::BTreeSet;

/// Configurable percent encoder matching Hutool's mutable safe-character model.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PercentCodec {
    safe_characters: BTreeSet<char>,
    encode_space_as_plus: bool,
}

impl PercentCodec {
    /// Creates a codec with no predefined safe characters, matching Hutool's constructor.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            safe_characters: BTreeSet::new(),
            encode_space_as_plus: false,
        }
    }

    /// Creates a codec whose initial safe set is `characters`.
    #[must_use]
    pub fn with_safe(characters: impl IntoIterator<Item = char>) -> Self {
        Self {
            safe_characters: characters.into_iter().collect(),
            encode_space_as_plus: false,
        }
    }

    /// Adds one safe character.
    pub fn add_safe(&mut self, character: char) -> &mut Self {
        self.safe_characters.insert(character);
        self
    }

    /// Removes one safe character.
    pub fn remove_safe(&mut self, character: char) -> &mut Self {
        self.safe_characters.remove(&character);
        self
    }

    /// Adds another codec's safe characters to this codec.
    pub fn union(&mut self, other: &Self) -> &mut Self {
        self.safe_characters
            .extend(other.safe_characters.iter().copied());
        self
    }

    /// Returns a cloned codec containing the union of both safe sets.
    #[must_use]
    pub fn union_new(&self, other: &Self) -> Self {
        let mut combined = self.clone();
        combined.union(other);
        combined
    }

    /// Selects `+` instead of `%20` for spaces.
    pub fn set_encode_space_as_plus(&mut self, enabled: bool) -> &mut Self {
        self.encode_space_as_plus = enabled;
        self
    }

    /// Percent-encodes text using a selected character encoding and per-call safe characters.
    #[must_use]
    pub fn encode(&self, input: &str, encoding: &'static Encoding, custom_safe: &[char]) -> String {
        let mut output = String::with_capacity(input.len());
        for character in input.chars() {
            if self.safe_characters.contains(&character) || custom_safe.contains(&character) {
                output.push(character);
            } else if self.encode_space_as_plus && character == ' ' {
                output.push('+');
            } else {
                let value = character.to_string();
                let (bytes, _, _) = encoding.encode(&value);
                for byte in bytes.as_ref() {
                    const HEX: &[u8; 16] = b"0123456789ABCDEF";
                    output.push('%');
                    output.push(char::from(HEX[usize::from(byte >> 4)]));
                    output.push(char::from(HEX[usize::from(byte & 0x0f)]));
                }
            }
        }
        output
    }
}
