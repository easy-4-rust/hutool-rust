//! Hutool-named type facades that delegate to idiomatic helpers.
//!
//! These zero-sized / thin types mirror Hutool class names so callers can find
//! `DigestUtil.md5_hex`, `HMac`, `AES`, `RSA`, `HOTP`, `TOTP`, etc. without
//! changing the underlying RustCrypto implementations.

/// 对齐: `cn.hutool.crypto.digest.DigestUtil`
/// 摘要工具类
use crate::{CryptoError, md5_hex, md5_hex16, sha1_hex, sha256_hex, sha512_hex};
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
