//! Key/parameter spec helpers aligned with Hutool `SpecUtil`.

use crate::{generate_random_key_bytes, CryptoError};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use num_bigint::BigUint;

/// Algorithm key material stand-in for Java `KeySpec` / `SecretKeySpec`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeySpecBytes {
    /// Algorithm name (DES / DESede / AES / …).
    pub algorithm: String,
    /// Raw key bytes.
    pub key: Vec<u8>,
}
