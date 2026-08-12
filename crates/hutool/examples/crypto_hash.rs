//! Hutool 能力对照：`hutool-crypto` → `hutool-crypto`（feature `crypto`）。
//!
//! 演示 RustCrypto 安全默认：SHA-256、HMAC-SHA256、AES-256-GCM 往返。
//! 密码哈希（Argon2id）见库 API `hash_password` / `verify_password`。

use hutool::crypto::{aes256_gcm_decrypt, aes256_gcm_encrypt, hmac_sha256_hex, sha256_hex};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let digest = sha256_hex("abc");
    assert_eq!(
        digest,
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    println!("sha256(abc) = {digest}");

    let mac = hmac_sha256_hex(b"key", b"message")?;
    println!("hmac_sha256 = {mac}");

    let key = [7_u8; 32];
    let ciphertext = aes256_gcm_encrypt(&key, b"hutool-rs")?;
    let plaintext = aes256_gcm_decrypt(&key, &ciphertext)?;
    assert_eq!(plaintext, b"hutool-rs");
    println!(
        "aes256_gcm round-trip ok (ciphertext {} bytes)",
        ciphertext.len()
    );

    Ok(())
}
