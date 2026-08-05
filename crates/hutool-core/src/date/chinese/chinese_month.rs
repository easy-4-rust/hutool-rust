//! 对齐: `cn.hutool.core.date.chinese.ChineseMonth`

#![allow(dead_code)]

use crate::Result;
use crate::date::chinese::lunar_info::LunarInfo;

const MONTH_NAME: &[&str] = &[
    "一", "二", "三", "四", "五", "六", "七", "八", "九", "十", "十一", "十二",
];
const MONTH_NAME_TRADITIONAL: &[&str] = &[
    "正", "二", "三", "四", "五", "六", "七", "八", "九", "寒", "冬", "腊",
];

/// 对齐 Java: `cn.hutool.core.date.chinese.ChineseMonth`
#[derive(Debug, Clone, Copy, Default)]
pub struct ChineseMonth;

impl ChineseMonth {
    /// 是否闰月。
    pub fn is_leap_month(year: i32, month: i32) -> bool {
        month == LunarInfo::leap_month(year)
    }

    /// 农历月中文名。
    pub fn get_chinese_month_name(is_leap_month: bool, month: i32, is_traditional: bool) -> String {
        let names = if is_traditional {
            MONTH_NAME_TRADITIONAL
        } else {
            MONTH_NAME
        };
        format!(
            "{}{}月",
            if is_leap_month { "闰" } else { "" },
            names[(month - 1) as usize]
        )
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
