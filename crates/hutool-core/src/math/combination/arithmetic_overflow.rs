//! 对齐: `cn.hutool.core.math.Combination`
//! 来源: hutool-core/src/main/java/cn/hutool/core/math/Combination.java

use num_bigint::{BigInt, Sign};

/// 对齐 Java `ArithmeticException`（countSafe 溢出）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArithmeticOverflow {
    /// 错误描述。
    pub message: String,
}

impl std::fmt::Display for ArithmeticOverflow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ArithmeticOverflow {}
