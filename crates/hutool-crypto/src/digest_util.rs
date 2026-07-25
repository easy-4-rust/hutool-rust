//! 对齐: `cn.hutool.crypto.digest.DigestUtil`
//! 来源: hutool-crypto/src/main/java/cn/hutool/crypto/digest/DigestUtil.java
//! 中文说明: 消息摘要工具（MD5/SHA1/SHA256/SHA512/SM3），对齐 Hutool DigestUtil

use md5::{Digest as _, Md5};
use sha1::{Digest as _, Sha1};
use sha2::{Digest as _, Sha256, Sha512};
use sm3::{Digest as Sm3Digest, Sm3};

/// Returns lowercase MD5 hex (Hutool `DigestUtil.md5Hex`).
#[must_use]
pub fn md5_hex(input: impl AsRef<[u8]>) -> String {
    hex::encode(Md5::digest(input.as_ref()))
}

/// Returns the middle 16 hex chars of MD5 (Hutool `DigestUtil.md5Hex` 16-bit form).
#[must_use]
pub fn md5_hex16(input: impl AsRef<[u8]>) -> String {
    let full = md5_hex(input);
    full[8..24].to_string()
}

/// Digests `count` times (Hutool `Digester.setDigestCount`).
#[must_use]
pub fn md5_hex_repeat(input: impl AsRef<[u8]>, count: usize) -> String {
    let mut data = input.as_ref().to_vec();
    for _ in 0..count.max(1) {
        data = Md5::digest(&data).to_vec();
    }
    hex::encode(data)
}

/// MD5 hex with salt prefix (Hutool `Digester.setSalt`).
#[must_use]
pub fn md5_hex_with_salt(input: impl AsRef<[u8]>, salt: &[u8]) -> String {
    let mut buf = salt.to_vec();
    buf.extend_from_slice(input.as_ref());
    md5_hex(buf)
}

/// Returns lowercase SHA-1 hex.
#[must_use]
pub fn sha1_hex(input: impl AsRef<[u8]>) -> String {
    hex::encode(Sha1::digest(input.as_ref()))
}

/// Returns lowercase SHA-256 hex.
#[must_use]
pub fn sha256_hex(input: impl AsRef<[u8]>) -> String {
    hex::encode(Sha256::digest(input.as_ref()))
}

/// Returns lowercase SHA-512 hex.
#[must_use]
pub fn sha512_hex(input: impl AsRef<[u8]>) -> String {
    hex::encode(Sha512::digest(input.as_ref()))
}

/// Returns lowercase SM3 hex (Hutool `SmUtil.sm3`).
#[must_use]
pub fn sm3_hex(input: impl AsRef<[u8]>) -> String {
    hex::encode(Sm3::digest(input.as_ref()))
}

/// MD5 with salt prepended (`Digester.setSalt`, `saltPosition <= 0`).
#[must_use]
pub fn md5_hex_salt(salt: &[u8], input: impl AsRef<[u8]>) -> String {
    let mut buf = Vec::with_capacity(salt.len() + input.as_ref().len());
    buf.extend_from_slice(salt);
    buf.extend_from_slice(input.as_ref());
    md5_hex(buf)
}

/// MD5 with salt and repeat count (`Digester.setDigestCount`).
#[must_use]
pub fn md5_hex_salt_repeat(salt: &[u8], input: impl AsRef<[u8]>, count: usize) -> String {
    let mut data = {
        let mut buf = Vec::with_capacity(salt.len() + input.as_ref().len());
        buf.extend_from_slice(salt);
        buf.extend_from_slice(input.as_ref());
        Md5::digest(&buf).to_vec()
    };
    for _ in 1..count.max(1) {
        data = Md5::digest(&data).to_vec();
    }
    hex::encode(data)
}
