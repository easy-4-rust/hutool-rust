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
