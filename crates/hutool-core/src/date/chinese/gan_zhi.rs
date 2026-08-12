//! 对齐: `cn.hutool.core.date.chinese.GanZhi`

#![allow(dead_code)]

use chrono::NaiveDate;

use crate::Result;
use crate::date::chinese::lunar_info::{BASE_DAY, BASE_YEAR, LunarInfo};
use crate::date::chinese::solar_terms::SolarTerms;

const GAN: &[&str] = &["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
const ZHI: &[&str] = &[
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];

/// 对齐 Java: `cn.hutool.core.date.chinese.GanZhi`
#[derive(Debug, Clone, Copy, Default)]
pub struct GanZhi;

impl GanZhi {
    /// offset → 干支。
    pub fn cyclicalm(num: i32) -> String {
        let n = num.rem_euclid(60);
        format!("{}{}", GAN[(n % 10) as usize], ZHI[(n % 12) as usize])
    }

    /// 农历年干支。
    pub fn get_ganzhi_of_year(year: i32) -> String {
        Self::cyclicalm(year - BASE_YEAR + 36)
    }

    /// 公历月干支（month 从 1 起）。
    pub fn get_ganzhi_of_month(year: i32, month: i32, day: i32) -> String {
        let first_node = SolarTerms::get_term_day(year, month * 2 - 1);
        let mut month_offset = (year - BASE_YEAR) * 12 + month + 11;
        if day >= first_node {
            month_offset += 1;
        }
        Self::cyclicalm(month_offset)
    }

    /// 公历日干支。
    pub fn get_ganzhi_of_day(year: i32, month: i32, day: i32) -> String {
        let days = NaiveDate::from_ymd_opt(year, month as u32, day as u32)
            .unwrap()
            .signed_duration_since(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
            .num_days()
            - 1;
        Self::cyclicalm((days - BASE_DAY + 41) as i32)
    }

    /// 年月日干支串。
    pub fn get_cyclical_ymd(year: i32, month: i32, day: i32) -> String {
        // 使用农历年干支需 ChineseDate；此处提供公历接口的年月日干支
        format!(
            "{}年{}月{}日",
            Self::get_ganzhi_of_year(year),
            Self::get_ganzhi_of_month(year, month, day),
            Self::get_ganzhi_of_day(year, month, day)
        )
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        let _ = LunarInfo::leap_month(2020);
        Ok(())
    }
}
