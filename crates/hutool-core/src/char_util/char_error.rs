use std::any::{Any, TypeId};

use unicode_general_category::{GeneralCategory, get_general_category};

/// Errors returned by checked character conversions.

/// 对齐: `cn.hutool.core.util.CharUtil`
/// 字符工具错误
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum CharError {
    /// Enclosed decimal numbers are defined only for 1 through 20.
    #[error("number must be in the inclusive range 1..=20")]
    InvalidEnclosedNumber,
}

use super::{java_category, java_category_abbreviation, lower_char};
