use thiserror::Error;

/// Errors returned by checked pagination calculations.

/// 对齐: `cn.hutool.core.util.PageUtil`
/// 分页错误
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PageError {
    /// A computed page count does not fit Hutool's signed 32-bit return type.
    #[error("computed page count does not fit i32")]
    PageCountOverflow,
    /// Rainbow pagination cannot allocate a negative number of entries.
    #[error("total pages and display count must be non-negative")]
    NegativeRainbowSize,
}
