//! 对齐: `cn.hutool.core.date.StopWatch`
//! 来源: hutool-core StopWatch（Spring Framework 风格秒表）

use std::fmt;
use std::time::{Duration, Instant};

/// 时间单位（对齐 Java `TimeUnit` 的常用子集）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeUnit {
    /// 纳秒
    Nanos,
    /// 微秒
    Micros,
    /// 毫秒
    Millis,
    /// 秒
    Seconds,
}

impl TimeUnit {
    /// 将纳秒时长换算为本单位数值。
    pub fn convert_from_nanos(self, nanos: u128) -> f64 {
        match self {
            Self::Nanos => nanos as f64,
            Self::Micros => nanos as f64 / 1_000.0,
            Self::Millis => nanos as f64 / 1_000_000.0,
            Self::Seconds => nanos as f64 / 1_000_000_000.0,
        }
    }

    /// 单位短名（prettyPrint 用）。
    pub fn short_name(self) -> &'static str {
        match self {
            Self::Nanos => "ns",
            Self::Micros => "µs",
            Self::Millis => "ms",
            Self::Seconds => "s",
        }
    }
}
