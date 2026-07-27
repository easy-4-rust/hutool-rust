//! 对齐: cn.hutool.jwt.signers.NoneJwtSigner
//! 来源: hutool-jwt/src/main/java/cn/hutool/jwt/signers/NoneJwtSigner.java
//!
//! 中文说明: Hutool JWT NoneJwtSigner 类型的 Rust 实现。

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