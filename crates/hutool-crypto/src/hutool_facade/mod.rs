//! Hutool-named type facades that delegate to idiomatic helpers.
//!
//! These zero-sized / thin types mirror Hutool class names so callers can find
//! `DigestUtil.md5_hex`, `HMac`, `AES`, `RSA`, `HOTP`, `TOTP`, etc. without
//! changing the underlying RustCrypto implementations.

mod aes;
mod digest_util;
mod digester;
mod h_mac;
mod hotp;
mod md5_util;
mod rsa;
mod sign_util;
mod sm3_util;
mod sm4;
mod totp;

pub use aes::Aes;
pub use digest_util::DigestUtil;
pub use digester::Digester;
pub use h_mac::HMac;
pub use hotp::Hotp;
pub use md5_util::Md5Util;
pub use rsa::Rsa;
pub use sign_util::SignUtil;
pub use sm3_util::Sm3Util;
pub use sm4::Sm4;
pub use totp::Totp;
