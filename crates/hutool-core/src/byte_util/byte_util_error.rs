//! Endian-aware numeric byte conversion aligned with Hutool's `ByteUtil` family.

/// 对齐: `cn.hutool.core.util.ByteUtil`
/// 字节工具错误

/// Errors produced by checked byte conversions.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum ByteUtilError {
    /// The requested fixed-width number did not fit in the remaining input.
    #[error("insufficient bytes at offset {start}: required {required}, available {available}")]
    InsufficientBytes {
        /// Requested starting offset.
        start: usize,
        /// Required number of bytes.
        required: usize,
        /// Bytes available after `start`.
        available: usize,
    },

    /// An IEEE-754 value could not be represented as a decimal number.
    #[error("floating-point value cannot be converted to Decimal: {0}")]
    Decimal(String),
}
