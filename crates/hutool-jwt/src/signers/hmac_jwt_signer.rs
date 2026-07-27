//! 对齐: cn.hutool.jwt.signers.HmacJwtSigner
//! 来源: hutool-jwt/src/main/java/cn/hutool/jwt/signers/HmacJwtSigner.java
//!
//! 中文说明: Hutool JWT HmacJwtSigner 类型的 Rust 实现。

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