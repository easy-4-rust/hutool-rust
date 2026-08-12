//! 对齐: cn.hutool.jwt.signers.AsymmetricJwtSigner
//! 来源: hutool-jwt/src/main/java/cn/hutool/jwt/signers/AsymmetricJwtSigner.java
//!
//! 中文说明: Hutool JWT AsymmetricJwtSigner 类型的 Rust 实现。

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