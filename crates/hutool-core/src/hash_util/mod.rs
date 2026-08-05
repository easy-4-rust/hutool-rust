//! Classic non-cryptographic hashes aligned with Hutool's UTF-16 and wrapping rules.

#![allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

/// 对齐: `cn.hutool.core.util.HashUtil`
/// 哈希工具类
mod hash_error;
mod hash_util;

pub use hash_error::HashError;
pub use hash_util::HashUtil;
