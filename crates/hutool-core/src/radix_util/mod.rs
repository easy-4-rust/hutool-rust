use std::collections::HashSet;

use thiserror::Error;

mod radix_error;
mod radix_util;

pub use radix_error::RadixError;
pub use radix_util::RadixUtil;

const MAX_RADIX: usize = 256;

fn encode_unsigned(alphabet: &str, mut value: u64) -> Result<String, RadixError> {
    let alphabet = validate_alphabet(alphabet)?;
    #[allow(clippy::cast_possible_truncation)]
    let radix = alphabet.len() as u64;
    let mut encoded = Vec::new();
    loop {
        #[allow(clippy::cast_possible_truncation)]
        let index = (value % radix) as usize;
        encoded.push(alphabet[index]);
        value /= radix;
        if value == 0 {
            break;
        }
    }
    Ok(encoded.into_iter().rev().collect())
}

fn validate_alphabet(alphabet: &str) -> Result<Vec<char>, RadixError> {
    let characters: Vec<char> = alphabet.chars().collect();
    if characters.len() < 2 {
        return Err(RadixError::AlphabetTooShort);
    }
    if characters.len() > MAX_RADIX {
        return Err(RadixError::AlphabetTooLarge);
    }
    let mut unique = HashSet::with_capacity(characters.len());
    if let Some(duplicate) = characters
        .iter()
        .copied()
        .find(|character| !unique.insert(*character))
    {
        return Err(RadixError::DuplicateCharacter(duplicate));
    }
    Ok(characters)
}
