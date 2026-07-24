//! 对齐: `cn.hutool.core.lang.hash.MetroHash`
//! 常量按 Java `int` 字面量拓宽为 `long`（高位符号扩展）。

mod number128;
mod metro_hash;

pub use number128::Number128;
pub use metro_hash::MetroHash;
