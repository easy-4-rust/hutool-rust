//! SM2 helpers aligned with Hutool `SM2Test` / `BCUtilTest`.

use crate::CryptoError;
use sm2::dsa::signature::{Signer, Verifier};
use sm2::dsa::{Signature, SigningKey, VerifyingKey};
use sm2::{PublicKey, Scalar, SecretKey};

/// Opaque SM2 private parameters (Hutool `ECPrivateKeyParameters` stand-in).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sm2PrivateParams {
    /// Private scalar valid.
    pub valid: bool,
}

use super::{DEFAULT_DISTID, SM2_OID_HEX};
