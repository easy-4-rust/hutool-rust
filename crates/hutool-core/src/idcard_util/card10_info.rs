use std::fmt;

use chrono::{Datelike, Local, NaiveDate};
use thiserror::Error;

/// Parsed information returned for ten-character Taiwan, Macao, and Hong Kong cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card10Info {
    pub(crate) region: &'static str,
    pub(crate) gender: char,
    pub(crate) valid: bool,
}

impl Card10Info {
    /// Returns the card's region name.
    #[must_use]
    pub const fn region(self) -> &'static str {
        self.region
    }

    /// Returns `M`, `F`, or `N`, matching Hutool's information array.
    #[must_use]
    pub const fn gender(self) -> char {
        self.gender
    }

    /// Returns whether the regional checksum or syntax is valid.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.valid
    }
}

use super::{CHECK_CODES, CHINA_ID_MAX_LENGTH, CHINA_ID_MIN_LENGTH, CITY_CODES, POWER, TW_FIRST_CODES, check_code_18, compact_parenthesized_card};
use super::{is_blank, parse_birth, parse_birth_component, parse_two_digits, prefix_for_supported_length, province_name, tw_first_code, weighted_check_code};
