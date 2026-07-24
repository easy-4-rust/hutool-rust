//! 对齐: `cn.hutool.core.convert.impl.NumberConverter`

#![allow(dead_code)]

use rust_decimal::Decimal;
use std::str::FromStr;

use crate::convert::convert::ConvertValue;
use crate::convert::convert_exception::ConvertException;
use crate::byte_util::ByteUtil;

use super::number_converter::NumberConverter;

#[derive(Debug, Clone, Copy)]
pub enum NumberTarget {
    Number,
    Integer,
    Long,
    Double,
    Float,
    Decimal,
}

impl NumberConverter {
    /// 对齐桩 sentinel
    pub fn pending_alignment() -> &'static str {
        "pending"
    }

    pub fn new(target: NumberTarget) -> Self {
        Self { target }
    }

    pub fn for_double() -> Self {
        Self::new(NumberTarget::Double)
    }

    pub fn for_integer() -> Self {
        Self::new(NumberTarget::Integer)
    }

    /// 对齐 Java: `numberConverter.convert(value, null)`
    pub fn convert(&self, value: &str) -> Result<f64, ConvertException> {
        let cleaned = value.trim().replace(',', "");
        match self.target {
            NumberTarget::Integer => cleaned
                .parse::<f64>()
                .map(|f| f as i32 as f64)
                .map_err(|_| ConvertException::new("NumberFormatException")),
            NumberTarget::Double | NumberTarget::Number | NumberTarget::Float => cleaned
                .parse::<f64>()
                .map_err(|_| ConvertException::new("NumberFormatException")),
            NumberTarget::Long => cleaned
                .parse::<f64>()
                .map(|f| f as i64 as f64)
                .map_err(|_| ConvertException::new("NumberFormatException")),
            NumberTarget::Decimal => cleaned
                .parse::<f64>()
                .map_err(|_| ConvertException::new("NumberFormatException")),
        }
    }

    pub fn convert_i32(value: &ConvertValue) -> Option<i32> {
        match value {
            ConvertValue::Null => None,
            ConvertValue::I64(n) => Some(*n as i32),
            ConvertValue::F64(n) => Some(*n as i32),
            ConvertValue::Bool(b) => Some(if *b { 1 } else { 0 }),
            ConvertValue::EnumOrdinal(o) => Some(*o),
            ConvertValue::DateMs(ms) => Some(*ms as i32),
            ConvertValue::NumberWithFormat(n) => Some(n.value() as i32),
            ConvertValue::Decimal(d) => d.to_string().parse::<f64>().ok().map(|f| f as i32),
            ConvertValue::Str(s) => parse_number_str(s).map(|f| f as i32),
            ConvertValue::Bytes(b) => ByteUtil::bytes_to_f32(b).ok().map(|f| f as i32),
            ConvertValue::List(_) | ConvertValue::Map(_) => None,
            _ => Self::convert_f64(value).map(|f| f as i32),
        }
    }

    pub fn convert_i64(value: &ConvertValue) -> Option<i64> {
        match value {
            ConvertValue::Null => None,
            ConvertValue::I64(n) => Some(*n),
            ConvertValue::F64(n) => Some(*n as i64),
            ConvertValue::Bool(b) => Some(if *b { 1 } else { 0 }),
            ConvertValue::NumberWithFormat(n) => Some(n.value()),
            ConvertValue::DateMs(ms) => Some(*ms),
            ConvertValue::Decimal(d) => d.to_string().parse::<f64>().ok().map(|f| f as i64),
            ConvertValue::Str(s) => parse_number_str(s).map(|f| f as i64),
            _ => Self::convert_f64(value).map(|f| f as i64),
        }
    }

    pub fn convert_i128(value: &ConvertValue) -> Option<i128> {
        match value {
            ConvertValue::Str(s) => s.trim().parse().ok(),
            ConvertValue::I64(n) => Some(*n as i128),
            ConvertValue::Decimal(d) => d.to_string().parse().ok(),
            _ => Self::convert_i64(value).map(|n| n as i128),
        }
    }

    pub fn convert_f64(value: &ConvertValue) -> Option<f64> {
        match value {
            ConvertValue::Null => None,
            ConvertValue::F64(n) => Some(*n),
            ConvertValue::I64(n) => Some(*n as f64),
            ConvertValue::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            ConvertValue::Decimal(d) => d.to_string().parse().ok(),
            ConvertValue::Str(s) => {
                if s.trim().is_empty() {
                    None
                } else {
                    parse_number_str(s)
                }
            }
            ConvertValue::Bytes(b) => {
                if b.len() >= 4 {
                    ByteUtil::bytes_to_f32(b).ok().map(|f| f as f64)
                } else {
                    None
                }
            }
            ConvertValue::NumberWithFormat(n) => Some(n.value() as f64),
            _ => None,
        }
    }

    pub fn convert_f32(value: &ConvertValue) -> Option<f32> {
        match value {
            ConvertValue::Bytes(b) => ByteUtil::bytes_to_f32(b).ok(),
            ConvertValue::F64(n) => Some(*n as f32),
            ConvertValue::I64(n) => Some(*n as f32),
            other => Self::convert_f64(other).map(|f| f as f32),
        }
    }

    pub fn convert_decimal(value: &ConvertValue) -> Option<Decimal> {
        match value {
            ConvertValue::Decimal(d) => Some(*d),
            ConvertValue::Str(s) => {
                let s = strip_number_suffix(s.trim());
                Decimal::from_str(s).ok()
            }
            ConvertValue::I64(n) => Some(Decimal::from(*n)),
            ConvertValue::F64(n) => Decimal::from_str(&n.to_string()).ok(),
            _ => Self::convert_f64(value).and_then(|f| Decimal::from_str(&f.to_string()).ok()),
        }
    }
}

fn parse_number_str(s: &str) -> Option<f64> {
    let s = s.trim().replace(',', "");
    if s.is_empty() {
        return None;
    }
    // 去类型后缀 D/L/F
    let s = if let Some(last) = s.chars().last() {
        let u = last.to_ascii_uppercase();
        if matches!(u, 'D' | 'L' | 'F') && s.len() > 1 {
            &s[..s.len() - 1]
        } else {
            s.as_str()
        }
    } else {
        s.as_str()
    };
    s.parse::<f64>().ok()
}

fn strip_number_suffix(s: &str) -> &str {
    if let Some(last) = s.chars().last() {
        let u = last.to_ascii_uppercase();
        if matches!(u, 'D' | 'L' | 'F') && s.len() > 1 {
            return &s[..s.len() - 1];
        }
    }
    s
}
