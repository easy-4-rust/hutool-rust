//! 对齐: `cn.hutool.core.convert.Convert`
//! 来源: hutool-core/src/main/java/cn/hutool/core/convert/Convert.java

#![allow(dead_code, clippy::too_many_arguments)]

/// 对齐 Java `TimeUnit`
#[derive(Debug, Clone, Copy)]
pub enum TimeUnit {
    /// 纳秒
    Nanoseconds,
    /// 微秒
    Microseconds,
    /// 毫秒
    Milliseconds,
    /// 秒
    Seconds,
    /// 分钟
    Minutes,
    /// 小时
    Hours,
    /// 天
    Days,
}
