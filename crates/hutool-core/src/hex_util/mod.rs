//! Convenient hexadecimal operations aligned with Hutool's `HexUtil` family.

/// 对齐: `cn.hutool.core.util.HexUtil`
/// 十六进制工具类

mod hex_util_error;
mod rgb_color;
mod hex_util;

pub use hex_util_error::HexUtilError;
pub use rgb_color::RgbColor;
pub use hex_util::HexUtil;
