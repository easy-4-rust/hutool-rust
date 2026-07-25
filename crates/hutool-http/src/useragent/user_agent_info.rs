//! 对齐: `cn.hutool.http.useragent.UserAgentInfo`
//! 来源: hutool-http/src/main/java/cn/hutool/http/useragent/UserAgentInfo.java
//! 中文说明: User-Agent匹配规则，包含名称和正则表达式模式

use regex::{Regex, RegexBuilder};
use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::{OnceLock, RwLock},
};
use woothee::parser::Parser as WootheeParser;

use super::rule_error::RuleError;

/// Named User-Agent matching rule.
#[derive(Debug, Clone)]
pub struct UserAgentInfo {
    name: String,
    pattern: Option<Regex>,
}

impl UserAgentInfo {
    /// Creates a case-insensitive matching rule.
    pub fn new(name: impl Into<String>, regex: Option<&str>) -> Result<Self, RuleError> {
        Ok(Self {
            name: name.into(),
            pattern: regex.map(case_insensitive_regex).transpose()?,
        })
    }

    /// Creates an information object from an already compiled pattern.
    #[must_use]
    pub fn from_pattern(name: impl Into<String>, pattern: Option<Regex>) -> Self {
        Self {
            name: name.into(),
            pattern,
        }
    }

    /// Returns the display name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the compiled matching pattern, when one exists.
    #[must_use]
    pub const fn pattern(&self) -> Option<&Regex> {
        self.pattern.as_ref()
    }

    /// Returns whether this rule occurs in `content`.
    #[must_use]
    pub fn is_match(&self, content: &str) -> bool {
        self.pattern
            .as_ref()
            .is_some_and(|pattern| pattern.is_match(content))
    }

    /// Returns whether this represents an unidentified component.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.name == UNKNOWN_NAME
    }
}

impl PartialEq for UserAgentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for UserAgentInfo {}

impl Hash for UserAgentInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl fmt::Display for UserAgentInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)
    }
}

use super::{UNKNOWN_NAME, built_in_browsers, built_in_engines, built_in_operating_systems, built_in_platforms, capture, case_insensitive_regex, custom_browsers};
use super::{custom_operating_systems, engine_version, find_browser, find_engine, find_operating_system, find_platform, is_mobile_browser_name, is_mobile_platform_name};
use super::{read_rules, unknown_browser, unknown_engine, unknown_operating_system, unknown_platform, woothee_browser, woothee_operating_system, write_rules};
