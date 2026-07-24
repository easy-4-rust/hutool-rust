//! Hutool-named type facades that delegate to idiomatic helpers.
//!
//! These zero-sized / thin types mirror Hutool class names so callers can find
//! `DigestUtil.md5_hex`, `HMac`, `AES`, `RSA`, `HOTP`, `TOTP`, etc. without
//! changing the underlying RustCrypto implementations.

use crate::{
    aes128_cbc_decrypt, aes128_cbc_encrypt, aes128_ecb_decrypt, aes128_ecb_encrypt,
    aes256_gcm_decrypt, aes256_gcm_encrypt, hotp, hmac_md5_hex, hmac_sha1_hex, hmac_sha256,
    hmac_sha256_hex, hmac_sm3_hex, md5_hex, md5_hex16, md5_hex_repeat, md5_hex_salt,
    md5_hex_salt_repeat, md5_hex_with_salt, sha1_hex, sha256_hex, sha512_hex, sm3_hex,
    sm4_ecb_decrypt, sm4_ecb_encrypt, totp, totp_validate, CryptoError, OtpAlgorithm,
};
use secrecy::SecretString;

/// Hutool `TOTP` facade.
#[derive(Debug, Clone)]
pub struct Totp {
    key: Vec<u8>,
    digits: u32,
    step_secs: u64,
    algorithm: OtpAlgorithm,
}

impl Totp {
    /// Creates TOTP with defaults (6 digits, 30s, HMAC-SHA1).
    #[must_use]
    pub fn new(key: impl Into<Vec<u8>>) -> Self {
        Self {
            key: key.into(),
            digits: 6,
            step_secs: 30,
            algorithm: OtpAlgorithm::HmacSha1,
        }
    }

    /// Sets digit count.
    #[must_use]
    pub fn digits(mut self, digits: u32) -> Self {
        self.digits = digits;
        self
    }

    /// Sets time step seconds.
    #[must_use]
    pub fn step_secs(mut self, step_secs: u64) -> Self {
        self.step_secs = step_secs;
        self
    }

    /// Sets HMAC algorithm.
    #[must_use]
    pub fn algorithm(mut self, algorithm: OtpAlgorithm) -> Self {
        self.algorithm = algorithm;
        self
    }

    /// Generates TOTP for epoch seconds (Hutool `generate`).
    pub fn generate(&self, epoch_secs: u64) -> Result<u32, CryptoError> {
        totp(
            &self.key,
            epoch_secs,
            self.step_secs,
            self.digits,
            self.algorithm,
        )
    }

    /// Validates within offset windows (Hutool `validate`).
    pub fn validate(
        &self,
        epoch_secs: u64,
        offset_size: u64,
        expected: u32,
    ) -> Result<bool, CryptoError> {
        totp_validate(
            &self.key,
            epoch_secs,
            self.step_secs,
            offset_size,
            expected,
            self.digits,
            self.algorithm,
        )
    }
}
