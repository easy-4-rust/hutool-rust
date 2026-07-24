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

/// 动态值载体，对齐 Java `Object` 入参
#[derive(Debug, Clone)]
pub enum ConvertValue {
    Null,
    Str(String),
    Char(char),
    Bool(bool),
    I64(i64),
    F64(f64),
    Decimal(Decimal),
    Bytes(Vec<u8>),
    I64Array(Vec<i64>),
    StrArray(Vec<String>),
    List(Vec<ConvertValue>),
    Map(HashMap<String, ConvertValue>),
    DateMs(i64),
    NumberWithFormat(NumberWithFormat),
    EnumOrdinal(i32),
    ClassName(String),
    Json(String),
}

impl From<&str> for ConvertValue {
    fn from(s: &str) -> Self {
        ConvertValue::Str(s.to_string())
    }
}

impl From<String> for ConvertValue {
    fn from(s: String) -> Self {
        ConvertValue::Str(s)
    }
}

impl From<i32> for ConvertValue {
    fn from(n: i32) -> Self {
        ConvertValue::I64(n as i64)
    }
}

impl From<i64> for ConvertValue {
    fn from(n: i64) -> Self {
        ConvertValue::I64(n)
    }
}

impl From<f64> for ConvertValue {
    fn from(n: f64) -> Self {
        ConvertValue::F64(n)
    }
}

impl From<f32> for ConvertValue {
    fn from(n: f32) -> Self {
        ConvertValue::F64(n as f64)
    }
}

impl From<bool> for ConvertValue {
    fn from(b: bool) -> Self {
        ConvertValue::Bool(b)
    }
}

impl From<char> for ConvertValue {
    fn from(c: char) -> Self {
        ConvertValue::Char(c)
    }
}

impl From<Vec<i64>> for ConvertValue {
    fn from(a: Vec<i64>) -> Self {
        ConvertValue::I64Array(a)
    }
}

impl From<Vec<u8>> for ConvertValue {
    fn from(b: Vec<u8>) -> Self {
        ConvertValue::Bytes(b)
    }
}
