//! Hutool bean-validation result types (no Jakarta Validator runtime).
//!
//! 对齐: `cn.hutool.extra.validation.BeanValidationResult`
//! 来源: hutool-extra/src/main/java/cn/hutool/extra/validation/BeanValidationResult.java
//!
//! `ValidationUtil` / Hibernate Validator remain planned — Java bean-validation SPI.

use super::error_message::ErrorMessage;

/// Aggregate bean validation outcome (Hutool `BeanValidationResult`).
///
/// 对齐 Java 类: `cn.hutool.extra.validation.BeanValidationResult`
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BeanValidationResult {
    success: bool,
    error_messages: Vec<ErrorMessage>,
}

impl BeanValidationResult {
    /// Creates a result with an explicit success flag (Hutool constructor).
    #[must_use]
    pub fn new(success: bool) -> Self {
        Self {
            success,
            error_messages: Vec::new(),
        }
    }

    /// Returns whether validation passed (Hutool `isSuccess`).
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Sets success flag (Hutool `setSuccess`).
    #[must_use]
    pub fn set_success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    /// Returns error messages (Hutool `getErrorMessages`).
    #[must_use]
    pub fn error_messages(&self) -> &[ErrorMessage] {
        &self.error_messages
    }

    /// Replaces error messages (Hutool `setErrorMessages`).
    #[must_use]
    pub fn set_error_messages(mut self, error_messages: Vec<ErrorMessage>) -> Self {
        self.error_messages = error_messages;
        self.success = self.error_messages.is_empty();
        self
    }

    /// Appends one error and marks failure (Hutool `addErrorMessage`).
    #[must_use]
    pub fn add_error_message(mut self, error_message: ErrorMessage) -> Self {
        self.error_messages.push(error_message);
        self.success = false;
        self
    }

    /// Builds a result from explicit property checks (Rust-native warp helper).
    ///
    /// Callers supply `(property, message, value)` triples when a field fails;
    /// empty input yields a successful result.
    #[must_use]
    pub fn from_failures(
        failures: impl IntoIterator<Item = (String, String, Option<String>)>,
    ) -> Self {
        let mut result = Self::new(true);
        for (property_name, message, value) in failures {
            result = result.add_error_message(ErrorMessage::new(property_name, message, value));
        }
        result
    }
}
