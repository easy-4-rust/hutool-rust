//! 对齐: cn.hutool.jwt.signers.AlgorithmUtil
//! 来源: hutool-jwt/src/main/java/cn/hutool/jwt/signers/AlgorithmUtil.java
//!
//! 中文说明: Hutool JWT AlgorithmUtil 类型的 Rust 实现。

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