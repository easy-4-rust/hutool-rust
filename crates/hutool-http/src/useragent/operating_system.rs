//! 对齐: `cn.hutool.http.useragent.OperatingSystem`
//! 来源: hutool-http/src/main/java/cn/hutool/http/useragent/OperatingSystem.java
//! 中文说明: 操作系统识别规则，通过正则匹配识别Windows、macOS、Linux等系统

use regex::Regex;
use std::{
    fmt,
    hash::{Hash, Hasher},
};

use super::rule_error::RuleError;
use super::user_agent_info::UserAgentInfo;
use super::{capture, case_insensitive_regex, custom_operating_systems, write_rules};

/// Operating-system identification rule.
#[derive(Debug, Clone)]
pub struct OperatingSystem {
    pub(crate) info: UserAgentInfo,
    version_pattern: Option<Regex>,
}

impl OperatingSystem {
    /// Creates an operating-system rule without version extraction.
    pub fn new(name: impl Into<String>, regex: Option<&str>) -> Result<Self, RuleError> {
        Self::with_version(name, regex, None)
    }

    /// Creates an operating-system rule with version extraction.
    pub fn with_version(
        name: impl Into<String>,
        regex: Option<&str>,
        version_regex: Option<&str>,
    ) -> Result<Self, RuleError> {
        Ok(Self {
            info: UserAgentInfo::new(name, regex)?,
            version_pattern: version_regex.map(case_insensitive_regex).transpose()?,
        })
    }

    /// Registers an operating-system rule after the built-in rules.
    pub fn add_custom_os(
        name: impl Into<String>,
        regex: &str,
        version_regex: &str,
    ) -> Result<(), RuleError> {
        let os = Self::with_version(name, Some(regex), Some(version_regex))?;
        write_rules(custom_operating_systems()).push(os);
        Ok(())
    }

    /// Returns the operating-system name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.info.name()
    }

    /// Extracts the operating-system version.
    #[must_use]
    pub fn version(&self, user_agent: &str) -> Option<String> {
        capture(self.version_pattern.as_ref(), user_agent).map(|version| version.replace('_', "."))
    }

    /// Returns whether this is macOS.
    #[must_use]
    pub fn is_macos(&self) -> bool {
        matches!(self.name(), "OSX" | "macOS" | "Mac OS X")
    }

    /// Returns whether this operating system is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.info.is_unknown()
    }
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.info.fmt(formatter)
    }
}

impl PartialEq for OperatingSystem {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}

impl Eq for OperatingSystem {}

impl Hash for OperatingSystem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info.hash(state);
    }
}
