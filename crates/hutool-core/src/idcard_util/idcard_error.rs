use std::fmt;

use chrono::{Datelike, Local, NaiveDate};
use thiserror::Error;

/// Errors returned by identity-card accessors that Java Hutool exposes through exceptions.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IdcardError {
    /// A required identity-card value was blank.
    #[error("identity card must not be blank")]
    Blank,
    /// A value cannot be interpreted as the requested identity-card representation.
    #[error("invalid identity card")]
    InvalidCard,
    /// The embedded birthday is not a real Gregorian date.
    #[error("invalid identity-card birthday: {0}")]
    InvalidBirthDate(String),
    /// Age cannot be calculated before the embedded birthday.
    #[error("comparison date precedes identity-card birthday")]
    BirthAfterComparison,
}

use super::{CHECK_CODES, CHINA_ID_MAX_LENGTH, CHINA_ID_MIN_LENGTH, CITY_CODES, POWER, TW_FIRST_CODES, check_code_18, compact_parenthesized_card};
use super::{is_blank, parse_birth, parse_birth_component, parse_two_digits, prefix_for_supported_length, province_name, tw_first_code, weighted_check_code};
