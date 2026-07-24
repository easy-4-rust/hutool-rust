//! 对齐: `cn.hutool.core.math.Combination`
//! 来源: hutool-core/src/main/java/cn/hutool/core/math/Combination.java

use num_bigint::{BigInt, Sign};

mod combination;
mod arithmetic_overflow;

pub use combination::Combination;
pub use arithmetic_overflow::ArithmeticOverflow;
