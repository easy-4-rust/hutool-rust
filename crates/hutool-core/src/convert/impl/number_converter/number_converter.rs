//! 对齐: `cn.hutool.core.convert.impl.NumberConverter`

#![allow(dead_code)]

use rust_decimal::Decimal;
use std::str::FromStr;

use crate::convert::convert::ConvertValue;
use crate::convert::convert_exception::ConvertException;
use crate::byte_util::ByteUtil;

use super::number_target::NumberTarget;

/// 对齐 Java 类: `NumberConverter`
#[derive(Debug, Clone)]
pub struct NumberConverter {
    target: NumberTarget,
}
