//! Shared decode helpers aligned with Hutool `SecureUtil.decode`.

/// Decodes hex (when all chars are hex) or standard Base64, like Hutool `SecureUtil.decode`.
pub fn decode(input: &str) -> Result<Vec<u8>, crate::CryptoError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    if is_hex(trimmed) {
        hex::decode(trimmed).map_err(|_| crate::CryptoError::InvalidEncoding)
    } else {
        use base64::Engine as _;
        base64::engine::general_purpose::STANDARD
            .decode(trimmed)
            .map_err(|_| crate::CryptoError::InvalidEncoding)
    }
}

fn is_hex(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Parses an OpenSSH / PKCS#8 / SEC1 private key blob from Base64 or hex.
#[allow(dead_code)] // 对齐 Hutool `SecureUtil` 的密钥 blob 解析入口，供后续接入使用
pub(crate) fn decode_private_key_blob(key: &str) -> Result<Vec<u8>, crate::CryptoError> {
    decode(key)
}

/// Parses a SubjectPublicKeyInfo / PKCS#1 public key blob from Base64 or hex.
#[allow(dead_code)] // 对齐 Hutool `SecureUtil` 的密钥 blob 解析入口，供后续接入使用
pub(crate) fn decode_public_key_blob(key: &str) -> Result<Vec<u8>, crate::CryptoError> {
    decode(key)
}
