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

use crate::convert::basic_type::BasicType;
use crate::convert::convert_exception::ConvertException;
use crate::convert::number_chinese_formatter::NumberChineseFormatter;
use crate::convert::number_with_format::NumberWithFormat;
use crate::convert::number_word_formatter::NumberWordFormatter;
use crate::convert::impl_::number_converter::NumberConverter;

/// 对齐 Java `TimeUnit`
#[derive(Debug, Clone, Copy)]
pub enum TimeUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
}
