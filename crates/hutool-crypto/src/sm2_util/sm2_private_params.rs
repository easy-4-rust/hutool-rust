//! SM2 helpers aligned with Hutool `SM2Test` / `BCUtilTest`.

/// Opaque SM2 private parameters (Hutool `ECPrivateKeyParameters` stand-in).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sm2PrivateParams {
    /// Private scalar valid.
    pub valid: bool,
}
