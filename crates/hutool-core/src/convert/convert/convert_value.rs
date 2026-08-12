//! 对齐: `cn.hutool.core.convert.Convert`
//! 来源: hutool-core/src/main/java/cn/hutool/core/convert/Convert.java

#![allow(dead_code, clippy::too_many_arguments)]

use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::convert::number_with_format::NumberWithFormat;

/// 动态值载体，对齐 Java `Object` 入参
#[derive(Debug, Clone)]
pub enum ConvertValue {
    /// 空值
    Null,
    /// 字符串
    Str(String),
    /// 字符
    Char(char),
    /// 布尔
    Bool(bool),
    /// 整数
    I64(i64),
    /// 浮点
    F64(f64),
    /// 十进制
    Decimal(Decimal),
    /// 字节数组
    Bytes(Vec<u8>),
    /// 整数数组
    I64Array(Vec<i64>),
    /// 字符串数组
    StrArray(Vec<String>),
    /// 列表
    List(Vec<ConvertValue>),
    /// 映射
    Map(HashMap<String, ConvertValue>),
    /// 时间毫秒
    DateMs(i64),
    /// 带格式数字
    NumberWithFormat(NumberWithFormat),
    /// 枚举序号
    EnumOrdinal(i32),
    /// 类名
    ClassName(String),
    /// JSON 文本
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
