//! Key/parameter spec helpers aligned with Hutool `SpecUtil`.

/// PBE password stand-in for Java `PBEKeySpec`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PbeKeySpec {
    /// Password characters as UTF-8 bytes.
    pub password: Vec<u8>,
}
