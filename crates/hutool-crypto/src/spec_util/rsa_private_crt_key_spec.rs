//! Key/parameter spec helpers aligned with Hutool `SpecUtil`.

use crate::{generate_random_key_bytes, CryptoError};
use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use num_bigint::BigUint;

use super::spec_util::SpecUtil;

/// RSA CRT components from C# XML key export (`SpecUtil.xmlToRSAPrivateCrtKeySpec`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsaPrivateCrtKeySpec {
    /// Modulus `n`.
    pub modulus: BigUint,
    /// Public exponent `e`.
    pub public_exponent: BigUint,
    /// Private exponent `d`.
    pub private_exponent: BigUint,
    /// Prime `p`.
    pub prime_p: BigUint,
    /// Prime `q`.
    pub prime_q: BigUint,
    /// `d mod (p-1)`.
    pub prime_exponent_p: BigUint,
    /// `d mod (q-1)`.
    pub prime_exponent_q: BigUint,
    /// CRT coefficient `q^-1 mod p`.
    pub crt_coefficient: BigUint,
}
