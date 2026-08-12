//! Key/parameter spec helpers aligned with Hutool `SpecUtil`.

/// PBE salt + iteration stand-in for Java `PBEParameterSpec`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PbeParameterSpec {
    /// Salt bytes.
    pub salt: Vec<u8>,
    /// Iteration count.
    pub iteration_count: u32,
}
