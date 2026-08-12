//! Key/parameter spec helpers aligned with Hutool `SpecUtil`.

/// Algorithm key material stand-in for Java `KeySpec` / `SecretKeySpec`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeySpecBytes {
    /// Algorithm name (DES / DESede / AES / …).
    pub algorithm: String,
    /// Raw key bytes.
    pub key: Vec<u8>,
}
