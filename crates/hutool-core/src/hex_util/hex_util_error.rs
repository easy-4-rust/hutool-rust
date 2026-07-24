//! Convenient hexadecimal operations aligned with Hutool's `HexUtil` family.

use std::num::ParseIntError;

use encoding_rs::{Encoding, UTF_8, UTF_16BE, UTF_16LE};
use num_bigint::BigInt;

use crate::{Base16Codec, CoreError};

use super::hex_util::HexUtil;

/// Errors produced by [`HexUtil`] conversions.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum HexUtilError {
    /// Base16 decoding failed.
    #[error(transparent)]
    Decode(#[from] CoreError),

    /// A fixed-width integer could not be parsed.
    #[error("invalid hexadecimal integer: {0}")]
    Integer(#[from] ParseIntError),

    /// An arbitrary-precision hexadecimal integer could not be parsed.
    #[error("invalid hexadecimal big integer: {0}")]
    BigInteger(String),

    /// A Java-style color literal was malformed or out of range.
    #[error("invalid color literal: {0}")]
    Color(String),
}
