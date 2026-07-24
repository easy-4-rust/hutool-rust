//! Endian-aware numeric byte conversion aligned with Hutool's `ByteUtil` family.

/// 对齐: `cn.hutool.core.util.ByteUtil`
/// 字节转数字

use std::{
    str::FromStr,
    sync::atomic::{AtomicI32, AtomicI64, Ordering},
};

use num_bigint::BigInt;
use parking_lot::Mutex;
use rust_decimal::Decimal;

use super::byte_order::ByteOrder;
use super::byte_util_error::ByteUtilError;

/// Rust-native target contract replacing Hutool's runtime `Class<T>` argument.
pub trait BytesToNumber: Sized {
    /// Reads this numeric type from `bytes` in `order`.
    fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError>;
}

use super::{java_f32_bits, java_f64_bits, read_array};
