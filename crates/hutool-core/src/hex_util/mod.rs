//! Convenient hexadecimal operations aligned with Hutool's `HexUtil` family.

use std::num::ParseIntError;

use encoding_rs::{Encoding, UTF_8, UTF_16BE, UTF_16LE};
use num_bigint::BigInt;

use crate::{Base16Codec, CoreError};

mod hex_util_error;
mod rgb_color;
mod hex_util;

pub use hex_util_error::HexUtilError;
pub use rgb_color::RgbColor;
pub use hex_util::HexUtil;
