//! 对齐: cn.hutool.jwt.signers.JWTSigner
//! 来源: hutool-jwt/src/main/java/cn/hutool/jwt/jwt.signers/JWTSigner.java
//!
//! 中文说明: Hutool JWT JWTSigner 类型。

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

pub trait JWTSigner: Send + Sync {
    /// Signs encoded header and payload components.
    fn sign(&self, header: &str, payload: &str) -> Result<String, JWTException>;
    /// Verifies a JWS signature.
    fn verify(&self, header: &str, payload: &str, signature: &str) -> Result<bool, JWTException>;
    /// Returns the standard algorithm ID.
    fn algorithm_id(&self) -> &'static str;
}

fn signing_input(header: &str, payload: &str) -> Vec<u8> {
    format!("{header}.{payload}").into_bytes()
}

fn signing_result(
    result: Result<String, jsonwebtoken::errors::Error>,
) -> Result<String, JWTException> {
    result.map_err(Into::into)
}

fn verification_result(
    result: Result<bool, jsonwebtoken::errors::Error>,
) -> Result<bool, JWTException> {
    result.map_err(Into::into)
}

fn key_error(error: impl fmt::Display) -> JWTException {
    JWTException::formatted("invalid PEM key: {}", &[&error])
}

fn pem_text(pem: &[u8]) -> Result<&str, JWTException> {
    std::str::from_utf8(pem).map_err(key_error)
}

fn rsa_private_der(pem: &[u8]) -> Result<Vec<u8>, JWTException> {
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

fn rsa_public_der(pem: &[u8]) -> Result<Vec<u8>, JWTException> {
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

fn ec256_keys_from_pem(
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

fn ec384_keys_from_pem(
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