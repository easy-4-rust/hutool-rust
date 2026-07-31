//! 对齐: `cn.hutool.core.date.Month`

#![allow(dead_code)]

use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.date.Month`（Calendar 值，0=一月）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Month {
    /// 一月
    January = 0,
    /// 二月
    February = 1,
    /// 三月
    March = 2,
    /// 四月
    April = 3,
    /// 五月
    May = 4,
    /// 六月
    June = 5,
    /// 七月
    July = 6,
    /// 八月
    August = 7,
    /// 九月
    September = 8,
    /// 十月
    October = 9,
    /// 十一月
    November = 10,
    /// 十二月
    December = 11,
    /// 十三月（农历专用）
    Undecimber = 12,
}

const ALIASES: &[&str] = &[
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];

impl Month {
    /// 对齐 Java: `Month.of(int)` — Calendar 月值（0-12）。
    pub fn of_value(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::January),
            1 => Some(Self::February),
            2 => Some(Self::March),
            3 => Some(Self::April),
            4 => Some(Self::May),
            5 => Some(Self::June),
            6 => Some(Self::July),
            7 => Some(Self::August),
            8 => Some(Self::September),
            9 => Some(Self::October),
            10 => Some(Self::November),
            11 => Some(Self::December),
            12 => Some(Self::Undecimber),
            _ => None,
        }
    }

    /// 对齐 Java: `Month.of(String)` / `of(java.time.Month)`。
    pub fn of(name: &str) -> Option<Self> {
        let lower = name.to_ascii_lowercase();
        if let Some(idx) = ALIASES.iter().position(|a| *a == lower) {
            return Self::of_value(idx as i32);
        }
        match lower.as_str() {
            "january" => Some(Self::January),
            "february" => Some(Self::February),
            "march" => Some(Self::March),
            "april" => Some(Self::April),
            "may" => Some(Self::May),
            "june" => Some(Self::June),
            "july" => Some(Self::July),
            "august" => Some(Self::August),
            "september" => Some(Self::September),
            "october" => Some(Self::October),
            "november" => Some(Self::November),
            "december" => Some(Self::December),
            "undecimber" => Some(Self::Undecimber),
            _ => None,
        }
    }

    /// 从 chrono Month（1-12）构造。
    pub fn of_chrono(m: chrono::Month) -> Self {
        Self::of_value((m.number_from_month() as i32) - 1).unwrap_or(Self::January)
    }

    /// Calendar 值（从 0 开始）。
    pub fn get_value(self) -> i32 {
        self as i32
    }

    /// 对齐 Java: `getValueBaseOne()` — 1=一月。
    pub fn get_value_base_one(self) -> i32 {
        if self == Self::Undecimber {
            13
        } else {
            self.get_value() + 1
        }
    }

    /// 对齐 Java: `getLastDay(boolean isLeapYear)`。
    pub fn get_last_day(self, is_leap_year: bool) -> i32 {
        match self {
            Self::January | Self::March | Self::May | Self::July | Self::August | Self::October
            | Self::December => 31,
            Self::April | Self::June | Self::September | Self::November => 30,
            Self::February => {
                if is_leap_year {
                    29
                } else {
                    28
                }
            }
            Self::Undecimber => 0,
        }
    }

    /// 对齐 Java: `toJdkMonth()`。
    pub fn to_jdk_month(self) -> Result<chrono::Month> {
        chrono::Month::try_from(self.get_value_base_one() as u8).map_err(|_| {
            CoreError::InvalidArgument {
                name: "month",
                reason: "UNDECIMBER cannot convert to java.time.Month",
            }
        })
    }

    /// 对齐 Java: `getDisplayName(TextStyle, Locale)` — 仅支持 SHORT / US。
    pub fn get_display_name(self, style: &str, locale: &str) -> String {
        let full = match self {
            Self::January => "January",
            Self::February => "February",
            Self::March => "March",
            Self::April => "April",
            Self::May => "May",
            Self::June => "June",
            Self::July => "July",
            Self::August => "August",
            Self::September => "September",
            Self::October => "October",
            Self::November => "November",
            Self::December => "December",
            Self::Undecimber => return String::new(),
        };
        if style.eq_ignore_ascii_case("SHORT") {
            let _ = locale;
            full[..3].to_string()
        } else {
            full.to_string()
        }
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
