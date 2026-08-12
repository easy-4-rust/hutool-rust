//! `ProviderFactory` — 对齐 `cn.hutool.crypto.ProviderFactory`。
//! BouncyCastle Provider 工厂（Java-only；Rust 返回 Err）。

use std::any::Any;

use crate::CryptoError;

/// Provider 工厂，对齐 `cn.hutool.crypto.ProviderFactory`。
///
/// Java 用于创建 `org.bouncycastle.jce.provider.BouncyCastleProvider`；
/// Rust 不需要 Provider 概念（RustCrypto 直接静态注册算法）。
pub struct ProviderFactory;

impl ProviderFactory {
    /// 对齐 `ProviderFactory.createBouncyCastleProvider()`
    ///
    /// 返回 Err 引导用户使用 RustCrypto（hutool-crypto 默认后端）。
    pub fn create_bouncy_castle_provider() -> Result<Box<dyn Any>, CryptoError> {
        Err(CryptoError::LegacyRejected(
            "ProviderFactory::create_bouncy_castle_provider requires org.bouncycastle (Java-only); use RustCrypto in Rust",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CipherWrapper, StubCipherWrapper};

    #[test]
    fn test_stub_cipher_wrapper_from_algorithm() {
        let w = StubCipherWrapper::from_algorithm("AES/CBC/PKCS5Padding").unwrap();
        assert_eq!(w.algorithm(), "AES/CBC/PKCS5Padding");
    }

    #[test]
    fn test_stub_cipher_wrapper_from_cipher_unsupported() {
        let r = StubCipherWrapper::from_cipher(Box::new(42i32));
        assert!(r.is_err());
    }

    #[test]
    fn test_stub_cipher_wrapper_set_params_unsupported() {
        let mut w = StubCipherWrapper::from_algorithm("AES").unwrap();
        let r = w.set_params(Box::new("params"));
        assert!(r.is_err());
    }

    #[test]
    fn test_stub_cipher_wrapper_set_random_unsupported() {
        let mut w = StubCipherWrapper::from_algorithm("AES").unwrap();
        let r = w.set_random(Box::new(42i32));
        assert!(r.is_err());
    }

    #[test]
    fn test_stub_cipher_wrapper_init_mode_unsupported() {
        let mut w = StubCipherWrapper::from_algorithm("AES").unwrap();
        let r = w.init_mode(1, &[0u8; 16]);
        assert!(r.is_err());
    }

    #[test]
    fn test_provider_factory_bouncy_castle_unsupported() {
        let r = ProviderFactory::create_bouncy_castle_provider();
        assert!(r.is_err());
    }
}
