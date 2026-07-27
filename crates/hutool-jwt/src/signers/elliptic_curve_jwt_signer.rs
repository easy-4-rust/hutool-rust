//! 对齐: cn.hutool.jwt.signers.EllipticCurveJwtSigner
//! 来源: hutool-jwt/src/main/java/cn/hutool/jwt/signers/EllipticCurveJwtSigner.java
//!
//! 中文说明: Hutool JWT EllipticCurveJwtSigner 类型的 Rust 实现。

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