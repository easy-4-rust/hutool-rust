//! 对齐: `cn.hutool.core.convert.Convert`
//! 来源: hutool-core/src/main/java/cn/hutool/core/convert/Convert.java

#![allow(dead_code, clippy::too_many_arguments)]

use crate::boolean_util::BooleanUtil;
use crate::byte_util::ByteUtil;
use crate::charset_util::CharsetUtil;
use crate::hex_util::HexUtil;
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::time::Duration;

use super::basic_type::BasicType;
use super::convert_exception::ConvertException;
use super::number_chinese_formatter::NumberChineseFormatter;
use super::number_with_format::NumberWithFormat;
use super::number_word_formatter::NumberWordFormatter;
use super::impl_::number_converter::NumberConverter;

mod convert;
mod convert_value;
mod time_unit;

pub use convert::Convert;
pub use convert_value::ConvertValue;
pub use time_unit::TimeUnit;
