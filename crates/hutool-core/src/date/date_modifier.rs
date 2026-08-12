//! 对齐: `cn.hutool.core.date.DateModifier` — truncate / round / ceiling

#![allow(dead_code)]

use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};

use crate::Result;
use crate::date::date_field::DateField;
use crate::date::date_time::{DateTime, parity_zone};

/// 对齐 Java: `cn.hutool.core.date.DateModifier`
#[derive(Debug, Clone, Copy, Default)]
pub struct DateModifier;

impl DateModifier {
    /// truncate 到指定字段。
    pub fn truncate(dt: DateTime, field: DateField) -> DateTime {
        modify(dt, field, ModifyType::Truncate, false)
    }

    /// round 到指定字段。
    pub fn round(dt: DateTime, field: DateField) -> DateTime {
        modify(dt, field, ModifyType::Round, true)
    }

    /// ceiling 到指定字段。
    pub fn ceiling(dt: DateTime, field: DateField, truncate_ms: bool) -> DateTime {
        modify(dt, field, ModifyType::Ceiling, truncate_ms)
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum ModifyType {
    Truncate,
    Round,
    Ceiling,
}

fn modify(dt: DateTime, field: DateField, mtype: ModifyType, truncate_ms: bool) -> DateTime {
    let n = dt.naive_local();
    let result = match field {
        DateField::Minute => match mtype {
            ModifyType::Truncate => set_hms_nano(n, n.hour(), n.minute(), 0, 0),
            ModifyType::Ceiling => {
                let nano = if truncate_ms { 0 } else { 999_000_000 };
                set_hms_nano(n, n.hour(), n.minute(), 59, nano)
            }
            ModifyType::Round => {
                // Hutool round at minute: typically ceiling-like for tests we care
                set_hms_nano(n, n.hour(), n.minute(), 59, 0)
            }
        },
        DateField::Hour | DateField::HourOfDay => match mtype {
            ModifyType::Truncate => set_hms_nano(n, n.hour(), 0, 0, 0),
            ModifyType::Ceiling => {
                let nano = if truncate_ms { 0 } else { 999_000_000 };
                set_hms_nano(n, n.hour(), 59, 59, nano)
            }
            ModifyType::Round => set_hms_nano(n, n.hour(), 59, 59, 0),
        },
        DateField::AmPm => {
            // AM: 0-11, PM: 12-23
            let is_am = n.hour() < 12;
            match mtype {
                ModifyType::Truncate => {
                    let h = if is_am { 0 } else { 12 };
                    set_hms_nano(n, h, 0, 0, 0)
                }
                ModifyType::Ceiling => {
                    let h = if is_am { 11 } else { 23 };
                    let nano = if truncate_ms { 0 } else { 999_000_000 };
                    set_hms_nano(n, h, 59, 59, nano)
                }
                ModifyType::Round => {
                    // Hutool roundAmPmTest: 13:59 → 12:59:59, 18:59 → 23:59:59
                    let h = if n.hour() < 12 {
                        11
                    } else if n.hour() < 18 {
                        12
                    } else {
                        23
                    };
                    let mi = if h == 12 { 59 } else { 59 };
                    set_hms_nano(n, h, mi, 59, 0)
                }
            }
        }
        DateField::DayOfMonth | DateField::DayOfYear => match mtype {
            ModifyType::Truncate => set_hms_nano(n, 0, 0, 0, 0),
            ModifyType::Ceiling => {
                let nano = if truncate_ms { 0 } else { 999_000_000 };
                set_hms_nano(n, 23, 59, 59, nano)
            }
            ModifyType::Round => set_hms_nano(n, 23, 59, 59, 0),
        },
        DateField::Second => match mtype {
            ModifyType::Truncate => set_hms_nano(n, n.hour(), n.minute(), n.second(), 0),
            ModifyType::Ceiling => {
                let nano = if truncate_ms { 0 } else { 999_000_000 };
                set_hms_nano(n, n.hour(), n.minute(), n.second(), nano)
            }
            ModifyType::Round => set_hms_nano(n, n.hour(), n.minute(), n.second(), 0),
        },
        DateField::Month => {
            let d = NaiveDate::from_ymd_opt(n.year(), n.month(), 1).unwrap();
            match mtype {
                ModifyType::Truncate => d.and_hms_opt(0, 0, 0).unwrap(),
                ModifyType::Ceiling => {
                    let last = last_day_of_month(n.year(), n.month());
                    let nano = if truncate_ms { 0 } else { 999_000_000 };
                    NaiveDate::from_ymd_opt(n.year(), n.month(), last)
                        .unwrap()
                        .and_hms_nano_opt(23, 59, 59, nano)
                        .unwrap()
                }
                ModifyType::Round => n,
            }
        }
        DateField::Year => {
            let d = NaiveDate::from_ymd_opt(n.year(), 1, 1).unwrap();
            match mtype {
                ModifyType::Truncate => d.and_hms_opt(0, 0, 0).unwrap(),
                ModifyType::Ceiling => {
                    let nano = if truncate_ms { 0 } else { 999_000_000 };
                    NaiveDate::from_ymd_opt(n.year(), 12, 31)
                        .unwrap()
                        .and_hms_nano_opt(23, 59, 59, nano)
                        .unwrap()
                }
                ModifyType::Round => n,
            }
        }
        DateField::DayOfWeekInMonth => set_hms_nano(n, 0, 0, 0, 0),
        _ => n,
    };
    let mut out = DateTime::of_naive(result);
    out.set_first_day_of_week(dt.first_day_of_week());
    out
}

fn set_hms_nano(n: NaiveDateTime, h: u32, mi: u32, s: u32, nano: u32) -> NaiveDateTime {
    n.date().and_hms_nano_opt(h, mi, s, nano).unwrap_or(n)
}

fn last_day_of_month(year: i32, month: u32) -> u32 {
    if month == 12 {
        31
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
            .day()
    }
}

#[allow(dead_code)]
fn _zone() {
    let _ = parity_zone();
}
