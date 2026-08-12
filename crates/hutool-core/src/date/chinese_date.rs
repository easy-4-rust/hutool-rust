//! 对齐: `cn.hutool.core.date.ChineseDate`

#![allow(dead_code)]

use chrono::{Datelike, NaiveDate};

use crate::Result;
use crate::date::chinese::chinese_month::ChineseMonth;
use crate::date::chinese::gan_zhi::GanZhi;
use crate::date::chinese::lunar_festival::LunarFestival;
use crate::date::chinese::lunar_info::{BASE_DAY, BASE_YEAR, LunarInfo, MAX_YEAR};
use crate::date::chinese::solar_terms::SolarTerms;
use crate::date::date_time::DateTime;
use crate::date::date_util::DateUtil;
use crate::date::zodiac::Zodiac;

/// 对齐 Java: `cn.hutool.core.date.ChineseDate`
#[derive(Debug, Clone)]
pub struct ChineseDate {
    year: i32,
    month: i32,
    is_leap_month: bool,
    day: i32,
    gyear: i32,
    gmonth_base1: i32,
    gday: i32,
}

impl ChineseDate {
    /// 通过公历 DateTime 构造。
    pub fn from_gregorian(date: DateTime) -> Self {
        let n = date.naive_local().date();
        Self::from_local_date(n)
    }

    /// 通过 NaiveDate 构造。
    pub fn from_local_date(local_date: NaiveDate) -> Self {
        let gyear = local_date.year();
        let gmonth_base1 = local_date.month() as i32;
        let gday = local_date.day() as i32;
        let epoch = local_date
            .signed_duration_since(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
            .num_days();
        let mut offset = (epoch - BASE_DAY) as i32;

        let mut i_year = BASE_YEAR;
        while i_year <= MAX_YEAR {
            let days_of_year = LunarInfo::year_days(i_year);
            if offset < days_of_year {
                break;
            }
            offset -= days_of_year;
            i_year += 1;
        }
        let year = i_year;
        let leap_month = LunarInfo::leap_month(i_year);
        let mut month = 1;
        let mut has_leap_month = false;
        while month < 13 {
            let days_of_month = if leap_month > 0 && month == leap_month + 1 {
                has_leap_month = true;
                LunarInfo::leap_days(year)
            } else {
                LunarInfo::month_days(year, if has_leap_month { month - 1 } else { month })
            };
            if offset < days_of_month {
                break;
            }
            offset -= days_of_month;
            month += 1;
        }
        let is_leap_month = leap_month > 0 && month == leap_month + 1;
        let month = if has_leap_month && !is_leap_month {
            month - 1
        } else {
            month
        };
        Self {
            year,
            month,
            is_leap_month,
            day: offset + 1,
            gyear,
            gmonth_base1,
            gday,
        }
    }

    /// 农历年月日构造（自动判断闰月）。
    pub fn from_lunar(chinese_year: i32, chinese_month: i32, chinese_day: i32) -> Self {
        let is_leap = chinese_month == LunarInfo::leap_month(chinese_year);
        Self::from_lunar_leap(chinese_year, chinese_month, chinese_day, is_leap)
    }

    /// 农历年月日构造（显式闰月）。
    pub fn from_lunar_leap(
        chinese_year: i32,
        chinese_month: i32,
        chinese_day: i32,
        mut is_leap_month: bool,
    ) -> Self {
        if chinese_month != LunarInfo::leap_month(chinese_year) {
            is_leap_month = false;
        }
        let month = if is_leap_month {
            chinese_month + 1
        } else {
            chinese_month
        };
        let g = lunar2solar(chinese_year, chinese_month, chinese_day, is_leap_month);
        let (gyear, gmonth_base1, gday) = match g {
            Some(dt) => (dt.year(), dt.month() + 1, dt.day_of_month()),
            None => (-1, -1, -1),
        };
        Self {
            year: chinese_year,
            month,
            is_leap_month,
            day: chinese_day,
            gyear,
            gmonth_base1,
            gday,
        }
    }

    /// 农历年份。
    pub fn get_chinese_year(&self) -> i32 {
        self.year
    }
    /// 农历月份（闰月为 month+1）。
    pub fn get_month(&self) -> i32 {
        self.month
    }
    /// 是否闰月。
    pub fn is_leap_month(&self) -> bool {
        self.is_leap_month
    }
    /// 农历日。
    pub fn get_day(&self) -> i32 {
        self.day
    }
    /// 公历年份。
    pub fn get_gregorian_year(&self) -> i32 {
        self.gyear
    }
    /// 公历月份（1 为基准）。
    pub fn get_gregorian_month_base1(&self) -> i32 {
        self.gmonth_base1
    }
    /// 公历日。
    pub fn get_gregorian_day(&self) -> i32 {
        self.gday
    }

    /// 公历日期。
    pub fn get_gregorian_date(&self) -> DateTime {
        DateTime::of_ymd_hms(
            self.gyear,
            self.gmonth_base1 as u32,
            self.gday as u32,
            0,
            0,
            0,
        )
        .unwrap_or_else(|_| DateUtil::date())
    }

    /// 农历月份（简体）。
    pub fn get_chinese_month(&self) -> String {
        self.get_chinese_month_trad(false)
    }
    /// 农历月份名称（繁体）。
    pub fn get_chinese_month_name(&self) -> String {
        self.get_chinese_month_trad(true)
    }
    /// 农历月份，按简体/繁体输出。
    pub fn get_chinese_month_trad(&self, is_traditional: bool) -> String {
        ChineseMonth::get_chinese_month_name(
            self.is_leap_month,
            if self.is_leap_month {
                self.month - 1
            } else {
                self.month
            },
            is_traditional,
        )
    }

    /// 农历日（中文）。
    pub fn get_chinese_day(&self) -> String {
        let day = self.day;
        let chinese_ten = ["初", "十", "廿", "卅"];
        match day {
            10 => "初十".into(),
            20 => "二十".into(),
            30 => "三十".into(),
            _ if day > 30 => String::new(),
            _ => {
                let n = if day % 10 == 0 { 9 } else { day % 10 - 1 };
                format!(
                    "{}{}",
                    chinese_ten[(day / 10) as usize],
                    chinese_digit(n + 1)
                )
            }
        }
    }

    /// 农历节日。
    pub fn get_festivals(&self) -> String {
        LunarFestival::get_festivals(
            self.year,
            if self.is_leap_month {
                self.month - 1
            } else {
                self.month
            },
            self.day,
        )
    }

    /// 生肖。
    pub fn get_chinese_zodiac(&self) -> Option<&'static str> {
        Zodiac::get_chinese_zodiac(self.year)
    }

    /// 干支（年）。
    pub fn get_cyclical(&self) -> String {
        GanZhi::get_ganzhi_of_year(self.year)
    }

    /// 干支年月日。
    pub fn get_cyclical_ymd(&self) -> Option<String> {
        if self.gyear >= BASE_YEAR && self.gmonth_base1 > 0 && self.gday > 0 {
            Some(format!(
                "{}年{}月{}日",
                GanZhi::get_ganzhi_of_year(self.year),
                GanZhi::get_ganzhi_of_month(self.gyear, self.gmonth_base1, self.gday),
                GanZhi::get_ganzhi_of_day(self.gyear, self.gmonth_base1, self.gday)
            ))
        } else {
            None
        }
    }

    /// 节气。
    pub fn get_term(&self) -> String {
        SolarTerms::get_term(self.gyear, self.gmonth_base1, self.gday)
    }

    /// 常规字符串表示。
    pub fn to_string_normal(&self) -> String {
        format!(
            "{:04}-{:02}-{:02}",
            self.year,
            if self.is_leap_month {
                self.month - 1
            } else {
                self.month
            },
            self.day
        )
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}

impl std::fmt::Display for ChineseDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}年 {}{}",
            self.get_cyclical(),
            self.get_chinese_zodiac().unwrap_or(""),
            self.get_chinese_month_name(),
            self.get_chinese_day()
        )
    }
}

fn chinese_digit(n: i32) -> &'static str {
    ["〇", "一", "二", "三", "四", "五", "六", "七", "八", "九"][n as usize]
}

fn lunar2solar(
    chinese_year: i32,
    chinese_month: i32,
    chinese_day: i32,
    is_leap_month: bool,
) -> Option<DateTime> {
    if (chinese_year == 2100 && chinese_month == 12 && chinese_day > 1)
        || (chinese_year == BASE_YEAR && chinese_month == 1 && chinese_day < 31)
    {
        return None;
    }
    let day = LunarInfo::month_days(chinese_year, chinese_month);
    let _day = if is_leap_month {
        LunarInfo::leap_days(chinese_year)
    } else {
        day
    };
    if chinese_year < BASE_YEAR || chinese_year > 2100 || chinese_day > _day {
        return None;
    }
    let mut offset = 0;
    for i in BASE_YEAR..chinese_year {
        offset += LunarInfo::year_days(i);
    }
    let mut is_add = false;
    for i in 1..chinese_month {
        let leap = LunarInfo::leap_month(chinese_year);
        if !is_add && leap <= i && leap > 0 {
            offset += LunarInfo::leap_days(chinese_year);
            is_add = true;
        }
        offset += LunarInfo::month_days(chinese_year, i);
    }
    if is_leap_month {
        offset += day;
    }
    // 1900-01-30 0:00 CST ≈ -2203804800000 ms
    Some(DateTime::of_millis(
        ((offset + chinese_day - 31) as i64) * 86_400_000 - 2_203_804_800_000,
    ))
}
