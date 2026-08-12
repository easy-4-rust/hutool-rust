//! BouncyCastle-shaped helpers aligned with Hutool `BCUtil`.
//!
//! Java `ECDomainParameters` / `EC*KeyParameters` collapse to named-curve tags and
//! opaque SM2/P-256 byte params; no BouncyCastle provider is linked.

mod bc_util;
mod ec_domain_params;
mod ec_private_params;
mod ec_public_params;

pub use bc_util::BcUtil;
pub use ec_domain_params::EcDomainParams;
pub use ec_private_params::EcPrivateParams;
pub use ec_public_params::EcPublicParams;
