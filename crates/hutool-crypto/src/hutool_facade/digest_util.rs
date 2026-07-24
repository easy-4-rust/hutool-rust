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

/// Hutool `DigestUtil` static facade.
#[derive(Debug, Clone, Copy, Default)]
pub struct DigestUtil;

impl DigestUtil {
    /// Hutool `DigestUtil.md5Hex`.
    #[must_use]
    pub fn md5_hex(input: impl AsRef<[u8]>) -> String {
        md5_hex(input)
    }

    /// Hutool `DigestUtil.md5Hex` 16-char form.
    #[must_use]
    pub fn md5_hex16(input: impl AsRef<[u8]>) -> String {
        md5_hex16(input)
    }

    /// Hutool `DigestUtil.sha1Hex`.
    #[must_use]
    pub fn sha1_hex(input: impl AsRef<[u8]>) -> String {
        sha1_hex(input)
    }

    /// Hutool `DigestUtil.sha256Hex`.
    #[must_use]
    pub fn sha256_hex(input: impl AsRef<[u8]>) -> String {
        sha256_hex(input)
    }

    /// Hutool `DigestUtil.sha512Hex`.
    #[must_use]
    pub fn sha512_hex(input: impl AsRef<[u8]>) -> String {
        sha512_hex(input)
    }

    /// Hutool `DigestUtil.bcrypt` → Argon2id PHC string.
    pub fn bcrypt(password: &str) -> Result<String, CryptoError> {
        crate::hash_password(&SecretString::from(password.to_owned()))
    }

    /// Hutool `DigestUtil.bcryptCheck`.
    pub fn bcrypt_check(password: &str, hashed: &str) -> Result<bool, CryptoError> {
        crate::verify_password(&SecretString::from(password.to_owned()), hashed)
    }
}
