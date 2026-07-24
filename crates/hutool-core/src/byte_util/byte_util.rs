//! Endian-aware numeric byte conversion aligned with Hutool's `ByteUtil` family.

/// 对齐: `cn.hutool.core.util.ByteUtil`
/// 字节工具类

use std::{
    str::FromStr,
    sync::atomic::{AtomicI32, AtomicI64, Ordering},
};

use num_bigint::BigInt;
use parking_lot::Mutex;
use rust_decimal::Decimal;

use super::byte_order::ByteOrder;
use super::byte_util_error::ByteUtilError;
use super::bytes_to_number::BytesToNumber;
use super::double_adder::DoubleAdder;
use super::long_adder::LongAdder;
use super::number_to_bytes::NumberToBytes;

/// Zero-sized facade for endian-aware numeric byte conversion.
#[derive(Debug, Clone, Copy, Default)]
pub struct ByteUtil;

impl ByteUtil {
    /// Hutool's default conversion order.
    pub const DEFAULT_ORDER: ByteOrder = ByteOrder::LittleEndian;

    /// Native CPU byte order determined at compile time.
    #[cfg(target_endian = "little")]
    pub const CPU_ENDIAN: ByteOrder = ByteOrder::LittleEndian;

    /// Native CPU byte order determined at compile time.
    #[cfg(target_endian = "big")]
    pub const CPU_ENDIAN: ByteOrder = ByteOrder::BigEndian;

    /// Narrows an `i32` to the low eight bits with Java cast semantics.
    #[must_use]
    pub const fn int_to_byte(value: i32) -> i8 {
        i8::from_ne_bytes([value.to_le_bytes()[0]])
    }

    /// Reinterprets a signed byte as the unsigned integer range `0..=255`.
    #[must_use]
    pub fn byte_to_unsigned_int(value: i8) -> u32 {
        u32::from(value.to_ne_bytes()[0])
    }

    /// Reads a little-endian `i16` from offset zero.
    pub fn bytes_to_i16(bytes: &[u8]) -> Result<i16, ByteUtilError> {
        Self::bytes_to_i16_with_order(bytes, Self::DEFAULT_ORDER)
    }

    /// Reads an `i16` from offset zero in `order`.
    pub fn bytes_to_i16_with_order(bytes: &[u8], order: ByteOrder) -> Result<i16, ByteUtilError> {
        Self::bytes_to_i16_at(bytes, 0, order)
    }

    /// Reads an `i16` from `start` in `order`.
    pub fn bytes_to_i16_at(
        bytes: &[u8],
        start: usize,
        order: ByteOrder,
    ) -> Result<i16, ByteUtilError> {
        let bytes = read_array::<2>(bytes, start)?;
        Ok(match order {
            ByteOrder::LittleEndian => i16::from_le_bytes(bytes),
            ByteOrder::BigEndian => i16::from_be_bytes(bytes),
        })
    }

    /// Writes an `i16` in Hutool's default little-endian order.
    #[must_use]
    pub const fn i16_to_bytes(value: i16) -> [u8; 2] {
        Self::i16_to_bytes_with_order(value, Self::DEFAULT_ORDER)
    }

    /// Writes an `i16` in `order`.
    #[must_use]
    pub const fn i16_to_bytes_with_order(value: i16, order: ByteOrder) -> [u8; 2] {
        match order {
            ByteOrder::LittleEndian => value.to_le_bytes(),
            ByteOrder::BigEndian => value.to_be_bytes(),
        }
    }

    /// Reads a little-endian `i32` from offset zero.
    pub fn bytes_to_i32(bytes: &[u8]) -> Result<i32, ByteUtilError> {
        Self::bytes_to_i32_with_order(bytes, Self::DEFAULT_ORDER)
    }

    /// Reads an `i32` from offset zero in `order`.
    pub fn bytes_to_i32_with_order(bytes: &[u8], order: ByteOrder) -> Result<i32, ByteUtilError> {
        Self::bytes_to_i32_at(bytes, 0, order)
    }

    /// Reads an `i32` from `start` in `order`.
    pub fn bytes_to_i32_at(
        bytes: &[u8],
        start: usize,
        order: ByteOrder,
    ) -> Result<i32, ByteUtilError> {
        let bytes = read_array::<4>(bytes, start)?;
        Ok(match order {
            ByteOrder::LittleEndian => i32::from_le_bytes(bytes),
            ByteOrder::BigEndian => i32::from_be_bytes(bytes),
        })
    }

    /// Writes an `i32` in Hutool's default little-endian order.
    #[must_use]
    pub const fn i32_to_bytes(value: i32) -> [u8; 4] {
        Self::i32_to_bytes_with_order(value, Self::DEFAULT_ORDER)
    }

    /// Writes an `i32` in `order`.
    #[must_use]
    pub const fn i32_to_bytes_with_order(value: i32, order: ByteOrder) -> [u8; 4] {
        match order {
            ByteOrder::LittleEndian => value.to_le_bytes(),
            ByteOrder::BigEndian => value.to_be_bytes(),
        }
    }

    /// Reads a little-endian `i64` from offset zero.
    pub fn bytes_to_i64(bytes: &[u8]) -> Result<i64, ByteUtilError> {
        Self::bytes_to_i64_with_order(bytes, Self::DEFAULT_ORDER)
    }

    /// Reads an `i64` from offset zero in `order`.
    pub fn bytes_to_i64_with_order(bytes: &[u8], order: ByteOrder) -> Result<i64, ByteUtilError> {
        Self::bytes_to_i64_at(bytes, 0, order)
    }

    /// Reads an `i64` from `start` in `order`.
    pub fn bytes_to_i64_at(
        bytes: &[u8],
        start: usize,
        order: ByteOrder,
    ) -> Result<i64, ByteUtilError> {
        let bytes = read_array::<8>(bytes, start)?;
        Ok(match order {
            ByteOrder::LittleEndian => i64::from_le_bytes(bytes),
            ByteOrder::BigEndian => i64::from_be_bytes(bytes),
        })
    }

    /// Writes an `i64` in Hutool's default little-endian order.
    #[must_use]
    pub const fn i64_to_bytes(value: i64) -> [u8; 8] {
        Self::i64_to_bytes_with_order(value, Self::DEFAULT_ORDER)
    }

    /// Writes an `i64` in `order`.
    #[must_use]
    pub const fn i64_to_bytes_with_order(value: i64, order: ByteOrder) -> [u8; 8] {
        match order {
            ByteOrder::LittleEndian => value.to_le_bytes(),
            ByteOrder::BigEndian => value.to_be_bytes(),
        }
    }

    /// Writes an `f32` in Hutool's default little-endian order.
    #[must_use]
    pub fn f32_to_bytes(value: f32) -> [u8; 4] {
        Self::f32_to_bytes_with_order(value, Self::DEFAULT_ORDER)
    }

    /// Writes Java-canonicalized `f32` bits in `order`.
    #[must_use]
    pub fn f32_to_bytes_with_order(value: f32, order: ByteOrder) -> [u8; 4] {
        let bits = java_f32_bits(value);
        match order {
            ByteOrder::LittleEndian => bits.to_le_bytes(),
            ByteOrder::BigEndian => bits.to_be_bytes(),
        }
    }

    /// Reads a little-endian `f32` from offset zero.
    pub fn bytes_to_f32(bytes: &[u8]) -> Result<f32, ByteUtilError> {
        Self::bytes_to_f32_with_order(bytes, Self::DEFAULT_ORDER)
    }

    /// Reads an `f32` from offset zero in `order`.
    pub fn bytes_to_f32_with_order(bytes: &[u8], order: ByteOrder) -> Result<f32, ByteUtilError> {
        let bits = read_array::<4>(bytes, 0)?;
        Ok(f32::from_bits(match order {
            ByteOrder::LittleEndian => u32::from_le_bytes(bits),
            ByteOrder::BigEndian => u32::from_be_bytes(bits),
        }))
    }

    /// Writes an `f64` in Hutool's default little-endian order.
    #[must_use]
    pub fn f64_to_bytes(value: f64) -> [u8; 8] {
        Self::f64_to_bytes_with_order(value, Self::DEFAULT_ORDER)
    }

    /// Writes Java-canonicalized `f64` bits in `order`.
    #[must_use]
    pub fn f64_to_bytes_with_order(value: f64, order: ByteOrder) -> [u8; 8] {
        let bits = java_f64_bits(value);
        match order {
            ByteOrder::LittleEndian => bits.to_le_bytes(),
            ByteOrder::BigEndian => bits.to_be_bytes(),
        }
    }

    /// Reads a little-endian `f64` from offset zero.
    pub fn bytes_to_f64(bytes: &[u8]) -> Result<f64, ByteUtilError> {
        Self::bytes_to_f64_with_order(bytes, Self::DEFAULT_ORDER)
    }

    /// Reads an `f64` from offset zero in `order`.
    pub fn bytes_to_f64_with_order(bytes: &[u8], order: ByteOrder) -> Result<f64, ByteUtilError> {
        let bits = read_array::<8>(bytes, 0)?;
        Ok(f64::from_bits(match order {
            ByteOrder::LittleEndian => u64::from_le_bytes(bits),
            ByteOrder::BigEndian => u64::from_be_bytes(bits),
        }))
    }

    /// Serializes a supported numeric type in the default little-endian order.
    #[must_use]
    pub fn number_to_bytes<T: NumberToBytes>(number: T) -> Vec<u8> {
        Self::number_to_bytes_with_order(number, Self::DEFAULT_ORDER)
    }

    /// Serializes a supported numeric type in `order`.
    #[must_use]
    pub fn number_to_bytes_with_order<T: NumberToBytes>(number: T, order: ByteOrder) -> Vec<u8> {
        number.number_to_bytes(order)
    }

    /// Converts bytes to a statically selected numeric target type.
    pub fn bytes_to_number<T: BytesToNumber>(
        bytes: &[u8],
        order: ByteOrder,
    ) -> Result<T, ByteUtilError> {
        T::bytes_to_number(bytes, order)
    }
}

impl NumberToBytes for i8 {
    fn number_to_bytes(self, _order: ByteOrder) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl BytesToNumber for i8 {
    fn bytes_to_number(bytes: &[u8], _order: ByteOrder) -> Result<Self, ByteUtilError> {
        Ok(i8::from_ne_bytes(read_array::<1>(bytes, 0)?))
    }
}

impl BytesToNumber for AtomicI32 {
    fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError> {
        Ok(Self::new(ByteUtil::bytes_to_i32_with_order(bytes, order)?))
    }
}

impl BytesToNumber for AtomicI64 {
    fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError> {
        Ok(Self::new(ByteUtil::bytes_to_i64_with_order(bytes, order)?))
    }
}

impl BytesToNumber for LongAdder {
    fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError> {
        Ok(Self::new(ByteUtil::bytes_to_i64_with_order(bytes, order)?))
    }
}

impl BytesToNumber for DoubleAdder {
    fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError> {
        Ok(Self::new(ByteUtil::bytes_to_f64_with_order(bytes, order)?))
    }
}

impl BytesToNumber for BigInt {
    fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError> {
        Ok(Self::from(ByteUtil::bytes_to_i64_with_order(bytes, order)?))
    }
}

impl BytesToNumber for Decimal {
    fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError> {
        let value = ByteUtil::bytes_to_f64_with_order(bytes, order)?;
        Self::from_str(&value.to_string()).map_err(|_| ByteUtilError::Decimal(value.to_string()))
    }
}

macro_rules! impl_number_to_bytes {
    ($type:ty, $method:ident) => {
        impl NumberToBytes for $type {
            fn number_to_bytes(self, order: ByteOrder) -> Vec<u8> {
                ByteUtil::$method(self, order).to_vec()
            }
        }
    };
}

macro_rules! impl_bytes_to_number {
    ($type:ty, $method:ident) => {
        impl BytesToNumber for $type {
            fn bytes_to_number(bytes: &[u8], order: ByteOrder) -> Result<Self, ByteUtilError> {
                ByteUtil::$method(bytes, order)
            }
        }
    };
}

use super::{java_f32_bits, java_f64_bits, read_array};
