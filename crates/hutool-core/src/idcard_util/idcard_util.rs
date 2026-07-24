use std::fmt;

use chrono::{Datelike, Local, NaiveDate};
use thiserror::Error;

use super::card10_info::Card10Info;
use super::idcard::Idcard;
use super::idcard_error::IdcardError;

/// Hutool-compatible identity-card facade backed by checked Rust algorithms.
pub struct IdcardUtil;

impl IdcardUtil {
    /// Converts a numeric legacy 15-character mainland card into an 18-character card.
    #[must_use]
    pub fn convert_15_to_18(idcard: &str) -> Option<String> {
        let chars: Vec<char> = idcard.chars().collect();
        if chars.len() != CHINA_ID_MIN_LENGTH || !chars.iter().all(char::is_ascii_digit) {
            return None;
        }
        let year = parse_two_digits(&chars[6..8]);
        // Fifteen-character cards predate 2000. Hutool corrects a parsed 20xx
        // year by subtracting one century, while exactly `00` remains 2000.
        let full_year = if year == 0 { 2000 } else { 1900 + year };
        let mut code17 = chars[..6].iter().collect::<String>();
        code17.push_str(&full_year.to_string());
        code17.extend(chars[8..].iter());
        let check = weighted_check_code(code17.as_bytes());
        code17.push(check);
        Some(code17)
    }

    /// Converts a valid 18-character card to the legacy 15-character form.
    #[must_use]
    pub fn convert_18_to_15(idcard: &str) -> String {
        if !Self::is_valid_card_18(idcard) {
            return idcard.to_owned();
        }
        let chars: Vec<char> = idcard.chars().collect();
        chars[..6].iter().chain(chars[8..17].iter()).collect()
    }

    /// Validates mainland 15/18-character cards and regional 10-character cards.
    #[must_use]
    pub fn is_valid_card(idcard: &str) -> bool {
        if is_blank(idcard) {
            return false;
        }
        match idcard.chars().count() {
            CHINA_ID_MAX_LENGTH => Self::is_valid_card_18(idcard),
            CHINA_ID_MIN_LENGTH => Self::is_valid_card_15(idcard),
            10 => Self::is_valid_card_10(idcard).is_some_and(Card10Info::is_valid),
            _ => false,
        }
    }

    /// Validates an 18-character card while ignoring a checksum `X`'s case.
    #[must_use]
    pub fn is_valid_card_18(idcard: &str) -> bool {
        Self::is_valid_card_18_case(idcard, true)
    }

    /// Validates an 18-character card with explicit checksum case handling.
    #[must_use]
    pub fn is_valid_card_18_case(idcard: &str, ignore_case: bool) -> bool {
        if !idcard.is_ascii() || idcard.len() != CHINA_ID_MAX_LENGTH {
            return false;
        }
        let bytes = idcard.as_bytes();
        let province = if bytes[0] == b'9' {
            &idcard[1..3]
        } else {
            &idcard[..2]
        };
        if province_name(province).is_none() || parse_birth(&idcard[6..14]).is_none() {
            return false;
        }
        let Some(expected) = check_code_18(&bytes[..17]) else {
            return false;
        };
        let actual = char::from(bytes[17]);
        if ignore_case {
            expected.eq_ignore_ascii_case(&actual)
        } else {
            expected == actual
        }
    }

    /// Validates a numeric legacy 15-character mainland card.
    #[must_use]
    pub fn is_valid_card_15(idcard: &str) -> bool {
        if !idcard.is_ascii() || idcard.len() != CHINA_ID_MIN_LENGTH {
            return false;
        }
        idcard.bytes().all(|value| value.is_ascii_digit())
            && province_name(&idcard[..2]).is_some()
            && parse_birth(&format!("19{}", &idcard[6..12])).is_some()
    }

    /// Parses and validates Taiwan, Macao, or Hong Kong card information.
    #[must_use]
    pub fn is_valid_card_10(idcard: &str) -> Option<Card10Info> {
        if is_blank(idcard) {
            return None;
        }
        let normalized = idcard.replace('（', "(").replace('）', ")");
        let compact = compact_parenthesized_card(&normalized)?;
        let bytes = compact.as_bytes();
        if compact.len() == 10
            && bytes[0].is_ascii_alphabetic()
            && bytes[1..].iter().all(u8::is_ascii_digit)
        {
            let gender = match bytes[1] {
                b'1' => 'M',
                b'2' => 'F',
                _ => 'N',
            };
            let valid = gender != 'N' && Self::is_valid_tw_card(&compact);
            return Some(Card10Info {
                region: "台湾",
                gender,
                valid,
            });
        }
        if compact.len() == 8
            && matches!(bytes[0], b'1' | b'5' | b'7')
            && bytes[1..7].iter().all(u8::is_ascii_digit)
            && (bytes[7].is_ascii_digit() || bytes[7].is_ascii_uppercase())
        {
            return Some(Card10Info {
                region: "澳门",
                gender: 'N',
                valid: true,
            });
        }
        let letters = compact.len().saturating_sub(7);
        if matches!(letters, 1 | 2)
            && bytes[..letters].iter().all(u8::is_ascii_uppercase)
            && bytes[letters..letters + 6].iter().all(u8::is_ascii_digit)
            && matches!(bytes[letters + 6], b'0'..=b'9' | b'A')
        {
            return Some(Card10Info {
                region: "香港",
                gender: 'N',
                valid: Self::is_valid_hk_card(&normalized),
            });
        }
        None
    }

    /// Validates a Taiwan identity-card checksum.
    #[must_use]
    pub fn is_valid_tw_card(idcard: &str) -> bool {
        if !idcard.is_ascii() || idcard.len() != 10 {
            return false;
        }
        let bytes = idcard.as_bytes();
        let Some(first) = tw_first_code(char::from(bytes[0])) else {
            return false;
        };
        if !bytes[1..].iter().all(u8::is_ascii_digit) {
            return false;
        }
        let mut sum = first / 10 + (first % 10) * 9;
        for (value, weight) in bytes[1..9].iter().zip((1_u32..=8).rev()) {
            sum += u32::from(value - b'0') * weight;
        }
        let expected = if sum % 10 == 0 { 0 } else { 10 - sum % 10 };
        expected == u32::from(bytes[9] - b'0')
    }

    /// Validates a Hong Kong identity-card checksum.
    #[must_use]
    pub fn is_valid_hk_card(idcard: &str) -> bool {
        if !idcard.is_ascii() {
            return false;
        }
        let Some(compact) = compact_parenthesized_card(idcard) else {
            return false;
        };
        let bytes = compact.as_bytes();
        let letters = compact.len().saturating_sub(7);
        if !matches!(letters, 1 | 2)
            || !bytes[..letters].iter().all(u8::is_ascii_uppercase)
            || !bytes[letters..letters + 6].iter().all(u8::is_ascii_digit)
            || !matches!(bytes[letters + 6], b'0'..=b'9' | b'A')
        {
            return false;
        }
        let letter_value = |value: u8| u32::from(value - b'A' + 10);
        let mut sum = if letters == 2 {
            letter_value(bytes[0]) * 9 + letter_value(bytes[1]) * 8
        } else {
            522 + letter_value(bytes[0]) * 8
        };
        for (value, weight) in bytes[letters..letters + 6].iter().zip((2_u32..=7).rev()) {
            sum += u32::from(value - b'0') * weight;
        }
        sum += match bytes[letters + 6] {
            b'A' => 10,
            value => u32::from(value - b'0'),
        };
        sum % 11 == 0
    }

    /// Returns the embedded birthday as `yyyyMMdd`.
    pub fn get_birth_by_id_card(idcard: &str) -> Result<Option<String>, IdcardError> {
        Self::get_birth(idcard)
    }

    /// Returns the embedded birthday as `yyyyMMdd`.
    pub fn get_birth(idcard: &str) -> Result<Option<String>, IdcardError> {
        if is_blank(idcard) {
            return Err(IdcardError::Blank);
        }
        let chars: Vec<char> = idcard.chars().collect();
        if chars.len() < CHINA_ID_MIN_LENGTH {
            return Ok(None);
        }
        let expanded;
        let chars = if chars.len() == CHINA_ID_MIN_LENGTH {
            expanded = Self::convert_15_to_18(idcard).ok_or(IdcardError::InvalidCard)?;
            expanded.chars().collect::<Vec<_>>()
        } else {
            chars
        };
        Ok(Some(chars[6..14].iter().collect()))
    }

    /// Returns the embedded birthday as a checked Gregorian date.
    pub fn get_birth_date(idcard: &str) -> Result<Option<NaiveDate>, IdcardError> {
        let Some(birth) = Self::get_birth(idcard)? else {
            return Ok(None);
        };
        parse_birth(&birth)
            .map(Some)
            .ok_or(IdcardError::InvalidBirthDate(birth))
    }

    /// Calculates age at the machine's current local date.
    pub fn get_age_by_id_card(idcard: &str) -> Result<i32, IdcardError> {
        Self::get_age_by_id_card_at(idcard, Local::now().date_naive())
    }

    /// Calculates age at a caller-supplied comparison date.
    pub fn get_age_by_id_card_at(idcard: &str, comparison: NaiveDate) -> Result<i32, IdcardError> {
        let birth = Self::get_birth_date(idcard)?.ok_or(IdcardError::InvalidCard)?;
        if comparison < birth {
            return Err(IdcardError::BirthAfterComparison);
        }
        // 对齐 Hutool `CalendarUtil.age(long, long)`:
        // 1. 月份小于生日月份 → 年龄减 1
        // 2. 月份相等且日小于等于生日当天 → 年龄减 1
        //    注: Hutool issue#I6E6ZG,法定生日当天不算年龄,从第二天开始计算,
        //    即生日当天 dayOfMonth <= dayOfMonthBirth 仍减 1。
        let comp_md = (comparison.month(), comparison.day());
        let birth_md = (birth.month(), birth.day());
        let not_yet_birthday_this_year = match comp_md.0.cmp(&birth_md.0) {
            std::cmp::Ordering::Less => true,
            std::cmp::Ordering::Equal => comp_md.1 <= birth_md.1,
            std::cmp::Ordering::Greater => false,
        };
        Ok(comparison.year() - birth.year() - i32::from(not_yet_birthday_this_year))
    }

    /// Returns the embedded four-digit birth year.
    pub fn get_year_by_id_card(idcard: &str) -> Result<Option<i16>, IdcardError> {
        parse_birth_component(idcard, 0..4)
    }

    /// Returns the embedded two-digit birth month.
    pub fn get_month_by_id_card(idcard: &str) -> Result<Option<i16>, IdcardError> {
        parse_birth_component(idcard, 4..6)
    }

    /// Returns the embedded two-digit birth day.
    pub fn get_day_by_id_card(idcard: &str) -> Result<Option<i16>, IdcardError> {
        parse_birth_component(idcard, 6..8)
    }

    /// Returns `1` for male and `0` for female using the sequence-code parity.
    pub fn get_gender_by_id_card(idcard: &str) -> Result<u8, IdcardError> {
        if is_blank(idcard) {
            return Err(IdcardError::Blank);
        }
        let mut chars: Vec<char> = idcard.chars().collect();
        if chars.len() == CHINA_ID_MIN_LENGTH {
            chars = Self::convert_15_to_18(idcard)
                .ok_or(IdcardError::InvalidCard)?
                .chars()
                .collect();
        } else if chars.len() != CHINA_ID_MAX_LENGTH {
            return Err(IdcardError::InvalidCard);
        }
        Ok(u8::from(u32::from(chars[16]) % 2 != 0))
    }

    /// Returns the two-character province code for 15/18-character values.
    #[must_use]
    pub fn get_province_code_by_id_card(idcard: &str) -> Option<String> {
        prefix_for_supported_length(idcard, 2)
    }

    /// Returns the province name for a known province code.
    #[must_use]
    pub fn get_province_by_id_card(idcard: &str) -> Option<&'static str> {
        let code = Self::get_province_code_by_id_card(idcard)?;
        province_name(&code)
    }

    /// Returns the four-character city-level code for 15/18-character values.
    #[must_use]
    pub fn get_city_code_by_id_card(idcard: &str) -> Option<String> {
        prefix_for_supported_length(idcard, 4)
    }

    /// Returns the six-character district-level code for 15/18-character values.
    #[must_use]
    pub fn get_district_code_by_id_card(idcard: &str) -> Option<String> {
        prefix_for_supported_length(idcard, 6)
    }

    /// Replaces the requested Unicode scalar-value interval with asterisks.
    #[must_use]
    pub fn hide(idcard: &str, start_include: i32, end_exclude: i32) -> String {
        let length = i64::try_from(idcard.chars().count()).unwrap_or(i64::MAX);
        let start_include = i64::from(start_include);
        let end_exclude = i64::from(end_exclude);
        if start_include > length || start_include > end_exclude {
            return idcard.to_owned();
        }
        let end_exclude = end_exclude.min(length);
        idcard
            .chars()
            .enumerate()
            .map(|(index, value)| {
                let index = i64::try_from(index).unwrap_or(i64::MAX);
                if index >= start_include && index < end_exclude {
                    '*'
                } else {
                    value
                }
            })
            .collect()
    }

    /// Builds the complete identity-card information value using the current local date.
    pub fn get_idcard_info(idcard: &str) -> Result<Idcard, IdcardError> {
        Idcard::new(idcard)
    }
}

use super::{CHECK_CODES, CHINA_ID_MAX_LENGTH, CHINA_ID_MIN_LENGTH, CITY_CODES, POWER, TW_FIRST_CODES, check_code_18, compact_parenthesized_card};
use super::{is_blank, parse_birth, parse_birth_component, parse_two_digits, prefix_for_supported_length, province_name, tw_first_code, weighted_check_code};
