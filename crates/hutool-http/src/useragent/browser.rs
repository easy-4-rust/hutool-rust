//! 对齐: `cn.hutool.http.useragent.Browser`
//! 来源: hutool-http/src/main/java/cn/hutool/http/useragent/Browser.java
//! 中文说明: 浏览器识别规则，通过正则匹配User-Agent字符串识别浏览器类型

use regex::Regex;
use std::{fmt, hash::{Hash, Hasher}};

use super::rule_error::RuleError;
use super::user_agent_info::UserAgentInfo;
use super::{case_insensitive_regex, capture, custom_browsers, is_mobile_browser_name, write_rules};

/// Browser identification rule.
#[derive(Debug, Clone)]
pub struct Browser {
    pub(crate) info: UserAgentInfo,
    version_pattern: Option<Regex>,
    pub(crate) mobile: bool,
}

impl Browser {
    /// Creates a browser rule.
    pub fn new(
        name: impl Into<String>,
        regex: Option<&str>,
        version_regex: Option<&str>,
    ) -> Result<Self, RuleError> {
        let name = name.into();
        Ok(Self {
            mobile: is_mobile_browser_name(&name),
            info: UserAgentInfo::new(name, regex)?,
            version_pattern: version_regex.map(case_insensitive_regex).transpose()?,
        })
    }

    /// Registers a browser rule after the built-in Hutool-compatible rules.
    pub fn add_custom_browser(
        name: impl Into<String>,
        regex: &str,
        version_regex: &str,
    ) -> Result<(), RuleError> {
        let browser = Self::new(name, Some(regex), Some(version_regex))?;
        write_rules(custom_browsers()).push(browser);
        Ok(())
    }

    /// Returns the browser name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.info.name()
    }

    /// Extracts the browser version.
    #[must_use]
    pub fn version(&self, user_agent: &str) -> Option<String> {
        capture(self.version_pattern.as_ref(), user_agent)
    }

    /// Returns whether the browser itself represents a mobile client.
    #[must_use]
    pub const fn is_mobile(&self) -> bool {
        self.mobile
    }

    /// Returns whether this browser is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.info.is_unknown()
    }
}

impl fmt::Display for Browser {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.info.fmt(formatter)
    }
}

impl PartialEq for Browser {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}

impl Eq for Browser {}

impl Hash for Browser {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info.hash(state);
    }
}
