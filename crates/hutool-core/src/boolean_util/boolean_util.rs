//! Boolean conversion and aggregation helpers aligned with Hutool.

/// 对齐: `cn.hutool.core.util.BooleanUtil`
/// 布尔值工具类
use std::{any::TypeId, fmt};

use super::boolean_error::BooleanError;

/// Hutool-aligned boolean convenience methods.
#[derive(Debug, Clone, Copy, Default)]
pub struct BooleanUtil;

impl BooleanUtil {
    /// Negates an optional boolean while preserving `None`.
    #[must_use]
    pub const fn negate_option(value: Option<bool>) -> Option<bool> {
        match value {
            Some(value) => Some(!value),
            None => None,
        }
    }

    /// Returns whether the optional value is exactly true.
    #[must_use]
    pub const fn is_true(value: Option<bool>) -> bool {
        matches!(value, Some(true))
    }

    /// Returns whether the optional value is exactly false.
    #[must_use]
    pub const fn is_false(value: Option<bool>) -> bool {
        matches!(value, Some(false))
    }

    /// Negates a primitive boolean.
    #[must_use]
    pub const fn negate(value: bool) -> bool {
        !value
    }

    /// Parses Hutool's true vocabulary; blank and unknown values are false.
    #[must_use]
    pub fn to_boolean(value: &str) -> bool {
        Self::to_boolean_object(value).unwrap_or(false)
    }

    /// Parses Hutool's true and false vocabularies, returning `None` for unknown input.
    #[must_use]
    pub fn to_boolean_object(value: &str) -> Option<bool> {
        let value = value.trim();
        if value.is_empty() {
            return None;
        }
        if TRUE_VALUES
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(value))
        {
            return Some(true);
        }
        FALSE_VALUES
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(value))
            .then_some(false)
    }

    /// Converts false/true to 0/1.
    #[must_use]
    pub const fn to_int(value: bool) -> i32 {
        if value { 1 } else { 0 }
    }

    /// Boxed-Java alias represented by the same Rust primitive.
    #[must_use]
    pub const fn to_integer(value: bool) -> i32 {
        Self::to_int(value)
    }

    /// Converts false/true to the NUL/SOH Unicode scalar.
    #[must_use]
    pub const fn to_char(value: bool) -> char {
        if value { '\u{1}' } else { '\0' }
    }

    /// Boxed-Java alias represented by the same Rust `char`.
    #[must_use]
    pub const fn to_character(value: bool) -> char {
        Self::to_char(value)
    }

    /// Converts false/true to 0/1 as `i8`.
    #[must_use]
    pub const fn to_byte(value: bool) -> i8 {
        if value { 1 } else { 0 }
    }

    /// Boxed-Java alias represented by the same Rust primitive.
    #[must_use]
    pub const fn to_byte_object(value: bool) -> i8 {
        Self::to_byte(value)
    }

    /// Converts false/true to 0/1 as `i64`.
    #[must_use]
    pub const fn to_long(value: bool) -> i64 {
        if value { 1 } else { 0 }
    }

    /// Boxed-Java alias represented by the same Rust primitive.
    #[must_use]
    pub const fn to_long_object(value: bool) -> i64 {
        Self::to_long(value)
    }

    /// Converts false/true to 0/1 as `i16`.
    #[must_use]
    pub const fn to_short(value: bool) -> i16 {
        if value { 1 } else { 0 }
    }

    /// Boxed-Java alias represented by the same Rust primitive.
    #[must_use]
    pub const fn to_short_object(value: bool) -> i16 {
        Self::to_short(value)
    }

    /// Converts false/true to 0.0/1.0 as `f32`.
    #[must_use]
    pub const fn to_float(value: bool) -> f32 {
        if value { 1.0 } else { 0.0 }
    }

    /// Boxed-Java alias represented by the same Rust primitive.
    #[must_use]
    pub const fn to_float_object(value: bool) -> f32 {
        Self::to_float(value)
    }

    /// Converts false/true to 0.0/1.0 as `f64`.
    #[must_use]
    pub const fn to_double(value: bool) -> f64 {
        if value { 1.0 } else { 0.0 }
    }

    /// Boxed-Java alias represented by the same Rust primitive.
    #[must_use]
    pub const fn to_double_object(value: bool) -> f64 {
        Self::to_double(value)
    }

    /// Returns `"true"` or `"false"`.
    #[must_use]
    pub const fn to_string_true_false(value: bool) -> &'static str {
        Self::to_string(value, "true", "false")
    }

    /// Returns `"on"` or `"off"`.
    #[must_use]
    pub const fn to_string_on_off(value: bool) -> &'static str {
        Self::to_string(value, "on", "off")
    }

    /// Returns `"yes"` or `"no"`.
    #[must_use]
    pub const fn to_string_yes_no(value: bool) -> &'static str {
        Self::to_string(value, "yes", "no")
    }

    /// Selects one of two caller-provided strings.
    #[must_use]
    pub const fn to_string<'a>(
        value: bool,
        true_string: &'a str,
        false_string: &'a str,
    ) -> &'a str {
        if value { true_string } else { false_string }
    }

    /// Selects a caller-provided string for true, false, or `None`.
    #[must_use]
    pub const fn option_to_string<'a>(
        value: Option<bool>,
        true_string: &'a str,
        false_string: &'a str,
        none_string: &'a str,
    ) -> &'a str {
        match value {
            Some(true) => true_string,
            Some(false) => false_string,
            None => none_string,
        }
    }

    /// Returns true when every operand is true.
    pub fn and(values: &[bool]) -> Result<bool, BooleanError> {
        require_values(values)?;
        Ok(values.iter().all(|value| *value))
    }

    /// Optional-value variant where `None` follows Hutool's false behavior.
    pub fn and_wrapped(values: &[Option<bool>]) -> Result<bool, BooleanError> {
        require_values(values)?;
        Ok(values.iter().all(|value| Self::is_true(*value)))
    }

    /// Returns true when any operand is true.
    pub fn or(values: &[bool]) -> Result<bool, BooleanError> {
        require_values(values)?;
        Ok(values.iter().any(|value| *value))
    }

    /// Optional-value variant where `None` follows Hutool's false behavior.
    pub fn or_wrapped(values: &[Option<bool>]) -> Result<bool, BooleanError> {
        require_values(values)?;
        Ok(values.iter().any(|value| Self::is_true(*value)))
    }

    /// Returns true when the number of true operands is odd.
    pub fn xor(values: &[bool]) -> Result<bool, BooleanError> {
        require_values(values)?;
        Ok(values.iter().fold(false, |result, value| result ^ value))
    }

    /// Returns true only when exactly one operand is true.
    pub fn exactly_one_true(values: &[bool]) -> Result<bool, BooleanError> {
        require_values(values)?;
        let mut found = false;
        for value in values {
            if *value {
                if found {
                    return Ok(false);
                }
                found = true;
            }
        }
        Ok(found)
    }

    /// Optional-value parity variant where `None` is false.
    pub fn xor_wrapped(values: &[Option<bool>]) -> Result<bool, BooleanError> {
        require_values(values)?;
        Ok(values
            .iter()
            .fold(false, |result, value| result ^ value.unwrap_or(false)))
    }

    /// Returns whether `T` is Rust's primitive boolean type.
    #[must_use]
    pub fn is_boolean_type<T: 'static>() -> bool {
        TypeId::of::<T>() == TypeId::of::<bool>()
    }
}

impl fmt::Display for BooleanUtil {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("BooleanUtil")
    }
}

const TRUE_VALUES: &[&str] = &[
    "true", "yes", "y", "t", "ok", "correct", "success", "on", "1", "是", "对", "真", "對", "正确",
    "开", "开启", "√", "☑",
];

fn require_values<T>(values: &[T]) -> Result<(), BooleanError> {
    if values.is_empty() {
        Err(BooleanError::EmptyInput)
    } else {
        Ok(())
    }
}

const FALSE_VALUES: &[&str] = &[
    "false", "no", "n", "f", "wrong", "fail", "off", "0", "否", "错", "假", "錯", "错误", "关",
    "关闭", "×", "☒",
];
