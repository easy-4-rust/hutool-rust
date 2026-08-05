//! 对齐: `cn.hutool.crypto.symmetric.ChaCha20`
//! 来源: hutool-crypto/src/main/java/cn/hutool/crypto/symmetric/ChaCha20.java
//! 中文说明: ChaCha20 对称加密工具，对齐 Hutool ChaCha20 测试向量

use crate::CryptoError;
use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};

type ChaCha20Cipher = ChaCha20;

/// Encrypts with ChaCha20 and returns lowercase hex (Hutool `ChaCha20.encryptHex`).
pub fn chacha20_encrypt_hex(
    key: &[u8],
    iv: &[u8],
    plaintext: &[u8],
) -> Result<String, CryptoError> {
    Ok(hex::encode(chacha20_encrypt(key, iv, plaintext)?))
}

/// Encrypts with ChaCha20 (32-byte key, 12-byte nonce).
pub fn chacha20_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if key.len() != 32 || iv.len() != 12 {
        return Err(CryptoError::InvalidChaChaKey);
    }
    let mut cipher =
        ChaCha20Cipher::new_from_slices(key, iv).map_err(|_| CryptoError::InvalidChaChaKey)?;
    let mut buf = plaintext.to_vec();
    cipher.apply_keystream(&mut buf);
    Ok(buf)
}

/// Decrypts ChaCha20 ciphertext (symmetric XOR stream).
pub fn chacha20_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    chacha20_encrypt(key, iv, ciphertext)
}
