//! BouncyCastle-shaped helpers aligned with Hutool `BCUtil`.
//!
//! Java `ECDomainParameters` / `EC*KeyParameters` collapse to named-curve tags and
//! opaque SM2/P-256 byte params; no BouncyCastle provider is linked.

use crate::{
    read_pem_private_key, read_pem_public_key, sm2_private_from_hex, sm2_public_from_xy, CryptoError,
    Sm2PrivateParams, Sm2PublicParams,
};
use p256::elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint};
use p256::{EncodedPoint, PublicKey, SecretKey};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};

use super::ec_domain_params::EcDomainParams;
use super::ec_private_params::EcPrivateParams;
use super::ec_public_params::EcPublicParams;

/// Zero-sized facade for Hutool `BCUtil` static methods.
#[derive(Debug, Clone, Copy, Default)]
pub struct BcUtil;

impl BcUtil {
    /// Encodes EC private scalar `d` (Hutool `BCUtil.encodeECPrivateKey`).
    #[must_use]
    pub fn encode_ec_private_key(secret: &SecretKey) -> Vec<u8> {
        secret.to_bytes().to_vec()
    }

    /// Decodes EC private key for P-256 (Hutool `BCUtil.decodeECPrivateKey`).
    pub fn decode_ec_private_key(d: &[u8], curve_name: &str) -> Result<SecretKey, CryptoError> {
        let domain = parse_curve_name(curve_name)?;
        if domain != EcDomainParams::P256 {
            return Err(CryptoError::InvalidPem);
        }
        let mut fb = p256::FieldBytes::default();
        if d.len() > 32 {
            return Err(CryptoError::InvalidPem);
        }
        fb[32 - d.len()..].copy_from_slice(d);
        SecretKey::from_bytes(&fb).map_err(|_| CryptoError::InvalidPem)
    }

    /// Encodes compressed EC public key Q (Hutool `BCUtil.encodeECPublicKey`).
    #[must_use]
    pub fn encode_ec_public_key(public: &PublicKey) -> Vec<u8> {
        Self::encode_ec_public_key_ex(public, true)
    }

    /// Encodes EC public key Q with compression flag.
    #[must_use]
    pub fn encode_ec_public_key_ex(public: &PublicKey, compressed: bool) -> Vec<u8> {
        public.as_affine().to_encoded_point(compressed).as_bytes().to_vec()
    }

    /// Decodes SEC1 EC point from hex/base64 text (Hutool `BCUtil.decodeECPoint(String,…)`).
    pub fn decode_ec_point_str(encode: &str, curve_name: &str) -> Result<PublicKey, CryptoError> {
        let bytes = crate::asymmetric::decode(encode).map_err(|_| CryptoError::InvalidEncoding)?;
        Self::decode_ec_point(&bytes, curve_name)
    }

    /// Decodes SEC1 EC point bytes (Hutool `BCUtil.decodeECPoint(byte[],…)`).
    pub fn decode_ec_point(encode: &[u8], curve_name: &str) -> Result<PublicKey, CryptoError> {
        let domain = parse_curve_name(curve_name)?;
        if domain != EcDomainParams::P256 {
            return Err(CryptoError::InvalidPem);
        }
        let point = EncodedPoint::from_bytes(encode).map_err(|_| CryptoError::InvalidPem)?;
        Option::<PublicKey>::from(PublicKey::from_encoded_point(&point))
            .ok_or(CryptoError::InvalidPem)
    }

    /// Builds domain params from curve name (Hutool `BCUtil.toDomainParams(String)`).
    pub fn to_domain_params(curve_name: &str) -> Result<EcDomainParams, CryptoError> {
        parse_curve_name(curve_name)
    }

    /// SM2 private params from hex `d` (Hutool `BCUtil.toSm2Params(String)`).
    pub fn to_sm2_params_d(d_hex: &str) -> Result<Sm2PrivateParams, CryptoError> {
        sm2_private_from_hex(d_hex)
    }

    /// SM2 private params from raw `d` bytes.
    pub fn to_sm2_params_d_bytes(d: &[u8]) -> Result<Sm2PrivateParams, CryptoError> {
        sm2_private_from_hex(&hex::encode(d))
    }

    /// SM2 public params from hex X/Y (Hutool `BCUtil.toSm2Params(xHex, yHex)`).
    pub fn to_sm2_params_xy(x_hex: &str, y_hex: &str) -> Result<Sm2PublicParams, CryptoError> {
        sm2_public_from_xy(x_hex, y_hex)
    }

    /// SM2 public params from raw X/Y bytes.
    pub fn to_sm2_params_xy_bytes(x: &[u8], y: &[u8]) -> Result<Sm2PublicParams, CryptoError> {
        sm2_public_from_xy(&hex::encode(x), &hex::encode(y))
    }

    /// Private params from hex `d` and domain (Hutool `BCUtil.toParams(dHex, domain)`).
    pub fn to_private_params_hex(
        d_hex: &str,
        domain: EcDomainParams,
    ) -> Result<EcPrivateParams, CryptoError> {
        let d = hex::decode(d_hex).map_err(|_| CryptoError::InvalidEncoding)?;
        Self::to_private_params(&d, domain)
    }

    /// Private params from raw `d` and domain.
    pub fn to_private_params(d: &[u8], domain: EcDomainParams) -> Result<EcPrivateParams, CryptoError> {
        if d.is_empty() || d.len() > 32 {
            return Err(CryptoError::InvalidPem);
        }
        Ok(EcPrivateParams {
            domain,
            d: d.to_vec(),
        })
    }

    /// Public params from hex X/Y and domain.
    pub fn to_public_params_hex(
        x_hex: &str,
        y_hex: &str,
        domain: EcDomainParams,
    ) -> Result<EcPublicParams, CryptoError> {
        let x = hex::decode(x_hex).map_err(|_| CryptoError::InvalidEncoding)?;
        let y = hex::decode(y_hex).map_err(|_| CryptoError::InvalidEncoding)?;
        Self::to_public_params(&x, &y, domain)
    }

    /// Public params from raw X/Y and domain.
    pub fn to_public_params(
        x: &[u8],
        y: &[u8],
        domain: EcDomainParams,
    ) -> Result<EcPublicParams, CryptoError> {
        if x.len() != 32 || y.len() != 32 {
            return Err(CryptoError::InvalidPem);
        }
        let mut q = Vec::with_capacity(65);
        q.push(0x04);
        q.extend_from_slice(x);
        q.extend_from_slice(y);
        Ok(EcPublicParams { domain, q })
    }

    /// Reads RSA private key PEM (Hutool `BCUtil.readPemPrivateKey`).
    pub fn read_pem_private_key(pem: &str) -> Result<RsaPrivateKey, CryptoError> {
        read_pem_private_key(pem)
    }

    /// Reads RSA public key PEM (Hutool `BCUtil.readPemPublicKey`).
    pub fn read_pem_public_key(pem: &str) -> Result<RsaPublicKey, CryptoError> {
        read_pem_public_key(pem)
    }

    /// PKCS#8 RSA private → PKCS#1 DER (Hutool `BCUtil.toPkcs1(PrivateKey)`).
    pub fn to_pkcs1_private(private_key: &RsaPrivateKey) -> Result<Vec<u8>, CryptoError> {
        private_key
            .to_pkcs1_der()
            .map(|doc| doc.as_bytes().to_vec())
            .map_err(|_| CryptoError::RsaKey)
    }

    /// SPKI RSA public → PKCS#1 DER (Hutool `BCUtil.toPkcs1(PublicKey)`).
    pub fn to_pkcs1_public(public_key: &RsaPublicKey) -> Result<Vec<u8>, CryptoError> {
        public_key
            .to_pkcs1_der()
            .map(|doc| doc.as_bytes().to_vec())
            .map_err(|_| CryptoError::RsaKey)
    }
}

fn parse_curve_name(name: &str) -> Result<EcDomainParams, CryptoError> {
    let n = name.trim().to_ascii_lowercase();
    if matches!(
        n.as_str(),
        "secp256r1" | "p-256" | "p256" | "prime256v1" | "ecdsa" | "ec"
    ) {
        Ok(EcDomainParams::P256)
    } else if matches!(n.as_str(), "sm2" | "sm2p256v1") {
        Ok(EcDomainParams::Sm2)
    } else {
        Err(CryptoError::InvalidPem)
    }
}
