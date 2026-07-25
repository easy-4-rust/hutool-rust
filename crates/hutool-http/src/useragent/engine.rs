//! 对齐: `cn.hutool.http.useragent.Engine`
//! 来源: hutool-http/src/main/java/cn/hutool/http/useragent/Engine.java
//! 中文说明: 渲染引擎识别规则，通过正则匹配识别WebKit、Gecko等引擎

use regex::{Regex, RegexBuilder};
use std::{
    fmt,
    hash::{Hash, Hasher},
    sync::{OnceLock, RwLock},
};
use woothee::parser::Parser as WootheeParser;

use super::rule_error::RuleError;
use super::user_agent_info::UserAgentInfo;

/// Rendering-engine identification rule.
#[derive(Debug, Clone)]
pub struct Engine {
    pub(crate) info: UserAgentInfo,
    version_name: Option<String>,
}

impl Engine {
    /// Creates an engine rule and its conventional version matcher.
    pub fn new(name: impl Into<String>, regex: Option<&str>) -> Result<Self, RuleError> {
        let name = name.into();
        let version_name = (name != UNKNOWN_NAME).then(|| name.clone());
        Ok(Self {
            info: UserAgentInfo::new(name, regex)?,
            version_name,
        })
    }

    /// Returns the engine name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.info.name()
    }

    /// Extracts the engine version.
    #[must_use]
    pub fn version(&self, user_agent: &str) -> Option<String> {
        engine_version(self.version_name.as_deref()?, user_agent)
    }

    /// Returns whether this engine is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        self.info.is_unknown()
    }
}

impl fmt::Display for Engine {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.info.fmt(formatter)
    }
}

impl PartialEq for Engine {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}

impl Eq for Engine {}

impl Hash for Engine {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.info.hash(state);
    }
}

use super::{UNKNOWN_NAME, built_in_browsers, built_in_engines, built_in_operating_systems, built_in_platforms, capture, case_insensitive_regex, custom_browsers};
use super::{custom_operating_systems, engine_version, find_browser, find_engine, find_operating_system, find_platform, is_mobile_browser_name, is_mobile_platform_name};
use super::{read_rules, unknown_browser, unknown_engine, unknown_operating_system, unknown_platform, woothee_browser, woothee_operating_system, write_rules};
