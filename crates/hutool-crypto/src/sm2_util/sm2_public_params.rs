//! SM2 helpers aligned with Hutool `SM2Test` / `BCUtilTest`.

/// Opaque SM2 public parameters (Hutool `ECPublicKeyParameters` stand-in).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sm2PublicParams {
    /// X coordinate valid.
    pub x_valid: bool,
    /// Y coordinate valid.
    pub y_valid: bool,
}
