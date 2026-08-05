//! 对齐: `cn.hutool.core.date.Zodiac`

#![allow(dead_code)]

use crate::Result;
use crate::date::date_time::DateTime;
use crate::date::month::Month;

const DAY_ARR: [i32; 12] = [20, 19, 21, 20, 21, 22, 23, 23, 23, 24, 23, 22];
const ZODIACS: [&str; 13] = [
    "摩羯座",
    "水瓶座",
    "双鱼座",
    "白羊座",
    "金牛座",
    "双子座",
    "巨蟹座",
    "狮子座",
    "处女座",
    "天秤座",
    "天蝎座",
    "射手座",
    "摩羯座",
];
const CHINESE_ZODIACS: [&str; 12] = [
    "鼠", "牛", "虎", "兔", "龙", "蛇", "马", "羊", "猴", "鸡", "狗", "猪",
];

/// 对齐 Java: `cn.hutool.core.date.Zodiac`
#[derive(Debug, Clone, Copy, Default)]
pub struct Zodiac;

impl Zodiac {
    /// 通过生日计算星座（月从 0 开始）。
    pub fn get_zodiac(month0: i32, day: i32) -> Option<&'static str> {
        if !(0..=11).contains(&month0) {
            return None;
        }
        let idx = month0 as usize;
        Some(if day < DAY_ARR[idx] {
            ZODIACS[idx]
        } else {
            ZODIACS[idx + 1]
        })
    }

    /// 通过 Month 枚举。
    pub fn get_zodiac_month(month: Month, day: i32) -> Option<&'static str> {
        Self::get_zodiac(month.get_value(), day)
    }

    /// 通过 DateTime。
    pub fn get_zodiac_date(date: DateTime) -> Option<&'static str> {
        Self::get_zodiac(date.month(), date.day_of_month())
    }

    /// 生肖（1900 年后）。
    pub fn get_chinese_zodiac(year: i32) -> Option<&'static str> {
        if year < 1900 {
            return None;
        }
        Some(CHINESE_ZODIACS[((year - 1900) % 12) as usize])
    }

    /// 通过 DateTime 取生肖（按公历年）。
    pub fn get_chinese_zodiac_date(date: DateTime) -> Option<&'static str> {
        Self::get_chinese_zodiac(date.year())
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
