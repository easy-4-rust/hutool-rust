//! Hutool bean-validation result types (no Jakarta Validator runtime).
//!
//! 对齐: `cn.hutool.extra.validation.BeanValidationResult`
//! 来源: hutool-extra/src/main/java/cn/hutool/extra/validation/BeanValidationResult.java
//!
//! `ValidationUtil` / Hibernate Validator remain planned — Java bean-validation SPI.

mod error_message;
mod bean_validation_result;
mod validation_util;

pub use error_message::ErrorMessage;
pub use bean_validation_result::BeanValidationResult;
pub use validation_util::ValidationUtil;
