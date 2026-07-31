//! Hutool-named type facades that delegate to idiomatic helpers.
//!
//! These zero-sized / thin types mirror Hutool class names so callers can find
//! `DigestUtil.md5_hex`, `HMac`, `AES`, `RSA`, `HOTP`, `TOTP`, etc. without
//! changing the underlying RustCrypto implementations.

/// 对齐: `cn.hutool.crypto.asymmetric.RSA`
/// RSA 加密器

use crate::CryptoError;

/// Hutool `RSA` facade over [`crate::RsaKeyPair`] helpers.
#[derive(Debug, Clone, Copy, Default)]
pub struct Rsa;

impl Rsa {
    /// Generates a 2048-bit key pair.
    pub fn generate_keypair() -> Result<crate::RsaKeyPair, CryptoError> {
        crate::generate_rsa_keypair()
    }

    /// PKCS#1 v1.5 encrypt.
    pub fn encrypt(
        public_key: &rsa::RsaPublicKey,
        plaintext: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        crate::rsa_encrypt_pkcs1v15(public_key, plaintext)
    }

    /// PKCS#1 v1.5 decrypt.
    pub fn decrypt(
        private_key: &rsa::RsaPrivateKey,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        crate::rsa_decrypt_pkcs1v15(private_key, ciphertext)
    }
}
