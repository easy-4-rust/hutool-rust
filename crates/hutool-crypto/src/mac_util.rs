//! 对齐: `cn.hutool.crypto.digest.HMac`
//! 来源: hutool-crypto/src/main/java/cn/hutool/crypto/digest/HMac.java
//! 中文说明: HMAC 消息认证码工具（MD5/SHA1/SHA256/SHA512/SM3），对齐 Hutool HMac

use crate::CryptoError;
use hmac::{Hmac, Mac};
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use sm3::Sm3;

type HmacMd5 = Hmac<Md5>;
type HmacSha1 = Hmac<Sha1>;
type HmacSha256 = Hmac<Sha256>;
type HmacSha512 = Hmac<Sha512>;
type HmacSm3 = Hmac<Sm3>;

/// HMAC-MD5 hex (Hutool `HmacAlgorithm.HmacMD5`).
pub fn hmac_md5_hex(key: &[u8], message: &[u8]) -> Result<String, CryptoError> {
    let mut mac = HmacMd5::new_from_slice(key).map_err(|_| CryptoError::InvalidMacKey)?;
    mac.update(message);
    Ok(hex::encode(mac.finalize().into_bytes()))
}

/// HMAC-SHA1 hex.
pub fn hmac_sha1_hex(key: &[u8], message: &[u8]) -> Result<String, CryptoError> {
    let mut mac = HmacSha1::new_from_slice(key).map_err(|_| CryptoError::InvalidMacKey)?;
    mac.update(message);
    Ok(hex::encode(mac.finalize().into_bytes()))
}

/// HMAC-SHA256 bytes.
pub fn hmac_sha256(key: &[u8], message: &[u8]) -> Result<[u8; 32], CryptoError> {
    let mut mac = HmacSha256::new_from_slice(key).map_err(|_| CryptoError::InvalidMacKey)?;
    mac.update(message);
    let bytes = mac.finalize().into_bytes();
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    Ok(out)
}

/// HMAC-SHA256 hex.
pub fn hmac_sha256_hex(key: &[u8], message: &[u8]) -> Result<String, CryptoError> {
    Ok(hex::encode(hmac_sha256(key, message)?))
}

/// Verifies HMAC-SHA256 in constant time.
pub fn verify_hmac_sha256(
    key: &[u8],
    message: &[u8],
    expected: &[u8],
) -> Result<bool, CryptoError> {
    let mut mac = HmacSha256::new_from_slice(key).map_err(|_| CryptoError::InvalidMacKey)?;
    mac.update(message);
    Ok(mac.verify_slice(expected).is_ok())
}

/// HMAC-SHA512 bytes.
pub fn hmac_sha512(key: &[u8], message: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let mut mac = HmacSha512::new_from_slice(key).map_err(|_| CryptoError::InvalidMacKey)?;
    mac.update(message);
    Ok(mac.finalize().into_bytes().to_vec())
}

/// HMAC-SM3 hex (Hutool `SmUtil.hmacSm3`).
pub fn hmac_sm3_hex(key: &[u8], message: &[u8]) -> Result<String, CryptoError> {
    let mut mac = HmacSm3::new_from_slice(key).map_err(|_| CryptoError::InvalidMacKey)?;
    mac.update(message);
    Ok(hex::encode(mac.finalize().into_bytes()))
}
