//! 对齐: `cn.hutool.jwt.signers` 子包（集中实现）
//! 来源: hutool-jwt/src/main/java/cn/hutool/jwt/signers/
//!
//! 中文说明: Hutool JWT 签名器子包，对齐 Java `cn.hutool.jwt.signers.*` 全部 7 个类型
//! （RegisteredPayload trait + JWTSigner trait + HMacJWTSigner / AsymmetricJWTSigner /
//! EllipticCurveJWTSigner / NoneJWTSigner / AlgorithmUtil / JWTSignerUtil）的 Rust 实现。
//!
//! 实现策略：单文件 `all.rs` 集中实现（Rust trait `JWTSigner` 是公开 trait，
//! 其余具体结构体在 impl 时互引用。本文件保持 single source 是为了避免 trait 方法体内
//! 私有 helper 跨文件访问造成 `pub(crate)` 泄漏）。

// 共享 imports (与 crates/hutool-jwt/src/compat/mod.rs 头部相同)
use std::fmt;
use std::sync::Arc;

use jsonwebtoken::crypto;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use p256::elliptic_curve::sec1::ToEncodedPoint as _;
use rsa::pkcs1::{
    DecodeRsaPrivateKey as _, DecodeRsaPublicKey as _, EncodeRsaPrivateKey as _,
    EncodeRsaPublicKey as _,
};
use rsa::pkcs8::{DecodePrivateKey as _, DecodePublicKey as _, EncodePrivateKey as _};
use serde_json::Value;

// 引用 compat 顶层模块的其他类型
use crate::compat::jwt_exception::JWTException;
use crate::compat::jwt_payload::JWTPayload;

/// 注册声明（iss/sub/aud/exp/nbf/iat/jti）的便捷 setter。
///
/// 对齐 Java: `cn.hutool.jwt.signers.RegisteredPayload`
pub trait RegisteredPayload {
    /// Sets a registered payload value.
    fn set_registered(&mut self, name: &'static str, value: Value) -> &mut Self;

    /// Sets issuer.
    fn set_issuer(&mut self, value: impl Into<String>) -> &mut Self {
        self.set_registered(JWTPayload::ISSUER, Value::String(value.into()))
    }
    /// Sets subject.
    fn set_subject(&mut self, value: impl Into<String>) -> &mut Self {
        self.set_registered(JWTPayload::SUBJECT, Value::String(value.into()))
    }
    /// Sets audience.
    fn set_audience(&mut self, value: impl Into<String>) -> &mut Self {
        self.set_registered(JWTPayload::AUDIENCE, Value::String(value.into()))
    }
    /// Sets expiration epoch seconds.
    fn set_expires_at(&mut self, value: u64) -> &mut Self {
        self.set_registered(JWTPayload::EXPIRES_AT, Value::from(value))
    }
    /// Sets not-before epoch seconds.
    fn set_not_before(&mut self, value: u64) -> &mut Self {
        self.set_registered(JWTPayload::NOT_BEFORE, Value::from(value))
    }
    /// Sets issued-at epoch seconds.
    fn set_issued_at(&mut self, value: u64) -> &mut Self {
        self.set_registered(JWTPayload::ISSUED_AT, Value::from(value))
    }
    /// Sets JWT ID.
    fn set_jwt_id(&mut self, value: impl Into<String>) -> &mut Self {
        self.set_registered(JWTPayload::JWT_ID, Value::String(value.into()))
    }
}

impl RegisteredPayload for JWTPayload {
    fn set_registered(&mut self, name: &'static str, value: Value) -> &mut Self {
        self.set_payload(name, value)
    }
}

/// Pluggable JWS signer/verifier.
pub trait JWTSigner: Send + Sync {
    /// Signs encoded header and payload components.
    fn sign(&self, header: &str, payload: &str) -> Result<String, JWTException>;
    /// Verifies a JWS signature.
    fn verify(&self, header: &str, payload: &str, signature: &str) -> Result<bool, JWTException>;
    /// Returns the standard algorithm ID.
    fn algorithm_id(&self) -> &'static str;
}

pub(crate) fn signing_input(header: &str, payload: &str) -> Vec<u8> {
    format!("{header}.{payload}").into_bytes()
}

pub(crate) fn signing_result(
    result: Result<String, jsonwebtoken::errors::Error>,
) -> Result<String, JWTException> {
    result.map_err(Into::into)
}

pub(crate) fn verification_result(
    result: Result<bool, jsonwebtoken::errors::Error>,
) -> Result<bool, JWTException> {
    result.map_err(Into::into)
}

pub(crate) fn key_error(error: impl fmt::Display) -> JWTException {
    JWTException::formatted("invalid PEM key: {}", &[&error])
}

pub(crate) fn pem_text(pem: &[u8]) -> Result<&str, JWTException> {
    std::str::from_utf8(pem).map_err(key_error)
}

pub(crate) fn rsa_private_der(pem: &[u8]) -> Result<Vec<u8>, JWTException> {
    let pem = pem_text(pem)?;
    let key = if pem.contains("BEGIN RSA PRIVATE KEY") {
        rsa::RsaPrivateKey::from_pkcs1_pem(pem).map_err(key_error)?
    } else {
        rsa::RsaPrivateKey::from_pkcs8_pem(pem).map_err(key_error)?
    };
    Ok(key
        .to_pkcs1_der()
        .expect("validated RSA private keys always encode as PKCS#1")
        .as_bytes()
        .to_vec())
}

pub(crate) fn rsa_public_der(pem: &[u8]) -> Result<Vec<u8>, JWTException> {
    let pem = pem_text(pem)?;
    let key = if pem.contains("BEGIN RSA PUBLIC KEY") {
        rsa::RsaPublicKey::from_pkcs1_pem(pem).map_err(key_error)?
    } else {
        rsa::RsaPublicKey::from_public_key_pem(pem).map_err(key_error)?
    };
    Ok(key
        .to_pkcs1_der()
        .expect("validated RSA public keys always encode as PKCS#1")
        .as_bytes()
        .to_vec())
}

pub(crate) fn ec256_keys_from_pem(
    private_key: &[u8],
    public_key: &[u8],
) -> Result<(EncodingKey, DecodingKey), JWTException> {
    let private_key = pem_text(private_key)?;
    let public_key = pem_text(public_key)?;
    let private = if private_key.contains("BEGIN EC PRIVATE KEY") {
        p256::SecretKey::from_sec1_pem(private_key).map_err(key_error)?
    } else {
        p256::SecretKey::from_pkcs8_pem(private_key).map_err(key_error)?
    };
    let private = private
        .to_pkcs8_der()
        .expect("validated P-256 private keys always encode as PKCS#8");
    let public = p256::PublicKey::from_public_key_pem(public_key).map_err(key_error)?;
    let public = public.to_encoded_point(false);
    Ok((
        EncodingKey::from_ec_der(private.as_bytes()),
        DecodingKey::from_ec_der(public.as_bytes()),
    ))
}

pub(crate) fn ec384_keys_from_pem(
    private_key: &[u8],
    public_key: &[u8],
) -> Result<(EncodingKey, DecodingKey), JWTException> {
    let private_key = pem_text(private_key)?;
    let public_key = pem_text(public_key)?;
    let private = if private_key.contains("BEGIN EC PRIVATE KEY") {
        p384::SecretKey::from_sec1_pem(private_key).map_err(key_error)?
    } else {
        p384::SecretKey::from_pkcs8_pem(private_key).map_err(key_error)?
    };
    let private = private
        .to_pkcs8_der()
        .expect("validated P-384 private keys always encode as PKCS#8");
    let public = p384::PublicKey::from_public_key_pem(public_key).map_err(key_error)?;
    let public = public.to_encoded_point(false);
    Ok((
        EncodingKey::from_ec_der(private.as_bytes()),
        DecodingKey::from_ec_der(public.as_bytes()),
    ))
}

/// HMAC JWT signer supporting HS256/384/512.
#[derive(Clone)]
pub struct HMacJWTSigner {
    algorithm: Algorithm,
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl HMacJWTSigner {
    /// Creates a validated HMAC signer.
    pub fn new(algorithm: Algorithm, key: &[u8]) -> Result<Self, JWTException> {
        if !matches!(
            algorithm,
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512
        ) {
            return Err(JWTException::new("algorithm is not an HMAC JWT algorithm"));
        }
        Ok(Self {
            algorithm,
            encoding: EncodingKey::from_secret(key),
            decoding: DecodingKey::from_secret(key),
        })
    }
}

impl JWTSigner for HMacJWTSigner {
    fn sign(&self, header: &str, payload: &str) -> Result<String, JWTException> {
        signing_result(crypto::sign(
            &signing_input(header, payload),
            &self.encoding,
            self.algorithm,
        ))
    }

    fn verify(&self, header: &str, payload: &str, signature: &str) -> Result<bool, JWTException> {
        verification_result(crypto::verify(
            signature,
            &signing_input(header, payload),
            &self.decoding,
            self.algorithm,
        ))
    }

    fn algorithm_id(&self) -> &'static str {
        AlgorithmUtil::get_id(self.algorithm)
    }
}

/// RSA signer supporting RS256/384/512 with separate private/public PEM keys.
#[derive(Clone)]
pub struct AsymmetricJWTSigner {
    algorithm: Algorithm,
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl AsymmetricJWTSigner {
    /// Creates an RSA signer from PEM keys.
    pub fn from_rsa_pem(
        algorithm: Algorithm,
        private_key: &[u8],
        public_key: &[u8],
    ) -> Result<Self, JWTException> {
        if !matches!(
            algorithm,
            Algorithm::RS256
                | Algorithm::RS384
                | Algorithm::RS512
                | Algorithm::PS256
                | Algorithm::PS384
                | Algorithm::PS512
        ) {
            return Err(JWTException::new(
                "algorithm is not an RSA or RSA-PSS JWT algorithm",
            ));
        }
        let private_key = rsa_private_der(private_key)?;
        let public_key = rsa_public_der(public_key)?;
        Ok(Self {
            algorithm,
            encoding: EncodingKey::from_rsa_der(&private_key),
            decoding: DecodingKey::from_rsa_der(&public_key),
        })
    }
}

impl JWTSigner for AsymmetricJWTSigner {
    fn sign(&self, header: &str, payload: &str) -> Result<String, JWTException> {
        signing_result(crypto::sign(
            &signing_input(header, payload),
            &self.encoding,
            self.algorithm,
        ))
    }

    fn verify(&self, header: &str, payload: &str, signature: &str) -> Result<bool, JWTException> {
        verification_result(crypto::verify(
            signature,
            &signing_input(header, payload),
            &self.decoding,
            self.algorithm,
        ))
    }

    fn algorithm_id(&self) -> &'static str {
        AlgorithmUtil::get_id(self.algorithm)
    }
}

/// ECDSA signer supporting ES256/384 from separate PEM keys.
#[derive(Clone)]
pub struct EllipticCurveJWTSigner(AsymmetricJWTSigner);

impl EllipticCurveJWTSigner {
    /// Creates an ECDSA signer from PEM keys.
    pub fn from_pem(
        algorithm: Algorithm,
        private_key: &[u8],
        public_key: &[u8],
    ) -> Result<Self, JWTException> {
        let (encoding, decoding) = match algorithm {
            Algorithm::ES256 => ec256_keys_from_pem(private_key, public_key)?,
            Algorithm::ES384 => ec384_keys_from_pem(private_key, public_key)?,
            _ => {
                return Err(JWTException::new(
                    "algorithm is not a supported ECDSA JWT algorithm",
                ));
            }
        };
        Ok(Self(AsymmetricJWTSigner {
            algorithm,
            encoding,
            decoding,
        }))
    }
}

impl JWTSigner for EllipticCurveJWTSigner {
    fn sign(&self, header: &str, payload: &str) -> Result<String, JWTException> {
        self.0.sign(header, payload)
    }

    fn verify(&self, header: &str, payload: &str, signature: &str) -> Result<bool, JWTException> {
        self.0.verify(header, payload, signature)
    }

    fn algorithm_id(&self) -> &'static str {
        self.0.algorithm_id()
    }
}

/// Unsecured `alg=none` signer, available only for explicit compatibility.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoneJWTSigner;

impl NoneJWTSigner {
    /// Returns whether an algorithm denotes no signature.
    #[must_use]
    pub fn is_none(value: Option<&str>) -> bool {
        value.is_none_or(|value| {
            let value = value.trim();
            value.is_empty() || value.eq_ignore_ascii_case("none")
        })
    }
}

impl JWTSigner for NoneJWTSigner {
    fn sign(&self, _header: &str, _payload: &str) -> Result<String, JWTException> {
        Ok(String::new())
    }

    fn verify(&self, _header: &str, _payload: &str, signature: &str) -> Result<bool, JWTException> {
        Ok(signature.is_empty())
    }

    fn algorithm_id(&self) -> &'static str {
        "none"
    }
}

/// Algorithm name conversion.
pub struct AlgorithmUtil;

impl AlgorithmUtil {
    /// Parses standard and JCA-style names.
    pub fn get_algorithm(value: &str) -> Result<Algorithm, JWTException> {
        match value.to_ascii_uppercase().replace(['-', '_'], "").as_str() {
            "HS256" | "HMACSHA256" => Ok(Algorithm::HS256),
            "HS384" | "HMACSHA384" => Ok(Algorithm::HS384),
            "HS512" | "HMACSHA512" => Ok(Algorithm::HS512),
            "RS256" | "SHA256WITHRSA" => Ok(Algorithm::RS256),
            "RS384" | "SHA384WITHRSA" => Ok(Algorithm::RS384),
            "RS512" | "SHA512WITHRSA" => Ok(Algorithm::RS512),
            "ES256" | "SHA256WITHECDSA" => Ok(Algorithm::ES256),
            "ES384" | "SHA384WITHECDSA" => Ok(Algorithm::ES384),
            "PS256" | "SHA256WITHRSAANDMGF1" => Ok(Algorithm::PS256),
            "PS384" | "SHA384WITHRSAANDMGF1" => Ok(Algorithm::PS384),
            "PS512" | "SHA512WITHRSAANDMGF1" => Ok(Algorithm::PS512),
            _ => Err(JWTException::formatted(
                "unsupported JWT algorithm: {}",
                &[&value],
            )),
        }
    }

    /// Returns a JOSE algorithm ID.
    #[must_use]
    pub const fn get_id(algorithm: Algorithm) -> &'static str {
        match algorithm {
            Algorithm::HS256 => "HS256",
            Algorithm::HS384 => "HS384",
            Algorithm::HS512 => "HS512",
            Algorithm::RS256 => "RS256",
            Algorithm::RS384 => "RS384",
            Algorithm::RS512 => "RS512",
            Algorithm::PS256 => "PS256",
            Algorithm::PS384 => "PS384",
            Algorithm::PS512 => "PS512",
            Algorithm::ES256 => "ES256",
            Algorithm::ES384 => "ES384",
            Algorithm::EdDSA => "EdDSA",
        }
    }
}

/// Signer constructors aligned with Hutool names.
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
