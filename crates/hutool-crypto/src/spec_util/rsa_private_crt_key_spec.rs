//! Key/parameter spec helpers aligned with Hutool `SpecUtil`.

use num_bigint::BigUint;

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
