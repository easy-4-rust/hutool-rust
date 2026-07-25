use std::fmt;

use chrono::{Datelike, Local, NaiveDate};
use thiserror::Error;

use super::idcard_error::IdcardError;
use super::idcard_util::IdcardUtil;

/// Owned identity-card information corresponding to Hutool's nested `Idcard` value.

/// 对齐: `cn.hutool.core.util.IdcardUtil`
/// 身份证
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Idcard {
    province_code: Option<String>,
    city_code: Option<String>,
    birth_date: NaiveDate,
    pub(crate) gender: u8,
    age: i32,
}

impl Idcard {
    /// Creates identity-card information using the current local date.
    pub fn new(idcard: &str) -> Result<Self, IdcardError> {
        Self::new_at(idcard, Local::now().date_naive())
    }

    /// Creates identity-card information at a deterministic comparison date.
    pub fn new_at(idcard: &str, comparison: NaiveDate) -> Result<Self, IdcardError> {
        Ok(Self {
            province_code: IdcardUtil::get_province_code_by_id_card(idcard),
            city_code: IdcardUtil::get_city_code_by_id_card(idcard),
            birth_date: IdcardUtil::get_birth_date(idcard)?.ok_or(IdcardError::InvalidCard)?,
            gender: IdcardUtil::get_gender_by_id_card(idcard)?,
            age: IdcardUtil::get_age_by_id_card_at(idcard, comparison)?,
        })
    }

    /// Returns the province code.
    #[must_use]
    pub fn province_code(&self) -> Option<&str> {
        self.province_code.as_deref()
    }

    /// Returns the province name.
    #[must_use]
    pub fn province(&self) -> Option<&'static str> {
        self.province_code.as_deref().and_then(province_name)
    }

    /// Returns the city-level code.
    #[must_use]
    pub fn city_code(&self) -> Option<&str> {
        self.city_code.as_deref()
    }

    /// Returns the birth date.
    #[must_use]
    pub const fn birth_date(&self) -> NaiveDate {
        self.birth_date
    }

    /// Returns `1` for male and `0` for female.
    #[must_use]
    pub const fn gender(&self) -> u8 {
        self.gender
    }

    /// Returns the age captured when this value was built.
    #[must_use]
    pub const fn age(&self) -> i32 {
        self.age
    }
}

impl fmt::Display for Idcard {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Idcard{{provinceCode='{}', cityCode='{}', birthDate={} 00:00:00, gender={}, age={}}}",
            self.province_code.as_deref().unwrap_or("null"),
            self.city_code.as_deref().unwrap_or("null"),
            self.birth_date.format("%Y-%m-%d"),
            self.gender,
            self.age
        )
    }
}

use super::{CHECK_CODES, CHINA_ID_MAX_LENGTH, CHINA_ID_MIN_LENGTH, CITY_CODES, POWER, TW_FIRST_CODES, check_code_18, compact_parenthesized_card};
use super::{is_blank, parse_birth, parse_birth_component, parse_two_digits, prefix_for_supported_length, province_name, tw_first_code, weighted_check_code};
