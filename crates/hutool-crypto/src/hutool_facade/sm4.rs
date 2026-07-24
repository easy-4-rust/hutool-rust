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

/// Hutool `SM4` facade.
#[derive(Debug, Clone, Copy, Default)]
pub struct Sm4;

impl Sm4 {
    /// SM4-ECB encrypt.
    pub fn ecb_encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        sm4_ecb_encrypt(key, plaintext)
    }

    /// SM4-ECB decrypt.
    pub fn ecb_decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        sm4_ecb_decrypt(key, ciphertext)
    }
}
