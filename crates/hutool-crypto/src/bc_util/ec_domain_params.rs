//! BouncyCastle-shaped helpers aligned with Hutool `BCUtil`.
//!
//! Java `ECDomainParameters` / `EC*KeyParameters` collapse to named-curve tags and
//! opaque SM2/P-256 byte params; no BouncyCastle provider is linked.

/// Named EC curve stand-in for Hutool `ECDomainParameters` (`BCUtil.toDomainParams(String)`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcDomainParams {
    /// NIST P-256 / `secp256r1`.
    P256,
    /// SM2 curve (`sm2p256v1`).
    Sm2,
}
