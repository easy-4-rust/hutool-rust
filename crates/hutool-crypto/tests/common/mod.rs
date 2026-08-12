//! Shared helpers for Hutool crypto parity integration tests.

use hutool_crypto as hc;
use std::fs;

pub const RSA_PLAINTEXT: &str = "我是一段测试aaaa";
#[allow(dead_code)] // 保留给后续长度/边界类 parity 测试使用
pub const RSA_LONG_BASE: &str = "我是一段特别长的测试";

/// Loads a PEM fixture from `tests/resources/`.
pub fn load_resource(name: &str) -> String {
    let path = format!("{}/tests/resources/{name}", env!("CARGO_MANIFEST_DIR"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"))
}

/// AES-256-GCM round-trip helper.
#[allow(dead_code)] // 保留给后续 AES parity 测试使用
pub fn aes_gcm_round_trip(plaintext: &[u8]) {
    let key = [7u8; 32];
    let ct = hc::aes256_gcm_encrypt(&key, plaintext).expect("encrypt");
    let pt = hc::aes256_gcm_decrypt(&key, &ct).expect("decrypt");
    assert_eq!(pt, plaintext);
}

/// RSA PKCS#1 v1.5 public encrypt / private decrypt round-trip.
pub fn rsa_pub_enc_priv_dec(plaintext: &str) {
    let pair = hc::generate_rsa_keypair().expect("keygen");
    let ct = hc::rsa_encrypt_pkcs1v15(&pair.public_key, plaintext.as_bytes()).expect("enc");
    let pt = hc::rsa_decrypt_pkcs1v15(&pair.private_key, &ct).expect("dec");
    assert_eq!(pt, plaintext.as_bytes());
}

/// RSA OAEP round-trip.
#[allow(dead_code)] // 保留给后续 RSA-OAEP parity 测试使用
pub fn rsa_oaep_round_trip(plaintext: &str) {
    let pair = hc::generate_rsa_keypair().expect("keygen");
    let ct = hc::rsa_encrypt_oaep(&pair.public_key, plaintext.as_bytes()).expect("enc");
    let pt = hc::rsa_decrypt_oaep(&pair.private_key, &ct).expect("dec");
    assert_eq!(pt, plaintext.as_bytes());
}

/// SM2 sign/verify round-trip with generated keys.
pub fn sm2_sign_verify_round_trip(message: &[u8]) {
    let (secret, public) = hc::generate_sm2_keypair().expect("sm2 keygen");
    let sig = hc::sm2_sign(&secret, message).expect("sign");
    assert_eq!(sig.len(), 64);
    let pub_hex = hc::sm2_public_hex_from_secret(&secret);
    assert!(hc::sm2_verify(&pub_hex, message, &sig).expect("verify"));
    let _ = public;
}
