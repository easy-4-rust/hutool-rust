//! 对齐: cn.hutool.jwt.signers.JwtSignerUtil
//! 来源: hutool-jwt/src/main/java/cn/hutool/jwt/signers/JwtSignerUtil.java
//!
//! 中文说明: Hutool JWT JwtSignerUtil 类型的 Rust 实现。

//! 对齐: `cn.hutool.jwt` 兼容层共享导入
//! 来源: hutool-jwt/src/main/java/cn/hutool/jwt/

use std::fmt;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine as _;
use base64::engine::general_purpose::{STANDARD, URL_SAFE, URL_SAFE_NO_PAD};
use jsonwebtoken::crypto;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use p256::elliptic_curve::sec1::ToEncodedPoint as _;
use rsa::pkcs1::{
    DecodeRsaPrivateKey as _, DecodeRsaPublicKey as _, EncodeRsaPrivateKey as _,
    EncodeRsaPublicKey as _,
};
use rsa::pkcs8::{DecodePrivateKey as _, DecodePublicKey as _, EncodePrivateKey as _};
use serde_json::{Map, Value};


pub struct JWTSignerUtil;

impl JWTSignerUtil {
    /// No-signature compatibility signer.
    #[must_use]
    pub const fn none() -> NoneJWTSigner {
        NoneJWTSigner
    }
    /// HS256 signer.
    pub fn hs256(key: &[u8]) -> Result<HMacJWTSigner, JWTException> {
        HMacJWTSigner::new(Algorithm::HS256, key)
    }
    /// HS384 signer.
    pub fn hs384(key: &[u8]) -> Result<HMacJWTSigner, JWTException> {
        HMacJWTSigner::new(Algorithm::HS384, key)
    }
    /// HS512 signer.
    pub fn hs512(key: &[u8]) -> Result<HMacJWTSigner, JWTException> {
        HMacJWTSigner::new(Algorithm::HS512, key)
    }
    /// RS256 signer from separate PKCS#8/PKCS#1 private and public PEM keys.
    pub fn rs256(
        private_key: &[u8],
        public_key: &[u8],
    ) -> Result<AsymmetricJWTSigner, JWTException> {
        AsymmetricJWTSigner::from_rsa_pem(Algorithm::RS256, private_key, public_key)
    }
    /// RS384 signer from separate private and public PEM keys.
    pub fn rs384(
        private_key: &[u8],
        public_key: &[u8],
    ) -> Result<AsymmetricJWTSigner, JWTException> {
        AsymmetricJWTSigner::from_rsa_pem(Algorithm::RS384, private_key, public_key)
    }
    /// RS512 signer from separate private and public PEM keys.
    pub fn rs512(
        private_key: &[u8],
        public_key: &[u8],
    ) -> Result<AsymmetricJWTSigner, JWTException> {
        AsymmetricJWTSigner::from_rsa_pem(Algorithm::RS512, private_key, public_key)
    }
    /// ES256 signer from separate private and public PEM keys.
    pub fn es256(
        private_key: &[u8],
        public_key: &[u8],
    ) -> Result<EllipticCurveJWTSigner, JWTException> {
        EllipticCurveJWTSigner::from_pem(Algorithm::ES256, private_key, public_key)
    }
    /// ES384 signer from separate private and public PEM keys.
    pub fn es384(
        private_key: &[u8],
        public_key: &[u8],
    ) -> Result<EllipticCurveJWTSigner, JWTException> {
        EllipticCurveJWTSigner::from_pem(Algorithm::ES384, private_key, public_key)
    }
    /// PS256 (RSASSA-PSS) signer from separate private and public PEM keys.
    pub fn ps256(
        private_key: &[u8],
        public_key: &[u8],
    ) -> Result<AsymmetricJWTSigner, JWTException> {
        AsymmetricJWTSigner::from_rsa_pem(Algorithm::PS256, private_key, public_key)
    }
    /// PS384 (RSASSA-PSS) signer from separate private and public PEM keys.
    pub fn ps384(
        private_key: &[u8],
        public_key: &[u8],
    ) -> Result<AsymmetricJWTSigner, JWTException> {
        AsymmetricJWTSigner::from_rsa_pem(Algorithm::PS384, private_key, public_key)
    }
    /// PS512 (RSASSA-PSS) signer from separate private and public PEM keys.
    pub fn ps512(
        private_key: &[u8],
        public_key: &[u8],
    ) -> Result<AsymmetricJWTSigner, JWTException> {
        AsymmetricJWTSigner::from_rsa_pem(Algorithm::PS512, private_key, public_key)
    }
    /// Rejects ES512 because the selected `RustCrypto` JOSE engine does not expose it.
    pub fn es512(
        _private_key: &[u8],
        _public_key: &[u8],
    ) -> Result<Arc<dyn JWTSigner>, JWTException> {
        Self::reject_legacy("ES512")
    }
    /// Rejects Hutool's non-JOSE HMAC-MD5 compatibility alias.
    pub fn hmd5(_key: &[u8]) -> Result<Arc<dyn JWTSigner>, JWTException> {
        Self::reject_legacy("HMD5")
    }
    /// Rejects Hutool's non-JOSE HMAC-SHA1 compatibility alias.
    pub fn hsha1(_key: &[u8]) -> Result<Arc<dyn JWTSigner>, JWTException> {
        Self::reject_legacy("HSHA1")
    }
    /// Rejects Hutool's non-JOSE SM4-CMAC compatibility alias.
    pub fn sm4cmac(_key: &[u8]) -> Result<Arc<dyn JWTSigner>, JWTException> {
        Self::reject_legacy("SM4CMAC")
    }
    /// Rejects Hutool's obsolete RSA-MD2 compatibility alias.
    pub fn rmd2(_key: &[u8]) -> Result<Arc<dyn JWTSigner>, JWTException> {
        Self::reject_legacy("RMD2")
    }
    /// Rejects Hutool's obsolete RSA-MD5 compatibility alias.
    pub fn rmd5(_key: &[u8]) -> Result<Arc<dyn JWTSigner>, JWTException> {
        Self::reject_legacy("RMD5")
    }
    /// Rejects Hutool's obsolete RSA-SHA1 compatibility alias.
    pub fn rsha1(_key: &[u8]) -> Result<Arc<dyn JWTSigner>, JWTException> {
        Self::reject_legacy("RSHA1")
    }
    /// Rejects Hutool's non-JOSE raw DSA compatibility alias.
    pub fn dnone(_key: &[u8]) -> Result<Arc<dyn JWTSigner>, JWTException> {
        Self::reject_legacy("DNONE")
    }
    /// Rejects Hutool's non-JOSE DSA-SHA1 compatibility alias.
    pub fn dsha1(_key: &[u8]) -> Result<Arc<dyn JWTSigner>, JWTException> {
        Self::reject_legacy("DSHA1")
    }
    /// Rejects Hutool's non-JOSE raw ECDSA compatibility alias.
    pub fn enone(_key: &[u8]) -> Result<Arc<dyn JWTSigner>, JWTException> {
        Self::reject_legacy("ENONE")
    }
    /// Rejects Hutool's non-JOSE ECDSA-SHA1 compatibility alias.
    pub fn esha1(_key: &[u8]) -> Result<Arc<dyn JWTSigner>, JWTException> {
        Self::reject_legacy("ESHA1")
    }
    /// Creates a secure HMAC signer by name; legacy algorithms are rejected.
    pub fn create_signer(algorithm: &str, key: &[u8]) -> Result<HMacJWTSigner, JWTException> {
        HMacJWTSigner::new(AlgorithmUtil::get_algorithm(algorithm)?, key)
    }

    /// Creates an RSA or ECDSA signer by JOSE algorithm name and PEM key pair.
    pub fn create_signer_from_pem(
        algorithm: &str,
        private_key: &[u8],
        public_key: &[u8],
    ) -> Result<Arc<dyn JWTSigner>, JWTException> {
        match AlgorithmUtil::get_algorithm(algorithm)? {
            Algorithm::RS256 => Ok(Arc::new(Self::rs256(private_key, public_key)?)),
            Algorithm::RS384 => Ok(Arc::new(Self::rs384(private_key, public_key)?)),
            Algorithm::RS512 => Ok(Arc::new(Self::rs512(private_key, public_key)?)),
            Algorithm::PS256 => Ok(Arc::new(Self::ps256(private_key, public_key)?)),
            Algorithm::PS384 => Ok(Arc::new(Self::ps384(private_key, public_key)?)),
            Algorithm::PS512 => Ok(Arc::new(Self::ps512(private_key, public_key)?)),
            Algorithm::ES256 => Ok(Arc::new(Self::es256(private_key, public_key)?)),
            Algorithm::ES384 => Ok(Arc::new(Self::es384(private_key, public_key)?)),
            _ => Err(JWTException::new(
                "PEM key pairs require an RSA, RSA-PSS, or ECDSA JWT algorithm",
            )),
        }
    }

    fn reject_legacy<T>(algorithm: &str) -> Result<T, JWTException> {
        Err(JWTException::formatted(
            "algorithm {} is intentionally unavailable: it is obsolete or not a JOSE algorithm",
            &[&algorithm],
        ))
    }
}

/// Dynamic JWT builder/parser.
#[derive(Clone)]
pub struct JWT {