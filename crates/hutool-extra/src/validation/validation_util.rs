//! Hutool bean-validation result types (no Jakarta Validator runtime).
//!
//! 对齐: `cn.hutool.extra.validation.BeanValidationResult`
//! 来源: hutool-extra/src/main/java/cn/hutool/extra/validation/BeanValidationResult.java
//!
//! `ValidationUtil` / Hibernate Validator remain planned — Java bean-validation SPI.

use super::bean_validation_result::BeanValidationResult;

/// Hutool `ValidationUtil` warp helpers without a Jakarta Validator factory.
///
/// 对齐 Java 类: `cn.hutool.extra.validation.ValidationUtil`（结果包装子集）
///
/// `getValidator` / reflective `validate` remain planned.
pub struct ValidationUtil;

impl ValidationUtil {
    /// Wraps precomputed failures into [`BeanValidationResult`] (Hutool `warpValidate` shape).
    #[must_use]
    pub fn warp_validate(
        failures: impl IntoIterator<Item = (String, String, Option<String>)>,
    ) -> BeanValidationResult {
        BeanValidationResult::from_failures(failures)
    }

    /// Wraps a single-property failure list (Hutool `warpValidateProperty` shape).
    #[must_use]
    pub fn warp_validate_property(
        property_name: impl Into<String>,
        failures: impl IntoIterator<Item = (String, Option<String>)>,
    ) -> BeanValidationResult {
        let property = property_name.into();
        BeanValidationResult::from_failures(failures.into_iter().map(|(message, value)| {
            (property.clone(), message, value)
        }))
    }
}
