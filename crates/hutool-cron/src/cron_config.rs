//! 对齐: `cn.hutool.cron.CronConfig`
//! 来源: hutool-cron/src/main/java/cn/hutool/cron/CronConfig.java
//! 中文说明: 调度器配置对象，封装时区与秒级匹配开关。
//!
//! Scheduler configuration.

/// 对齐: `cn.hutool.cron.CronConfig`
/// 中文说明: 调度器配置（时区、是否匹配秒等）。
///
/// Scheduler configuration.
#[derive(Debug, Clone)]
pub struct CronConfig {
    timezone: chrono::FixedOffset,
    match_second: bool,
}

impl Default for CronConfig {
    fn default() -> Self {
        Self {
            timezone: chrono::FixedOffset::east_opt(0).expect("UTC offset is valid"),
            match_second: false,
        }
    }
}

impl CronConfig {
    /// 中文说明: 设置固定时区偏移。
    /// 对齐 Java 方法: `setTimeZone`
    pub fn set_timezone(&mut self, timezone: chrono::FixedOffset) -> &mut Self {
        self.timezone = timezone;
        self
    }

    /// 中文说明: 返回固定时区偏移。
    /// 对齐 Java 方法: `getTimeZone`
    #[must_use]
    pub const fn timezone(&self) -> chrono::FixedOffset {
        self.timezone
    }

    /// 中文说明: 返回是否匹配秒字段。
    /// 对齐 Java 方法: `isMatchSecond`
    #[must_use]
    pub const fn is_match_second(&self) -> bool {
        self.match_second
    }

    /// 中文说明: 设置是否匹配秒字段。
    /// 对齐 Java 方法: `setMatchSecond`
    pub fn set_match_second(&mut self, match_second: bool) -> &mut Self {
        self.match_second = match_second;
        self
    }
}
