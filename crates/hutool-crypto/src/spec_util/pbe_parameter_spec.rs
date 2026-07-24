//! Key/parameter spec helpers aligned with Hutool `SpecUtil`.

use crate::{generate_random_key_bytes, CryptoError};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use num_bigint::BigUint;

/// PBE salt + iteration stand-in for Java `PBEParameterSpec`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PbeParameterSpec {
    /// Salt bytes.
    pub salt: Vec<u8>,
    /// Iteration count.
    pub iteration_count: u32,
}
