//! 对齐: `cn.hutool.cron.pattern.PatternUtil`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/pattern/PatternUtil.java
//! 中文说明: 从时间对象提取 cron 匹配用的七字段数组
//! `[秒, 分, 时, 日, 月, 周, 年]`。
//! Java 侧为包私有工具类（无公开方法），Rust 侧公开以对齐文件名与结构。

use chrono::{Datelike, NaiveDateTime, Timelike};

/// 对齐: `cn.hutool.cron.pattern.PatternUtil`
/// 中文说明: cron 七字段提取工具。
#[derive(Debug, Clone, Copy)]
pub struct PatternUtil;

impl PatternUtil {
    /// 对齐 Java: `getFields(LocalDateTime, boolean isMatchSecond)`
    /// 中文说明: 从本地日期时间提取七字段数组。
    ///
    /// - 秒：`is_match_second` 为真时取秒，否则为 `-1`（不匹配秒字段）
    /// - 月：从 1 开始（对齐 Java `getMonthValue()`）
    /// - 周：从 0 开始，0 表示周日，1-6 表示周一至周六（对齐 Java `getValue() - 1`）
    pub fn get_fields(datetime: NaiveDateTime, is_match_second: bool) -> [i32; 7] {
        let second = if is_match_second {
            // 秒字段范围 0..=59，u32 -> i32 安全
            i32::try_from(datetime.second()).unwrap_or(0)
        } else {
            -1
        };
        let minute = i32::try_from(datetime.minute()).unwrap_or(0);
        let hour = i32::try_from(datetime.hour()).unwrap_or(0);
        let day_of_month = i32::try_from(datetime.day()).unwrap_or(0);
        // 月份从 1 开始（对齐 Java `getMonthValue()`）
        let month = i32::try_from(datetime.month()).unwrap_or(0);
        // 星期从 0 开始：0 和 7 都表示周日（对齐 Java `getValue() - 1`）
        let day_of_week = i32::try_from(datetime.weekday().num_days_from_sunday()).unwrap_or(0);
        let year = datetime.year();
        [second, minute, hour, day_of_month, month, day_of_week, year]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn get_fields_matches_java_semantics() {
        // 2026-08-05 是周三；08:09:10
        let datetime = NaiveDate::from_ymd_opt(2026, 8, 5)
            .unwrap()
            .and_hms_opt(8, 9, 10)
            .unwrap();
        let fields = PatternUtil::get_fields(datetime, true);
        // [秒, 分, 时, 日, 月, 周, 年]
        assert_eq!(fields, [10, 9, 8, 5, 8, 3, 2026]);
        // 不匹配秒字段时秒为 -1
        let fields_no_second = PatternUtil::get_fields(datetime, false);
        assert_eq!(fields_no_second[0], -1);
        assert_eq!(fields_no_second[1..], [9, 8, 5, 8, 3, 2026]);
    }

    #[test]
    fn sunday_is_zero_based() {
        // 2026-08-02 是周日
        let sunday = NaiveDate::from_ymd_opt(2026, 8, 2)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(PatternUtil::get_fields(sunday, false)[5], 0);
        // 2026-08-03 是周一
        let monday = NaiveDate::from_ymd_opt(2026, 8, 3)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        assert_eq!(PatternUtil::get_fields(monday, false)[5], 1);
    }
}
