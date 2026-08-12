//! 对齐: `cn.hutool.core.date.Quarter`

#![allow(dead_code)]

use crate::date::month::Month;
use crate::{CoreError, Result};

/// 对齐 Java: `cn.hutool.core.date.Quarter`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Quarter {
    /// 第一季度
    Q1 = 1,
    /// 第二季度
    Q2 = 2,
    /// 第三季度
    Q3 = 3,
    /// 第四季度
    Q4 = 4,
}

impl Quarter {
    /// 对齐 Java: `Quarter.of(int)`。
    pub fn of(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Q1),
            2 => Some(Self::Q2),
            3 => Some(Self::Q3),
            4 => Some(Self::Q4),
            _ => None,
        }
    }

    /// 按名称解析（如 `Q1`）。
    pub fn value_of(name: &str) -> Option<Self> {
        match name.to_ascii_uppercase().as_str() {
            "Q1" => Some(Self::Q1),
            "Q2" => Some(Self::Q2),
            "Q3" => Some(Self::Q3),
            "Q4" => Some(Self::Q4),
            _ => None,
        }
    }

    /// 枚举名。
    pub fn name(self) -> &'static str {
        match self {
            Self::Q1 => "Q1",
            Self::Q2 => "Q2",
            Self::Q3 => "Q3",
            Self::Q4 => "Q4",
        }
    }

    /// 季度值 1-4。
    pub fn get_value(self) -> i32 {
        self as i32
    }

    /// 从月份（1-12）计算季度。
    pub fn from_month_value(month_value: i32) -> Result<Self> {
        if !(1..=12).contains(&month_value) {
            return Err(CoreError::InvalidArgument {
                name: "month",
                reason: "month must be 1..=12",
            });
        }
        Ok(Self::of((month_value - 1) / 3 + 1).unwrap())
    }

    /// 从 Hutool Month 计算季度。
    pub fn from_month(month: Month) -> Result<Self> {
        Self::from_month_value(month.get_value_base_one())
    }

    /// 季度首月（1-12）。
    pub fn first_month_value(self) -> i32 {
        (self.get_value() - 1) * 3 + 1
    }

    /// 季度末月（1-12）。
    pub fn last_month_value(self) -> i32 {
        self.get_value() * 3
    }

    /// 季度首月枚举。
    pub fn first_month(self) -> Month {
        Month::of_value(self.first_month_value() - 1).unwrap()
    }

    /// 季度末月枚举。
    pub fn last_month(self) -> Month {
        Month::of_value(self.last_month_value() - 1).unwrap()
    }

    /// 季度第一天（月,日），月从 1 起。
    pub fn first_month_day(self) -> (u32, u32) {
        (self.first_month_value() as u32, 1)
    }

    /// 季度最后一天（月,日）；非闰年二月按 28。
    pub fn last_month_day(self) -> (u32, u32) {
        let m = self.last_month();
        (m.get_value_base_one() as u32, m.get_last_day(false) as u32)
    }

    /// 对齐 Java: `plus(long)`。
    pub fn plus(self, quarters: i64) -> Self {
        let mut v = self.get_value() as i64 + quarters;
        v = ((v - 1).rem_euclid(4)) + 1;
        Self::of(v as i32).unwrap()
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
