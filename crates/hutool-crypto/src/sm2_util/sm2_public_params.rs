//! SM2 helpers aligned with Hutool `SM2Test` / `BCUtilTest`.

use crate::CryptoError;
use sm2::dsa::signature::{Signer, Verifier};
use sm2::dsa::{Signature, SigningKey, VerifyingKey};
use sm2::{PublicKey, Scalar, SecretKey};

/// Opaque SM2 public parameters (Hutool `ECPublicKeyParameters` stand-in).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sm2PublicParams {
    /// X coordinate valid.
    pub x_valid: bool,
    /// Y coordinate valid.
    pub y_valid: bool,
}

use super::{DEFAULT_DISTID, SM2_OID_HEX};
