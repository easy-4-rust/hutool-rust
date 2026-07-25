use std::collections::HashSet;

use thiserror::Error;

use super::radix_error::RadixError;

/// Arbitrary-alphabet integer conversion compatible with Hutool's `RadixUtil`.

/// 对齐: `cn.hutool.core.util.RadixUtil`
/// 进制转换工具类
#[derive(Debug, Clone, Copy, Default)]
pub struct RadixUtil;

impl RadixUtil {
    /// Ordered base-34 alphabet without `I` or `O`.
    pub const RADIXS_34: &'static str = "0123456789ABCDEFGHJKLMNPQRSTUVWXYZ";
    /// Hutool's shuffled base-34 alphabet.
    pub const RADIXS_SHUFFLE_34: &'static str = "H3UM16TDFPSBZJ90CW28QYRE45AXKNGV7L";
    /// Ordered base-59 alphabet without `I`, `O`, or lowercase `l`.
    pub const RADIXS_59: &'static str =
        "0123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
    /// Hutool's shuffled base-59 alphabet.
    pub const RADIXS_SHUFFLE_59: &'static str =
        "vh9wGkfK8YmqbsoENP3764SeCX0dVzrgy1HRtpnTaLjJW2xQiZAcBMUFDu5";

    /// Encodes an `i32`; negative values use their unsigned 32-bit representation.
    pub fn encode_i32(alphabet: &str, value: i32) -> Result<String, RadixError> {
        let unsigned = u32::from_ne_bytes(value.to_ne_bytes());
        encode_unsigned(alphabet, u64::from(unsigned))
    }

    /// Encodes a non-negative `i64`.
    pub fn encode_i64(alphabet: &str, value: i64) -> Result<String, RadixError> {
        if value < 0 {
            return Err(RadixError::NegativeI64);
        }
        encode_unsigned(alphabet, value.unsigned_abs())
    }

    /// Decodes into an `i64` with checked arithmetic.
    pub fn decode(alphabet: &str, encoded: &str) -> Result<i64, RadixError> {
        let alphabet = validate_alphabet(alphabet)?;
        if encoded.is_empty() {
            return Err(RadixError::EmptyInput);
        }
        #[allow(clippy::cast_possible_wrap)]
        let radix = alphabet.len() as i64;
        encoded.chars().try_fold(0_i64, |result, character| {
            let digit = alphabet
                .iter()
                .position(|candidate| *candidate == character)
                .ok_or(RadixError::InvalidCharacter(character))?;
            #[allow(clippy::cast_possible_wrap)]
            let digit = digit as i64;
            result
                .checked_mul(radix)
                .and_then(|value| value.checked_add(digit))
                .ok_or(RadixError::Overflow)
        })
    }

    /// Decodes and applies Java's narrowing `long`-to-`int` conversion.
    pub fn decode_to_i32(alphabet: &str, encoded: &str) -> Result<i32, RadixError> {
        Self::decode(alphabet, encoded).map(|value| {
            let bytes = value.to_le_bytes();
            i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        })
    }
}

use super::{MAX_RADIX, encode_unsigned, validate_alphabet};
