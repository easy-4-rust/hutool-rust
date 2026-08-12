//! BouncyCastle-shaped helpers aligned with Hutool `BCUtil`.
//!
//! Java `ECDomainParameters` / `EC*KeyParameters` collapse to named-curve tags and
//! opaque SM2/P-256 byte params; no BouncyCastle provider is linked.

use super::ec_domain_params::EcDomainParams;

/// Opaque EC private scalar params (Hutool `ECPrivateKeyParameters`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EcPrivateParams {
    /// Curve tag.
    pub domain: EcDomainParams,
    /// Private scalar bytes (32 for P-256/SM2).
    pub d: Vec<u8>,
}
