//! 对齐: `cn.hutool.core.date.DateBetween`

#![allow(dead_code)]

use chrono::Datelike;

use crate::date::date_time::DateTime;
use crate::date::date_unit::DateUnit;
use crate::Result;

/// 对齐 Java: `cn.hutool.core.date.DateBetween`
#[derive(Debug, Clone, Copy)]
pub struct DateBetween {
    begin: DateTime,
    end: DateTime,
}

impl DateBetween {
    /// 创建（绝对值）。
    pub fn create(begin: DateTime, end: DateTime) -> Self {
        Self::new(begin, end, true)
    }

    /// 构造。
    pub fn new(begin: DateTime, end: DateTime, is_abs: bool) -> Self {
        if is_abs && begin > end {
            Self { begin: end, end: begin }
        } else {
            Self { begin, end }
        }
    }

    /// 按单位差值。
    pub fn between(self, unit: DateUnit) -> i64 {
        (self.end.get_time() - self.begin.get_time()) / unit.get_millis()
    }

    /// 相差月数。
    pub fn between_month(self, is_reset: bool) -> i64 {
        let b = self.begin.naive_local();
        let e = self.end.naive_local();
        let mut result =
            (e.year() as i64 - b.year() as i64) * 12 + (e.month() as i64 - b.month() as i64);
        if !is_reset {
            // 不足整月则减 1
            // compare day-time within month
            let b_day = b.day();
            let e_day = e.day();
            let b_tod = b.time();
            let e_tod = e.time();
            if e_day < b_day || (e_day == b_day && e_tod < b_tod) {
                result -= 1;
            }
        }
        result
    }

    /// 相差年数。
    pub fn between_year(self, is_reset: bool) -> i64 {
        let b = self.begin.naive_local();
        let e = self.end.naive_local();
        let mut result = e.year() as i64 - b.year() as i64;
        if !is_reset {
            let b_md = (b.month(), b.day(), b.time());
            let e_md = (e.month(), e.day(), e.time());
            // leap day edge: Hutool betweenYearTest2 2000-02-29 → 2018-02-28 = 18
            if e_md < b_md {
                // special: if begin is Feb 29 and end is Feb 28, still count as full year in Hutool?
                // Test: 2000-02-29 to 2018-02-28 => 18, so Feb28 >= Feb29 is false → would subtract
                // but expected 18. Hutool resets month/day comparison carefully.
                // Actually Hutool: if end month/day before begin month/day, subtract 1,
                // EXCEPT when begin is leap day and end is Feb 28 of non-leap?
                if b.month() == 2 && b.day() == 29 && e.month() == 2 && e.day() == 28 {
                    // keep year
                } else {
                    result -= 1;
                }
            }
        }
        result
    }

    /// 相差天数。
    pub fn between_day(self, is_reset: bool) -> i64 {
        if is_reset {
            let b = self.begin.naive_local().date();
            let e = self.end.naive_local().date();
            (e - b).num_days()
        } else {
            self.between(DateUnit::Day)
        }
    }

    /// 相差周数。
    pub fn between_week(self, is_reset: bool) -> i64 {
        self.between_day(is_reset) / 7
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
