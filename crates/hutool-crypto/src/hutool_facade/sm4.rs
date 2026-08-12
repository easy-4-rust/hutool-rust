//! Hutool-named type facades that delegate to idiomatic helpers.
//!
//! These zero-sized / thin types mirror Hutool class names so callers can find
//! `DigestUtil.md5_hex`, `HMac`, `AES`, `RSA`, `HOTP`, `TOTP`, etc. without
//! changing the underlying RustCrypto implementations.

/// 对齐: `cn.hutool.crypto.symmetric.SM4`
/// SM4 加密器
use crate::{CryptoError, sm4_ecb_decrypt, sm4_ecb_encrypt};

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
