//! 对齐: `cn.hutool.core.util.NumberUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/NumberUtil.java
//!
//! Rust 版本提供算术、比较、最值、解析与 BigDecimal（`rust_decimal::Decimal`）对齐实现。

use rust_decimal::Decimal;
use std::str::FromStr;

/// 解析结果：对齐 Java `Number` 多形态
#[derive(Debug, Clone, PartialEq)]
pub enum ParsedNumber {
    I64(i64),
    F64(f64),
    Decimal(Decimal),
}

impl ParsedNumber {
    /// 转为 i64（对齐 Number.intValue/longValue 截断）
    pub fn as_i64(&self) -> i64 {
        match self {
            Self::I64(v) => *v,
            Self::F64(v) => *v as i64,
            Self::Decimal(d) => {
                let i = d.trunc();
                i.to_string().parse().unwrap_or(0)
            }
        }
    }

    /// 转为 f64
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::I64(v) => *v as f64,
            Self::F64(v) => *v,
            Self::Decimal(d) => decimal_to_f64(*d),
        }
    }

    /// 若为 Decimal 则返回
    pub fn as_decimal(&self) -> Option<Decimal> {
        match self {
            Self::Decimal(d) => Some(*d),
            Self::I64(v) => Some(Decimal::from(*v)),
            Self::F64(v) => Decimal::from_str(&f64_to_java_string(*v)).ok(),
        }
    }
}

use super::{decimal_to_f64, f64_to_java_string};
