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

/// Opaque EC public point params (Hutool `ECPublicKeyParameters`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EcPublicParams {
    /// Curve tag.
    pub domain: EcDomainParams,
    /// Uncompressed SEC1 point bytes (`04 || X || Y`).
    pub q: Vec<u8>,
}
