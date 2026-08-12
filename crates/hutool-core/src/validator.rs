//! 对齐: `cn.hutool.core.lang.Validator`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Validator.java
//!
//! Rust 版本提供数据验证的 idiomatic 实现；校验失败抛出 [`ValidateException`]。

use crate::exceptions::ValidateException;
use crate::lang::pattern_pool::PatternPool;
use crate::lang::regex_pool::RegexPool;
use crate::string::{format_template, is_blank};
use crate::{CoreError, Result};
use std::fmt::Display;

/// 对齐 Java: `cn.hutool.core.lang.Validator`
#[derive(Debug, Clone, Copy, Default)]
pub struct Validator;

impl Validator {
    // ── 布尔 / 空值 ──

    /// 对齐 Java: `Validator.isTrue(boolean)`
    pub fn is_true(value: bool) -> bool {
        value
    }

    /// 对齐 Java: `Validator.isFalse(boolean)`
    pub fn is_false(value: bool) -> bool {
        !value
    }

    /// 对齐 Java: `Validator.validateTrue(boolean, String, Object...)`
    pub fn validate_true(
        value: bool,
        template: &str,
        params: &[&dyn Display],
    ) -> std::result::Result<bool, ValidateException> {
        if value {
            Ok(true)
        } else {
            Err(ValidateException::with_template(template, params))
        }
    }

    /// 对齐 Java: `Validator.validateFalse(boolean, String, Object...)`
    pub fn validate_false(
        value: bool,
        template: &str,
        params: &[&dyn Display],
    ) -> std::result::Result<bool, ValidateException> {
        if !value {
            Ok(false)
        } else {
            Err(ValidateException::with_template(template, params))
        }
    }

    /// 对齐 Java: `Validator.isNull(Object)`
    pub fn is_null<T>(value: Option<T>) -> bool {
        value.is_none()
    }

    /// 对齐 Java: `Validator.isNotNull(Object)`
    pub fn is_not_null<T>(value: Option<&T>) -> bool {
        value.is_some()
    }

    /// 对齐 Java: `Validator.validateNull(T, String, Object...)`
    pub fn validate_null<T>(
        value: Option<T>,
        template: &str,
        params: &[&dyn Display],
    ) -> std::result::Result<(), ValidateException> {
        if value.is_none() {
            Ok(())
        } else {
            Err(ValidateException::with_template(template, params))
        }
    }

    /// 对齐 Java: `Validator.validateNotNull(T, String, Object...)`
    pub fn validate_not_null<T>(
        value: Option<T>,
        template: &str,
        params: &[&dyn Display],
    ) -> std::result::Result<T, ValidateException> {
        value.ok_or_else(|| ValidateException::with_template(template, params))
    }

    /// 对齐 Java: `Validator.isEmpty(CharSequence)` / `Object`
    pub fn is_empty(value: Option<&str>) -> bool {
        value.map_or(true, |s| s.is_empty())
    }

    /// 对齐 Java: `Validator.isNotEmpty(CharSequence)`
    pub fn is_not_empty(value: Option<&str>) -> bool {
        !Self::is_empty(value)
    }

    /// 对齐 Java: `Validator.validateEmpty(T, String)`
    pub fn validate_empty<'a>(
        value: Option<&'a str>,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_empty(value) {
            Ok(value.unwrap_or(""))
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.validateNotEmpty(T, String)`
    pub fn validate_not_empty_str<'a>(
        value: Option<&'a str>,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        match value {
            Some(s) if !s.is_empty() => Ok(s),
            _ => Err(ValidateException::new(error_msg)),
        }
    }

    /// 对齐 Java: `Validator.equal(Object, Object)`
    pub fn equal<T: PartialEq>(t1: &T, t2: &T) -> bool {
        t1 == t2
    }

    /// 对齐 Java: `Validator.validateEqual(Object, Object, String)`
    pub fn validate_equal<T: PartialEq + Clone>(
        t1: &T,
        t2: &T,
        error_msg: &str,
    ) -> std::result::Result<T, ValidateException> {
        if t1 == t2 {
            Ok(t1.clone())
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.validateNotEqual(Object, Object, String)`
    pub fn validate_not_equal<T: PartialEq>(
        t1: &T,
        t2: &T,
        error_msg: &str,
    ) -> std::result::Result<(), ValidateException> {
        if t1 != t2 {
            Ok(())
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.validateNotEmptyAndEqual`
    pub fn validate_not_empty_and_equal(
        t1: Option<&str>,
        t2: Option<&str>,
        error_msg: &str,
    ) -> std::result::Result<(), ValidateException> {
        let a = Self::validate_not_empty_str(t1, error_msg)?;
        let b = Self::validate_not_empty_str(t2, error_msg)?;
        Self::validate_equal(&a, &b, error_msg).map(|_| ())
    }

    /// 对齐 Java: `Validator.validateNotEmptyAndNotEqual`
    pub fn validate_not_empty_and_not_equal(
        t1: Option<&str>,
        t2: Option<&str>,
        error_msg: &str,
    ) -> std::result::Result<(), ValidateException> {
        let a = Self::validate_not_empty_str(t1, error_msg)?;
        let b = Self::validate_not_empty_str(t2, error_msg)?;
        Self::validate_not_equal(&a, &b, error_msg)
    }

    /// 对齐 Java: `Validator.validateMatchRegex(String, T, String)`
    pub fn validate_match_regex<'a>(
        regex: &str,
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_match_regex(regex, value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    // ── 长度 / 范围 ──

    /// 对齐 Java: `Validator.isBetween(CharSequence, int, int)`
    pub fn is_between(value: &str, min: usize, max: usize) -> bool {
        let len = value.chars().count();
        len >= min && len <= max
    }

    /// 对齐 Java: `Validator.minLength` / 长度下限
    pub fn min_length(value: &str, min: usize) -> bool {
        value.chars().count() >= min
    }

    /// 对齐 Java: `Validator.maxLength`
    pub fn max_length(value: &str, max: usize) -> bool {
        value.chars().count() <= max
    }

    /// 对齐 Java: `Validator.isBetween(Number, Number, Number)`
    pub fn is_between_number(value: f64, min: f64, max: f64) -> bool {
        value >= min && value <= max
    }

    /// 对齐 Java: `Validator.isBetween(long, long, long)`
    pub fn is_between_i64(value: i64, min: i64, max: i64) -> bool {
        value >= min && value <= max
    }

    /// 对齐 Java: `Validator.isBetween(double, double, double)`
    pub fn is_between_f64(value: f64, min: f64, max: f64) -> bool {
        value >= min && value <= max
    }

    /// 对齐 Java: `Validator.validateBetween(Number, Number, Number, String)`
    pub fn validate_between_number(
        value: f64,
        min: f64,
        max: f64,
        error_msg: &str,
    ) -> std::result::Result<(), ValidateException> {
        if Self::is_between_number(value, min, max) {
            Ok(())
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.checkIndexLimit(int, int)`
    pub fn check_index_limit(
        index: usize,
        size: usize,
    ) -> std::result::Result<(), ValidateException> {
        if index < size {
            Ok(())
        } else {
            Err(ValidateException::with_template(
                "Index {} out of bounds for length {}",
                &[&index, &size],
            ))
        }
    }

    // ── 正则类 ──

    /// 对齐 Java: `Validator.isMatchRegex(String, CharSequence)`
    pub fn is_match_regex(pattern: &str, value: &str) -> bool {
        PatternPool::get(pattern).is_match(value)
    }

    /// 对齐 Java: `Validator.isGeneral(CharSequence)`
    pub fn is_general(value: &str) -> bool {
        Self::is_match_regex(RegexPool::GENERAL, value)
    }

    /// 对齐 Java: `Validator.isGeneral(CharSequence, int, int)`
    pub fn is_general_len(value: &str, min: usize, max: usize) -> bool {
        Self::is_general(value) && Self::is_between(value, min, max)
    }

    /// 对齐 Java: `Validator.isGeneral(CharSequence, int)`
    pub fn is_general_min(value: &str, min: usize) -> bool {
        Self::is_general(value) && Self::min_length(value, min)
    }

    /// 对齐 Java: `Validator.validateGeneral`
    pub fn validate_general<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_general(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.validateGeneral(T, int, int, String)`
    pub fn validate_general_len<'a>(
        value: &'a str,
        min: usize,
        max: usize,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_general_len(value, min, max) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.validateGeneral(T, int, String)`
    pub fn validate_general_min<'a>(
        value: &'a str,
        min: usize,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_general_min(value, min) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isLetter`
    pub fn is_letter(value: &str) -> bool {
        !value.is_empty() && value.chars().all(|c| c.is_ascii_alphabetic())
    }

    /// 对齐 Java: `Validator.validateLetter`
    pub fn validate_letter<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_letter(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isUpperCase`
    pub fn is_upper_case(value: &str) -> bool {
        !value.is_empty() && value.chars().all(|c| c.is_ascii_uppercase())
    }

    /// 对齐 Java: `Validator.validateUpperCase`
    pub fn validate_upper_case<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_upper_case(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isLowerCase`
    pub fn is_lower_case(value: &str) -> bool {
        !value.is_empty() && value.chars().all(|c| c.is_ascii_lowercase())
    }

    /// 对齐 Java: `Validator.validateLowerCase`
    pub fn validate_lower_case<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_lower_case(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isNumber`
    pub fn is_number(value: &str) -> bool {
        crate::number_util::NumberUtil::is_number(value)
    }

    /// 对齐 Java: `Validator.hasNumber`
    pub fn has_number(value: &str) -> bool {
        value.chars().any(|c| c.is_ascii_digit())
    }

    /// 对齐 Java: `Validator.validateNumber`
    pub fn validate_number(
        value: &str,
        error_msg: &str,
    ) -> std::result::Result<String, ValidateException> {
        if Self::is_number(value) {
            Ok(value.to_string())
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isWord`
    pub fn is_word(value: &str) -> bool {
        Self::is_match_regex(RegexPool::WORD, value)
    }

    /// 对齐 Java: `Validator.validateWord`
    pub fn validate_word<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_word(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isMoney`
    pub fn is_money(value: &str) -> bool {
        Self::is_match_regex(RegexPool::MONEY, value)
    }

    /// 对齐 Java: `Validator.validateMoney`
    pub fn validate_money<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_money(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isZipCode`
    pub fn is_zip_code(value: &str) -> bool {
        Self::is_match_regex(RegexPool::ZIP_CODE, value)
    }

    /// 对齐 Java: `Validator.validateZipCode`
    pub fn validate_zip_code<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_zip_code(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isEmail`
    pub fn is_email(value: &str) -> bool {
        crate::re_util::ReUtil::is_email(value)
    }

    /// 对齐 Java: `Validator.isEmail(..., includChinese)` — 中文邮箱暂与标准邮箱共用。
    pub fn is_email_with_chinese(value: &str, _include_chinese: bool) -> bool {
        Self::is_email(value)
    }

    /// 对齐 Java: `Validator.validateEmail`
    pub fn validate_email<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_email(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isMobile`
    pub fn is_mobile(value: &str) -> bool {
        crate::re_util::ReUtil::is_mobile(value)
    }

    /// 对齐 Java: `Validator.validateMobile`
    pub fn validate_mobile<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_mobile(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isCitizenId`
    pub fn is_citizen_id(value: &str) -> bool {
        crate::idcard_util::IdcardUtil::is_valid_card(value)
    }

    /// 对齐 Java: `Validator.validateCitizenIdNumber`
    pub fn validate_citizen_id_number<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_citizen_id(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isBirthday(int, int, int)`
    pub fn is_birthday_ymd(year: i32, month: u32, day: u32) -> bool {
        chrono::NaiveDate::from_ymd_opt(year, month, day).is_some()
    }

    /// 对齐 Java: `Validator.isBirthday(CharSequence)`
    pub fn is_birthday(value: &str) -> bool {
        Self::is_match_regex(RegexPool::BIRTHDAY, value)
    }

    /// 对齐 Java: `Validator.validateBirthday`
    pub fn validate_birthday<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_birthday(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isIpv4`
    pub fn is_ipv4(value: &str) -> bool {
        crate::re_util::ReUtil::is_ipv4(value)
    }

    /// 对齐 Java: `Validator.validateIpv4`
    pub fn validate_ipv4<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_ipv4(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isIpv6`
    pub fn is_ipv6(value: &str) -> bool {
        crate::re_util::ReUtil::is_ipv6(value)
    }

    /// 对齐 Java: `Validator.validateIpv6`
    pub fn validate_ipv6<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_ipv6(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isMac`
    pub fn is_mac(value: &str) -> bool {
        Self::is_match_regex(RegexPool::MAC_ADDRESS, value)
    }

    /// 对齐 Java: `Validator.validateMac`
    pub fn validate_mac<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_mac(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isPlateNumber`
    pub fn is_plate_number(value: &str) -> bool {
        Self::is_match_regex(RegexPool::PLATE_NUMBER, value)
    }

    /// 对齐 Java: `Validator.validatePlateNumber`
    pub fn validate_plate_number<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_plate_number(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isUrl`
    pub fn is_url(value: &str) -> bool {
        crate::re_util::ReUtil::is_url(value)
    }

    /// 对齐 Java: `Validator.validateUrl`
    pub fn validate_url<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_url(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isChinese`
    pub fn is_chinese(value: &str) -> bool {
        crate::re_util::ReUtil::is_chinese(value)
    }

    /// 对齐 Java: `Validator.hasChinese`
    pub fn has_chinese(value: &str) -> bool {
        value
            .chars()
            .any(|c| ('\u{4e00}'..='\u{9fff}').contains(&c))
    }

    /// 对齐 Java: `Validator.validateChinese`
    pub fn validate_chinese<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_chinese(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isGeneralWithChinese`
    pub fn is_general_with_chinese(value: &str) -> bool {
        !value.is_empty()
            && value.chars().all(|c| {
                c.is_ascii_alphanumeric() || c == '_' || ('\u{4e00}'..='\u{9fff}').contains(&c)
            })
    }

    /// 对齐 Java: `Validator.validateGeneralWithChinese`
    pub fn validate_general_with_chinese<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_general_with_chinese(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isUUID`
    pub fn is_uuid(value: &str) -> bool {
        Self::is_match_regex(RegexPool::UUID, value)
    }

    /// 对齐 Java: `Validator.validateUUID`
    pub fn validate_uuid<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_uuid(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isHex`
    pub fn is_hex(value: &str) -> bool {
        Self::is_match_regex(RegexPool::HEX, value)
    }

    /// 对齐 Java: `Validator.validateHex`
    pub fn validate_hex<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_hex(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isCreditCode`
    pub fn is_credit_code(value: &str) -> bool {
        crate::credit_code_util::CreditCodeUtil::is_credit_code(value)
    }

    /// 对齐 Java: `Validator.isCarVin`
    pub fn is_car_vin(value: &str) -> bool {
        Self::is_match_regex(RegexPool::CAR_VIN, value)
    }

    /// 对齐 Java: `Validator.validateCarVin`
    pub fn validate_car_vin<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_car_vin(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isCarDrivingLicence`
    pub fn is_car_driving_licence(value: &str) -> bool {
        Self::is_match_regex(RegexPool::CAR_DRIVING_LICENCE, value)
    }

    /// 对齐 Java: `Validator.validateCarDrivingLicence`
    pub fn validate_car_driving_licence<'a>(
        value: &'a str,
        error_msg: &str,
    ) -> std::result::Result<&'a str, ValidateException> {
        if Self::is_car_driving_licence(value) {
            Ok(value)
        } else {
            Err(ValidateException::new(error_msg))
        }
    }

    /// 对齐 Java: `Validator.isChineseName`
    pub fn is_chinese_name(value: &str) -> bool {
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        let re = RE
            .get_or_init(|| regex::Regex::new("^[\u{3400}-\u{9FFF}\u{00b7}]{2,60}$").expect("cn"));
        re.is_match(value)
    }

    /// 对齐 Java: `Validator.isIdCard`（门面，同 citizenId）
    pub fn is_id_card(value: &str) -> bool {
        Self::is_citizen_id(value)
    }

    /// 对齐 Java: `Validator.isInteger`
    pub fn is_integer(value: &str) -> bool {
        crate::number_util::NumberUtil::is_integer(value)
    }

    /// 对齐 Java: `Validator.isNotEmpty(Collection)`
    pub fn is_not_empty_collection<T>(items: &[T]) -> bool {
        !items.is_empty()
    }

    /// 对齐 Java: `Validator.isEmpty(Collection)`
    pub fn is_empty_collection<T>(items: &[T]) -> bool {
        items.is_empty()
    }

    /// 对齐 Java: `Validator.validateNotEmpty(CharSequence, String)` — CoreError 门面（兼容旧调用）。
    pub fn validate_not_empty(value: Option<&str>, name: &str) -> Result<()> {
        if Self::is_empty(value) {
            return Err(CoreError::InvalidArgument {
                name: Box::leak(name.to_string().into_boxed_str()),
                reason: "must not be empty",
            });
        }
        Ok(())
    }

    /// 对齐 Java: `Validator.validateBetween(CharSequence, int, int, String)` — CoreError 门面。
    pub fn validate_between(value: &str, min: usize, max: usize, name: &str) -> Result<()> {
        if !Self::is_between(value, min, max) {
            return Err(CoreError::InvalidArgument {
                name: Box::leak(name.to_string().into_boxed_str()),
                reason: "length out of range",
            });
        }
        Ok(())
    }

    /// 格式化消息辅助（供调用方拼装 ValidateException）。
    pub fn format_error(template: &str, params: &[&dyn Display]) -> String {
        format_template(template, params)
    }

    /// 空白判断辅助。
    pub fn is_blank_str(value: &str) -> bool {
        is_blank(value)
    }
}

#[cfg(test)]
mod validator_idiomatic_parity {
    use super::*;

    /// 对齐 Java Validator 高流量校验路径的可执行证据。
    #[test]
    fn validator_bool_regex_range_and_identity_checks() {
        assert!(Validator::is_true(true));
        assert!(Validator::is_false(false));
        assert!(Validator::validate_true(true, "x", &[]).is_ok());
        assert!(Validator::is_general("abc_1"));
        assert!(Validator::is_email("a@b.com"));
        assert!(Validator::is_mobile("13800138000"));
        assert!(Validator::is_ipv4("127.0.0.1"));
        assert!(Validator::is_uuid("550e8400-e29b-41d4-a716-446655440000"));
        assert!(Validator::is_hex("deadBEEF"));
        assert!(Validator::is_between_i64(5, 1, 10));
        assert!(Validator::is_money("12.34"));
        assert!(Validator::equal(&1, &1));
        assert!(Validator::validate_not_empty_str(Some("x"), "e").is_ok());
        assert!(Validator::check_index_limit(0, 1).is_ok());
    }
}
