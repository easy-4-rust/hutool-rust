//! Endian-aware numeric byte conversion aligned with Hutool's `ByteUtil` family.

mod byte_order;
mod byte_util;
mod byte_util_error;
mod bytes_to_number;
mod double_adder;
mod long_adder;
mod number_to_bytes;

pub use byte_order::ByteOrder;
pub use byte_util::ByteUtil;
pub use byte_util_error::ByteUtilError;
pub use bytes_to_number::BytesToNumber;
pub use double_adder::DoubleAdder;
pub use long_adder::LongAdder;
pub use number_to_bytes::NumberToBytes;

fn read_array<const SIZE: usize>(
    bytes: &[u8],
    start: usize,
) -> std::result::Result<[u8; SIZE], ByteUtilError> {
    let available = bytes.len().saturating_sub(start);
    let source = bytes
        .get(start..)
        .and_then(|remaining| remaining.get(..SIZE))
        .ok_or(ByteUtilError::InsufficientBytes {
            start,
            required: SIZE,
            available,
        })?;
    let mut output = [0_u8; SIZE];
    output.copy_from_slice(source);
    Ok(output)
}

fn java_f32_bits(value: f32) -> u32 {
    if value.is_nan() {
        0x7fc0_0000
    } else {
        value.to_bits()
    }
}

fn java_f64_bits(value: f64) -> u64 {
    if value.is_nan() {
        0x7ff8_0000_0000_0000
    } else {
        value.to_bits()
    }
}
