//! Convenient hexadecimal operations aligned with Hutool's `HexUtil` family.

/// 对齐: `cn.hutool.core.util.HexUtil`
/// 十六进制工具类
use encoding_rs::{Encoding, UTF_8, UTF_16BE, UTF_16LE};
use num_bigint::BigInt;

use crate::Base16Codec;

use super::hex_util_error::HexUtilError;
use super::rgb_color::RgbColor;

/// Zero-sized facade for Hutool-style hexadecimal operations.
#[derive(Debug, Clone, Copy, Default)]
pub struct HexUtil;

impl HexUtil {
    /// Returns whether `value` is a non-negative hexadecimal number.
    #[must_use]
    pub fn is_hex_number(value: &str) -> bool {
        Self::is_hex_number_option(Some(value))
    }

    /// Nullable variant matching Hutool's `null -> false` behavior.
    #[must_use]
    pub fn is_hex_number_option(value: Option<&str>) -> bool {
        let Some(value) = value.filter(|value| !value.is_empty()) else {
            return false;
        };
        if value.starts_with('-') {
            return false;
        }
        let value = value
            .strip_prefix("0x")
            .or_else(|| value.strip_prefix("0X"))
            .or_else(|| value.strip_prefix('#'))
            .unwrap_or(value);
        !value.is_empty() && BigInt::parse_bytes(value.as_bytes(), 16).is_some()
    }

    /// Encodes bytes as lowercase hexadecimal text.
    #[must_use]
    pub fn encode_hex(data: impl AsRef<[u8]>) -> String {
        Base16Codec::LOWER.encode_bytes(data.as_ref())
    }

    /// Encodes bytes using the requested letter case.
    #[must_use]
    pub fn encode_hex_case(data: impl AsRef<[u8]>, lower_case: bool) -> String {
        Base16Codec::new(lower_case).encode_bytes(data.as_ref())
    }

    /// Encodes text with `encoding`, then converts the resulting bytes to lowercase hexadecimal.
    #[must_use]
    pub fn encode_hex_text(data: &str, encoding: &'static Encoding) -> String {
        Self::encode_hex(encode_text_bytes(data, encoding))
    }

    /// UTF-8 convenience variant of [`Self::encode_hex_text`].
    #[must_use]
    pub fn encode_hex_utf8(data: &str) -> String {
        Self::encode_hex_text(data, UTF_8)
    }

    /// Decodes hexadecimal text using Hutool's whitespace and odd-nibble rules.
    pub fn decode_hex(data: &str) -> Result<Vec<u8>, HexUtilError> {
        Ok(Base16Codec::LOWER.decode_text(data)?)
    }

    /// Decodes a Rust character slice as hexadecimal text.
    pub fn decode_hex_chars(data: &[char]) -> Result<Vec<u8>, HexUtilError> {
        Self::decode_hex(&data.iter().collect::<String>())
    }

    /// Decodes a Rust character slice and converts the bytes with `encoding`.
    pub fn decode_hex_chars_to_text(
        data: &[char],
        encoding: &'static Encoding,
    ) -> Result<String, HexUtilError> {
        Ok(encoding
            .decode(&Self::decode_hex_chars(data)?)
            .0
            .into_owned())
    }

    /// Decodes hexadecimal bytes and converts them to UTF-8 text.
    pub fn decode_hex_text(data: &str) -> Result<String, HexUtilError> {
        Self::decode_hex_text_with_encoding(data, UTF_8)
    }

    /// Decodes hexadecimal bytes and converts them with `encoding`.
    pub fn decode_hex_text_with_encoding(
        data: &str,
        encoding: &'static Encoding,
    ) -> Result<String, HexUtilError> {
        if data.is_empty() {
            return Ok(String::new());
        }
        Ok(encoding.decode(&Self::decode_hex(data)?).0.into_owned())
    }

    /// Encodes a color as lowercase `#rrggbb`.
    #[must_use]
    pub fn encode_color(color: RgbColor) -> String {
        Self::encode_color_with_prefix(color, "#")
    }

    /// Encodes a color with a custom prefix such as `#` or `0x`.
    #[must_use]
    pub fn encode_color_with_prefix(color: RgbColor, prefix: &str) -> String {
        format!(
            "{prefix}{:02x}{:02x}{:02x}",
            color.red, color.green, color.blue
        )
    }

    /// Decodes the same signed decimal, octal, `#`, and `0x` forms as Java `Color.decode`.
    pub fn decode_color(value: &str) -> Result<RgbColor, HexUtilError> {
        let value = decode_java_integer(value)?;
        let bytes = value.to_be_bytes();
        Ok(RgbColor::new(bytes[1], bytes[2], bytes[3]))
    }

    /// Formats an `i32` as a Java-compatible Unicode escape, padded to at least four digits.
    #[must_use]
    pub fn to_unicode_hex_i32(value: i32) -> String {
        let hex = Self::to_hex_i32(value);
        format!("\\u{hex:0>4}")
    }

    /// Formats a Rust character as one or two Java UTF-16 escapes.
    #[must_use]
    pub fn to_unicode_hex(character: char) -> String {
        Base16Codec::LOWER.to_unicode_hex(character)
    }

    /// Formats an `i32` with Java's two's-complement `Integer.toHexString` semantics.
    #[must_use]
    pub fn to_hex_i32(value: i32) -> String {
        format!("{:x}", u32::from_ne_bytes(value.to_ne_bytes()))
    }

    /// Parses a hexadecimal `i32` after applying Hutool's prefix-removal behavior.
    pub fn hex_to_i32(value: &str) -> Result<i32, HexUtilError> {
        Ok(i32::from_str_radix(remove_hex_prefix(value), 16)?)
    }

    /// Formats an `i64` with Java's two's-complement `Long.toHexString` semantics.
    #[must_use]
    pub fn to_hex_i64(value: i64) -> String {
        format!("{:x}", u64::from_ne_bytes(value.to_ne_bytes()))
    }

    /// Parses a hexadecimal `i64` after applying Hutool's prefix-removal behavior.
    pub fn hex_to_i64(value: &str) -> Result<i64, HexUtilError> {
        Ok(i64::from_str_radix(remove_hex_prefix(value), 16)?)
    }

    /// Formats Java-compatible `Float.floatToIntBits` bits as hexadecimal.
    #[must_use]
    pub fn to_hex_f32(value: f32) -> String {
        format!("{:x}", java_f32_bits(value))
    }

    /// Reconstructs an `f32` from its unsigned hexadecimal bit pattern.
    pub fn hex_to_f32(value: &str) -> Result<f32, HexUtilError> {
        Ok(f32::from_bits(u32::from_str_radix(
            remove_hex_prefix(value),
            16,
        )?))
    }

    /// Formats Java-compatible `Double.doubleToLongBits` bits as hexadecimal.
    #[must_use]
    pub fn to_hex_f64(value: f64) -> String {
        format!("{:x}", java_f64_bits(value))
    }

    /// Reconstructs an `f64` from its unsigned hexadecimal bit pattern.
    pub fn hex_to_f64(value: &str) -> Result<f64, HexUtilError> {
        Ok(f64::from_bits(u64::from_str_radix(
            remove_hex_prefix(value),
            16,
        )?))
    }

    /// Appends one byte as exactly two hexadecimal digits.
    pub fn append_hex(output: &mut String, byte: u8, lower_case: bool) {
        Base16Codec::new(lower_case).append_hex(output, byte);
    }

    /// Parses an optional arbitrary-precision hexadecimal integer.
    pub fn to_big_integer(value: Option<&str>) -> Result<Option<BigInt>, HexUtilError> {
        let Some(value) = value else {
            return Ok(None);
        };
        let value = remove_hex_prefix(value);
        BigInt::parse_bytes(value.as_bytes(), 16)
            .map(Some)
            .ok_or_else(|| HexUtilError::BigInteger(value.to_owned()))
    }

    /// Inserts a space after every pair of hexadecimal characters.
    #[must_use]
    pub fn format(data: &str) -> String {
        Self::format_with_prefix(data, "")
    }

    /// Prefixes every pair and inserts one space between pairs.
    #[must_use]
    pub fn format_with_prefix(data: &str, prefix: &str) -> String {
        if data.is_empty() {
            return String::new();
        }
        let mut output = String::with_capacity(data.len() + data.len() / 2);
        for (index, character) in data.chars().enumerate() {
            if index % 2 == 0 {
                if index != 0 {
                    output.push(' ');
                }
                output.push_str(prefix);
            }
            output.push(character);
        }
        output
    }
}

fn remove_hex_prefix(value: &str) -> &str {
    let bytes = value.as_bytes();
    if bytes.len() <= 1 {
        return value;
    }
    match bytes[0] {
        b'0' if matches!(bytes[1], b'x' | b'X') => &value[2..],
        // Hutool's Java switch intentionally falls through from `0` to `#`.
        b'0' | b'#' => &value[1..],
        _ => value,
    }
}

fn java_f32_bits(value: f32) -> u32 {
    if value.is_nan() {
        0x7fc0_0000
    } else {
        value.to_bits()
    }
}

fn encode_text_bytes(data: &str, encoding: &'static Encoding) -> Vec<u8> {
    if std::ptr::eq(encoding, UTF_16BE) {
        data.encode_utf16()
            .flat_map(u16::to_be_bytes)
            .collect::<Vec<_>>()
    } else if std::ptr::eq(encoding, UTF_16LE) {
        data.encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect::<Vec<_>>()
    } else {
        encoding.encode(data).0.into_owned()
    }
}

fn decode_java_integer(value: &str) -> Result<i32, HexUtilError> {
    let (negative, unsigned) = match value.as_bytes().first() {
        Some(b'-') => (true, &value[1..]),
        Some(b'+') => (false, &value[1..]),
        _ => (false, value),
    };
    let (radix, digits) = if let Some(digits) = unsigned
        .strip_prefix("0x")
        .or_else(|| unsigned.strip_prefix("0X"))
        .or_else(|| unsigned.strip_prefix('#'))
    {
        (16, digits)
    } else if unsigned.len() > 1 && unsigned.starts_with('0') {
        (8, &unsigned[1..])
    } else {
        (10, unsigned)
    };
    if digits.is_empty() || matches!(digits.as_bytes().first(), Some(b'+' | b'-')) {
        return Err(HexUtilError::Color(value.to_owned()));
    }
    let magnitude =
        i64::from_str_radix(digits, radix).map_err(|_| HexUtilError::Color(value.to_owned()))?;
    let signed = if negative { -magnitude } else { magnitude };
    i32::try_from(signed).map_err(|_| HexUtilError::Color(value.to_owned()))
}

fn java_f64_bits(value: f64) -> u64 {
    if value.is_nan() {
        0x7ff8_0000_0000_0000
    } else {
        value.to_bits()
    }
}
