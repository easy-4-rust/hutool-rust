//! 对齐: `cn.hutool.crypto.digest.PBKDF2`
//! 来源: hutool-crypto/src/main/java/cn/hutool/crypto/digest/PBKDF2.java
//! 中文说明: PBKDF2 密码派生工具，对齐 Hutool PBKDF2

use crate::CryptoError;
use pbkdf2::pbkdf2_hmac;
use sha1::Sha1;

const PBKDF2_OUTPUT_BYTES: usize = 64;

/// Derives a 512-bit key and returns 128-char lowercase hex (Hutool `SecureUtil.pbkdf2`).
pub fn pbkdf2_hex(password: &[u8], salt: &[u8]) -> Result<String, CryptoError> {
    let mut out = [0u8; PBKDF2_OUTPUT_BYTES];
    pbkdf2_hmac::<Sha1>(password, salt, 1000, &mut out);
    Ok(hex::encode(out))
}
