//! Hutool-named type facades that delegate to idiomatic helpers.
//!
//! These zero-sized / thin types mirror Hutool class names so callers can find
//! `DigestUtil.md5_hex`, `HMac`, `AES`, `RSA`, `HOTP`, `TOTP`, etc. without
//! changing the underlying RustCrypto implementations.

/// 对齐: `cn.hutool.crypto.digest.Sm3Util`
/// SM3 摘要
use crate::sm3_hex;

/// Hutool `SM3` / `SmUtil.sm3` convenience type.
#[derive(Debug, Clone, Copy, Default)]
pub struct Sm3Util;

impl Sm3Util {
    /// Hutool `SM3.create().digestHex` / `SmUtil.sm3`.
    #[must_use]
    pub fn digest_hex(input: impl AsRef<[u8]>) -> String {
        sm3_hex(input)
    }
}
