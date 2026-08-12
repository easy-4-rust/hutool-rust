//! BouncyCastle-shaped helpers aligned with Hutool `BCUtil`.
//!
//! Java `ECDomainParameters` / `EC*KeyParameters` collapse to named-curve tags and
//! opaque SM2/P-256 byte params; no BouncyCastle provider is linked.

use super::ec_domain_params::EcDomainParams;

/// Opaque EC public point params (Hutool `ECPublicKeyParameters`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EcPublicParams {
    /// Curve tag.
    pub domain: EcDomainParams,
    /// Uncompressed SEC1 point bytes (`04 || X || Y`).
    pub q: Vec<u8>,
}
