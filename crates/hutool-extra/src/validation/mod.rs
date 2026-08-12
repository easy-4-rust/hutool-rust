//! Hutool bean-validation result types (no Jakarta Validator runtime).
//!
//! 对齐: `cn.hutool.extra.validation.BeanValidationResult`
//! 来源: hutool-extra/src/main/java/cn/hutool/extra/validation/BeanValidationResult.java
//!
//! `ValidationUtil` / Hibernate Validator remain planned — Java bean-validation SPI.

mod bean_validation_result;
mod error_message;
mod validation_util;

pub use bean_validation_result::BeanValidationResult;
pub use error_message::ErrorMessage;
pub use validation_util::ValidationUtil;
