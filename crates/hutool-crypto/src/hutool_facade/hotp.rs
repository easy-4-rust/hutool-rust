//! Hutool-named type facades that delegate to idiomatic helpers.
//!
//! These zero-sized / thin types mirror Hutool class names so callers can find
//! `DigestUtil.md5_hex`, `HMac`, `AES`, `RSA`, `HOTP`, `TOTP`, etc. without
//! changing the underlying RustCrypto implementations.

/// 对齐: `cn.hutool.crypto.hotp.HOTP`
/// HOTP 一次性密码

use crate::{hotp, CryptoError};

/// Hutool `HOTP` facade.
#[derive(Debug, Clone)]
pub struct Hotp {
    key: Vec<u8>,
    digits: u32,
}

impl Hotp {
    /// Creates HOTP with key and digit count (Hutool `new HOTP(key, digits)`).
    #[must_use]
    pub fn new(key: impl Into<Vec<u8>>, digits: u32) -> Self {
        Self {
            key: key.into(),
            digits,
        }
    }

    /// Generates HOTP for counter (Hutool `generate`).
    pub fn generate(&self, counter: u64) -> Result<u32, CryptoError> {
        hotp(&self.key, counter, self.digits)
    }
}
