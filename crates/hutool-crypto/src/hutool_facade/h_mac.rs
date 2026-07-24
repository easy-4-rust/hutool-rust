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

/// Hutool `HMac` facade over typed HMAC helpers.
#[derive(Debug, Clone)]
pub struct HMac {
    key: Vec<u8>,
}

impl HMac {
    /// Creates HMAC with raw key bytes (Hutool `new HMac(algorithm, key)`).
    #[must_use]
    pub fn new(key: impl Into<Vec<u8>>) -> Self {
        Self { key: key.into() }
    }

    /// HMAC-SHA256 hex (default Hutool HmacSHA256 path).
    pub fn digest_hex(&self, message: &[u8]) -> Result<String, CryptoError> {
        hmac_sha256_hex(&self.key, message)
    }

    /// HMAC-SHA256 bytes.
    pub fn digest(&self, message: &[u8]) -> Result<[u8; 32], CryptoError> {
        hmac_sha256(&self.key, message)
    }

    /// HMAC-MD5 hex.
    pub fn digest_md5_hex(&self, message: &[u8]) -> Result<String, CryptoError> {
        hmac_md5_hex(&self.key, message)
    }

    /// HMAC-SHA1 hex.
    pub fn digest_sha1_hex(&self, message: &[u8]) -> Result<String, CryptoError> {
        hmac_sha1_hex(&self.key, message)
    }

    /// HMAC-SM3 hex (`SmUtil.hmacSm3`).
    pub fn digest_sm3_hex(&self, message: &[u8]) -> Result<String, CryptoError> {
        hmac_sm3_hex(&self.key, message)
    }
}
