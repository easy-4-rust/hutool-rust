//! 对齐: `cn.hutool.crypto.symmetric.DES` / `cn.hutool.crypto.symmetric.RC4`
//! 来源: hutool-crypto/src/main/java/cn/hutool/crypto/symmetric/DES.java
//! 中文说明: 已废弃的遗留算法（DES/RC4），安全策略拒绝执行

use crate::CryptoError;

/// DES encrypt — rejected by security policy (Hutool `DesTest` proxy).
pub fn des_encrypt(_key: &[u8], _plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    Err(CryptoError::LegacyRejected(
        "DES is deprecated and rejected by hutool-crypto security policy",
    ))
}

/// DES decrypt — rejected by security policy.
pub fn des_decrypt(_key: &[u8], _ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    Err(CryptoError::LegacyRejected(
        "DES is deprecated and rejected by hutool-crypto security policy",
    ))
}

/// RC4 encrypt — rejected by security policy (Hutool `RC4Test` proxy).
pub fn rc4_crypt(_key: &[u8], _message: &[u8]) -> Result<Vec<u8>, CryptoError> {
    Err(CryptoError::LegacyRejected(
        "RC4 is deprecated and rejected by hutool-crypto security policy",
    ))
}
