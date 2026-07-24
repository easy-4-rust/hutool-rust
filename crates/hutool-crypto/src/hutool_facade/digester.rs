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

/// Hutool `Digester` with salt + repeat count.
#[derive(Debug, Clone)]
pub struct Digester {
    salt: Vec<u8>,
    digest_count: usize,
}

impl Digester {
    /// Creates an MD5 digester (Hutool `new Digester(DigestAlgorithm.MD5)`).
    #[must_use]
    pub fn md5() -> Self {
        Self {
            salt: Vec::new(),
            digest_count: 1,
        }
    }

    /// Sets salt bytes (Hutool `setSalt`).
    #[must_use]
    pub fn set_salt(mut self, salt: impl Into<Vec<u8>>) -> Self {
        self.salt = salt.into();
        self
    }

    /// Sets digest repeat count (Hutool `setDigestCount`).
    #[must_use]
    pub fn set_digest_count(mut self, count: usize) -> Self {
        self.digest_count = count.max(1);
        self
    }

    /// Digests to lowercase hex (Hutool `digestHex`).
    #[must_use]
    pub fn digest_hex(&self, input: impl AsRef<[u8]>) -> String {
        if self.salt.is_empty() {
            md5_hex_repeat(input, self.digest_count)
        } else if self.digest_count <= 1 {
            md5_hex_with_salt(input, &self.salt)
        } else {
            md5_hex_salt_repeat(&self.salt, input, self.digest_count)
        }
    }

    /// Salt-prefixed MD5 without repeat (Hutool saltPosition ≤ 0 path).
    #[must_use]
    pub fn digest_hex_salt_prefix(&self, input: impl AsRef<[u8]>) -> String {
        md5_hex_salt(&self.salt, input)
    }
}
