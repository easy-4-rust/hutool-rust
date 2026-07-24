//! Convenient hexadecimal operations aligned with Hutool's `HexUtil` family.

use std::num::ParseIntError;

use encoding_rs::{Encoding, UTF_8, UTF_16BE, UTF_16LE};
use num_bigint::BigInt;

use crate::{Base16Codec, CoreError};

/// An RGB color whose channels are always in the valid `0..=255` range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbColor {
    /// Red channel.
    pub red: u8,
    /// Green channel.
    pub green: u8,
    /// Blue channel.
    pub blue: u8,
}

impl RgbColor {
    /// Creates a color from its red, green, and blue channels.
    #[must_use]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}
