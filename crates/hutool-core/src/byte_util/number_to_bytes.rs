//! Endian-aware numeric byte conversion aligned with Hutool's `ByteUtil` family.

/// 对齐: `cn.hutool.core.util.ByteUtil`
/// 数字转字节



use super::byte_order::ByteOrder;

/// Rust-native input contract for Hutool's `numberToBytes` overloads.
pub trait NumberToBytes {
    /// Serializes this numeric value in `order`.
    fn number_to_bytes(self, order: ByteOrder) -> Vec<u8>;
}

