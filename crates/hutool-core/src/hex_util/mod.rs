//! Convenient hexadecimal operations aligned with Hutool's `HexUtil` family.

mod hex_util;
/// 对齐: `cn.hutool.core.util.HexUtil`
/// 十六进制工具类
mod hex_util_error;
mod rgb_color;

pub use hex_util::HexUtil;
pub use hex_util_error::HexUtilError;
pub use rgb_color::RgbColor;
