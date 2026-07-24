//! 对齐: `cn.hutool.core.date.BetweenFormatter`

#![allow(dead_code)]

use crate::date::date_unit::DateUnit;

use super::level::Level;

/// 对齐 Java: `cn.hutool.core.date.BetweenFormatter`
#[derive(Debug, Clone)]
pub struct BetweenFormatter {
    between_ms: i64,
    level: Level,
    level_max_count: i32,
    level_formatter: fn(Level) -> String,
    separator: String,
}

impl BetweenFormatter {
    /// 构造。
    pub fn new(between_ms: i64, level: Level, level_max_count: i32) -> Self {
        Self {
            between_ms,
            level,
            level_max_count,
            level_formatter: default_level_name,
            separator: String::new(),
        }
    }

    /// 设置英文等自定义单位格式。
    pub fn set_level_formatter(&mut self, f: fn(Level) -> String) {
        self.level_formatter = f;
    }

    /// 设置分隔符。
    pub fn set_separator(&mut self, sep: impl Into<String>) {
        self.separator = sep.into();
    }

    /// 对齐 Java: `BetweenFormatter.getBetweenMs()`
    pub fn get_between_ms(&self) -> i64 {
        self.between_ms
    }

    /// 对齐 Java: `BetweenFormatter.setBetweenMs(long)`
    pub fn set_between_ms(&mut self, between_ms: i64) {
        self.between_ms = between_ms;
    }

    /// 对齐 Java: `BetweenFormatter.getLevel()`
    pub fn get_level(&self) -> Level {
        self.level
    }

    /// 对齐 Java: `BetweenFormatter.setLevel(Level)`
    pub fn set_level(&mut self, level: Level) {
        self.level = level;
    }

    /// 对齐 Java: `BetweenFormatter(long, Level)` — levelMaxCount 不限。
    pub fn new_level(between_ms: i64, level: Level) -> Self {
        Self::new(between_ms, level, 0)
    }

    /// 格式化输出。
    pub fn format(&self) -> String {
        let between_ms = self.between_ms;
        let mut sb = String::new();
        if between_ms > 0 {
            let day = between_ms / DateUnit::Day.get_millis();
            let hour = between_ms / DateUnit::Hour.get_millis() - day * 24;
            let minute =
                between_ms / DateUnit::Minute.get_millis() - day * 24 * 60 - hour * 60;
            let between_of_second = ((day * 24 + hour) * 60 + minute) * 60;
            let second = between_ms / DateUnit::Second.get_millis() - between_of_second;
            let millisecond = between_ms - (between_of_second + second) * 1000;

            let level = self.level as i32;
            let mut level_count = 0i32;

            if self.is_level_count_valid(level_count) && day > 0 {
                sb.push_str(&format!(
                    "{}{}{}",
                    day,
                    (self.level_formatter)(Level::Day),
                    self.separator
                ));
                level_count += 1;
            }
            if self.is_level_count_valid(level_count) && hour != 0 && level >= Level::Hour as i32 {
                sb.push_str(&format!(
                    "{}{}{}",
                    hour,
                    (self.level_formatter)(Level::Hour),
                    self.separator
                ));
                level_count += 1;
            }
            if self.is_level_count_valid(level_count) && minute != 0 && level >= Level::Minute as i32
            {
                sb.push_str(&format!(
                    "{}{}{}",
                    minute,
                    (self.level_formatter)(Level::Minute),
                    self.separator
                ));
                level_count += 1;
            }
            if self.is_level_count_valid(level_count) && second != 0 && level >= Level::Second as i32
            {
                sb.push_str(&format!(
                    "{}{}{}",
                    second,
                    (self.level_formatter)(Level::Second),
                    self.separator
                ));
                level_count += 1;
            }
            if self.is_level_count_valid(level_count)
                && millisecond != 0
                && level >= Level::Millisecond as i32
            {
                sb.push_str(&format!(
                    "{}{}{}",
                    millisecond,
                    (self.level_formatter)(Level::Millisecond),
                    self.separator
                ));
            }
        }

        if sb.is_empty() {
            sb.push_str(&format!("0{}", (self.level_formatter)(self.level)));
        } else if !self.separator.is_empty() {
            let sep_len = self.separator.len();
            if sb.ends_with(&self.separator) {
                sb.truncate(sb.len() - sep_len);
            }
        }
        sb
    }

    fn is_level_count_valid(&self, level_count: i32) -> bool {
        self.level_max_count <= 0 || level_count < self.level_max_count
    }
}

impl std::fmt::Display for BetweenFormatter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.format())
    }
}

fn default_level_name(level: Level) -> String {
    level.get_name().to_string()
}
