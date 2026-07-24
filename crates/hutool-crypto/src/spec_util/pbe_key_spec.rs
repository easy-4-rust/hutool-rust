//! Key/parameter spec helpers aligned with Hutool `SpecUtil`.

use crate::{generate_random_key_bytes, CryptoError};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use num_bigint::BigUint;

/// PBE password stand-in for Java `PBEKeySpec`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PbeKeySpec {
    /// Password characters as UTF-8 bytes.
    pub password: Vec<u8>,
}
