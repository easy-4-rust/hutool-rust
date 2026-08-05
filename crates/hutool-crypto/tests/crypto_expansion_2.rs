//! Crypto 扩展 parity 测试 2
//! 对齐: hutool-crypto 多个测试类

use hutool_crypto::{
    aes256_gcm_decrypt, aes256_gcm_encrypt, hash_password, hmac_sha256, sha256_hex,
    verify_hmac_sha256, verify_password,
};
use secrecy::SecretString;

// ── SHA-256 扩展测试 (8 tests) ──

#[test]
fn sha256_basic() {
    let hash = sha256_hex("hello");
    assert_eq!(hash.len(), 64);
}

#[test]
fn sha256_empty() {
    let hash = sha256_hex("");
    assert_eq!(hash.len(), 64);
}

#[test]
fn sha256_deterministic() {
    assert_eq!(sha256_hex("test"), sha256_hex("test"));
}

#[test]
fn sha256_different() {
    assert_ne!(sha256_hex("hello"), sha256_hex("world"));
}

#[test]
fn sha256_unicode() {
    let hash = sha256_hex("你好");
    assert_eq!(hash.len(), 64);
}

#[test]
fn sha256_long_string() {
    let long = "a".repeat(10000);
    let hash = sha256_hex(&long);
    assert_eq!(hash.len(), 64);
}

#[test]
fn sha256_binary_like() {
    let hash = sha256_hex("\x00\x01\x02");
    assert_eq!(hash.len(), 64);
}

#[test]
fn sha256_special_chars() {
    let hash = sha256_hex("!@#$%^&*()");
    assert_eq!(hash.len(), 64);
}

// ── HMAC-SHA256 扩展测试 (6 tests) ──

#[test]
fn hmac_basic() {
    let mac = hmac_sha256(b"key", b"message").unwrap();
    assert_eq!(mac.len(), 32);
}

#[test]
fn hmac_verify_correct() {
    let mac = hmac_sha256(b"key", b"msg").unwrap();
    assert!(verify_hmac_sha256(b"key", b"msg", &mac).unwrap());
}

#[test]
fn hmac_verify_wrong_key() {
    let mac = hmac_sha256(b"key", b"msg").unwrap();
    assert!(!verify_hmac_sha256(b"wrong", b"msg", &mac).unwrap());
}

#[test]
fn hmac_verify_wrong_msg() {
    let mac = hmac_sha256(b"key", b"msg").unwrap();
    assert!(!verify_hmac_sha256(b"key", b"other", &mac).unwrap());
}

#[test]
fn hmac_deterministic() {
    let mac1 = hmac_sha256(b"key", b"msg").unwrap();
    let mac2 = hmac_sha256(b"key", b"msg").unwrap();
    assert_eq!(mac1, mac2);
}

#[test]
fn hmac_different_keys() {
    let mac1 = hmac_sha256(b"key1", b"msg").unwrap();
    let mac2 = hmac_sha256(b"key2", b"msg").unwrap();
    assert_ne!(mac1, mac2);
}

// ── BCrypt 扩展测试 (5 tests) ──

#[test]
fn bcrypt_hash_verify() {
    let pw = SecretString::new("password".into());
    let hash = hash_password(&pw).unwrap();
    assert!(verify_password(&pw, &hash).unwrap());
}

#[test]
fn bcrypt_wrong_password() {
    let pw = SecretString::new("password".into());
    let hash = hash_password(&pw).unwrap();
    let wrong = SecretString::new("wrong".into());
    assert!(!verify_password(&wrong, &hash).unwrap());
}

#[test]
fn bcrypt_different_hashes() {
    let pw = SecretString::new("password".into());
    let hash1 = hash_password(&pw).unwrap();
    let hash2 = hash_password(&pw).unwrap();
    assert_ne!(hash1, hash2);
}

#[test]
fn bcrypt_empty_password() {
    let pw = SecretString::new("".into());
    let hash = hash_password(&pw).unwrap();
    assert!(verify_password(&pw, &hash).unwrap());
}

#[test]
fn bcrypt_long_password() {
    let pw = SecretString::new("a".repeat(100).into());
    let hash = hash_password(&pw).unwrap();
    assert!(verify_password(&pw, &hash).unwrap());
}

// ── AES-256-GCM 扩展测试 (5 tests) ──

#[test]
fn aes_encrypt_decrypt() {
    let key = b"0123456789abcdef0123456789abcdef";
    let plaintext = b"hello";
    let ct = aes256_gcm_encrypt(key, plaintext).unwrap();
    let pt = aes256_gcm_decrypt(key, &ct).unwrap();
    assert_eq!(pt, plaintext);
}

#[test]
fn aes_wrong_key() {
    let key = b"0123456789abcdef0123456789abcdef";
    let ct = aes256_gcm_encrypt(key, b"hello").unwrap();
    let wrong = b"abcdef0123456789abcdef0123456789";
    assert!(aes256_gcm_decrypt(wrong, &ct).is_err());
}

#[test]
fn aes_empty_plaintext() {
    let key = b"0123456789abcdef0123456789abcdef";
    let ct = aes256_gcm_encrypt(key, b"").unwrap();
    let pt = aes256_gcm_decrypt(key, &ct).unwrap();
    assert!(pt.is_empty());
}

#[test]
fn aes_large_plaintext() {
    let key = b"0123456789abcdef0123456789abcdef";
    let plaintext = vec![42u8; 10000];
    let ct = aes256_gcm_encrypt(key, &plaintext).unwrap();
    let pt = aes256_gcm_decrypt(key, &ct).unwrap();
    assert_eq!(pt, plaintext);
}

#[test]
fn aes_unicode_plaintext() {
    let key = b"0123456789abcdef0123456789abcdef";
    let plaintext = "你好世界".as_bytes();
    let ct = aes256_gcm_encrypt(key, plaintext).unwrap();
    let pt = aes256_gcm_decrypt(key, &ct).unwrap();
    assert_eq!(pt, plaintext);
}
