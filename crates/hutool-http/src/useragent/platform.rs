//! 对齐: `cn.hutool.http.useragent.Platform`
//! 来源: hutool-http/src/main/java/cn/hutool/http/useragent/Platform.java
//! 中文说明: 设备平台分类枚举，识别桌面、移动和平板设备类型

use std::fmt;

use super::is_mobile_platform_name;
use super::rule_error::RuleError;
use super::user_agent_info::UserAgentInfo;

/// Device platform classification.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Platform {
    pub(crate) info: UserAgentInfo,
    pub(crate) mobile: bool,
}

impl Platform {
    /// Creates a platform rule.
    pub fn new(name: impl Into<String>, regex: Option<&str>) -> Result<Self, RuleError> {
        let name = name.into();
        Ok(Self {
            mobile: is_mobile_platform_name(&name),
            info: UserAgentInfo::new(name, regex)?,
        })
    }

    /// Returns the platform name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.info.name()
    }

    /// Returns whether this is a mobile platform.
    #[must_use]
    pub const fn is_mobile(&self) -> bool {
        self.mobile
    }

    /// Returns whether this is an iPhone or iPod.
    #[must_use]
    pub fn is_iphone_or_ipod(&self) -> bool {
        matches!(self.name(), "iPhone" | "iPod")
    }

    /// Returns whether this is an iPad.
    #[must_use]
    pub fn is_ipad(&self) -> bool {
        self.name() == "iPad"
    }

    /// Returns whether this belongs to the iOS family.
    #[must_use]
    pub fn is_ios(&self) -> bool {
        self.is_iphone_or_ipod() || self.is_ipad()
    }

    /// Returns whether this belongs to the Android family.
    #[must_use]
    pub fn is_android(&self) -> bool {
        matches!(self.name(), "Android" | "GoogleTV")
    }

    /// Returns whether this is HarmonyOS/OpenHarmony.
    #[must_use]
    pub fn is_harmony(&self) -> bool {
        self.name() == "Harmony"
    }

    /// Returns whether this platform is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.info.is_unknown()
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.info.fmt(formatter)
    }
}
