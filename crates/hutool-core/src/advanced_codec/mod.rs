//! Hutool-aligned binary and text codecs with Rust-native error handling.

use data_encoding::{BASE32, BASE32HEX};
use idna::punycode;
use sha2::{Digest as _, Sha256};

use crate::{CoreError, Result};

mod morse_codec;
mod hash_ids;

pub use morse_codec::MorseCodec;
pub use hash_ids::HashIds;

const BASE58_ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

const BASE62_GMP: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

const BASE62_INVERTED: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

const CAESAR_TABLE: &[u8] = b"AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz";

const HASHIDS_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

const HASHIDS_SEPARATORS: &str = "cfhistuCFHISTU";

pub fn base32_encode(input: impl AsRef<[u8]>) -> String {
    BASE32.encode(input.as_ref())
}

pub fn base32_decode(input: &str) -> Result<Vec<u8>> {
    BASE32
        .decode(input.to_ascii_uppercase().as_bytes())
        .map_err(|error| CoreError::Codec(error.to_string()))
}

pub fn base32_hex_encode(input: impl AsRef<[u8]>) -> String {
    BASE32HEX.encode(input.as_ref())
}

pub fn base32_hex_decode(input: &str) -> Result<Vec<u8>> {
    BASE32HEX
        .decode(input.to_ascii_uppercase().as_bytes())
        .map_err(|error| CoreError::Codec(error.to_string()))
}

pub fn base58_encode(input: impl AsRef<[u8]>) -> String {
    translate_digits(&convert_base(input.as_ref(), 256, 58), BASE58_ALPHABET)
}

pub fn base58_decode(input: &str) -> Result<Vec<u8>> {
    decode_radix(input, BASE58_ALPHABET, 58)
}

pub fn base58_encode_checked(version: Option<u8>, payload: &[u8]) -> String {
    let mut bytes = Vec::with_capacity(payload.len() + 5);
    if let Some(version) = version {
        bytes.push(version);
    }
    bytes.extend_from_slice(payload);
    bytes.extend_from_slice(&double_sha256(payload)[..4]);
    base58_encode(bytes)
}

pub fn base58_decode_checked(input: &str, with_version: bool) -> Result<Vec<u8>> {
    let decoded = base58_decode(input)?;
    let minimum = if with_version { 5 } else { 4 };
    if decoded.len() < minimum {
        return Err(CoreError::Codec("Base58Check payload is too short".into()));
    }
    let payload_start = usize::from(with_version);
    let checksum_start = decoded.len() - 4;
    let payload = &decoded[payload_start..checksum_start];
    if decoded[checksum_start..] != double_sha256(payload)[..4] {
        return Err(CoreError::Codec("Base58 checksum is invalid".into()));
    }
    Ok(payload.to_vec())
}

pub fn base58_decode_checked_auto(input: &str) -> Result<Vec<u8>> {
    base58_decode_checked(input, true).or_else(|_| base58_decode_checked(input, false))
}

pub fn base62_encode(input: impl AsRef<[u8]>) -> String {
    base62_encode_with_alphabet(input.as_ref(), BASE62_GMP)
}

pub fn base62_decode(input: &str) -> Result<Vec<u8>> {
    decode_radix(input, BASE62_GMP, 62)
}

pub fn base62_inverted_encode(input: impl AsRef<[u8]>) -> String {
    base62_encode_with_alphabet(input.as_ref(), BASE62_INVERTED)
}

pub fn base62_inverted_decode(input: &str) -> Result<Vec<u8>> {
    decode_radix(input, BASE62_INVERTED, 62)
}

pub fn rot_encode(input: &str, offset: i32, rotate_digits: bool) -> String {
    input
        .chars()
        .map(|character| rotate_ascii(character, offset, rotate_digits))
        .collect()
}

pub fn rot_decode(input: &str, offset: i32, rotate_digits: bool) -> String {
    rot_encode(input, -offset, rotate_digits)
}

pub fn caesar_encode(input: &str, offset: i32) -> String {
    caesar(input, offset)
}

pub fn caesar_decode(input: &str, offset: i32) -> String {
    caesar(input, -offset)
}

pub fn bcd_encode(input: &str) -> Result<Vec<u8>> {
    let padded;
    let input = if input.len() % 2 == 0 {
        input
    } else {
        padded = format!("0{input}");
        &padded
    };
    input
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = hex_nibble(pair[0])?;
            let low = hex_nibble(pair[1])?;
            Ok((high << 4) | low)
        })
        .collect()
}

pub fn bcd_decode(input: &[u8]) -> String {
    const HEX: &[u8] = b"0123456789ABCDEF";
    let mut output = String::with_capacity(input.len() * 2);
    for byte in input {
        output.push(char::from(HEX[usize::from(byte >> 4)]));
        output.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    output
}

pub fn punycode_encode(input: &str) -> Result<String> {
    punycode_encode_prefixed(input, false)
}

pub fn punycode_encode_prefixed(input: &str, with_prefix: bool) -> Result<String> {
    if input.is_ascii() {
        return Ok(input.to_owned());
    }
    let encoded = punycode::encode_str(input)
        .ok_or_else(|| CoreError::Codec("Punycode encode failed".into()))?;
    Ok(if with_prefix {
        format!("xn--{encoded}")
    } else {
        encoded
    })
}

pub fn punycode_decode(input: &str) -> Result<String> {
    let input = input
        .get(..4)
        .filter(|prefix| prefix.eq_ignore_ascii_case("xn--"))
        .map_or(input, |_| &input[4..]);
    punycode::decode_to_string(input)
        .ok_or_else(|| CoreError::Codec("Punycode decode failed".into()))
}

pub fn idna_encode_domain(input: &str) -> Result<String> {
    input
        .split('.')
        .map(|label| punycode_encode_prefixed(label, true))
        .collect::<Result<Vec<_>>>()
        .map(|labels| labels.join("."))
}

pub fn idna_decode_domain(input: &str) -> Result<String> {
    input
        .split('.')
        .map(|label| {
            if label
                .get(..4)
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case("xn--"))
            {
                punycode_decode(label)
            } else {
                Ok(label.to_owned())
            }
        })
        .collect::<Result<Vec<_>>>()
        .map(|labels| labels.join("."))
}

fn hashids_shuffle(alphabet: &mut [char], salt: &[char]) {
    if salt.is_empty() {
        return;
    }
    let mut value = 0_usize;
    let mut sum = 0_u64;
    for index in (1..alphabet.len()).rev() {
        value %= salt.len();
        let code_point = u64::from(u32::from(salt[value]));
        sum = sum.wrapping_add(code_point);
        let target = usize::try_from(
            (code_point + value as u64 + sum) % u64::try_from(index).expect("index fits u64"),
        )
        .expect("shuffle index fits usize");
        alphabet.swap(index, target);
        value += 1;
    }
}

fn hashids_derive_alphabet(alphabet: &mut [char], salt: &[char], lottery: char) {
    let mut derived = Vec::with_capacity(alphabet.len());
    derived.push(lottery);
    derived.extend(salt.iter().copied().take(alphabet.len().saturating_sub(1)));
    derived.extend(
        alphabet
            .iter()
            .copied()
            .take(alphabet.len().saturating_sub(derived.len())),
    );
    hashids_shuffle(alphabet, &derived);
}

fn hashids_translate(mut value: u64, alphabet: &[char]) -> String {
    let radix = u64::try_from(alphabet.len()).expect("alphabet length fits u64");
    let mut encoded = Vec::new();
    loop {
        encoded.push(alphabet[usize::try_from(value % radix).expect("alphabet index fits usize")]);
        value /= radix;
        if value == 0 {
            break;
        }
    }
    encoded.into_iter().rev().collect()
}

fn hashids_untranslate(value: &[char], alphabet: &[char]) -> Result<u64> {
    let radix = u64::try_from(alphabet.len()).expect("alphabet length fits u64");
    value.iter().try_fold(0_u64, |number, character| {
        let digit = alphabet
            .iter()
            .position(|candidate| candidate == character)
            .ok_or_else(|| CoreError::Codec("invalid Hashids alphabet character".into()))?;
        number
            .checked_mul(radix)
            .and_then(|number| number.checked_add(digit as u64))
            .ok_or_else(|| CoreError::Codec("Hashids value overflows u64".into()))
    })
}

fn base62_encode_with_alphabet(input: &[u8], alphabet: &[u8]) -> String {
    translate_digits(&convert_base(input, 256, 62), alphabet)
}

pub(crate) fn decode_radix(input: &str, alphabet: &[u8], radix: u32) -> Result<Vec<u8>> {
    let digits = input
        .bytes()
        .enumerate()
        .map(|(index, byte)| {
            alphabet
                .iter()
                .position(|candidate| *candidate == byte)
                .and_then(|value| u8::try_from(value).ok())
                .ok_or_else(|| CoreError::Codec(format!("invalid radix character at byte {index}")))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(convert_base(&digits, radix, 256))
}

pub(crate) fn convert_base(message: &[u8], source_base: u32, target_base: u32) -> Vec<u8> {
    if message.is_empty() {
        return Vec::new();
    }
    let mut source = message.to_vec();
    let mut reversed = Vec::new();
    while !source.is_empty() {
        let mut quotient = Vec::with_capacity(source.len());
        let mut remainder = 0_u32;
        for byte in source {
            let accumulator = u32::from(byte) + remainder * source_base;
            let digit = accumulator / target_base;
            remainder = accumulator % target_base;
            if !quotient.is_empty() || digit > 0 {
                quotient.push(u8::try_from(digit).expect("base conversion digit is at most 255"));
            }
        }
        reversed.push(u8::try_from(remainder).expect("base conversion remainder is at most 255"));
        source = quotient;
    }
    reversed.extend(
        message
            .iter()
            .take(message.len().saturating_sub(1))
            .take_while(|byte| **byte == 0)
            .map(|_| 0),
    );
    reversed.reverse();
    reversed
}

pub(crate) fn translate_digits(digits: &[u8], alphabet: &[u8]) -> String {
    digits
        .iter()
        .map(|digit| char::from(alphabet[usize::from(*digit)]))
        .collect()
}

fn double_sha256(input: &[u8]) -> [u8; 32] {
    let first = Sha256::digest(input);
    Sha256::digest(first).into()
}

fn rotate_ascii(character: char, offset: i32, rotate_digits: bool) -> char {
    match character {
        'A'..='Z' => char::from(
            b'A' + u8::try_from(
                (i32::try_from(u32::from(character) - u32::from('A'))
                    .expect("ASCII offset fits i32")
                    + offset)
                    .rem_euclid(26),
            )
            .expect("ROT letter is bounded"),
        ),
        'a'..='z' => char::from(
            b'a' + u8::try_from(
                (i32::try_from(u32::from(character) - u32::from('a'))
                    .expect("ASCII offset fits i32")
                    + offset)
                    .rem_euclid(26),
            )
            .expect("ROT letter is bounded"),
        ),
        '0'..='9' if rotate_digits => char::from(
            b'0' + u8::try_from(
                (i32::try_from(u32::from(character) - u32::from('0'))
                    .expect("ASCII offset fits i32")
                    + offset)
                    .rem_euclid(10),
            )
            .expect("ROT digit is bounded"),
        ),
        _ => character,
    }
}

fn caesar(input: &str, offset: i32) -> String {
    input
        .chars()
        .map(|character| {
            u8::try_from(character)
                .ok()
                .and_then(|byte| CAESAR_TABLE.iter().position(|candidate| *candidate == byte))
                .map_or(character, |position| {
                    let position = (i32::try_from(position).expect("Caesar table is small")
                        + offset)
                        .rem_euclid(52);
                    char::from(
                        CAESAR_TABLE
                            [usize::try_from(position).expect("Caesar position is positive")],
                    )
                })
        })
        .collect()
}

fn hex_nibble(byte: u8) -> Result<u8> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(CoreError::Codec(
            "BCD input must contain hexadecimal ASCII".into(),
        )),
    }
}

fn morse_bits(character: char) -> Option<&'static str> {
    Some(match character {
        'A' => "01",
        'B' => "1000",
        'C' => "1010",
        'D' => "100",
        'E' => "0",
        'F' => "0010",
        'G' => "110",
        'H' => "0000",
        'I' => "00",
        'J' => "0111",
        'K' => "101",
        'L' => "0100",
        'M' => "11",
        'N' => "10",
        'O' => "111",
        'P' => "0110",
        'Q' => "1101",
        'R' => "010",
        'S' => "000",
        'T' => "1",
        'U' => "001",
        'V' => "0001",
        'W' => "011",
        'X' => "1001",
        'Y' => "1011",
        'Z' => "1100",
        '0' => "11111",
        '1' => "01111",
        '2' => "00111",
        '3' => "00011",
        '4' => "00001",
        '5' => "00000",
        '6' => "10000",
        '7' => "11000",
        '8' => "11100",
        '9' => "11110",
        '.' => "010101",
        ',' => "110011",
        '?' => "001100",
        '\'' => "011110",
        '!' => "101011",
        '/' => "10010",
        '(' => "10110",
        ')' => "101101",
        '&' => "01000",
        ':' => "111000",
        ';' => "101010",
        '=' => "10001",
        '+' => "01010",
        '-' => "100001",
        '_' => "001101",
        '"' => "010010",
        '$' => "0001001",
        '@' => "011010",
        _ => return None,
    })
}

fn morse_character(bits: &str) -> Option<char> {
    const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,?'!/()&:;=+-_\"$@";
    CHARACTERS
        .chars()
        .find(|character| morse_bits(*character) == Some(bits))
}
