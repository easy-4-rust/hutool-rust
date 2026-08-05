//! Crypto util parity tests
//! 对齐: hutool-crypto DigestTest/HmacTest/AESTest/BCryptTest

use hutool_crypto::{
    aes256_gcm_decrypt, aes256_gcm_encrypt, hash_password, hmac_sha256, sha256_hex,
    verify_hmac_sha256, verify_password,
};
use secrecy::SecretString;

// ── SHA-256 ──

#[test]
fn sha256_hex_basic() {
    let hash = sha256_hex("hello");
    assert_eq!(hash.len(), 64);
    assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn sha256_hex_deterministic() {
    let h1 = sha256_hex("hello");
    let h2 = sha256_hex("hello");
    assert_eq!(h1, h2);
}

#[test]
fn sha256_hex_different_inputs() {
    let h1 = sha256_hex("hello");
    let h2 = sha256_hex("world");
    assert_ne!(h1, h2);
}

// ── HMAC-SHA256 ──

#[test]
fn hmac_sha256_basic() {
    let key = b"secret_key";
    let message = b"hello";
    let mac = hmac_sha256(key, message).unwrap();
    assert_eq!(mac.len(), 32);
}

#[test]
fn hmac_sha256_verify() {
    let key = b"secret_key";
    let message = b"hello";
    let mac = hmac_sha256(key, message).unwrap();
    assert!(verify_hmac_sha256(key, message, &mac).unwrap());
}

#[test]
fn hmac_sha256_verify_wrong_key() {
    let key = b"secret_key";
    let message = b"hello";
    let mac = hmac_sha256(key, message).unwrap();
    assert!(!verify_hmac_sha256(b"wrong_key", message, &mac).unwrap());
}

#[test]
fn hmac_sha256_verify_wrong_message() {
    let key = b"secret_key";
    let message = b"hello";
    let mac = hmac_sha256(key, message).unwrap();
    assert!(!verify_hmac_sha256(key, b"world", &mac).unwrap());
}

// ── BCrypt password hashing ──

#[test]
fn bcrypt_hash_and_verify() {
    let password = SecretString::new("my_password".into());
    let hash = hash_password(&password).unwrap();
    assert!(verify_password(&password, &hash).unwrap());
}

#[test]
fn bcrypt_verify_wrong_password() {
    let password = SecretString::new("my_password".into());
    let hash = hash_password(&password).unwrap();
    let wrong = SecretString::new("wrong_password".into());
    assert!(!verify_password(&wrong, &hash).unwrap());
}

// ── AES-256-GCM ──

#[test]
fn aes256_gcm_encrypt_decrypt() {
    let key = b"0123456789abcdef0123456789abcdef"; // 32 bytes
    let plaintext = b"hello world";
    let ciphertext = aes256_gcm_encrypt(key, plaintext).unwrap();
    let decrypted = aes256_gcm_decrypt(key, &ciphertext).unwrap();
    assert_eq!(decrypted, plaintext);
}

#[test]
fn aes256_gcm_wrong_key() {
    let key = b"0123456789abcdef0123456789abcdef";
    let plaintext = b"hello world";
    let ciphertext = aes256_gcm_encrypt(key, plaintext).unwrap();
    let wrong_key = b"abcdef0123456789abcdef0123456789";
    let result = aes256_gcm_decrypt(wrong_key, &ciphertext);
    assert!(result.is_err());
}
