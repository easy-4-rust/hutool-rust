use std::cmp::Ordering;

use thiserror::Error;

/// Errors returned by Hutool-compatible version expression matching.

/// 对齐: `cn.hutool.core.util.VersionUtil`
/// 版本错误
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum VersionError {
    /// The expression separator is blank, a range marker, or a comparison operator.
    #[error("invalid version delimiter: {0:?}")]
    InvalidDelimiter(String),
}

use super::{DEFAULT_DELIMITER, LooseVersion, Token, compare_nullable, compare_tokens, compare_versions, comparison_target, java_string_cmp};
use super::{parse_version, split_operator, take_number, take_text, validate_delimiter};
