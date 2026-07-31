//! Hutool-compatible codec facades whose configuration exceeds the small core helpers.

/// 对齐: `cn.hutool.core.codec.Decoder`
/// 解码器

use crate::Result;

/// Rust-native equivalent of Hutool's generic decoder contract.
pub trait Decoder<Input: ?Sized, Output> {
    /// Decodes `input` into the configured output representation.
    fn decode(&self, input: &Input) -> Result<Output>;
}
