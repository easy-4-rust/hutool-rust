//! Endian-aware numeric byte conversion aligned with Hutool's `ByteUtil` family.

/// 对齐: `cn.hutool.core.util.ByteUtil.ByteOrder`
/// 字节序

use std::{
    str::FromStr,
    sync::atomic::{AtomicI32, AtomicI64, Ordering},
};

use num_bigint::BigInt;
use parking_lot::Mutex;
use rust_decimal::Decimal;

/// Byte order used by numeric conversions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ByteOrder {
    /// Least-significant byte first.
    LittleEndian,
    /// Most-significant byte first.
    BigEndian,
}

use super::{java_f32_bits, java_f64_bits, read_array};
