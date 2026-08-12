//! `Crypto` 对比验证测试 —— 对齐 Hutool `SecureUtilTest` / `Argon2Test` / `HmacTest`
//!
//! 对齐: `cn.hutool.crypto.SecureUtilTest` (14 个 @Test)
//! 对齐: `cn.hutool.crypto.digest.Argon2Test` (2 个 @Test)
//! 对齐: `cn.hutool.crypto.digest.HmacTest` (6 个 @Test)
//! 来源: hutool-crypto/src/test/java/cn/hutool/crypto/
//!
//! hutool-crypto 只有 7 个函数: sha256_hex, hmac_sha256, verify_hmac_sha256,
//! hash_password, verify_password, aes256_gcm_encrypt, aes256_gcm_decrypt。
//! 其余 738 个 API 都是桩。本文件只测试已实现的函数。

use hutool_crypto::{self as hc};

// ════════════════════════════════════════════════════════════
//  SecureUtilTest — SHA-256 系列
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `SecureUtilTest.sha256Test()`
#[test]
fn sha256_test() {
    let input = "test中文";
    let hash = hc::sha256_hex(input.as_bytes());
    assert_eq!(hash.len(), 64, "SHA-256 输出应为 64 位十六进制 (对齐 Java)");
    let hash2 = hc::sha256_hex(input.as_bytes());
    assert_eq!(hash, hash2, "SHA-256 幂等性 (对齐 Java)");
}

/// 对齐 Java: `SecureUtilTest.sha256Test()` — 空字符串
#[test]
fn sha256_empty_test() {
    let hash = hc::sha256_hex("");
    assert_eq!(
        hash, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        "SHA-256 空字符串 (对齐 Java)"
    );
}

// ════════════════════════════════════════════════════════════
//  SecureUtilTest — HMAC-SHA256 系列
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `SecureUtilTest.hmacSha256Test()`
#[test]
fn hmac_sha256_test() {
    let key = "password".as_bytes();
    let message = "test中文".as_bytes();
    let mac = hc::hmac_sha256(key, message).unwrap();
    let hex = hex::encode(mac);
    assert_eq!(
        hex.len(),
        64,
        "HMAC-SHA256 输出应为 64 位十六进制 (对齐 Java)"
    );
    let mac2 = hc::hmac_sha256(key, message).unwrap();
    assert_eq!(mac, mac2, "HMAC-SHA256 幂等性 (对齐 Java)");
}

/// 对齐 Java: `SecureUtilTest.hmacSha256Test()` — 验证密钥不同时结果不同
#[test]
fn hmac_sha256_different_keys() {
    let message = "test".as_bytes();
    let mac1 = hc::hmac_sha256("key1".as_bytes(), message).unwrap();
    let mac2 = hc::hmac_sha256("key2".as_bytes(), message).unwrap();
    assert_ne!(mac1, mac2, "不同密钥应产生不同 HMAC (对齐 Java)");
}

// ════════════════════════════════════════════════════════════
//  Argon2Test
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `Argon2Test.argon2Test()`
#[test]
fn argon2_test() {
    let password = secrecy::SecretString::from("123456");
    let encoded = hc::hash_password(&password).unwrap();
    assert!(
        encoded.starts_with("$argon2"),
        "Argon2 输出应以 $argon2 开头 (对齐 Java)"
    );
    assert!(
        hc::verify_password(&password, &encoded).unwrap(),
        "密码验证应成功 (对齐 Java)"
    );
}

/// 对齐 Java: `Argon2Test.argon2Test()` — 错误密码应失败
#[test]
fn argon2_wrong_password() {
    let password = secrecy::SecretString::from("123456");
    let encoded = hc::hash_password(&password).unwrap();
    let wrong = secrecy::SecretString::from("654321");
    assert!(
        !hc::verify_password(&wrong, &encoded).unwrap(),
        "错误密码应验证失败 (对齐 Java)"
    );
}

// ════════════════════════════════════════════════════════════
//  AESTest — AES-256-GCM
// ════════════════════════════════════════════════════════════

/// 对齐 Java: `SymmetricTest.aesTest()`
#[test]
fn aes256_gcm_round_trip() {
    let key = [0u8; 32];
    let plaintext = "test中文".as_bytes();
    let ciphertext = hc::aes256_gcm_encrypt(&key, plaintext).unwrap();
    assert!(
        ciphertext.len() > plaintext.len(),
        "密文应比明文长 (对齐 Java)"
    );
    let decrypted = hc::aes256_gcm_decrypt(&key, &ciphertext).unwrap();
    assert_eq!(
        decrypted, plaintext,
        "AES-256-GCM 解密后应等于原文 (对齐 Java)"
    );
}

/// 对齐 Java: `SymmetricTest.aesTest()` — 错误密钥应解密失败
#[test]
fn aes256_gcm_wrong_key() {
    let key1 = [0u8; 32];
    let key2 = [1u8; 32];
    let plaintext = b"secret";
    let ciphertext = hc::aes256_gcm_encrypt(&key1, plaintext).unwrap();
    let result = hc::aes256_gcm_decrypt(&key2, &ciphertext);
    assert!(result.is_err(), "错误密钥应解密失败 (对齐 Java)");
}

/// 对齐 Java: `SymmetricTest.aesTest()` — 空明文
#[test]
fn aes256_gcm_empty_plaintext() {
    let key = [0u8; 32];
    let plaintext = b"";
    let ciphertext = hc::aes256_gcm_encrypt(&key, plaintext).unwrap();
    let decrypted = hc::aes256_gcm_decrypt(&key, &ciphertext).unwrap();
    assert_eq!(
        decrypted, plaintext,
        "空明文 AES-256-GCM round-trip (对齐 Java)"
    );
}
