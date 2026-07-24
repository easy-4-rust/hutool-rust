//! Hutool-named type facades that delegate to idiomatic helpers.
//!
//! These zero-sized / thin types mirror Hutool class names so callers can find
//! `DigestUtil.md5_hex`, `HMac`, `AES`, `RSA`, `HOTP`, `TOTP`, etc. without
//! changing the underlying RustCrypto implementations.

use crate::{
    aes128_cbc_decrypt, aes128_cbc_encrypt, aes128_ecb_decrypt, aes128_ecb_encrypt,
    aes256_gcm_decrypt, aes256_gcm_encrypt, hotp, hmac_md5_hex, hmac_sha1_hex, hmac_sha256,
    hmac_sha256_hex, hmac_sm3_hex, md5_hex, md5_hex16, md5_hex_repeat, md5_hex_salt,
    md5_hex_salt_repeat, md5_hex_with_salt, sha1_hex, sha256_hex, sha512_hex, sm3_hex,
    sm4_ecb_decrypt, sm4_ecb_encrypt, totp, totp_validate, CryptoError, OtpAlgorithm,
};
use secrecy::SecretString;

mod digest_util;
mod digester;
mod md5_util;
mod sm3_util;
mod h_mac;
mod aes;
mod sm4;
mod rsa;
mod sign_util;
mod hotp;
mod totp;

pub use digest_util::DigestUtil;
pub use digester::Digester;
pub use md5_util::Md5Util;
pub use sm3_util::Sm3Util;
pub use h_mac::HMac;
pub use aes::Aes;
pub use sm4::Sm4;
pub use rsa::Rsa;
pub use sign_util::SignUtil;
pub use hotp::Hotp;
pub use totp::Totp;
