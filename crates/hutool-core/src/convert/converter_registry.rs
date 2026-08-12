//! 对齐: `cn.hutool.core.convert.ConverterRegistry`

#![allow(dead_code)]

use std::sync::OnceLock;

use super::convert::ConvertValue;
use super::convert_exception::ConvertException;
use super::impl_::number_converter::{NumberConverter, NumberTarget};

/// 对齐 Java 类: `cn.hutool.core.convert.ConverterRegistry`
#[derive(Debug, Default)]
pub struct ConverterRegistry {
    custom_i32: Option<fn(&ConvertValue) -> Option<i32>>,
}

static INSTANCE: OnceLock<ConverterRegistry> = OnceLock::new();

impl ConverterRegistry {
    /// 兼容 sentinel。
    pub fn pending_alignment() -> &'static str {
        "pending"
    }

    /// 全局单例。
    pub fn get_instance() -> &'static ConverterRegistry {
        INSTANCE.get_or_init(ConverterRegistry::default)
    }

    /// 创建空注册表。
    pub fn new() -> Self {
        Self::default()
    }

    /// 对齐 Java: `getConverter(Type)` — 返回是否内置支持
    pub fn has_converter(&self, type_name: &str) -> bool {
        matches!(
            type_name,
            "int" | "Integer" | "long" | "Long" | "double" | "Double" | "String" | "Boolean"
        )
    }

    /// 注册自定义转换（测试用）
    pub fn put_custom_i32(&mut self, f: fn(&ConvertValue) -> Option<i32>) {
        self.custom_i32 = Some(f);
    }

    /// 转换值到 int。
    ///
    /// # Errors
    ///
    /// 转换失败时返回 [`ConvertException`]。
    pub fn convert_i32(&self, value: &ConvertValue) -> Result<i32, ConvertException> {
        if let Some(f) = self.custom_i32 {
            return f(value).ok_or_else(|| ConvertException::new("custom convert failed"));
        }
        NumberConverter::convert_i32(value)
            .ok_or_else(|| ConvertException::new("convert to int failed"))
    }

    /// 转换值到 long。
    ///
    /// # Errors
    ///
    /// 转换失败时返回 [`ConvertException`]。
    pub fn convert_i64(&self, value: &ConvertValue) -> Result<i64, ConvertException> {
        NumberConverter::convert_i64(value)
            .ok_or_else(|| ConvertException::new("convert to long failed"))
    }

    /// 获取 Double 目标转换器。
    pub fn number_converter_double(&self) -> NumberConverter {
        NumberConverter::new(NumberTarget::Double)
    }
}
