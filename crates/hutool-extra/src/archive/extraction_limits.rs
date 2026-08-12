//! ZIP creation and bounded, path-safe extraction.
//!
//! 对齐: `cn.hutool.extra.compress.CompressUtil`
//! 对齐: `cn.hutool.core.util.ZipUtil`（extra 侧安全 ZIP 子集）

/// Limits applied before and during archive extraction.
#[derive(Debug, Clone, Copy)]
pub struct ExtractionLimits {
    /// Maximum entry count.
    pub max_entries: usize,
    /// Maximum sum of declared uncompressed bytes.
    pub max_uncompressed_bytes: u64,
}

impl Default for ExtractionLimits {
    fn default() -> Self {
        Self {
            max_entries: 10_000,
            max_uncompressed_bytes: 1_073_741_824,
        }
    }
}
