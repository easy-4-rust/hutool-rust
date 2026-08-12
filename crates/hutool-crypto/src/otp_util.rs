//! 对齐: `cn.hutool.crypto.digest.OTP`
//! 来源: hutool-crypto/src/main/java/cn/hutool/crypto/digest/OTP.java
//! 中文说明: HOTP/TOTP 一次性密码工具（RFC 4226/6238），对齐 Hutool OTP

use crate::CryptoError;
use hmac::{Hmac, KeyInit, Mac};
use sha1::Sha1;
use sha2::{Sha256, Sha512};

/// OTP HMAC algorithm selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtpAlgorithm {
    /// HMAC-SHA1 (default HOTP/TOTP).
    HmacSha1,
    /// HMAC-SHA256.
    HmacSha256,
    /// HMAC-SHA512.
    HmacSha512,
}

/// Generates HOTP per RFC 4226 (Hutool `HOTP.generate`).
pub fn hotp(key: &[u8], counter: u64, digits: u32) -> Result<u32, CryptoError> {
    hotp_with_algorithm(key, counter, digits, OtpAlgorithm::HmacSha1)
}

/// Generates HOTP with explicit HMAC algorithm.
pub fn hotp_with_algorithm(
    key: &[u8],
    counter: u64,
    digits: u32,
    algorithm: OtpAlgorithm,
) -> Result<u32, CryptoError> {
    if digits > 8 {
        return Err(CryptoError::InvalidOtpDigits);
    }
    let mac = otp_mac(key, counter, algorithm)?;
    Ok(truncate_otp(&mac, digits))
}

/// Generates TOTP for epoch seconds (Hutool `TOTP.generate(Instant)`).
pub fn totp(
    key: &[u8],
    epoch_secs: u64,
    step_secs: u64,
    digits: u32,
    algorithm: OtpAlgorithm,
) -> Result<u32, CryptoError> {
    let counter = epoch_secs / step_secs.max(1);
    hotp_with_algorithm(key, counter, digits, algorithm)
}

/// Validates TOTP within `offset_size` steps (Hutool `TOTP.validate`).
pub fn totp_validate(
    key: &[u8],
    epoch_secs: u64,
    step_secs: u64,
    offset_size: u64,
    expected: u32,
    digits: u32,
    algorithm: OtpAlgorithm,
) -> Result<bool, CryptoError> {
    if offset_size == 0 {
        return Ok(totp(key, epoch_secs, step_secs, digits, algorithm)? == expected);
    }
    let step = step_secs.max(1);
    for i in -(offset_size as i64)..=(offset_size as i64) {
        let adj = epoch_secs as i64 + i * step as i64;
        if adj < 0 {
            continue;
        }
        if totp(key, adj as u64, step, digits, algorithm)? == expected {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Decodes a Base32 secret key string (Hutool `Base32.decode(key)`).
pub fn decode_base32_secret(secret: &str) -> Result<Vec<u8>, CryptoError> {
    data_encoding::BASE32_NOPAD
        .decode(secret.as_bytes())
        .map_err(|_| CryptoError::InvalidMacKey)
}

/// Generates a random Base32 secret key (Hutool `TOTP.generateSecretKey`).
pub fn generate_totp_secret_key(byte_len: usize) -> Result<String, CryptoError> {
    use rand_core06::RngCore as _;
    let mut bytes = vec![0u8; byte_len];
    rand_core06::OsRng.fill_bytes(&mut bytes);
    Ok(data_encoding::BASE32_NOPAD.encode(&bytes))
}

fn otp_mac(key: &[u8], counter: u64, algorithm: OtpAlgorithm) -> Result<Vec<u8>, CryptoError> {
    let msg = counter.to_be_bytes();
    match algorithm {
        OtpAlgorithm::HmacSha1 => {
            let mut mac =
                Hmac::<Sha1>::new_from_slice(key).map_err(|_| CryptoError::InvalidMacKey)?;
            mac.update(&msg);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        OtpAlgorithm::HmacSha256 => {
            let mut mac =
                Hmac::<Sha256>::new_from_slice(key).map_err(|_| CryptoError::InvalidMacKey)?;
            mac.update(&msg);
            Ok(mac.finalize().into_bytes().to_vec())
        }
        OtpAlgorithm::HmacSha512 => {
            let mut mac =
                Hmac::<Sha512>::new_from_slice(key).map_err(|_| CryptoError::InvalidMacKey)?;
            mac.update(&msg);
            Ok(mac.finalize().into_bytes().to_vec())
        }
    }
}

fn truncate_otp(mac: &[u8], digits: u32) -> u32 {
    let offset = (mac[mac.len() - 1] & 0x0f) as usize;
    let bin = ((mac[offset] as u32 & 0x7f) << 24)
        | ((mac[offset + 1] as u32) << 16)
        | ((mac[offset + 2] as u32) << 8)
        | (mac[offset + 3] as u32);
    bin % 10u32.pow(digits)
}
