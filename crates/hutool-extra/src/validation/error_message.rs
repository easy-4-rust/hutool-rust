//! Hutool bean-validation result types (no Jakarta Validator runtime).
//!
//! 对齐: `cn.hutool.extra.validation.BeanValidationResult`
//! 来源: hutool-extra/src/main/java/cn/hutool/extra/validation/BeanValidationResult.java
//!
//! `ValidationUtil` / Hibernate Validator remain planned — Java bean-validation SPI.

use super::bean_validation_result::BeanValidationResult;

/// Single property validation failure (Hutool `BeanValidationResult.ErrorMessage`).
///
/// 对齐 Java 内部类: `cn.hutool.extra.validation.BeanValidationResult.ErrorMessage`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorMessage {
    /// Property path / field name.
    pub property_name: String,
    /// Human-readable message.
    pub message: String,
    /// Optional stringified invalid value.
    pub value: Option<String>,
}

impl ErrorMessage {
    /// Creates an error message (Hutool setters path).
    #[must_use]
    pub fn new(
        property_name: impl Into<String>,
        message: impl Into<String>,
        value: Option<String>,
    ) -> Self {
        Self {
            property_name: property_name.into(),
            message: message.into(),
            value,
        }
    }

    /// Sets the property name (Hutool `setPropertyName`).
    #[must_use]
    pub fn set_property_name(mut self, property_name: impl Into<String>) -> Self {
        self.property_name = property_name.into();
        self
    }

    /// Sets the message (Hutool `setMessage`).
    #[must_use]
    pub fn set_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Sets the invalid value display (Hutool `setValue`).
    #[must_use]
    pub fn set_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
}
