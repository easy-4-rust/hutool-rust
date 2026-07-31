//! Hutool-aligned binary and text codecs with Rust-native error handling.

/// 对齐: `cn.hutool.core.codec.Hashids`
/// 哈希ID编码器

use crate::{CoreError, Result};

/// Hashids-compatible reversible identifier codec.
#[derive(Debug, Clone)]
pub struct HashIds {
    alphabet: Vec<char>,
    separators: Vec<char>,
    salt: Vec<char>,
    guards: Vec<char>,
    minimum_length: usize,
}

impl HashIds {
    /// Builds a salted codec with an optional minimum output length.
    pub fn new(salt: impl AsRef<str>, minimum_length: usize) -> Result<Self> {
        Self::with_alphabet(salt, HASHIDS_ALPHABET, minimum_length)
    }

    /// Builds a salted codec with a custom alphabet.
    pub fn with_alphabet(
        salt: impl AsRef<str>,
        alphabet: impl AsRef<str>,
        minimum_length: usize,
    ) -> Result<Self> {
        let salt: Vec<char> = salt.as_ref().chars().collect();
        let original_alphabet: Vec<char> = alphabet.as_ref().chars().collect();
        if original_alphabet.contains(&' ') {
            return Err(CoreError::Codec(
                "Hashids alphabet must not contain spaces".into(),
            ));
        }

        let mut separators: Vec<char> = HASHIDS_SEPARATORS
            .chars()
            .filter(|separator| original_alphabet.contains(separator))
            .collect();
        hashids_shuffle(&mut separators, &salt);

        let mut alphabet = Vec::new();
        for character in original_alphabet.iter().copied() {
            if !separators.contains(&character) && !alphabet.contains(&character) {
                alphabet.push(character);
            }
        }
        if alphabet.len() < 16 {
            return Err(CoreError::Codec(
                "Hashids alphabet must contain at least 16 unique characters".into(),
            ));
        }

        if separators.is_empty() || alphabet.len() / separators.len() > 3 {
            let minimum_separators = alphabet.len().saturating_mul(2).div_ceil(7);
            if minimum_separators > separators.len() {
                let missing = minimum_separators - separators.len();
                separators.extend_from_slice(&alphabet[..missing]);
                alphabet.drain(..missing);
            }
        }
        hashids_shuffle(&mut alphabet, &salt);

        let guard_count = alphabet.len().div_ceil(12);
        let guards = alphabet.drain(..guard_count).collect();
        Ok(Self {
            alphabet,
            separators,
            salt,
            guards,
            minimum_length,
        })
    }

    /// Encodes one or more unsigned identifiers.
    #[must_use]
    pub fn encode(&self, values: &[u64]) -> String {
        self.encode_numbers(values)
    }

    /// Decodes identifiers and rejects invalid or non-canonical input.
    pub fn decode(&self, input: &str) -> Result<Vec<u64>> {
        self.decode_numbers(input)
    }

    /// Encodes an arbitrarily long hexadecimal value.
    pub fn encode_hex(&self, input: &str) -> Result<String> {
        let input = input
            .strip_prefix("0x")
            .or_else(|| input.strip_prefix("0X"))
            .unwrap_or(input);
        if !input.chars().all(|character| character.is_ascii_hexdigit()) {
            return Err(CoreError::Codec("Hashids hex input is invalid".into()));
        }
        let values = input
            .as_bytes()
            .chunks(12)
            .map(|chunk| {
                let chunk = std::str::from_utf8(chunk)
                    .map_err(|error| CoreError::Codec(error.to_string()))?;
                u64::from_str_radix(&format!("1{chunk}"), 16)
                    .map_err(|error| CoreError::Codec(error.to_string()))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(self.encode_numbers(&values))
    }

    /// Decodes a Hashids value back to lowercase hexadecimal.
    pub fn decode_hex(&self, input: &str) -> Result<String> {
        self.decode_numbers(input).map(|values| {
            values
                .iter()
                .map(|value| format!("{value:x}").chars().skip(1).collect::<String>())
                .collect()
        })
    }

    fn encode_numbers(&self, values: &[u64]) -> String {
        if values.is_empty() {
            return String::new();
        }
        let lottery_id = values
            .iter()
            .enumerate()
            .fold(0_u64, |state, (index, value)| {
                state.wrapping_add(value % (u64::try_from(index).expect("index fits u64") + 100))
            });
        let lottery = self.alphabet
            [usize::try_from(lottery_id % self.alphabet.len() as u64).expect("index fits usize")];
        let mut alphabet = self.alphabet.clone();
        let mut output = String::new();

        for (index, value) in values.iter().copied().enumerate() {
            hashids_derive_alphabet(&mut alphabet, &self.salt, lottery);
            let encoded = hashids_translate(value, &alphabet);
            let first_encoded = encoded.chars().next().expect("translation is non-empty");
            if index == 0 {
                output.push(lottery);
            }
            output.push_str(&encoded);
            if index + 1 < values.len() {
                let divisor =
                    u64::from(u32::from(if index == 0 { lottery } else { first_encoded })) + 1;
                let separator = self.separators[usize::try_from(
                    (value % divisor) % self.separators.len() as u64,
                )
                .expect("separator index fits usize")];
                output.push(separator);
            }
        }

        if self.minimum_length > output.chars().count() {
            let guard_index = usize::try_from(
                (lottery_id + u64::from(u32::from(lottery))) % self.guards.len() as u64,
            )
            .expect("guard index fits usize");
            output.insert(0, self.guards[guard_index]);
            if self.minimum_length > output.chars().count() {
                let third = output
                    .chars()
                    .nth(2)
                    .expect("guarded hash has three characters");
                let guard_index = usize::try_from(
                    (lottery_id + u64::from(u32::from(third))) % self.guards.len() as u64,
                )
                .expect("guard index fits usize");
                output.push(self.guards[guard_index]);
            }
        }

        while self.minimum_length > output.chars().count() {
            let salt = alphabet.clone();
            hashids_shuffle(&mut alphabet, &salt);
            let half = alphabet.len() / 2;
            let output_length = output.chars().count();
            let padding_left = self.minimum_length - output_length;
            let old: Vec<char> = output.chars().collect();
            if padding_left > alphabet.len() {
                let offset = half + usize::from(alphabet.len() % 2 != 0);
                output = alphabet[half..half + offset]
                    .iter()
                    .chain(old.iter())
                    .chain(alphabet[..half].iter())
                    .collect();
            } else {
                let excess = alphabet.len() + output_length - self.minimum_length;
                let second_start = half + excess / 2;
                let second_length = alphabet.len() - second_start;
                let first_length = padding_left - second_length;
                output = alphabet[second_start..]
                    .iter()
                    .chain(old.iter())
                    .chain(alphabet[..first_length].iter())
                    .collect();
            }
        }
        output
    }

    fn decode_numbers(&self, input: &str) -> Result<Vec<u64>> {
        if input.is_empty() {
            return Ok(Vec::new());
        }
        let input: Vec<char> = input.chars().collect();
        let guard_positions: Vec<usize> = input
            .iter()
            .enumerate()
            .filter_map(|(index, character)| self.guards.contains(character).then_some(index))
            .collect();
        let (start, end) = guard_positions.first().map_or((0, input.len()), |first| {
            (
                first + 1,
                guard_positions.get(1).copied().unwrap_or(input.len()),
            )
        });
        if start >= end {
            return Err(CoreError::Codec("invalid Hashids value".into()));
        }
        let lottery = input[start];
        let mut alphabet = self.alphabet.clone();
        let mut values = Vec::new();
        let mut block = Vec::new();
        for character in input[start + 1..end].iter().copied() {
            if self.separators.contains(&character) {
                if !block.is_empty() {
                    hashids_derive_alphabet(&mut alphabet, &self.salt, lottery);
                    values.push(hashids_untranslate(&block, &alphabet)?);
                    block.clear();
                }
            } else {
                block.push(character);
            }
        }
        if !block.is_empty() {
            hashids_derive_alphabet(&mut alphabet, &self.salt, lottery);
            values.push(hashids_untranslate(&block, &alphabet)?);
        }
        if self.encode_numbers(&values) != input.iter().collect::<String>() {
            return Err(CoreError::Codec("invalid Hashids value".into()));
        }
        Ok(values)
    }
}

use super::{HASHIDS_ALPHABET, HASHIDS_SEPARATORS, hashids_derive_alphabet, hashids_shuffle, hashids_translate, hashids_untranslate};
