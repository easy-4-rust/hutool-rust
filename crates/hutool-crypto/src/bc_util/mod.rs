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

mod ec_domain_params;
mod ec_private_params;
mod ec_public_params;
mod bc_util;

pub use ec_domain_params::EcDomainParams;
pub use ec_private_params::EcPrivateParams;
pub use ec_public_params::EcPublicParams;
pub use bc_util::BcUtil;
