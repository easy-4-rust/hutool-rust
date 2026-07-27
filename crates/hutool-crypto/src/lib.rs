//! 对齐: `cn.hutool.crypto.SecureUtil`
//! 来源: hutool-crypto/src/main/java/cn/hutool/crypto/SecureUtil.java
//! 中文说明: 加密工具库入口，默认使用认证加密，对齐 Hutool SecureUtil

#![forbid(unsafe_code)]

mod aes_modes;
mod asn1_util;
mod asymmetric;
mod bc_util;
mod chacha_util;
mod cipher_wrapper;
mod provider_factory;
mod digest_util;
mod ecies_util;
mod hutool_facade;
mod key_util;
mod legacy;
mod mac_util;
mod otp_util;
mod pbkdf2_util;
mod pem_util;
pub mod prelude;
mod rsa_util;
mod sm2_util;
mod spec_util;
mod symmetric_legacy;

pub use cipher_wrapper::{CipherWrapper, StubCipherWrapper};
pub use provider_factory::ProviderFactory;

use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng as AeadOsRng},
};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{Error as PasswordHashError, SaltString, rand_core::OsRng as PasswordOsRng},
};
use secrecy::{ExposeSecret, SecretString};
use thiserror::Error;

pub use aes_modes::*;
pub use asn1_util::*;
pub use asymmetric::{asc_to_bcd, bcd_to_str, decode, KeyType};
pub use bc_util::*;
pub use chacha_util::*;
pub use digest_util::*;
pub use ecies_util::*;
pub use hutool_facade::*;
pub use key_util::*;
pub use legacy::*;
pub use mac_util::*;
pub use otp_util::*;
pub use pbkdf2_util::*;
pub use pem_util::*;
pub use rsa_util::*;
pub use sm2_util::*;
pub use spec_util::*;
pub use symmetric_legacy::*;

const AES_256_KEY_LENGTH: usize = 32;
const GCM_NONCE_LENGTH: usize = 12;

/// Errors returned by cryptographic operations.
#[derive(Debug, Error)]
pub enum CryptoError {
    /// AES-256 requires a 32-byte key.
    #[error("AES-256 key must contain exactly 32 bytes")]
    InvalidAesKey,
    /// Ciphertext does not contain the nonce and authentication tag.
    #[error("ciphertext is too short")]
    InvalidCiphertext,
    /// Authentication or encryption failed.
    #[error("authenticated encryption operation failed")]
    Aead,
    /// The supplied MAC key is invalid.
    #[error("invalid HMAC key")]
    InvalidMacKey,
    /// ChaCha20 key or IV length is invalid.
    #[error("invalid ChaCha20 key or nonce")]
    InvalidChaChaKey,
    /// OTP digit count is invalid.
    #[error("invalid OTP digit count")]
    InvalidOtpDigits,
    /// PEM or DER key material is invalid.
    #[error("invalid PEM key material")]
    InvalidPem,
    /// Hex/base64 decoding failed.
    #[error("invalid encoded data")]
    InvalidEncoding,
    /// RSA key parsing or export failed.
    #[error("RSA key error")]
    RsaKey,
    /// RSA encrypt/decrypt failed.
    #[error("RSA operation failed")]
    RsaOperation,
    /// SM2 key material is invalid.
    #[error("SM2 key error")]
    Sm2Key,
    /// SM2 signature is invalid.
    #[error("SM2 signature error")]
    Sm2Signature,
    /// Legacy algorithm rejected by security policy.
    #[error("{0}")]
    LegacyRejected(&'static str),
    /// Password hashing or encoded-hash parsing failed.
    #[error(transparent)]
    PasswordHash(#[from] PasswordHashError),
}

/// Encrypts using AES-256-GCM and prefixes the random 96-bit nonce.
pub fn aes256_gcm_encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if key.len() != AES_256_KEY_LENGTH {
        return Err(CryptoError::InvalidAesKey);
    }
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CryptoError::InvalidAesKey)?;
    let nonce = Aes256Gcm::generate_nonce(&mut AeadOsRng);
    let encrypted = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| CryptoError::Aead)?;
    let mut output = Vec::with_capacity(GCM_NONCE_LENGTH + encrypted.len());
    output.extend_from_slice(&nonce);
    output.extend_from_slice(&encrypted);
    Ok(output)
}

/// Decrypts bytes produced by [`aes256_gcm_encrypt`].
pub fn aes256_gcm_decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if key.len() != AES_256_KEY_LENGTH {
        return Err(CryptoError::InvalidAesKey);
    }
    if ciphertext.len() <= GCM_NONCE_LENGTH {
        return Err(CryptoError::InvalidCiphertext);
    }
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CryptoError::InvalidAesKey)?;
    let (nonce, encrypted) = ciphertext.split_at(GCM_NONCE_LENGTH);
    cipher
        .decrypt(Nonce::from_slice(nonce), encrypted)
        .map_err(|_| CryptoError::Aead)
}

/// Hashes a password with Argon2id and a fresh operating-system random salt.
pub fn hash_password(password: &SecretString) -> Result<String, CryptoError> {
    let salt = SaltString::generate(&mut PasswordOsRng);
    Ok(Argon2::default()
        .hash_password(password.expose_secret().as_bytes(), &salt)?
        .to_string())
}

/// Verifies a password against a PHC-formatted Argon2 hash.
pub fn verify_password(password: &SecretString, encoded_hash: &str) -> Result<bool, CryptoError> {
    let parsed = PasswordHash::new(encoded_hash)?;
    match Argon2::default().verify_password(password.expose_secret().as_bytes(), &parsed) {
        Ok(()) => Ok(true),
        Err(PasswordHashError::Password) => Ok(false),
        Err(error) => Err(CryptoError::PasswordHash(error)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_and_macs_known_values() {
        assert_eq!(
            sha256_hex("abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        let mac = hmac_sha256(b"key", b"message").unwrap();
        assert!(verify_hmac_sha256(b"key", b"message", &mac).unwrap());
        assert!(!verify_hmac_sha256(b"key", b"changed", &mac).unwrap());
    }

    #[test]
    fn authenticated_encryption_round_trips_and_rejects_tampering() {
        let key = [7_u8; 32];
        let mut ciphertext = aes256_gcm_encrypt(&key, b"secret").unwrap();
        assert_eq!(aes256_gcm_decrypt(&key, &ciphertext).unwrap(), b"secret");
        *ciphertext.last_mut().unwrap() ^= 1;
        assert!(matches!(
            aes256_gcm_decrypt(&key, &ciphertext),
            Err(CryptoError::Aead)
        ));
    }

    #[test]
    fn argon2_password_hashes_use_random_salts_and_verify() {
        let password = SecretString::from("correct horse battery staple".to_owned());
        let first = hash_password(&password).unwrap();
        let second = hash_password(&password).unwrap();
        assert_ne!(first, second);
        assert!(verify_password(&password, &first).unwrap());
        let wrong = SecretString::from("wrong".to_owned());
        assert!(!verify_password(&wrong, &first).unwrap());
    }
}
