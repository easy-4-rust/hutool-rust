//! 对齐: `cn.hutool.core.date.Week`

#![allow(dead_code)]

use chrono::Weekday;

use crate::Result;

/// 对齐 Java: `cn.hutool.core.date.Week`（Calendar：1=周日 … 7=周六）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Week {
    /// 星期日
    Sunday = 1,
    /// 星期一
    Monday = 2,
    /// 星期二
    Tuesday = 3,
    /// 星期三
    Wednesday = 4,
    /// 星期四
    Thursday = 5,
    /// 星期五
    Friday = 6,
    /// 星期六
    Saturday = 7,
}

const ALIASES: &[&str] = &["sun", "mon", "tue", "wed", "thu", "fri", "sat"];
const FULL: &[&str] = &[
    "sunday", "monday", "tuesday", "wednesday", "thursday", "friday", "saturday",
];
const CN: &[&[&str]] = &[
    &["星期日", "周日"],
    &["星期一", "周一"],
    &["星期二", "周二"],
    &["星期三", "周三"],
    &["星期四", "周四"],
    &["星期五", "周五"],
    &["星期六", "周六"],
];

impl Week {
    /// 对齐 Java: `Week.of(int)`。
    pub fn of_value(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Sunday),
            2 => Some(Self::Monday),
            3 => Some(Self::Tuesday),
            4 => Some(Self::Wednesday),
            5 => Some(Self::Thursday),
            6 => Some(Self::Friday),
            7 => Some(Self::Saturday),
            _ => None,
        }
    }

    /// 对齐 Java: `Week.of(String)` / `of(DayOfWeek)`。
    pub fn of(name: &str) -> Option<Self> {
        for (i, names) in CN.iter().enumerate() {
            if names.iter().any(|n| *n == name) {
                return Self::of_value(i as i32 + 1);
            }
        }
        let lower = name.to_ascii_lowercase();
        if let Some(idx) = ALIASES.iter().position(|a| *a == lower) {
            return Self::of_value(idx as i32 + 1);
        }
        if let Some(idx) = FULL.iter().position(|a| *a == lower) {
            return Self::of_value(idx as i32 + 1);
        }
        None
    }

    /// 从 chrono Weekday 构造。
    pub fn of_weekday(w: Weekday) -> Self {
        match w {
            Weekday::Sun => Self::Sunday,
            Weekday::Mon => Self::Monday,
            Weekday::Tue => Self::Tuesday,
            Weekday::Wed => Self::Wednesday,
            Weekday::Thu => Self::Thursday,
            Weekday::Fri => Self::Friday,
            Weekday::Sat => Self::Saturday,
        }
    }

    /// Calendar 值。
    pub fn get_value(self) -> i32 {
        self as i32
    }

    /// 对齐 Java: `toJdkDayOfWeek()`。
    pub fn to_jdk_day_of_week(self) -> Weekday {
        match self {
            Self::Sunday => Weekday::Sun,
            Self::Monday => Weekday::Mon,
            Self::Tuesday => Weekday::Tue,
            Self::Wednesday => Weekday::Wed,
            Self::Thursday => Weekday::Thu,
            Self::Friday => Weekday::Fri,
            Self::Saturday => Weekday::Sat,
        }
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
