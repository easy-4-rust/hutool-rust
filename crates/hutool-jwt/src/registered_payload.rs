//! 对齐: cn.hutool.jwt.RegisteredPayload
//! 来源: hutool-jwt/src/main/java/cn/hutool/jwt/cn.hutool.jwt/RegisteredPayload.java
//!
//! 中文说明: Hutool JWT RegisteredPayload 类型的 Rust 实现。

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


//! hutool-jwt 兼容层模块入口。
//!
//! 该模块继续承载 Hutool `cn.hutool.jwt` 兼容实现，已开始按 Java 对象逐步拆分文件。

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

mod claims;
mod jwt_exception;
mod jwt_header;
mod jwt_payload;
mod jwt_util;
mod jwt_validator;

pub use claims::Claims;
pub use jwt_exception::JWTException;
pub use jwt_header::JWTHeader;
pub use jwt_payload::JWTPayload;
pub use jwt_util::JWTUtil;
pub use jwt_validator::JWTValidator;

/// Decodes a JWT header/payload segment (URL-safe or standard Base64, optional padding).
fn decode_jwt_part(part: &str) -> Result<Vec<u8>, JWTException> {
    let pad = |input: &str| -> String {
        let rem = input.len() % 4;
        if rem == 0 {
            input.to_owned()
        } else {
            format!("{input}{}", "=".repeat(4 - rem))
        }
    };
    let padded = pad(part);
    if let Ok(bytes) = URL_SAFE.decode(padded.as_bytes()) {
        return Ok(bytes);
    }
    if let Ok(bytes) = URL_SAFE_NO_PAD.decode(part.as_bytes()) {
        return Ok(bytes);
    }
    let standard = padded.replace('-', "+").replace('_', "/");
    STANDARD
        .decode(standard.as_bytes())
        .map_err(JWTException::from)
}

/// Fluent registered-claim setters.

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