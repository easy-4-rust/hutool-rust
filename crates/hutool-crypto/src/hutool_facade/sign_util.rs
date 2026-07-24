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

/// Hutool `Sign` / `SignUtil` SM2-oriented facade (RSA-PSS variants stay on typed helpers).
#[derive(Debug, Clone, Copy, Default)]
pub struct SignUtil;

impl SignUtil {
    /// SM2 sign → 64-byte signature.
    pub fn sm2_sign(secret: &sm2::SecretKey, message: &[u8]) -> Result<[u8; 64], CryptoError> {
        crate::sm2_sign(secret, message)
    }

    /// SM2 verify with uncompressed public hex.
    pub fn sm2_verify(
        public_hex: &str,
        message: &[u8],
        signature: &[u8],
    ) -> Result<bool, CryptoError> {
        crate::sm2_verify(public_hex, message, signature)
    }
}
