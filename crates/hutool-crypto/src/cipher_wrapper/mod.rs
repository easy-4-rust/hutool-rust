//! CipherWrapper + ProviderFactory facade，对齐 hutool 的：
//! - `cn.hutool.crypto.CipherWrapper`
//! - `cn.hutool.crypto.ProviderFactory`
//!
//! **仅提供 trait 抽象 + 桩实现**。具体 JCE Cipher SPI / BouncyCastle Provider
//! 是 `bouncycastle_only` unsafe-to-copy，Rust 用 RustCrypto 替代。

use std::any::Any;

use crate::CryptoError;

mod cipher_wrapper;
mod stub_cipher_wrapper;

pub use cipher_wrapper::CipherWrapper;
pub use stub_cipher_wrapper::StubCipherWrapper;
