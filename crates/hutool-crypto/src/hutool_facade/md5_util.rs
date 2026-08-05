//! Hutool-named type facades that delegate to idiomatic helpers.
//!
//! These zero-sized / thin types mirror Hutool class names so callers can find
//! `DigestUtil.md5_hex`, `HMac`, `AES`, `RSA`, `HOTP`, `TOTP`, etc. without
//! changing the underlying RustCrypto implementations.

/// 对齐: `cn.hutool.crypto.digest.MD5`
/// MD5 摘要
use crate::md5_hex;

/// Hutool `MD5` convenience type.
#[derive(Debug, Clone, Copy, Default)]
pub struct Md5Util;

impl Md5Util {
    /// Hutool `MD5.create().digestHex`.
    #[must_use]
    pub fn digest_hex(input: impl AsRef<[u8]>) -> String {
        md5_hex(input)
    }
}
